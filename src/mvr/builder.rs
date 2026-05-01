use std::{
    collections::HashMap,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use uuid::Uuid;

use crate::{
    CieColor, gdtf,
    mvr::{
        self, Mvr, NodeId, ObjectPath, Provider, Version,
        aux::{Class, MappingDefinition, Position, Symdef},
        bundle::{self, ResourceKey},
        geo::Geometry,
        layer::{
            Alignment, Connection, CustomCommand, DmxAddress, FixtureObject, FocusPointObject,
            GdtfInfo, Gobo, GroupObject, Layer, Mapping, NetworkAddress, Object, ObjectIdentifier,
            ObjectKind, Overwrite, Projection, ProjectorObject, Protocol, ScaleHandling,
            SceneObject, Source, SourceType, SupportObject, Transmission, TrussObject,
            VideoScreenObject,
        },
    },
};

pub struct MvrBuilder {
    bundle: bundle::Bundle,
}

impl MvrBuilder {
    pub fn new(bundle: bundle::Bundle) -> Self {
        Self { bundle }
    }

    pub fn build(self) -> Mvr {
        let version = Version {
            major: self.bundle.description().ver_major as u32,
            minor: self.bundle.description().ver_minor as u32,
        };

        let provider = Provider {
            name: self.bundle.description().provider.clone().unwrap_or_default(),
            version: self.bundle.description().provider_version.clone().unwrap_or_default(),
        };

        let mut symdefs = HashMap::new();
        let mut classes = HashMap::new();
        let mut positions = HashMap::new();
        let mut layers = Vec::new();
        let mut layers_ix = HashMap::new();
        let mut objects_path_ix = HashMap::new();
        let mut mapping_definitions = HashMap::new();

        static EMPTY_AUX_DATA: bundle::AuxData = bundle::AuxData {
            class: Vec::new(),
            symdef: Vec::new(),
            position: Vec::new(),
            mapping_definition: Vec::new(),
        };
        let aux_data = self.bundle.description().scene.aux_data.as_ref().unwrap_or(&EMPTY_AUX_DATA);

        Self::build_classes(aux_data, &mut classes);
        Self::build_positions(aux_data, &mut positions);
        Self::build_symdefs(aux_data, &mut symdefs);
        Self::build_mapping_definitions(aux_data, &mut mapping_definitions);
        Self::build_layers(&self.bundle, &classes, aux_data, &mut layers);

        let gdtfs = Self::build_gdtfs(&self.bundle);

        for (layer_ix, layer) in layers.iter().enumerate() {
            layers_ix.insert(layer.id(), layer_ix);

            for (object_ix, object) in layer.objects().iter().enumerate() {
                Self::index_object_paths(layer.id(), vec![object_ix], object, &mut objects_path_ix);
            }
        }

        Mvr {
            bundle: self.bundle,
            version,
            provider,
            symdefs,
            classes,
            mapping_definitions,
            positions,
            layers,
            layers_ix,
            objects_path_ix,
            gdtfs,
        }
    }

    fn index_object_paths(
        layer_id: NodeId<Layer>,
        indices: Vec<usize>,
        object: &Object,
        objects_path_ix: &mut HashMap<NodeId<Object>, ObjectPath>,
    ) {
        objects_path_ix.insert(object.id(), ObjectPath::new(layer_id, indices.clone()));

        if let Some(children) = object.child_objects() {
            for (child_ix, child) in children.iter().enumerate() {
                let mut child_indices = indices.clone();
                child_indices.push(child_ix);
                Self::index_object_paths(layer_id, child_indices, child, objects_path_ix);
            }
        }
    }

    fn build_classes(aux_data: &bundle::AuxData, classes: &mut HashMap<NodeId<Class>, Class>) {
        for class in &aux_data.class {
            let id: NodeId<Class> = Uuid::parse_str(&class.uuid).unwrap().into();
            classes.insert(id, Class { name: class.name.clone(), id });
        }
    }

    fn build_positions(
        aux_data: &bundle::AuxData,
        positions: &mut HashMap<NodeId<Position>, Position>,
    ) {
        for position in &aux_data.position {
            let id: NodeId<Position> = Uuid::parse_str(&position.uuid).unwrap().into();
            positions.insert(id, Position { name: position.name.clone(), id });
        }
    }

    fn build_symdefs(aux_data: &bundle::AuxData, symdefs: &mut HashMap<NodeId<Symdef>, Symdef>) {
        for symdef in &aux_data.symdef {
            let id: NodeId<Symdef> = Uuid::parse_str(&symdef.uuid).unwrap().into();
            symdefs.insert(id, Self::build_symdef(&symdef.uuid, aux_data));
        }
    }

    fn build_mapping_definitions(
        aux_data: &bundle::AuxData,
        mapping_definitions: &mut HashMap<NodeId<MappingDefinition>, MappingDefinition>,
    ) {
        for mapping_definition in &aux_data.mapping_definition {
            let id: NodeId<MappingDefinition> =
                Uuid::parse_str(&mapping_definition.uuid).unwrap().into();
            mapping_definitions
                .insert(id, Self::build_mapping_definition(&mapping_definition.uuid, aux_data));
        }
    }

    fn build_symdef(symdef_uuid: &str, aux_data: &bundle::AuxData) -> Symdef {
        let symdef = aux_data.symdef.iter().find(|s| s.uuid == symdef_uuid).unwrap();
        let id: NodeId<Symdef> = Uuid::parse_str(symdef_uuid).unwrap().into();
        let geometries = Self::build_geometries(
            &symdef.child_list.geometry_3d,
            &symdef.child_list.symbol,
            aux_data,
        );
        Symdef { name: symdef.name.clone(), id, geometries }
    }

    fn build_geometries(
        geometry_3ds: &[bundle::Geometry3D],
        symbols: &[bundle::Symbol],
        aux_data: &bundle::AuxData,
    ) -> Vec<Geometry> {
        let mut geometries = Vec::new();

        for geo3d in geometry_3ds {
            geometries.push(Geometry {
                local_transform: build_transform_optional(geo3d.matrix.as_deref()),
                model: bundle::ResourceKey::new(&geo3d.file_name),
            });
        }

        for symbol in symbols {
            let symbol_transform = build_transform_optional(symbol.matrix.as_deref());
            let nested_symdef = Self::build_symdef(&symbol.symdef, aux_data);
            for mut geo in nested_symdef.geometries {
                geo.local_transform *= symbol_transform;
                geometries.push(geo);
            }
        }

        geometries
    }

    fn build_mapping_definition(
        mapping_definition_uuid: &str,
        aux_data: &bundle::AuxData,
    ) -> MappingDefinition {
        let md = aux_data
            .mapping_definition
            .iter()
            .find(|md| md.uuid == mapping_definition_uuid)
            .unwrap();

        let source = Source {
            linked_geometry: gdtf::Node::from_str(&md.source.linked_geometry).unwrap(),
            r#type: match md.source.r#type {
                bundle::SourceType::Ndi => SourceType::Ndi,
                bundle::SourceType::File => SourceType::File,
                bundle::SourceType::Citp => SourceType::Citp,
                bundle::SourceType::CaptureDevice => SourceType::CaptureDevice,
            },
            value: md.source.content.clone(),
        };
        let scale_handling = match &md.scale_handeling {
            Some(sh) => match sh.r#enum {
                bundle::Scale::ScaleKeepRatio => ScaleHandling::ScaleKeepRatio,
                bundle::Scale::ScaleIgnoreRatio => ScaleHandling::ScaleIgnoreRatio,
                bundle::Scale::KeepSizeCenter => ScaleHandling::KeepSizeCenter,
            },
            None => ScaleHandling::default(),
        };

        MappingDefinition {
            name: md.name.clone(),
            id: Uuid::parse_str(mapping_definition_uuid).unwrap().into(),
            size_x: md.size_x as u32,
            size_y: md.size_y as u32,
            source,
            scale_handling,
        }
    }

    fn build_layers(
        bundle: &bundle::Bundle,
        classes: &HashMap<NodeId<Class>, Class>,
        aux_data: &bundle::AuxData,
        layers: &mut Vec<Layer>,
    ) {
        for layer_data in &bundle.description().scene.layers.layer {
            let uuid: Uuid = Uuid::from_str(&layer_data.uuid).unwrap();
            let objects = layer_data
                .child_list
                .as_ref()
                .map(|cl| {
                    cl.content
                        .iter()
                        .map(|child| Self::build_object(child, classes, aux_data))
                        .collect()
                })
                .unwrap_or_default();

            layers.push(Layer {
                id: uuid.into(),
                name: layer_data.name.clone(),
                local_transform: build_transform_optional(layer_data.matrix.as_deref()),
                objects,
            });
        }
    }

    fn build_object(
        child: &bundle::ChildListContent,
        classes: &HashMap<NodeId<Class>, Class>,
        aux_data: &bundle::AuxData,
    ) -> Object {
        let (uuid_str, name, matrix, classing, kind) = match child {
            bundle::ChildListContent::SceneObject(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::SceneObject(Self::build_scene_object(c, classes, aux_data)),
            ),
            bundle::ChildListContent::GroupObject(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::GroupObject(Self::build_group_object(c, classes, aux_data)),
            ),
            bundle::ChildListContent::FocusPoint(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::FocusPoint(Self::build_focus_point_object(c, aux_data)),
            ),
            bundle::ChildListContent::Fixture(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::Fixture(Self::build_fixture_object(c, aux_data)),
            ),
            bundle::ChildListContent::Support(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::Support(Self::build_support_object(c, aux_data)),
            ),
            bundle::ChildListContent::Truss(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::Truss(Self::build_truss_object(c, aux_data)),
            ),
            bundle::ChildListContent::VideoScreen(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::VideoScreen(Self::build_video_screen_object(c, aux_data)),
            ),
            bundle::ChildListContent::Projector(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::Projector(Self::build_projector_object(c, aux_data)),
            ),
        };

        Object {
            id: Uuid::from_str(uuid_str).unwrap().into(),
            name: name.to_string(),
            class: classing
                .map(|id| Uuid::from_str(id).unwrap())
                .and_then(|id| classes.get(&id.into()))
                .map(|class| class.id()),
            local_transform: build_transform_optional(matrix),
            kind,
        }
    }

    fn build_child_objects(
        child_list: Option<&bundle::ChildList>,
        classes: &HashMap<NodeId<Class>, Class>,
        aux_data: &bundle::AuxData,
    ) -> Vec<Object> {
        child_list
            .map(|cl| {
                cl.content
                    .iter()
                    .map(|child| Self::build_object(child, classes, aux_data))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn build_scene_object(
        c: &bundle::SceneObject,
        classes: &HashMap<NodeId<Class>, Class>,
        aux_data: &bundle::AuxData,
    ) -> SceneObject {
        // FIXME: If invalid UUID, error instead of falling back to Single.
        let id = Self::build_id_from_multipatch(
            &c.multipatch,
            c.fixture_id.to_owned(),
            c.fixture_id_numeric,
            c.custom_id,
            c.custom_id_type,
        );

        SceneObject {
            id,
            gdtf: Self::build_gdtf_info(&c.gdtf_spec, &c.gdtf_mode),
            cast_shadow: c.cast_shadow.unwrap_or_default(),
            unit_number: c.unit_number,
            dmx_addresses: Self::build_dmx_addresses(c.addresses.as_ref()),
            network_addresses: Self::build_network_addresses(c.addresses.as_ref()),
            alignments: Self::build_alignments(c.alignments.as_ref()),
            custom_commands: Self::build_custom_commands(c.custom_commands.as_ref()),
            overwrites: Self::build_overwrites(c.overwrites.as_ref()),
            connections: Self::build_connections(c.connections.as_ref()),
            geometries: Self::build_geometries(
                &c.geometries.geometry_3d,
                &c.geometries.symbol,
                aux_data,
            ),
            child_objects: Self::build_child_objects(c.child_list.as_deref(), classes, aux_data),
        }
    }

    fn build_group_object(
        c: &bundle::GroupObject,
        classes: &HashMap<NodeId<Class>, Class>,
        aux_data: &bundle::AuxData,
    ) -> GroupObject {
        GroupObject {
            child_objects: c
                .child_list
                .content
                .iter()
                .map(|child| Self::build_object(child, classes, aux_data))
                .collect(),
        }
    }

    fn build_focus_point_object(
        c: &bundle::FocusPoint,
        aux_data: &bundle::AuxData,
    ) -> FocusPointObject {
        FocusPointObject {
            geometries: Self::build_geometries(
                &c.geometries.geometry_3d,
                &c.geometries.symbol,
                aux_data,
            ),
        }
    }

    fn build_fixture_object(c: &bundle::Fixture, aux_data: &bundle::AuxData) -> FixtureObject {
        FixtureObject {
            id: Self::build_id_from_multipatch(
                &c.multipatch,
                Some(c.fixture_id.to_owned()),
                c.fixture_id_numeric,
                c.custom_id,
                c.custom_id_type,
            ),
            gdtf: Self::build_gdtf_info(&c.gdtf_spec, &c.gdtf_mode),
            cast_shadow: c.cast_shadow.unwrap_or_default(),
            child_position: c.child_position.as_ref().map(|s| gdtf::Node::from_str(s).unwrap()),
            color: c.color.as_ref().map(|s| CieColor::from_str(s).unwrap()),
            dmx_invert_pan: c.dmx_invert_pan.unwrap_or(false),
            dmx_invert_tilt: c.dmx_invert_tilt.unwrap_or(false),
            focus: c.focus.as_ref().map(|s| mvr::NodeId::from_str(s).unwrap()),
            function: c.function.clone(),
            gobo: c.gobo.as_ref().map(|gobo| Gobo {
                resource: ResourceKey::new(&gobo.file_name),
                rotation: gobo.rotation,
            }),
            mappings: c
                .mappings
                .as_ref()
                .map(|mappings| {
                    mappings
                        .mapping
                        .iter()
                        .map(|m| Mapping {
                            linked_def: NodeId::from_str(&m.linked_def).unwrap(),
                            ux: m.ux.unwrap_or_default(),
                            uy: m.uy.unwrap_or_default(),
                            ox: m.ox.unwrap_or_default(),
                            oy: m.oy.unwrap_or_default(),
                            rz: m.rz.unwrap_or_default(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            position: c.position.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            protocols: c
                .protocols
                .as_ref()
                .map(|protocols| {
                    protocols
                        .protocol
                        .iter()
                        .map(|protocol| Protocol {
                            geometry: gdtf::Node::from_str(&protocol.geometry).unwrap_or_else(
                                |_| gdtf::Node::from_str("NetworkInOut_1").unwrap(),
                            ),
                            name: protocol.name.clone(),
                            r#type: if protocol.r#type.is_empty() {
                                None
                            } else {
                                Some(protocol.r#type.clone())
                            },
                            version: if protocol.version.is_empty() {
                                None
                            } else {
                                Some(protocol.version.clone())
                            },
                            transmission: protocol.transmission.as_ref().map(|t| match t {
                                bundle::Transmission::Unicast => Transmission::Unicast,
                                bundle::Transmission::Multicast => Transmission::Multicast,
                                bundle::Transmission::Broadcast => Transmission::Broadcast,
                                bundle::Transmission::Anycast => Transmission::Anycast,
                            }),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            unit_number: Some(c.unit_number),
            dmx_addresses: Self::build_dmx_addresses(c.addresses.as_ref()),
            network_addresses: Self::build_network_addresses(c.addresses.as_ref()),
            alignments: Self::build_alignments(c.alignments.as_ref()),
            custom_commands: Self::build_custom_commands(c.custom_commands.as_ref()),
            overwrites: Self::build_overwrites(c.overwrites.as_ref()),
            connections: Self::build_connections(c.connections.as_ref()),
            child_objects: c
                .child_list
                .as_ref()
                .map(|cl| {
                    cl.content
                        .iter()
                        .map(|child| Self::build_object(child, &HashMap::new(), aux_data))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn build_support_object(c: &bundle::Support, aux_data: &bundle::AuxData) -> SupportObject {
        SupportObject {
            gdtf: Self::build_gdtf_info(&c.gdtf_spec, &c.gdtf_mode),
            id: Self::build_id_from_multipatch(
                &c.multipatch,
                Some(c.fixture_id.to_owned()),
                c.fixture_id_numeric,
                c.custom_id,
                c.custom_id_type,
            ),
            cast_shadow: c.cast_shadow.unwrap_or_default(),
            chain_length: c.chain_length,
            function: c.function.clone(),
            position: c.position.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            unit_number: c.unit_number,
            dmx_addresses: Self::build_dmx_addresses(c.addresses.as_ref()),
            network_addresses: Self::build_network_addresses(c.addresses.as_ref()),
            alignments: Self::build_alignments(c.alignments.as_ref()),
            custom_commands: Self::build_custom_commands(c.custom_commands.as_ref()),
            overwrites: Self::build_overwrites(c.overwrites.as_ref()),
            connections: Self::build_connections(c.connections.as_ref()),
            geometries: Self::build_geometries(
                &c.geometries.geometry_3d,
                &c.geometries.symbol,
                aux_data,
            ),
            child_objects: c
                .child_list
                .as_ref()
                .map(|cl| {
                    cl.content
                        .iter()
                        .map(|child| Self::build_object(child, &HashMap::new(), aux_data))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn build_truss_object(c: &bundle::Truss, aux_data: &bundle::AuxData) -> TrussObject {
        TrussObject {
            gdtf: Self::build_gdtf_info(&c.gdtf_spec, &c.gdtf_mode),
            id: Self::build_id_from_multipatch(
                &c.multipatch,
                Some(c.fixture_id.to_owned()),
                c.fixture_id_numeric,
                c.custom_id,
                c.custom_id_type,
            ),
            cast_shadow: c.cast_shadow.unwrap_or_default(),
            child_position: c.child_position.as_ref().map(|s| gdtf::Node::from_str(s).unwrap()),
            function: c.function.clone(),
            position: c.position.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            unit_number: c.unit_number,
            dmx_addresses: Self::build_dmx_addresses(c.addresses.as_ref()),
            network_addresses: Self::build_network_addresses(c.addresses.as_ref()),
            alignments: Self::build_alignments(c.alignments.as_ref()),
            custom_commands: Self::build_custom_commands(c.custom_commands.as_ref()),
            overwrites: Self::build_overwrites(c.overwrites.as_ref()),
            connections: Self::build_connections(c.connections.as_ref()),
            geometries: Self::build_geometries(
                &c.geometries.geometry_3d,
                &c.geometries.symbol,
                aux_data,
            ),
            child_objects: c
                .child_list
                .as_ref()
                .map(|cl| {
                    cl.content
                        .iter()
                        .map(|child| Self::build_object(child, &HashMap::new(), aux_data))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn build_video_screen_object(
        c: &bundle::VideoScreen,
        aux_data: &bundle::AuxData,
    ) -> VideoScreenObject {
        VideoScreenObject {
            gdtf: Self::build_gdtf_info(&c.gdtf_spec, &c.gdtf_mode),
            id: Self::build_id_from_multipatch(
                &c.multipatch,
                Some(c.fixture_id.to_owned()),
                c.fixture_id_numeric,
                c.custom_id,
                c.custom_id_type,
            ),
            function: c.function.clone(),
            sources: c
                .sources
                .as_ref()
                .map(|sources| {
                    sources
                        .source
                        .iter()
                        .map(|s| Source {
                            linked_geometry: gdtf::Node::from_str(&s.linked_geometry).unwrap(),
                            r#type: match s.r#type {
                                bundle::SourceType::Ndi => SourceType::Ndi,
                                bundle::SourceType::File => SourceType::File,
                                bundle::SourceType::Citp => SourceType::Citp,
                                bundle::SourceType::CaptureDevice => SourceType::CaptureDevice,
                            },
                            value: s.content.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            dmx_addresses: Self::build_dmx_addresses(c.addresses.as_ref()),
            network_addresses: Self::build_network_addresses(c.addresses.as_ref()),
            alignments: Self::build_alignments(c.alignments.as_ref()),
            custom_commands: Self::build_custom_commands(c.custom_commands.as_ref()),
            overwrites: Self::build_overwrites(c.overwrites.as_ref()),
            connections: Self::build_connections(c.connections.as_ref()),
            cast_shadow: c.cast_shadow.unwrap_or_default(),
            geometries: Self::build_geometries(
                &c.geometries.geometry_3d,
                &c.geometries.symbol,
                aux_data,
            ),
            child_objects: c
                .child_list
                .as_ref()
                .map(|cl| {
                    cl.content
                        .iter()
                        .map(|child| Self::build_object(child, &HashMap::new(), aux_data))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn build_projector_object(
        c: &bundle::Projector,
        aux_data: &bundle::AuxData,
    ) -> ProjectorObject {
        ProjectorObject {
            gdtf: Self::build_gdtf_info(&c.gdtf_spec, &c.gdtf_mode),
            id: Self::build_id_from_multipatch(
                &c.multipatch,
                Some(c.fixture_id.to_owned()),
                c.fixture_id_numeric,
                c.custom_id,
                c.custom_id_type,
            ),
            cast_shadow: c.cast_shadow.unwrap_or_default(),
            projections: c
                .projections
                .projection
                .iter()
                .filter_map(|p| {
                    let source = p.source.first().map(|s| Source {
                        linked_geometry: gdtf::Node::from_str(&s.linked_geometry).unwrap(),
                        r#type: match s.r#type {
                            bundle::SourceType::Ndi => SourceType::Ndi,
                            bundle::SourceType::File => SourceType::File,
                            bundle::SourceType::Citp => SourceType::Citp,
                            bundle::SourceType::CaptureDevice => SourceType::CaptureDevice,
                        },
                        value: s.content.clone(),
                    })?;

                    let scale_handling = p
                        .scale_handeling
                        .first()
                        .map(|sh| match sh.r#enum {
                            bundle::Scale::ScaleKeepRatio => ScaleHandling::ScaleKeepRatio,
                            bundle::Scale::ScaleIgnoreRatio => ScaleHandling::ScaleIgnoreRatio,
                            bundle::Scale::KeepSizeCenter => ScaleHandling::KeepSizeCenter,
                        })
                        .unwrap_or_default();

                    Some(Projection { source, scale_handling })
                })
                .collect(),
            unit_number: c.unit_number,
            dmx_addresses: Self::build_dmx_addresses(c.addresses.as_ref()),
            network_addresses: Self::build_network_addresses(c.addresses.as_ref()),
            alignments: Self::build_alignments(c.alignments.as_ref()),
            custom_commands: Self::build_custom_commands(c.custom_commands.as_ref()),
            overwrites: Self::build_overwrites(c.overwrites.as_ref()),
            connections: Self::build_connections(c.connections.as_ref()),
            geometries: Self::build_geometries(
                &c.geometries.geometry_3d,
                &c.geometries.symbol,
                aux_data,
            ),
            child_objects: c
                .child_list
                .as_ref()
                .map(|cl| {
                    cl.content
                        .iter()
                        .map(|child| Self::build_object(child, &HashMap::new(), aux_data))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn build_dmx_addresses(addresses: Option<&bundle::Addresses>) -> Vec<DmxAddress> {
        addresses
            .map(|addrs| {
                addrs
                    .address
                    .iter()
                    .map(|addr| {
                        let absolute_value = if let Some(dot) = addr.content.find('.') {
                            // "<universe>.<channel>"
                            let (universe_str, channel_str) = addr.content.split_at(dot);
                            let universe = universe_str.parse::<u32>().unwrap();
                            let channel = channel_str[1..].parse::<u32>().unwrap(); // Skip Dot.
                            (universe - 1) * 512 + channel
                        } else {
                            // "<absolute_channel>"
                            addr.content.parse::<u32>().unwrap()
                        };

                        DmxAddress { r#break: addr.r#break as u32, absolute_value }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn build_network_addresses(addresses: Option<&bundle::Addresses>) -> Vec<NetworkAddress> {
        addresses
            .map(|addrs| {
                addrs
                    .network
                    .iter()
                    .map(|addr| NetworkAddress {
                        geometry: gdtf::Node::from_str(&addr.geometry).unwrap(),
                        ipv4: addr.ipv_4.as_ref().map(|s| Ipv4Addr::from_str(&s).unwrap()),
                        subnetmask: addr
                            .subnetmask
                            .as_ref()
                            .map(|s| Ipv4Addr::from_str(&s).unwrap()),
                        ipv6: addr.ipv_6.as_ref().map(|s| Ipv6Addr::from_str(&s).unwrap()),
                        dhcp: addr.dhcp.as_ref().is_some_and(|s| s == "on"),
                        hostname: addr.hostname.to_owned(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn build_alignments(alignments: Option<&bundle::Alignments>) -> Vec<Alignment> {
        alignments
            .map(|alignments| {
                alignments
                    .alignment
                    .iter()
                    .map(|alignment| Alignment {
                        // FIXME: The XSD says geometry is optional. decide how to handle None.
                        geometry: gdtf::Node::from_str(
                            alignment.geometry.as_ref().expect("FIXME: handle missing geometry"),
                        )
                        .unwrap(),
                        up: parse_vec3(&alignment.up),
                        direction: parse_vec3(&alignment.direction),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn build_custom_commands(commands: Option<&bundle::CustomCommands>) -> Vec<CustomCommand> {
        commands
            .map(|commands| {
                commands
                    .custom_command
                    .iter()
                    .map(|command| CustomCommand(command.to_owned()))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn build_overwrites(overwrites: Option<&bundle::Overwrites>) -> Vec<Overwrite> {
        overwrites
            .map(|ows| {
                ows.overwrite
                    .iter()
                    .map(|ow| Overwrite {
                        universal: gdtf::Node::from_str(&ow.universal).unwrap(),
                        target: Some(gdtf::Node::from_str(&ow.target).unwrap()),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn build_connections(connections: Option<&bundle::Connections>) -> Vec<Connection> {
        connections
            .map(|conns| {
                conns
                    .connection
                    .iter()
                    .map(|conn| Connection {
                        own: gdtf::Node::from_str(&conn.own).unwrap(),
                        other: gdtf::Node::from_str(&conn.other).unwrap(),
                        to_object: mvr::NodeId::from_str(&conn.to_object).unwrap(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn build_gdtf_info(gdtf_spec: &Option<String>, gdtf_mode: &Option<String>) -> Option<GdtfInfo> {
        match (gdtf_spec, gdtf_mode) {
            (Some(spec), Some(mode)) => Some(GdtfInfo::new(spec.to_owned(), mode.to_owned())),
            _ => None,
        }
    }

    fn build_id_from_multipatch(
        multipatch: &str,
        fixture_id: Option<String>,
        fixture_id_numeric: Option<i32>,
        custom_id: Option<i32>,
        custom_id_type: Option<i32>,
    ) -> ObjectIdentifier {
        match Uuid::from_str(multipatch) {
            Ok(uuid) => ObjectIdentifier::Multipatch(uuid.into()),
            Err(_) => ObjectIdentifier::Single {
                fixture_id,
                fixture_id_numeric,
                custom_id,
                custom_id_type,
            },
        }
    }

    fn build_gdtfs(bundle: &bundle::Bundle) -> HashMap<bundle::ResourceKey, gdtf::Gdtf> {
        bundle
            .resources()
            .entries()
            .filter(|e| e.kind() == bundle::ResourceKind::Gdtf)
            .map(|e| {
                let path = bundle.root_folder().join(e.key().relative_path());
                (e.key().clone(), gdtf::Gdtf::from_archive(path))
            })
            .collect()
    }
}

fn build_transform_optional(s: Option<&str>) -> glam::Affine3A {
    s.map(build_transform).unwrap_or(glam::Affine3A::IDENTITY)
}

fn build_transform(s: &str) -> glam::Affine3A {
    let rows = s
        .split('}')
        .filter_map(|row| {
            let row = row.trim_start_matches('{').trim();
            if row.is_empty() { None } else { Some(row) }
        })
        .collect::<Vec<_>>();

    assert!(rows.len() == 4, "Matrix string must have 4 rows");

    let parse_row =
        |row: &str| row.split(',').map(|v| v.trim().parse::<f32>().unwrap()).collect::<Vec<_>>();

    let m1 = parse_row(rows[0]);
    let m2 = parse_row(rows[1]);
    let m3 = parse_row(rows[2]);
    let t = parse_row(rows[3]);

    assert!(m1.len() == 3 && m2.len() == 3 && m3.len() == 3 && t.len() == 3);

    #[rustfmt::skip]
    let cols_array = [
        m1[0], m1[1], m1[2],
        m2[0], m2[1], m2[2],
        m3[0], m3[1], m3[2],
        t[0] / 1000.0, t[1] / 1000.0, t[2] / 1000.0,
    ];

    glam::Affine3A::from_cols_array(&cols_array)
}

fn parse_vec3(s: &str) -> glam::Vec3A {
    let vals: Vec<f32> = s.split(',').map(|v| v.trim().parse().unwrap()).collect();
    glam::Vec3A::new(vals[0], vals[1], vals[2])
}
