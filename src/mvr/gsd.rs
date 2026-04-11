use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct AuxData {
    #[serde(default, rename = "Class")]
    pub class: Vec<BasicChildListAttribute>,
    #[serde(default, rename = "Symdef")]
    pub symdef: Vec<Symdef>,
    #[serde(default, rename = "Position")]
    pub position: Vec<BasicChildListAttribute>,
    #[serde(default, rename = "MappingDefinition")]
    pub mapping_definition: Vec<MappingDefinition>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    #[serde(default = "Address::default_break_", rename = "@break")]
    pub break_: i32,
    #[serde(rename = "$text")]
    pub content: i32,
}
impl Address {
    #[must_use]
    pub fn default_break_() -> i32 {
        0i32
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Addresses {
    #[serde(default, rename = "Address")]
    pub address: Vec<Address>,
    #[serde(default, rename = "Network")]
    pub network: Vec<Network>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Alignment {
    #[serde(default, rename = "@geometry")]
    pub geometry: Option<String>,
    #[serde(default = "Alignment::default_up", rename = "@up")]
    pub up: String,
    #[serde(default = "Alignment::default_direction", rename = "@direction")]
    pub direction: String,
}
impl Alignment {
    #[must_use]
    pub fn default_up() -> String {
        String::from("0,0,1")
    }
    #[must_use]
    pub fn default_direction() -> String {
        String::from("0,0,-1")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Alignments {
    #[serde(default, rename = "Alignment")]
    pub alignment: Vec<Alignment>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BasicChildListAttribute {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "BasicChildListAttribute::default_name", rename = "@name")]
    pub name: String,
}
impl BasicChildListAttribute {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChildList {
    #[serde(default, rename = "$value")]
    pub content: Vec<ChildListContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum ChildListContent {
    #[serde(rename = "SceneObject")]
    SceneObject(SceneObject),
    #[serde(rename = "GroupObject")]
    GroupObject(GroupObject),
    #[serde(rename = "FocusPoint")]
    FocusPoint(FocusPoint),
    #[serde(rename = "Fixture")]
    Fixture(Fixture),
    #[serde(rename = "Support")]
    Support(Support),
    #[serde(rename = "Truss")]
    Truss(Truss),
    #[serde(rename = "VideoScreen")]
    VideoScreen(VideoScreen),
    #[serde(rename = "Projector")]
    Projector(Projector),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "@own")]
    pub own: String,
    #[serde(rename = "@other")]
    pub other: String,
    #[serde(rename = "@toObject")]
    pub to_object: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Connections {
    #[serde(default, rename = "Connection")]
    pub connection: Vec<Connection>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomCommands {
    #[serde(default, rename = "CustomCommand")]
    pub custom_command: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename = "@provider")]
    pub provider: String,
    #[serde(default = "Data::default_ver", rename = "@ver")]
    pub ver: String,
}
impl Data {
    #[must_use]
    pub fn default_ver() -> String {
        String::from("1")
    }
}
pub type FileName = String;
#[derive(Debug, Deserialize, Serialize)]
pub struct Fixture {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Fixture::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Fixture::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "Focus")]
    pub focus: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "DMXInvertPan")]
    pub dmx_invert_pan: Option<bool>,
    #[serde(default, rename = "DMXInvertTilt")]
    pub dmx_invert_tilt: Option<bool>,
    #[serde(default, rename = "Position")]
    pub position: Option<String>,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(rename = "UnitNumber")]
    pub unit_number: i32,
    #[serde(default, rename = "ChildPosition")]
    pub child_position: Option<String>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Protocols")]
    pub protocols: Option<Protocols>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "Color")]
    pub color: Option<String>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "Mappings")]
    pub mappings: Option<Mappings>,
    #[serde(default, rename = "Gobo")]
    pub gobo: Option<Gobo>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
}
impl Fixture {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FocusPoint {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "FocusPoint::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
}
impl FocusPoint {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralSceneDescription {
    #[serde(rename = "@verMajor")]
    pub ver_major: i32,
    #[serde(rename = "@verMinor")]
    pub ver_minor: i32,
    #[serde(default, rename = "@provider")]
    pub provider: Option<String>,
    #[serde(default, rename = "@providerVersion")]
    pub provider_version: Option<String>,
    #[serde(default, rename = "UserData")]
    pub user_data: Option<UserData>,
    #[serde(rename = "Scene")]
    pub scene: Scene,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Geometries {
    #[serde(default, rename = "Geometry3D")]
    pub geometry_3d: Vec<Geometry3D>,
    #[serde(default, rename = "Symbol")]
    pub symbol: Vec<Symbol>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry3D {
    #[serde(rename = "@fileName")]
    pub file_name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Gobo {
    #[serde(default = "Gobo::default_rotation", rename = "@rotation")]
    pub rotation: f32,
}
impl Gobo {
    #[must_use]
    pub fn default_rotation() -> f32 {
        0f32
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupObject {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "GroupObject::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "ChildList")]
    pub child_list: Box<ChildList>,
}
impl GroupObject {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}
pub type Ipv4Adress = String;
pub type Ipv6Adress = String;
#[derive(Debug, Deserialize, Serialize)]
pub struct Layer {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Layer::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<ChildList>,
}
impl Layer {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Layers {
    #[serde(default, rename = "Layer")]
    pub layer: Vec<Layer>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Mapping {
    #[serde(rename = "@linkedDef")]
    pub linked_def: String,
    #[serde(default, rename = "ux")]
    pub ux: Option<i32>,
    #[serde(default, rename = "uy")]
    pub uy: Option<i32>,
    #[serde(default, rename = "ox")]
    pub ox: Option<i32>,
    #[serde(default, rename = "oy")]
    pub oy: Option<i32>,
    #[serde(default, rename = "rz")]
    pub rz: Option<f32>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MappingDefinition {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "MappingDefinition::default_name", rename = "@name")]
    pub name: String,
    #[serde(rename = "SizeX")]
    pub size_x: i32,
    #[serde(rename = "SizeY")]
    pub size_y: i32,
    #[serde(rename = "Source")]
    pub source: Source,
    #[serde(default, rename = "ScaleHandeling")]
    pub scale_handeling: Option<ScaleHandeling>,
}
impl MappingDefinition {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Mappings {
    #[serde(default, rename = "Mapping")]
    pub mapping: Vec<Mapping>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    #[serde(rename = "@geometry")]
    pub geometry: String,
    #[serde(default, rename = "@ipv4")]
    pub ipv_4: Option<String>,
    #[serde(default, rename = "@subnetmask")]
    pub subnetmask: Option<String>,
    #[serde(default, rename = "@ipv6")]
    pub ipv_6: Option<String>,
    #[serde(default, rename = "@dhcp")]
    pub dhcp: Option<String>,
    #[serde(default, rename = "@hostname")]
    pub hostname: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Overwrite {
    #[serde(rename = "@universal")]
    pub universal: String,
    #[serde(default = "Overwrite::default_target", rename = "@target")]
    pub target: String,
}
impl Overwrite {
    #[must_use]
    pub fn default_target() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Overwrites {
    #[serde(default, rename = "Overwrite")]
    pub overwrite: Vec<Overwrite>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Projection {
    #[serde(default, rename = "Source")]
    pub source: Vec<Source>,
    #[serde(default, rename = "ScaleHandeling")]
    pub scale_handeling: Vec<ScaleHandeling>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Projections {
    #[serde(default, rename = "Projection")]
    pub projection: Vec<Projection>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Projector {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Projector::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Projector::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(rename = "Projections")]
    pub projections: Projections,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}
impl Projector {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Protocol {
    #[serde(default = "Protocol::default_geometry", rename = "@geometry")]
    pub geometry: String,
    #[serde(default = "Protocol::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Protocol::default_type_", rename = "@type")]
    pub type_: String,
    #[serde(default = "Protocol::default_version", rename = "@version")]
    pub version: String,
    #[serde(default, rename = "@transmission")]
    pub transmission: Option<TransmissionEnum>,
}
impl Protocol {
    #[must_use]
    pub fn default_geometry() -> String {
        String::from("NetworkInOut_1")
    }
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_type_() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_version() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Protocols {
    #[serde(default, rename = "Protocol")]
    pub protocol: Vec<Protocol>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ScaleHandeling {
    #[serde(default = "ScaleHandeling::default_enum_", rename = "@Enum")]
    pub enum_: Scaleenum,
}
impl ScaleHandeling {
    #[must_use]
    pub fn default_enum_() -> Scaleenum {
        Scaleenum::ScaleKeepRatio
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
    #[serde(default, rename = "AUXData")]
    pub aux_data: Option<AuxData>,
    #[serde(rename = "Layers")]
    pub layers: Layers,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SceneObject {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "SceneObject::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "SceneObject::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: Option<String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
}
impl SceneObject {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    #[serde(rename = "@linkedGeometry")]
    pub linked_geometry: String,
    #[serde(rename = "@type")]
    pub type_: SourceEnum,
    #[serde(default, rename = "$text")]
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum SourceEnum {
    #[serde(rename = "NDI")]
    Ndi,
    #[serde(rename = "File")]
    File,
    #[serde(rename = "CITP")]
    Citp,
    #[serde(rename = "CaptureDevice")]
    CaptureDevice,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Sources {
    #[serde(default, rename = "Source")]
    pub source: Vec<Source>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Support {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Support::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Support::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(default, rename = "Position")]
    pub position: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(rename = "ChainLength")]
    pub chain_length: f32,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
}
impl Support {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(rename = "@symdef")]
    pub symdef: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Symdef {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Symdef::default_name", rename = "@name")]
    pub name: String,
    #[serde(rename = "ChildList")]
    pub child_list: Geometries,
}
impl Symdef {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
}
pub type SymdefChildList = Geometries;
#[derive(Debug, Deserialize, Serialize)]
pub enum TransmissionEnum {
    #[serde(rename = "Unicast")]
    Unicast,
    #[serde(rename = "Multicast")]
    Multicast,
    #[serde(rename = "Broadcast")]
    Broadcast,
    #[serde(rename = "Anycast")]
    Anycast,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Truss {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "Truss::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Truss::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(default, rename = "Position")]
    pub position: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "ChildPosition")]
    pub child_position: Option<String>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}
impl Truss {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    #[serde(default, rename = "Data")]
    pub data: Vec<Data>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoScreen {
    #[serde(rename = "@uuid")]
    pub uuid: String,
    #[serde(default = "VideoScreen::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "VideoScreen::default_multipatch", rename = "@multipatch")]
    pub multipatch: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<String>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Sources")]
    pub sources: Option<Sources>,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: Option<Connections>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Option<Box<ChildList>>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}
impl VideoScreen {
    #[must_use]
    pub fn default_name() -> String {
        String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> String {
        String::from("")
    }
}
pub type Ciecolortype = String;
pub type Guidtype = String;
pub type Matrixtype = String;
pub type Positiveinteger = i32;
#[derive(Debug, Deserialize, Serialize)]
pub enum Scaleenum {
    #[serde(rename = "ScaleKeepRatio")]
    ScaleKeepRatio,
    #[serde(rename = "ScaleIgnoreRatio")]
    ScaleIgnoreRatio,
    #[serde(rename = "KeepSizeCenter")]
    KeepSizeCenter,
}
