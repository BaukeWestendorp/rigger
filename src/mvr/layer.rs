use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr as _,
};

use crate::{
    CieColor, gdtf,
    mvr::{
        self,
        aux::{Class, MappingDefinition, Position},
        bundle,
        geo::Geometry,
    },
    util,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub(crate) id: mvr::NodeId<Layer>,
    pub(crate) name: String,
    pub(crate) local_transform: glam::Affine3A,

    pub(crate) objects: Vec<Object>,
}

impl Layer {
    pub fn id(&self) -> mvr::NodeId<Layer> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn local_transform(&self) -> &glam::Affine3A {
        &self.local_transform
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub(crate) id: mvr::NodeId<Object>,
    pub(crate) name: String,
    pub(crate) class: Option<mvr::NodeId<Class>>,
    pub(crate) local_transform: glam::Affine3A,

    pub(crate) kind: ObjectKind,
}

impl Object {
    pub fn id(&self) -> mvr::NodeId<Object> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> Option<mvr::NodeId<Class>> {
        self.class
    }

    pub fn local_transform(&self) -> &glam::Affine3A {
        &self.local_transform
    }

    pub fn kind(&self) -> &ObjectKind {
        &self.kind
    }

    pub fn as_scene_object(&self) -> Option<&SceneObject> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_group_object(&self) -> Option<&GroupObject> {
        match &self.kind {
            ObjectKind::GroupObject(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_focus_point_object(&self) -> Option<&FocusPointObject> {
        match &self.kind {
            ObjectKind::FocusPoint(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_fixture_object(&self) -> Option<&FixtureObject> {
        match &self.kind {
            ObjectKind::Fixture(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_support_object(&self) -> Option<&SupportObject> {
        match &self.kind {
            ObjectKind::Support(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_truss_object(&self) -> Option<&TrussObject> {
        match &self.kind {
            ObjectKind::Truss(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_video_screen_object(&self) -> Option<&VideoScreenObject> {
        match &self.kind {
            ObjectKind::VideoScreen(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_projector_object(&self) -> Option<&ProjectorObject> {
        match &self.kind {
            ObjectKind::Projector(v) => Some(v),
            _ => None,
        }
    }

    pub fn identifier(&self) -> Option<&ObjectIdentifier> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.id()),
            ObjectKind::Fixture(v) => Some(v.id()),
            ObjectKind::Support(v) => Some(v.id()),
            ObjectKind::Truss(v) => Some(v.id()),
            ObjectKind::VideoScreen(v) => Some(v.id()),
            ObjectKind::Projector(v) => Some(v.id()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn gdtf_info(&self) -> Option<&GdtfInfo> {
        match &self.kind {
            ObjectKind::SceneObject(v) => v.gdtf(),
            ObjectKind::Fixture(v) => v.gdtf(),
            ObjectKind::Support(v) => v.gdtf(),
            ObjectKind::Truss(v) => v.gdtf(),
            ObjectKind::VideoScreen(v) => v.gdtf(),
            ObjectKind::Projector(v) => v.gdtf(),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn geometries(&self) -> Option<&[Geometry]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.geometries()),
            ObjectKind::FocusPoint(v) => Some(v.geometries()),
            ObjectKind::Support(v) => Some(v.geometries()),
            ObjectKind::Projector(v) => Some(v.geometries()),
            ObjectKind::Truss(v) => Some(v.geometries()),
            ObjectKind::VideoScreen(v) => Some(v.geometries()),
            ObjectKind::GroupObject(_) | ObjectKind::Fixture(_) => None,
        }
    }

    pub fn child_objects(&self) -> Option<&[Object]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.child_objects()),
            ObjectKind::GroupObject(v) => Some(v.child_objects()),
            ObjectKind::Fixture(v) => Some(v.child_objects()),
            ObjectKind::Support(v) => Some(v.child_objects()),
            ObjectKind::Truss(v) => Some(v.child_objects()),
            ObjectKind::VideoScreen(v) => Some(v.child_objects()),
            ObjectKind::Projector(v) => Some(v.child_objects()),
            ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn has_children(&self) -> bool {
        self.child_objects().is_some_and(|c| !c.is_empty())
    }

    pub fn dmx_addresses(&self) -> Option<&[DmxAddress]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.dmx_addresses()),
            ObjectKind::Fixture(v) => Some(v.dmx_addresses()),
            ObjectKind::Support(v) => Some(v.dmx_addresses()),
            ObjectKind::Truss(v) => Some(v.dmx_addresses()),
            ObjectKind::VideoScreen(v) => Some(v.dmx_addresses()),
            ObjectKind::Projector(v) => Some(v.dmx_addresses()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn network_addresses(&self) -> Option<&[NetworkAddress]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.network_addresses()),
            ObjectKind::Fixture(v) => Some(v.network_addresses()),
            ObjectKind::Support(v) => Some(v.network_addresses()),
            ObjectKind::Truss(v) => Some(v.network_addresses()),
            ObjectKind::VideoScreen(v) => Some(v.network_addresses()),
            ObjectKind::Projector(v) => Some(v.network_addresses()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn alignments(&self) -> Option<&[Alignment]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.alignments()),
            ObjectKind::Fixture(v) => Some(v.alignments()),
            ObjectKind::Support(v) => Some(v.alignments()),
            ObjectKind::Truss(v) => Some(v.alignments()),
            ObjectKind::VideoScreen(v) => Some(v.alignments()),
            ObjectKind::Projector(v) => Some(v.alignments()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn custom_commands(&self) -> Option<&[CustomCommand]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.custom_commands()),
            ObjectKind::Fixture(v) => Some(v.custom_commands()),
            ObjectKind::Support(v) => Some(v.custom_commands()),
            ObjectKind::Truss(v) => Some(v.custom_commands()),
            ObjectKind::VideoScreen(v) => Some(v.custom_commands()),
            ObjectKind::Projector(v) => Some(v.custom_commands()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn overwrites(&self) -> Option<&[Overwrite]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.overwrites()),
            ObjectKind::Fixture(v) => Some(v.overwrites()),
            ObjectKind::Support(v) => Some(v.overwrites()),
            ObjectKind::Truss(v) => Some(v.overwrites()),
            ObjectKind::VideoScreen(v) => Some(v.overwrites()),
            ObjectKind::Projector(v) => Some(v.overwrites()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
    }

    pub fn connections(&self) -> Option<&[Connection]> {
        match &self.kind {
            ObjectKind::SceneObject(v) => Some(v.connections()),
            ObjectKind::Fixture(v) => Some(v.connections()),
            ObjectKind::Support(v) => Some(v.connections()),
            ObjectKind::Truss(v) => Some(v.connections()),
            ObjectKind::VideoScreen(v) => Some(v.connections()),
            ObjectKind::Projector(v) => Some(v.connections()),
            ObjectKind::GroupObject(_) | ObjectKind::FocusPoint(_) => None,
        }
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
pub struct SceneObject {
    pub(crate) id: ObjectIdentifier,
    pub(crate) gdtf: Option<GdtfInfo>,

    pub(crate) cast_shadow: bool,
    pub(crate) unit_number: Option<i32>,

    pub(crate) dmx_addresses: Vec<DmxAddress>,
    pub(crate) network_addresses: Vec<NetworkAddress>,
    pub(crate) alignments: Vec<Alignment>,
    pub(crate) custom_commands: Vec<CustomCommand>,
    pub(crate) overwrites: Vec<Overwrite>,
    pub(crate) connections: Vec<Connection>,

    pub(crate) geometries: Vec<Geometry>,
    pub(crate) child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl SceneObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupObject {
    pub(crate) child_objects: Vec<Object>,
}

impl GroupObject {
    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
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
    pub(crate) id: ObjectIdentifier,
    pub(crate) gdtf: Option<GdtfInfo>,

    pub(crate) cast_shadow: bool,
    pub(crate) child_position: Option<gdtf::Node>,
    pub(crate) color: Option<CieColor>,
    pub(crate) dmx_invert_pan: bool,
    pub(crate) dmx_invert_tilt: bool,
    pub(crate) focus: Option<mvr::NodeId<Object>>,
    pub(crate) function: Option<String>,
    pub(crate) gobo: Option<Gobo>,
    pub(crate) mappings: Vec<Mapping>,
    pub(crate) position: Option<mvr::NodeId<Position>>,
    pub(crate) protocols: Vec<Protocol>,
    pub(crate) unit_number: Option<i32>,

    pub(crate) dmx_addresses: Vec<DmxAddress>,
    pub(crate) network_addresses: Vec<NetworkAddress>,
    pub(crate) alignments: Vec<Alignment>,
    pub(crate) custom_commands: Vec<CustomCommand>,
    pub(crate) overwrites: Vec<Overwrite>,
    pub(crate) connections: Vec<Connection>,

    pub(crate) child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl FixtureObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    // FIXME: Add direct getter from `Mvr` for this.
    pub fn child_position(&self) -> Option<&gdtf::Node> {
        self.child_position.as_ref()
    }

    pub fn color(&self) -> Option<CieColor> {
        self.color
    }

    pub fn dmx_invert_pan(&self) -> bool {
        self.dmx_invert_pan
    }

    pub fn dmx_invert_tilt(&self) -> bool {
        self.dmx_invert_tilt
    }

    pub fn focus_point(&self) -> Option<mvr::NodeId<Object>> {
        self.focus
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn gobo(&self) -> Option<&Gobo> {
        self.gobo.as_ref()
    }

    pub fn mappings(&self) -> &[Mapping] {
        &self.mappings
    }

    pub fn position(&self) -> Option<mvr::NodeId<Position>> {
        self.position
    }

    pub fn protocols(&self) -> &[Protocol] {
        &self.protocols
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportObject {
    pub(crate) gdtf: Option<GdtfInfo>,
    pub(crate) id: ObjectIdentifier,

    pub(crate) cast_shadow: bool,
    pub(crate) chain_length: f32,
    pub(crate) function: Option<String>,
    pub(crate) position: Option<mvr::NodeId<Position>>,
    pub(crate) unit_number: Option<i32>,

    pub(crate) dmx_addresses: Vec<DmxAddress>,
    pub(crate) network_addresses: Vec<NetworkAddress>,
    pub(crate) alignments: Vec<Alignment>,
    pub(crate) custom_commands: Vec<CustomCommand>,
    pub(crate) overwrites: Vec<Overwrite>,
    pub(crate) connections: Vec<Connection>,

    pub(crate) geometries: Vec<Geometry>,
    pub(crate) child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl SupportObject {
    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    pub fn chain_length(&self) -> f32 {
        self.chain_length
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn position(&self) -> Option<mvr::NodeId<Position>> {
        self.position
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrussObject {
    pub(crate) id: ObjectIdentifier,
    pub(crate) gdtf: Option<GdtfInfo>,

    pub(crate) cast_shadow: bool,
    pub(crate) child_position: Option<gdtf::Node>,
    pub(crate) function: Option<String>,
    pub(crate) position: Option<mvr::NodeId<Position>>,
    pub(crate) unit_number: Option<i32>,

    pub(crate) dmx_addresses: Vec<DmxAddress>,
    pub(crate) network_addresses: Vec<NetworkAddress>,
    pub(crate) alignments: Vec<Alignment>,
    pub(crate) custom_commands: Vec<CustomCommand>,
    pub(crate) overwrites: Vec<Overwrite>,
    pub(crate) connections: Vec<Connection>,

    pub(crate) geometries: Vec<Geometry>,
    pub(crate) child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl TrussObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    // FIXME: Add direct getter from `Mvr` for this.
    pub fn child_position(&self) -> Option<&gdtf::Node> {
        self.child_position.as_ref()
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn position(&self) -> Option<mvr::NodeId<Position>> {
        self.position
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoScreenObject {
    pub(crate) id: ObjectIdentifier,
    pub(crate) gdtf: Option<GdtfInfo>,

    pub(crate) function: Option<String>,
    pub(crate) sources: Vec<Source>,
    pub(crate) cast_shadow: bool,

    pub(crate) dmx_addresses: Vec<DmxAddress>,
    pub(crate) network_addresses: Vec<NetworkAddress>,
    pub(crate) alignments: Vec<Alignment>,
    pub(crate) custom_commands: Vec<CustomCommand>,
    pub(crate) overwrites: Vec<Overwrite>,
    pub(crate) connections: Vec<Connection>,

    pub(crate) geometries: Vec<Geometry>,
    pub(crate) child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl VideoScreenObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn sources(&self) -> &[Source] {
        &self.sources
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectorObject {
    pub(crate) id: ObjectIdentifier,
    pub(crate) gdtf: Option<GdtfInfo>,

    pub(crate) cast_shadow: bool,
    pub(crate) projections: Vec<Projection>,
    pub(crate) unit_number: Option<i32>,

    pub(crate) dmx_addresses: Vec<DmxAddress>,
    pub(crate) network_addresses: Vec<NetworkAddress>,
    pub(crate) alignments: Vec<Alignment>,
    pub(crate) custom_commands: Vec<CustomCommand>,
    pub(crate) overwrites: Vec<Overwrite>,
    pub(crate) connections: Vec<Connection>,

    pub(crate) geometries: Vec<Geometry>,
    pub(crate) child_objects: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

impl ProjectorObject {
    pub fn id(&self) -> &ObjectIdentifier {
        &self.id
    }

    pub fn gdtf(&self) -> Option<&GdtfInfo> {
        self.gdtf.as_ref()
    }

    pub fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    pub fn projections(&self) -> &[Projection] {
        &self.projections
    }

    pub fn unit_number(&self) -> Option<i32> {
        self.unit_number
    }

    pub fn dmx_addresses(&self) -> &[DmxAddress] {
        &self.dmx_addresses
    }

    pub fn network_addresses(&self) -> &[NetworkAddress] {
        &self.network_addresses
    }

    pub fn alignments(&self) -> &[Alignment] {
        &self.alignments
    }

    pub fn custom_commands(&self) -> &[CustomCommand] {
        &self.custom_commands
    }

    pub fn overwrites(&self) -> &[Overwrite] {
        &self.overwrites
    }

    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }

    pub fn child_objects(&self) -> &[Object] {
        &self.child_objects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectIdentifier {
    Multipatch(mvr::NodeId<Object>),
    Single {
        fixture_id: Option<String>,
        fixture_id_numeric: Option<i32>,
        custom_id: Option<i32>,
        custom_id_type: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct GdtfInfo {
    gdtf_file_name: String,
    gdtf_mode: String,
}

impl GdtfInfo {
    pub fn new(gdtf_spec: impl Into<String>, gdtf_mode: impl Into<String>) -> Self {
        Self { gdtf_file_name: gdtf_spec.into(), gdtf_mode: gdtf_mode.into() }
    }

    pub fn gdtf_spec(&self) -> &str {
        &self.gdtf_file_name
    }

    pub fn gdtf_mode(&self) -> &str {
        &self.gdtf_mode
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxAddress {
    pub(crate) r#break: u32,
    pub(crate) absolute_value: u32,
}

impl DmxAddress {
    pub fn break_(&self) -> u32 {
        self.r#break
    }

    pub fn absolute_value(&self) -> u32 {
        self.absolute_value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkAddress {
    pub(crate) geometry: gdtf::Node,
    pub(crate) ipv4: Option<Ipv4Addr>,
    pub(crate) subnetmask: Option<Ipv4Addr>,
    pub(crate) ipv6: Option<Ipv6Addr>,
    pub(crate) dhcp: bool,
    pub(crate) hostname: Option<String>,
}

impl NetworkAddress {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn geometry(&self) -> &gdtf::Node {
        &self.geometry
    }

    pub fn ipv4(&self) -> Option<Ipv4Addr> {
        self.ipv4
    }

    pub fn subnetmask(&self) -> Option<Ipv4Addr> {
        self.subnetmask
    }

    pub fn ipv6(&self) -> Option<Ipv6Addr> {
        self.ipv6
    }

    pub fn dhcp(&self) -> bool {
        self.dhcp
    }

    pub fn hostname(&self) -> Option<&str> {
        self.hostname.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alignment {
    pub(crate) geometry: gdtf::Node,
    pub(crate) up: glam::Vec3A,
    pub(crate) direction: glam::Vec3A,
}

impl Alignment {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn geometry(&self) -> &gdtf::Node {
        &self.geometry
    }

    pub fn up(&self) -> &glam::Vec3A {
        &self.up
    }

    pub fn direction(&self) -> &glam::Vec3A {
        &self.direction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomCommand(pub(crate) String);

impl CustomCommand {
    pub fn command(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Overwrite {
    pub(crate) universal: gdtf::Node,
    pub(crate) target: Option<gdtf::Node>,
}

impl Overwrite {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn universal(&self) -> &gdtf::Node {
        &self.universal
    }

    // FIXME: Add direct getter from `Mvr` for this.
    pub fn target(&self) -> Option<&gdtf::Node> {
        self.target.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    pub(crate) own: gdtf::Node,
    pub(crate) other: gdtf::Node,
    pub(crate) to_object: mvr::NodeId<Object>,
}

impl Connection {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn own(&self) -> &gdtf::Node {
        &self.own
    }

    // FIXME: Add direct getter from `Mvr` for this.
    pub fn other(&self) -> &gdtf::Node {
        &self.other
    }

    pub fn to_object(&self) -> mvr::NodeId<Object> {
        self.to_object
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gobo {
    pub(crate) resource: bundle::ResourceKey,
    pub(crate) rotation: f32,
}

impl From<&bundle::Gobo> for Gobo {
    fn from(value: &bundle::Gobo) -> Self {
        Self { resource: bundle::ResourceKey::new(&value.file_name), rotation: value.rotation }
    }
}

impl Gobo {
    pub fn resource(&self) -> &bundle::ResourceKey {
        &self.resource
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Protocol {
    pub(crate) geometry: gdtf::Node,
    pub(crate) name: String,
    pub(crate) r#type: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) transmission: Option<Transmission>,
}

impl Protocol {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn geometry(&self) -> &gdtf::Node {
        &self.geometry
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_(&self) -> Option<&str> {
        self.r#type.as_deref()
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn transmission(&self) -> Option<Transmission> {
        self.transmission
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transmission {
    Unicast,
    Multicast,
    Broadcast,
    Anycast,
}

impl From<&bundle::Transmission> for Transmission {
    fn from(value: &bundle::Transmission) -> Self {
        match value {
            bundle::Transmission::Unicast => Transmission::Unicast,
            bundle::Transmission::Multicast => Transmission::Multicast,
            bundle::Transmission::Broadcast => Transmission::Broadcast,
            bundle::Transmission::Anycast => Transmission::Anycast,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    pub(crate) linked_def: mvr::NodeId<MappingDefinition>,
    pub(crate) ux: i32,
    pub(crate) uy: i32,
    pub(crate) ox: i32,
    pub(crate) oy: i32,
    pub(crate) rz: f32,
}

impl Mapping {
    pub fn linked_def(&self) -> mvr::NodeId<MappingDefinition> {
        self.linked_def
    }

    pub fn ux(&self) -> i32 {
        self.ux
    }

    pub fn uy(&self) -> i32 {
        self.uy
    }

    pub fn ox(&self) -> i32 {
        self.ox
    }

    pub fn oy(&self) -> i32 {
        self.oy
    }

    pub fn rz(&self) -> f32 {
        self.rz
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    pub(crate) linked_geometry: gdtf::Node,
    pub(crate) r#type: SourceType,
    pub(crate) value: String,
}

impl Source {
    // FIXME: Add direct getter from `Mvr` for this.
    pub fn linked_geometry(&self) -> &gdtf::Node {
        &self.linked_geometry
    }

    pub fn type_(&self) -> SourceType {
        self.r#type
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceType {
    Ndi,
    File,
    Citp,
    CaptureDevice,
}

impl From<&bundle::SourceType> for SourceType {
    fn from(value: &bundle::SourceType) -> Self {
        match value {
            bundle::SourceType::Ndi => SourceType::Ndi,
            bundle::SourceType::File => SourceType::File,
            bundle::SourceType::Citp => SourceType::Citp,
            bundle::SourceType::CaptureDevice => SourceType::CaptureDevice,
        }
    }
}

impl From<bundle::SourceType> for SourceType {
    fn from(value: bundle::SourceType) -> Self {
        (&value).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScaleHandling {
    #[default]
    ScaleKeepRatio,
    ScaleIgnoreRatio,
    KeepSizeCenter,
}

impl From<&bundle::Scale> for ScaleHandling {
    fn from(value: &bundle::Scale) -> Self {
        match value {
            bundle::Scale::ScaleKeepRatio => ScaleHandling::ScaleKeepRatio,
            bundle::Scale::ScaleIgnoreRatio => ScaleHandling::ScaleIgnoreRatio,
            bundle::Scale::KeepSizeCenter => ScaleHandling::KeepSizeCenter,
        }
    }
}

impl From<&bundle::ScaleHandeling> for ScaleHandling {
    fn from(value: &bundle::ScaleHandeling) -> Self {
        (&value.r#enum).into()
    }
}

impl From<bundle::ScaleHandeling> for ScaleHandling {
    fn from(value: bundle::ScaleHandeling) -> Self {
        (&value).into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Projection {
    pub(crate) source: Source,
    pub(crate) scale_handling: ScaleHandling,
}

impl From<&bundle::Address> for DmxAddress {
    fn from(value: &bundle::Address) -> Self {
        let absolute_value = if let Some(dot) = value.content.find('.') {
            let (universe_str, channel_str) = value.content.split_at(dot);
            let universe = universe_str.parse::<u32>().unwrap();
            let channel = channel_str[1..].parse::<u32>().unwrap();
            (universe - 1) * 512 + channel
        } else {
            value.content.parse::<u32>().unwrap()
        };

        Self { r#break: value.r#break as u32, absolute_value }
    }
}

impl From<&bundle::Network> for NetworkAddress {
    fn from(value: &bundle::Network) -> Self {
        Self {
            geometry: gdtf::Node::from_str(&value.geometry).unwrap(),
            ipv4: value.ipv_4.as_ref().map(|s| Ipv4Addr::from_str(s).unwrap()),
            subnetmask: value.subnetmask.as_ref().map(|s| Ipv4Addr::from_str(s).unwrap()),
            ipv6: value.ipv_6.as_ref().map(|s| Ipv6Addr::from_str(s).unwrap()),
            dhcp: value.dhcp.as_ref().is_some_and(|s| s == "on"),
            hostname: value.hostname.to_owned(),
        }
    }
}

impl From<&bundle::Alignment> for Alignment {
    fn from(value: &bundle::Alignment) -> Self {
        Self {
            geometry: gdtf::Node::from_str(
                value.geometry.as_ref().expect("FIXME: handle missing geometry"),
            )
            .unwrap(),
            up: util::parse_vec3(&value.up),
            direction: util::parse_vec3(&value.direction),
        }
    }
}

impl From<&String> for CustomCommand {
    fn from(value: &String) -> Self {
        CustomCommand(value.to_owned())
    }
}

impl From<&bundle::Overwrite> for Overwrite {
    fn from(value: &bundle::Overwrite) -> Self {
        Self {
            universal: gdtf::Node::from_str(&value.universal).unwrap(),
            target: Some(gdtf::Node::from_str(&value.target).unwrap()),
        }
    }
}

impl From<&bundle::Connection> for Connection {
    fn from(value: &bundle::Connection) -> Self {
        Self {
            own: gdtf::Node::from_str(&value.own).unwrap(),
            other: gdtf::Node::from_str(&value.other).unwrap(),
            to_object: mvr::NodeId::from_str(&value.to_object).unwrap(),
        }
    }
}

impl From<&bundle::Source> for Source {
    fn from(value: &bundle::Source) -> Self {
        Self {
            linked_geometry: gdtf::Node::from_str(&value.linked_geometry).unwrap(),
            r#type: (&value.r#type).into(),
            value: value.content.clone(),
        }
    }
}

impl From<&bundle::Mapping> for Mapping {
    fn from(value: &bundle::Mapping) -> Self {
        Self {
            linked_def: mvr::NodeId::from_str(&value.linked_def).unwrap(),
            ux: value.ux.unwrap_or_default(),
            uy: value.uy.unwrap_or_default(),
            ox: value.ox.unwrap_or_default(),
            oy: value.oy.unwrap_or_default(),
            rz: value.rz.unwrap_or_default(),
        }
    }
}

impl From<&bundle::Protocol> for Protocol {
    fn from(value: &bundle::Protocol) -> Self {
        Self {
            geometry: gdtf::Node::from_str(&value.geometry)
                .unwrap_or_else(|_| gdtf::Node::from_str("NetworkInOut_1").unwrap()),
            name: value.name.clone(),
            r#type: if value.r#type.is_empty() { None } else { Some(value.r#type.clone()) },
            version: if value.version.is_empty() { None } else { Some(value.version.clone()) },
            transmission: value.transmission.as_ref().map(Into::into),
        }
    }
}

impl TryFrom<&bundle::Projection> for Projection {
    type Error = ();

    fn try_from(value: &bundle::Projection) -> Result<Self, Self::Error> {
        let source = value.source.first().ok_or(())?;
        let source: Source = source.into();

        let scale_handling =
            value.scale_handeling.first().map(|sh| (&sh.r#enum).into()).unwrap_or_default();

        Ok(Self { source, scale_handling })
    }
}

impl Projection {
    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn scale_handling(&self) -> ScaleHandling {
        self.scale_handling
    }
}
