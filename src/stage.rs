use std::{path::Path, str::FromStr};

use glam::Affine3A;
use uuid::Uuid;

use crate::mvr::{self, ChildListContent, ModelHandle, Mvr};

pub struct Stage {
    mvr: Mvr,

    layers: Vec<Layer>,
}

impl Stage {
    pub fn new(mvr: Mvr) -> Self {
        let mut layers = Vec::new();
        for layer in &mvr.gsd().scene.layers.layer {
            layers.push(Layer::new(layer, &mvr))
        }

        let mut stage = Self { mvr, layers };
        stage.propagate_world_transforms();
        stage
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    pub fn layer(&self, uuid: Uuid) -> Option<&Layer> {
        // FIXME: We could make this faster with an index.
        self.layers.iter().find(|layer| layer.uuid() == uuid)
    }

    pub fn mvr(&self) -> &Mvr {
        &self.mvr
    }

    fn propagate_world_transforms(&mut self) {
        for layer in &mut self.layers {
            layer.propagate_world_transform(Affine3A::IDENTITY);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layer {
    uuid: Uuid,
    name: String,

    local_transform: Affine3A,
    world_transform: Affine3A,

    objects: Vec<Object>,
}

impl Layer {
    fn new(mvr_layer: &mvr::Layer, mvr: &Mvr) -> Self {
        Self {
            uuid: Uuid::from_str(&mvr_layer.uuid).unwrap(),
            name: mvr_layer.name.clone(),

            local_transform: Affine3A::IDENTITY,
            world_transform: Affine3A::IDENTITY,

            objects: mvr_layer
                .child_list
                .as_ref()
                .map(|cl| &cl.content)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|child| Object::new(child, mvr))
                .collect(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn local_transform(&self) -> Affine3A {
        self.local_transform
    }

    pub fn world_transform(&self) -> Affine3A {
        self.world_transform
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    fn propagate_world_transform(&mut self, parent_world: Affine3A) {
        self.world_transform = parent_world * self.local_transform;
        for child in &mut self.objects {
            child.propagate_world_transform(self.world_transform);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    uuid: Uuid,
    name: String,

    local_transform: Affine3A,
    world_transform: Affine3A,

    kind: ObjectKind,
}

impl Object {
    fn new(mvr_child_list_content: &ChildListContent, mvr: &Mvr) -> Self {
        let (uuid, name, matrix) = match mvr_child_list_content {
            ChildListContent::SceneObject(mvr::SceneObject { uuid, name, matrix, .. })
            | ChildListContent::GroupObject(mvr::GroupObject { uuid, name, matrix, .. })
            | ChildListContent::FocusPoint(mvr::FocusPoint { uuid, name, matrix, .. })
            | ChildListContent::Fixture(mvr::Fixture { uuid, name, matrix, .. })
            | ChildListContent::Support(mvr::Support { uuid, name, matrix, .. })
            | ChildListContent::Truss(mvr::Truss { uuid, name, matrix, .. })
            | ChildListContent::VideoScreen(mvr::VideoScreen { uuid, name, matrix, .. })
            | ChildListContent::Projector(mvr::Projector { uuid, name, matrix, .. }) => {
                (Uuid::from_str(uuid).unwrap(), name.to_owned(), matrix.to_owned())
            }
        };

        Self {
            uuid,
            name,

            local_transform: mvr_matrix_to_affine(matrix),
            world_transform: Affine3A::IDENTITY,

            kind: ObjectKind::new(mvr_child_list_content, mvr),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn local_transform(&self) -> Affine3A {
        self.local_transform
    }

    pub fn world_transform(&self) -> Affine3A {
        self.world_transform
    }

    pub fn kind(&self) -> &ObjectKind {
        &self.kind
    }

    fn propagate_world_transform(&mut self, parent_world: Affine3A) {
        self.world_transform = parent_world * self.local_transform;
        match &mut self.kind {
            ObjectKind::SceneObject { geometries }
            | ObjectKind::FocusPoint { geometries }
            | ObjectKind::Support { geometries }
            | ObjectKind::Truss { geometries }
            | ObjectKind::VideoScreen { geometries }
            | ObjectKind::Projector { geometries } => {
                for geometry in geometries {
                    geometry.propagate_world_transform(self.world_transform);
                }
            }
            ObjectKind::GroupObject { objects } => {
                for child in objects {
                    child.propagate_world_transform(self.world_transform);
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum ObjectKind {
    SceneObject { geometries: Vec<Geometry> },
    GroupObject { objects: Vec<Object> },
    FocusPoint { geometries: Vec<Geometry> },
    Fixture { gdtf: Option<mvr::GdtfHandle> },
    Support { geometries: Vec<Geometry> },
    Truss { geometries: Vec<Geometry> },
    VideoScreen { geometries: Vec<Geometry> },
    Projector { geometries: Vec<Geometry> },
}

impl ObjectKind {
    pub fn new(mvr_child_list_content: &ChildListContent, mvr: &Mvr) -> Self {
        fn generate_geometries(mvr: &Mvr, mvr_geometries: &mvr::Geometries) -> Vec<Geometry> {
            let mut geometries = Vec::new();

            for geo_3d in &mvr_geometries.geometry_3d {
                if let Some(handle) = mvr.model_handle(&geo_3d.file_name) {
                    geometries.push(Geometry::new(handle.clone(), geo_3d.matrix.to_owned()));
                }
            }

            for symbol in &mvr_geometries.symbol {
                let symdef_uuid = Uuid::from_str(&symbol.symdef).unwrap();
                let symdef = mvr.symdef(symdef_uuid).unwrap();
                geometries.extend(generate_geometries(mvr, &symdef.child_list));
            }

            geometries
        }

        match mvr_child_list_content {
            ChildListContent::SceneObject(c) => {
                Self::SceneObject { geometries: generate_geometries(mvr, &c.geometries) }
            }
            ChildListContent::GroupObject(c) => Self::GroupObject {
                objects: c
                    .child_list
                    .content
                    .iter()
                    .map(|mvr_child_list_content| Object::new(mvr_child_list_content, mvr))
                    .collect(),
            },
            ChildListContent::FocusPoint(c) => {
                Self::FocusPoint { geometries: generate_geometries(mvr, &c.geometries) }
            }
            ChildListContent::Fixture(c) => Self::Fixture {
                gdtf: c.gdtf_spec.as_ref().and_then(|gdtf_spec| {
                    mvr.gdtf_handle(Path::new(gdtf_spec).with_extension("gdtf"))
                }),
            },
            ChildListContent::Support(c) => {
                Self::Support { geometries: generate_geometries(mvr, &c.geometries) }
            }
            ChildListContent::Truss(c) => {
                Self::Truss { geometries: generate_geometries(mvr, &c.geometries) }
            }
            ChildListContent::VideoScreen(c) => {
                Self::VideoScreen { geometries: generate_geometries(mvr, &c.geometries) }
            }
            ChildListContent::Projector(c) => {
                Self::Projector { geometries: generate_geometries(mvr, &c.geometries) }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Geometry {
    model: ModelHandle,

    local_transform: Affine3A,
    world_transform: Affine3A,
}

impl Geometry {
    pub fn new(model: ModelHandle, matrix: Option<String>) -> Self {
        Self {
            model,

            local_transform: mvr_matrix_to_affine(matrix),
            world_transform: Affine3A::IDENTITY,
        }
    }

    pub fn model(&self) -> &ModelHandle {
        &self.model
    }

    pub fn local_transform(&self) -> Affine3A {
        self.local_transform
    }

    pub fn world_transform(&self) -> Affine3A {
        self.world_transform
    }

    pub fn propagate_world_transform(&mut self, parent_world: Affine3A) {
        self.world_transform = parent_world * self.local_transform;
    }
}

fn mvr_matrix_to_affine(matrix: Option<String>) -> Affine3A {
    let Some(matrix) = matrix else { return Affine3A::IDENTITY };

    let rows = matrix
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

    Affine3A::from_cols_array(&cols_array)
}
