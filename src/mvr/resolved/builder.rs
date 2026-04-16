use std::{collections::HashMap, str::FromStr, sync::Arc};

use uuid::Uuid;

use crate::mvr::{
    Class, FixtureObject, FocusPointObject, Geometry, GroupObject, Layer, Mvr, Object, ObjectData,
    ObjectKind, Position, ProjectorObject, Provider, SceneObject, SupportObject, Symdef,
    TrussObject, Version, VideoScreenObject,
    bundle::{self},
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
            major: self.bundle.description().ver_major,
            minor: self.bundle.description().ver_minor,
        };

        let provider = Provider {
            name: self.bundle.description().provider.clone().unwrap_or_default(),
            version: self.bundle.description().provider_version.clone().unwrap_or_default(),
        };

        let mut symdefs = HashMap::new();
        let mut classes = HashMap::new();
        let mut positions = HashMap::new();
        let mut layers = HashMap::new();

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

        Self::build_layers(&self.bundle, &classes, &aux_data, &mut layers);

        Mvr {
            bundle: self.bundle,
            version,
            provider,
            symdefs,
            classes,
            // FIXME: Implement MappingDefinition mapping_definitions: HashMap<Uuid, Arc<MappingDefinition>>,
            positions,
            layers,
        }
    }

    fn build_classes(aux_data: &bundle::AuxData, classes: &mut HashMap<Uuid, Arc<Class>>) {
        for class in &aux_data.class {
            let uuid = Uuid::parse_str(&class.uuid).unwrap();
            let name = class.name.clone();
            classes.insert(uuid, Arc::new(Class { name, uuid }));
        }
    }

    fn build_positions(aux_data: &bundle::AuxData, positions: &mut HashMap<Uuid, Arc<Position>>) {
        for position in &aux_data.position {
            let uuid = Uuid::parse_str(&position.uuid).unwrap();
            let name = position.name.clone();
            positions.insert(uuid, Arc::new(Position { name, uuid }));
        }
    }

    fn build_symdefs(aux_data: &bundle::AuxData, symdefs: &mut HashMap<Uuid, Arc<Symdef>>) {
        for symdef in &aux_data.symdef {
            let uuid = Uuid::parse_str(&symdef.uuid).unwrap();
            let parsed_symdef = Self::create_symdef(&symdef.uuid, aux_data);
            symdefs.insert(uuid, Arc::new(parsed_symdef));
        }
    }

    fn create_symdef(symdef_uuid: &str, aux_data: &bundle::AuxData) -> Symdef {
        let symdef = aux_data.symdef.iter().find(|s| s.uuid == symdef_uuid).unwrap();
        let uuid = Uuid::parse_str(symdef_uuid).unwrap();
        let name = symdef.name.clone();
        let geometries = Self::create_geometries(
            &symdef.child_list.geometry_3d,
            &symdef.child_list.symbol,
            aux_data,
        );

        Symdef { name, uuid, geometries }
    }

    fn create_geometries(
        geometry_3ds: &[bundle::Geometry3D],
        symbols: &[bundle::Symbol],
        aux_data: &bundle::AuxData,
    ) -> Vec<Geometry> {
        let mut geometries = Vec::new();

        for geo3d in geometry_3ds {
            let model_key = crate::mvr::bundle::ResourceKey::new(&geo3d.file_name);
            geometries.push(Geometry {
                local_transform: create_transform_optional(geo3d.matrix.as_deref()),
                model: model_key,
            });
        }

        for symbol in symbols {
            let symbol_transform = create_transform_optional(symbol.matrix.as_deref());
            let nested_symdef = Self::create_symdef(&symbol.symdef, aux_data);
            for mut geo in nested_symdef.geometries {
                geo.local_transform *= symbol_transform;
                geometries.push(geo);
            }
        }

        geometries
    }

    fn build_layers(
        bundle: &bundle::Bundle,
        classes: &HashMap<Uuid, Arc<Class>>,
        aux_data: &bundle::AuxData,
        layers: &mut HashMap<Uuid, Layer>,
    ) {
        for layer_data in &bundle.description().scene.layers.layer {
            let uuid = Uuid::from_str(&layer_data.uuid).unwrap();
            let name = layer_data.name.clone();
            let mut objects = HashMap::new();

            if let Some(child_list) = &layer_data.child_list {
                for child in &child_list.content {
                    let object = Self::create_object(child, classes, aux_data);
                    objects.insert(object.uuid(), object);
                }
            }

            layers.insert(
                uuid,
                Layer {
                    uuid,
                    name,
                    local_transform: create_transform_optional(layer_data.matrix.as_deref()),
                    objects,
                },
            );
        }
    }

    fn create_object_data(
        child_list: &Option<Box<bundle::ChildList>>,
        classes: &HashMap<Uuid, Arc<Class>>,
        aux_data: &bundle::AuxData,
    ) -> ObjectData {
        let children = match child_list {
            Some(list) => list
                .content
                .iter()
                .map(|child| Self::create_object(child, classes, aux_data))
                .collect(),
            None => Vec::new(),
        };

        ObjectData { children }
    }

    fn create_object(
        child: &bundle::ChildListContent,
        classes: &HashMap<Uuid, Arc<Class>>,
        aux_data: &bundle::AuxData,
    ) -> Object {
        let (uuid_str, name, matrix, classing, kind) = match child {
            bundle::ChildListContent::SceneObject(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::SceneObject(SceneObject {
                    data: Self::create_object_data(&c.child_list, classes, aux_data),
                    geometries: Self::create_geometries(
                        &c.geometries.geometry_3d,
                        &c.geometries.symbol,
                        aux_data,
                    ),
                }),
            ),
            bundle::ChildListContent::GroupObject(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::GroupObject(GroupObject {
                    children: c
                        .child_list
                        .content
                        .iter()
                        .map(|child| Self::create_object(child, classes, aux_data))
                        .collect(),
                }),
            ),
            bundle::ChildListContent::FocusPoint(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::FocusPoint(FocusPointObject {
                    geometries: Self::create_geometries(
                        &c.geometries.geometry_3d,
                        &c.geometries.symbol,
                        aux_data,
                    ),
                }),
            ),
            bundle::ChildListContent::Fixture(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::Fixture(FixtureObject {
                    data: Self::create_object_data(&c.child_list, classes, aux_data),
                }),
            ),
            bundle::ChildListContent::Support(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::Support(SupportObject {
                    data: Self::create_object_data(&c.child_list, classes, aux_data),
                    geometries: Self::create_geometries(
                        &c.geometries.geometry_3d,
                        &c.geometries.symbol,
                        aux_data,
                    ),
                }),
            ),
            bundle::ChildListContent::Truss(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::Truss(TrussObject {
                    data: Self::create_object_data(&c.child_list, classes, aux_data),
                    geometries: Self::create_geometries(
                        &c.geometries.geometry_3d,
                        &c.geometries.symbol,
                        aux_data,
                    ),
                }),
            ),
            bundle::ChildListContent::VideoScreen(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::VideoScreen(VideoScreenObject {
                    data: Self::create_object_data(&c.child_list, classes, aux_data),
                    geometries: Self::create_geometries(
                        &c.geometries.geometry_3d,
                        &c.geometries.symbol,
                        aux_data,
                    ),
                }),
            ),
            bundle::ChildListContent::Projector(c) => (
                &c.uuid,
                &c.name,
                c.matrix.as_deref(),
                c.classing.as_ref(),
                ObjectKind::Projector(ProjectorObject {
                    data: Self::create_object_data(&c.child_list, classes, aux_data),
                    geometries: Self::create_geometries(
                        &c.geometries.geometry_3d,
                        &c.geometries.symbol,
                        aux_data,
                    ),
                }),
            ),
        };

        Object {
            uuid: Uuid::from_str(uuid_str).unwrap(),
            name: name.to_string(),
            class: classing
                .map(|id| Uuid::from_str(id).unwrap())
                .and_then(|id| classes.get(&id))
                .map(Arc::clone),
            local_transform: create_transform_optional(matrix),
            kind,
        }
    }
}

fn create_transform_optional(s: Option<&str>) -> glam::Affine3A {
    s.map(create_transform).unwrap_or(glam::Affine3A::IDENTITY)
}

fn create_transform(s: &str) -> glam::Affine3A {
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
        t[0]  / 1000.0, t[1] / 1000.0, t[2] / 1000.0,
    ];

    glam::Affine3A::from_cols_array(&cols_array)
}
