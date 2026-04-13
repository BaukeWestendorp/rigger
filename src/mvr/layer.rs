use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::mvr::{Geometry, aux::Class};

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) local_transform: glam::Affine3A,

    pub(crate) objects: HashMap<Uuid, Object>,
}

impl Layer {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn objects(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    pub fn object(&self, uuid: Uuid) -> Option<&Object> {
        self.objects.get(&uuid)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) class: Option<Arc<Class>>,
    pub(crate) local_transform: glam::Affine3A,

    pub(crate) kind: ObjectKind,
}

impl Object {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> Option<&Class> {
        self.class.as_deref()
    }

    pub fn local_transform(&self) -> glam::Affine3A {
        self.local_transform
    }

    pub fn kind(&self) -> &ObjectKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectKind {
    SceneObject(SceneObject),
    GroupObject(GroupObject),
    FocusPoint(FocusPointObject),
    Fixture(FixtureObject),
    Support(SupportObject),
    Truss(TrussObject),
    VideoScreen(VideoScreenObject),
    Projector(ProjectorObject),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectData {
    // pub(crate) multipatch: String,
    // pub(crate) gdtf_spec: Option<String>,
    // pub(crate) gdtf_mode: Option<String>,
    // pub(crate) cast_shadow: Option<bool>,
    // pub(crate) addresses: Option<Addresses>,
    // pub(crate) alignments: Option<Alignments>,
    // pub(crate) custom_commands: Option<CustomCommands>,
    // pub(crate) overwrites: Option<Overwrites>,
    // pub(crate) connections: Option<Connections>,
    pub(crate) children: Vec<Object>,
}

impl ObjectData {
    pub fn children(&self) -> &[Object] {
        &self.children
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneObject {
    pub(crate) data: ObjectData,

    pub(crate) geometries: Vec<Geometry>,
    // FIXME: support fields.
    // pub(crate) fixture_id: Option<String>,
    // pub(crate) fixture_id_numeric: Option<i32>,
    // pub(crate) fixture_type_id: Option<i32>,
    // pub(crate) unit_number: Option<i32>,
    // pub(crate) custom_id: Option<i32>,
    // pub(crate) custom_id_type: Option<i32>,
}

impl SceneObject {
    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }
}

impl std::ops::Deref for SceneObject {
    type Target = ObjectData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for SceneObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupObject {
    pub(crate) children: Vec<Object>,
}

impl GroupObject {
    pub fn children(&self) -> &[Object] {
        &self.children
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FocusPointObject {
    pub(crate) geometries: Vec<Geometry>,
}

impl FocusPointObject {
    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureObject {
    pub(crate) data: ObjectData,
    // FIXME: support fields.
    // focus: Option<String>,
    // dmx_invert_pan: Option<bool>,
    // dmx_invert_tilt: Option<bool>,
    // position: Option<String>,
    // function: Option<String>,
    // fixture_id: String,
    // fixture_id_numeric: Option<i32>,
    // fixture_type_id: Option<i32>,
    // unit_number: i32,
    // child_position: Option<String>,
    // protocols: Option<Protocols>,
    // color: Option<String>,
    // custom_id_type: Option<i32>,
    // custom_id: Option<i32>,
    // mappings: Option<Mappings>,
    // gobo: Option<Gobo>,
}

impl std::ops::Deref for FixtureObject {
    type Target = ObjectData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for FixtureObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportObject {
    pub(crate) data: ObjectData,

    pub(crate) geometries: Vec<Geometry>,
    // FIXME: support fields.
    // position: Option<String>,
    // function: Option<String>,
    // chain_length: f32,
    // fixture_id: String,
    // fixture_id_numeric: Option<i32>,
    // fixture_type_id: Option<i32>,
    // unit_number: Option<i32>,
    // custom_id_type: Option<i32>,
    // custom_id: Option<i32>,
}

impl SupportObject {
    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }
}

impl std::ops::Deref for SupportObject {
    type Target = ObjectData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for SupportObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrussObject {
    pub(crate) data: ObjectData,

    pub(crate) geometries: Vec<Geometry>,
    // FIXME: support fields.
    // position: Option<String>,
    // function: Option<String>,
    // child_position: Option<String>,
    // fixture_id: String,
    // fixture_id_numeric: Option<i32>,
    // fixture_type_id: Option<i32>,
    // unit_number: Option<i32>,
    // custom_id_type: Option<i32>,
    // custom_id: Option<i32>,
}

impl TrussObject {
    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }
}

impl std::ops::Deref for TrussObject {
    type Target = ObjectData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for TrussObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoScreenObject {
    pub(crate) data: ObjectData,

    pub(crate) geometries: Vec<Geometry>,
    // FIXME: support fields.
    // sources: Option<Sources>,
    // function: Option<String>,
    // fixture_id: String,
    // fixture_id_numeric: Option<i32>,
    // fixture_type_id: Option<i32>,
    // unit_number: Option<i32>,
    // custom_id_type: Option<i32>,
    // custom_id: Option<i32>,
}

impl VideoScreenObject {
    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }
}

impl std::ops::Deref for VideoScreenObject {
    type Target = ObjectData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for VideoScreenObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectorObject {
    pub(crate) data: ObjectData,
    pub(crate) geometries: Vec<Geometry>,
    // FIXME: support fields.
    // projections: Projections,
    // fixture_id: String,
    // fixture_id_numeric: Option<i32>,
    // fixture_type_id: Option<i32>,
    // unit_number: Option<i32>,
    // custom_id_type: Option<i32>,
    // custom_id: Option<i32>,
}

impl ProjectorObject {
    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }
}

impl std::ops::Deref for ProjectorObject {
    type Target = ObjectData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for ProjectorObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
