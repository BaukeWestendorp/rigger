use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    CieColor, gdtf,
    mvr::{
        self,
        aux::{Class, MappingDefinition, Position},
        bundle::ResourceKey,
        geo::Geometry,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) local_transform: glam::Affine3A,

    pub(crate) objects: HashMap<Uuid, Object>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) class: Option<mvr::NodeId<Class>>,
    pub(crate) local_transform: glam::Affine3A,

    pub(crate) kind: ObjectKind,
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
    pub(crate) children: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupObject {
    pub(crate) children: Vec<Object>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FocusPointObject {
    pub(crate) geometries: Vec<Geometry>,
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
    pub(crate) focus: Option<mvr::NodeId<FocusPointObject>>,
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

    pub(crate) children: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
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
    pub(crate) children: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
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

    pub(crate) children: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoScreenObject {
    pub(crate) id: ObjectIdentifier,
    pub(crate) gdtf: Option<GdtfInfo>,

    pub(crate) function: Option<String>,
    pub(crate) sources: Vec<Source>,

    pub(crate) dmx_addresses: Vec<DmxAddress>,
    pub(crate) network_addresses: Vec<NetworkAddress>,
    pub(crate) alignments: Vec<Alignment>,
    pub(crate) custom_commands: Vec<CustomCommand>,
    pub(crate) overwrites: Vec<Overwrite>,
    pub(crate) connections: Vec<Connection>,

    pub(crate) cast_shadow: bool,
    pub(crate) children: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
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
    pub(crate) children: Vec<Object>,
    // FIXME: Only in MVR 1.5 this is needed.
    // fixture_type_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectIdentifier {
    Multipatch(Uuid),
    Single {
        fixture_id: Option<String>,
        fixture_id_numeric: Option<i32>,
        custom_id: Option<i32>,
        custom_id_type: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct GdtfInfo {
    pub(crate) gdtf_spec: String,
    pub(crate) gdtf_mode: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmxAddress {
    pub(crate) r#break: u32,
    pub(crate) absolute_value: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkAddress {
    pub(crate) geometry: gdtf::Node,
    pub(crate) ipv4: Option<std::net::Ipv4Addr>,
    pub(crate) subnetmask: Option<std::net::Ipv4Addr>,
    pub(crate) ipv6: Option<std::net::Ipv6Addr>,
    pub(crate) dhcp: bool,
    pub(crate) hostname: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alignment {
    pub(crate) geometry: gdtf::Node,
    pub(crate) up: glam::Vec3A,
    pub(crate) direction: glam::Vec3A,
}

// FIXME: Instead of a string, this should be a custom type that parses the commands.
#[derive(Debug, Clone, PartialEq)]
pub struct CustomCommand(pub(crate) String);

#[derive(Debug, Clone, PartialEq)]
pub struct Overwrite {
    pub(crate) universal: gdtf::Node,
    pub(crate) target: Option<gdtf::Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connection {
    pub(crate) own: gdtf::Node,
    pub(crate) other: gdtf::Node,
    pub(crate) to_object: mvr::NodeId<Object>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gobo {
    pub(crate) resource: ResourceKey,
    pub(crate) rotation: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Protocol {
    pub(crate) geometry: gdtf::Node,
    pub(crate) name: String,
    pub(crate) r#type: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) transmission: Option<Transmission>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Transmission {
    Unicast,
    Multicast,
    Broadcast,
    Anycast,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    pub(crate) linked_geometry: gdtf::Node,
    pub(crate) r#type: SourceType,
    pub(crate) value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceType {
    Ndi,
    File,
    Citp,
    CaptureDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScaleHandling {
    #[default]
    ScaleKeepRatio,
    ScaleIgnoreRatio,
    KeepSizeCenter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Projection {
    pub(crate) source: Source,
    pub(crate) scale_handling: ScaleHandling,
}
