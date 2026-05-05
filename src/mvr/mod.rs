use std::{
    collections::HashMap,
    fmt,
    path::PathBuf,
    str::{self, FromStr},
};

use uuid::Uuid;

use crate::{CieColor, gdtf};

pub mod aux;
pub mod bundle;
pub mod geo;
pub mod layer;

pub use aux::{Class, MappingDefinition, Position, Symdef};
pub use geo::Geometry;
pub use layer::{GdtfInfo, Layer, Object, ObjectKind};

pub struct Mvr {
    bundle: bundle::Bundle,

    version: Version,
    provider: Provider,

    symdefs: HashMap<NodeId<Symdef>, Symdef>,
    classes: HashMap<NodeId<Class>, Class>,
    mapping_definitions: HashMap<NodeId<MappingDefinition>, MappingDefinition>,
    positions: HashMap<NodeId<Position>, Position>,

    layers: Vec<Layer>,
    layers_ix: HashMap<NodeId<Layer>, usize>,
    objects_path_ix: HashMap<NodeId<Object>, ObjectPath>,

    gdtfs: HashMap<bundle::ResourceKey, gdtf::Gdtf>,
}

impl Mvr {
    pub fn new(bundle: bundle::Bundle) -> Self {
        bundle.into()
    }

    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        Self::new(bundle::Bundle::from_folder(path))
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        Self::new(bundle::Bundle::from_archive(path))
    }

    pub fn bundle(&self) -> &bundle::Bundle {
        &self.bundle
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    pub fn symdefs(&self) -> impl Iterator<Item = &Symdef> {
        self.symdefs.values()
    }

    pub fn symdef(&self, id: NodeId<Symdef>) -> Option<&Symdef> {
        self.symdefs.get(&id)
    }

    pub fn classes(&self) -> impl Iterator<Item = &Class> {
        self.classes.values()
    }

    pub fn class(&self, id: NodeId<Class>) -> Option<&Class> {
        self.classes.get(&id)
    }

    pub fn positions(&self) -> impl Iterator<Item = &Position> {
        self.positions.values()
    }

    pub fn position(&self, id: NodeId<Position>) -> Option<&Position> {
        self.positions.get(&id)
    }

    pub fn mapping_definitions(&self) -> impl Iterator<Item = &MappingDefinition> {
        self.mapping_definitions.values()
    }

    pub fn mapping_definition(&self, id: NodeId<MappingDefinition>) -> Option<&MappingDefinition> {
        self.mapping_definitions.get(&id)
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    pub fn layer(&self, id: NodeId<Layer>) -> Option<&Layer> {
        let layer_ix = *self.layers_ix.get(&id)?;
        Some(&self.layers[layer_ix])
    }

    pub fn object(&self, id: NodeId<Object>) -> Option<&Object> {
        let path = self.objects_path_ix.get(&id)?;
        self.object_by_path(path)
    }

    pub(crate) fn object_path(&self, id: NodeId<Object>) -> Option<&ObjectPath> {
        self.objects_path_ix.get(&id)
    }

    pub(crate) fn object_by_path(&self, path: &ObjectPath) -> Option<&Object> {
        let layer_ix = *self.layers_ix.get(&path.layer_id)?;
        let layer = self.layers.get(layer_ix)?;

        let mut indices = path.indices.iter();
        let first = *indices.next()?;
        let mut object = layer.objects.get(first)?;

        for &ix in indices {
            let child_objects = object.child_objects()?;
            object = child_objects.get(ix)?;
        }

        Some(object)
    }

    pub fn object_world_transform(&self, id: NodeId<Object>) -> Option<glam::Affine3A> {
        let path = self.object_path(id)?;
        self.object_world_transform_by_path(path)
    }

    fn object_world_transform_by_path(&self, path: &ObjectPath) -> Option<glam::Affine3A> {
        let layer_ix = *self.layers_ix.get(&path.layer_id)?;
        let layer = self.layers.get(layer_ix)?;

        let mut indices = path.indices.iter();
        let first = *indices.next()?;
        let mut object = layer.objects.get(first)?;
        let mut transform = *layer.local_transform() * *object.local_transform();

        for &ix in indices {
            let child_objects = object.child_objects()?;
            object = child_objects.get(ix)?;
            transform = transform * *object.local_transform();
        }

        Some(transform)
    }

    pub fn object_geometries_world<'a>(
        &'a self,
        id: NodeId<Object>,
    ) -> Option<impl Iterator<Item = (&'a geo::Geometry, glam::Affine3A)> + 'a> {
        let path = self.object_path(id)?;
        let world = self.object_world_transform_by_path(path)?;
        let object = self.object_by_path(path)?;
        let geometries = object.geometries()?;

        Some(geometries.iter().map(move |g| (g, world * g.local_transform())))
    }

    pub fn gdtfs(&self) -> impl Iterator<Item = (&bundle::ResourceKey, &gdtf::Gdtf)> {
        self.gdtfs.iter()
    }

    pub fn gdtf(&self, file_name: &str) -> Option<&gdtf::Gdtf> {
        let key = self.gdtf_resource_key(file_name)?;
        self.gdtfs.get(&key)
    }

    fn gdtf_resource_key(&self, gdtf_spec: &str) -> Option<bundle::ResourceKey> {
        let key = bundle::ResourceKey::new(gdtf_spec);
        if self.bundle.resources().contains_key(&key) {
            return Some(key);
        }

        for entry in self.bundle.resources().entries() {
            if entry.key().relative_path().file_name()?.to_str()? == gdtf_spec {
                return Some(entry.key().clone());
            }
        }

        None
    }

    pub fn models(&self) -> impl Iterator<Item = &bundle::ResourceEntry> {
        self.bundle.resources().entries().filter(|e| e.kind() == bundle::ResourceKind::Model)
    }

    pub fn textures(&self) -> impl Iterator<Item = &bundle::ResourceEntry> {
        self.bundle.resources().entries().filter(|e| e.kind() == bundle::ResourceKind::Texture)
    }
}

impl From<bundle::Bundle> for Mvr {
    fn from(bundle: bundle::Bundle) -> Self {
        let version = Version {
            major: bundle.description().ver_major as u32,
            minor: bundle.description().ver_minor as u32,
        };

        let provider = Provider {
            name: bundle.description().provider.clone().unwrap_or_default(),
            version: bundle.description().provider_version.clone().unwrap_or_default(),
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
        let aux_data = bundle.description().scene.aux_data.as_ref().unwrap_or(&EMPTY_AUX_DATA);

        Self::build_classes(aux_data, &mut classes);
        Self::build_positions(aux_data, &mut positions);
        Self::build_symdefs(aux_data, &mut symdefs);
        Self::build_mapping_definitions(aux_data, &mut mapping_definitions);
        Self::build_layers(&bundle, &classes, aux_data, &mut layers);

        let gdtfs = Self::build_gdtfs(&bundle);

        for (layer_ix, layer) in layers.iter().enumerate() {
            layers_ix.insert(layer.id(), layer_ix);

            for (object_ix, object) in layer.objects().iter().enumerate() {
                Self::index_object_paths(layer.id(), vec![object_ix], object, &mut objects_path_ix);
            }
        }

        Self {
            bundle,
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
}

impl Mvr {
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
            let class: Class = class.into();
            classes.insert(class.id(), class);
        }
    }

    fn build_positions(
        aux_data: &bundle::AuxData,
        positions: &mut HashMap<NodeId<Position>, Position>,
    ) {
        for position in &aux_data.position {
            let position: Position = position.into();
            positions.insert(position.id(), position);
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

        md.into()
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
    ) -> layer::SceneObject {
        let id = Self::build_id_from_multipatch(
            &c.multipatch,
            c.fixture_id.to_owned(),
            c.fixture_id_numeric,
            c.custom_id,
            c.custom_id_type,
        );

        layer::SceneObject {
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
    ) -> layer::GroupObject {
        layer::GroupObject {
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
    ) -> layer::FocusPointObject {
        layer::FocusPointObject {
            geometries: Self::build_geometries(
                &c.geometries.geometry_3d,
                &c.geometries.symbol,
                aux_data,
            ),
        }
    }

    fn build_fixture_object(
        c: &bundle::Fixture,
        aux_data: &bundle::AuxData,
    ) -> layer::FixtureObject {
        layer::FixtureObject {
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
            focus: c.focus.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            function: c.function.clone(),
            gobo: c.gobo.as_ref().map(Into::into),
            mappings: c
                .mappings
                .as_ref()
                .map(|mappings| mappings.mapping.iter().map(Into::into).collect())
                .unwrap_or_default(),
            position: c.position.as_ref().map(|s| NodeId::from_str(s).unwrap()),
            protocols: c
                .protocols
                .as_ref()
                .map(|protocols| protocols.protocol.iter().map(Into::into).collect())
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

    fn build_support_object(
        c: &bundle::Support,
        aux_data: &bundle::AuxData,
    ) -> layer::SupportObject {
        layer::SupportObject {
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

    fn build_truss_object(c: &bundle::Truss, aux_data: &bundle::AuxData) -> layer::TrussObject {
        layer::TrussObject {
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
    ) -> layer::VideoScreenObject {
        layer::VideoScreenObject {
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
                .map(|sources| sources.source.iter().map(Into::into).collect())
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
    ) -> layer::ProjectorObject {
        layer::ProjectorObject {
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
                    let source: layer::Source = p.source.first().map(Into::into)?;

                    let scale_handling: layer::ScaleHandling =
                        p.scale_handeling.first().map(Into::into).unwrap_or_default();

                    Some(layer::Projection { source, scale_handling })
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

    fn build_dmx_addresses(addresses: Option<&bundle::Addresses>) -> Vec<layer::DmxAddress> {
        addresses.map(|addrs| addrs.address.iter().map(Into::into).collect()).unwrap_or_default()
    }

    fn build_network_addresses(
        addresses: Option<&bundle::Addresses>,
    ) -> Vec<layer::NetworkAddress> {
        addresses.map(|addrs| addrs.network.iter().map(Into::into).collect()).unwrap_or_default()
    }

    fn build_alignments(alignments: Option<&bundle::Alignments>) -> Vec<layer::Alignment> {
        alignments
            .map(|alignments| alignments.alignment.iter().map(Into::into).collect())
            .unwrap_or_default()
    }

    fn build_custom_commands(
        commands: Option<&bundle::CustomCommands>,
    ) -> Vec<layer::CustomCommand> {
        commands
            .map(|commands| commands.custom_command.iter().map(Into::into).collect())
            .unwrap_or_default()
    }

    fn build_overwrites(overwrites: Option<&bundle::Overwrites>) -> Vec<layer::Overwrite> {
        overwrites.map(|ows| ows.overwrite.iter().map(Into::into).collect()).unwrap_or_default()
    }

    fn build_connections(connections: Option<&bundle::Connections>) -> Vec<layer::Connection> {
        connections
            .map(|conns| conns.connection.iter().map(Into::into).collect())
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
    ) -> layer::ObjectIdentifier {
        match Uuid::from_str(multipatch) {
            Ok(uuid) => layer::ObjectIdentifier::Multipatch(uuid.into()),
            Err(_) => layer::ObjectIdentifier::Single {
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

impl std::fmt::Debug for Mvr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mvr")
            .field("version", &self.version)
            .field("provider", &self.provider)
            .field("symdefs", &self.symdefs)
            .field("classes", &self.classes)
            .field("positions", &self.positions)
            .field("layers", &self.layers)
            .finish()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    major: u32,
    minor: u32,
}

impl Version {
    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provider {
    name: String,
    version: String,
}

impl Provider {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

pub struct NodeId<T> {
    uuid: Uuid,
    _marker: std::marker::PhantomData<T>,
}

impl<T> NodeId<T> {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid, _marker: std::marker::PhantomData }
    }

    pub fn as_uuid(&self) -> Uuid {
        self.uuid
    }
}

impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        Self { uuid: self.uuid, _marker: std::marker::PhantomData }
    }
}

impl<T> Copy for NodeId<T> {}

impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<T> Eq for NodeId<T> {}

impl<T> std::hash::Hash for NodeId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<T> From<Uuid> for NodeId<T> {
    fn from(uuid: Uuid) -> Self {
        Self { uuid, _marker: std::marker::PhantomData }
    }
}

impl<T> str::FromStr for NodeId<T> {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(Uuid::from_str(s)?))
    }
}

impl<T> fmt::Debug for NodeId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId<{}>({:?})", std::any::type_name::<T>(), self.uuid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ObjectPath {
    layer_id: NodeId<Layer>,
    indices: Vec<usize>,
}

impl ObjectPath {
    pub fn new(layer_id: NodeId<Layer>, indices: Vec<usize>) -> Self {
        Self { layer_id, indices }
    }
}
