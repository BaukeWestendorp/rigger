use std::{
    collections::HashMap,
    path::PathBuf,
    str::{self, FromStr},
};

use uuid::Uuid;

use crate::gdtf::bundle::ResourceKey;

pub mod attr;
pub mod bundle;
pub mod wheel;

pub struct Gdtf {
    bundle: bundle::Bundle,

    version: Version,

    name: String,
    short_name: Option<String>,
    long_name: Option<String>,
    manufacturer: String,
    description: String,

    fixture_type_id: FixtureTypeId,
    reference_fixture_type_id: Option<FixtureTypeId>,

    thumbnail: Thumbnail,

    can_have_children: bool,

    activation_groups: HashMap<String, attr::ActivationGroup>,
    feature_groups: HashMap<String, attr::FeatureGroup>,
    attributes: HashMap<String, attr::Attribute>,
    wheels: HashMap<String, wheel::Wheel>,
}

impl Gdtf {
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

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }

    pub fn long_name(&self) -> Option<&str> {
        self.long_name.as_deref()
    }

    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn fixture_type_id(&self) -> &FixtureTypeId {
        &self.fixture_type_id
    }

    pub fn reference_fixture_type_id(&self) -> Option<&FixtureTypeId> {
        self.reference_fixture_type_id.as_ref()
    }

    pub fn thumbnail(&self) -> &Thumbnail {
        &self.thumbnail
    }

    pub fn can_have_children(&self) -> bool {
        self.can_have_children
    }

    pub fn activation_groups(&self) -> impl Iterator<Item = &attr::ActivationGroup> {
        self.activation_groups.values()
    }

    pub fn activation_group(&self, name: &str) -> Option<&attr::ActivationGroup> {
        self.activation_groups.get(name)
    }

    pub fn feature_groups(&self) -> impl Iterator<Item = &attr::FeatureGroup> {
        self.feature_groups.values()
    }

    pub fn feature_group(&self, name: &str) -> Option<&attr::FeatureGroup> {
        self.feature_groups.get(name)
    }

    pub fn attributes(&self) -> impl Iterator<Item = &attr::Attribute> {
        self.attributes.values()
    }

    pub fn attribute(&self, name: &str) -> Option<&attr::Attribute> {
        self.attributes.get(name)
    }

    pub fn wheels(&self) -> impl Iterator<Item = &wheel::Wheel> {
        self.wheels.values()
    }

    pub fn wheel(&self, name: &str) -> Option<&wheel::Wheel> {
        self.wheels.get(name)
    }
}

impl From<bundle::Bundle> for Gdtf {
    fn from(bundle: bundle::Bundle) -> Self {
        let desc = bundle.description();
        let ft = &desc.fixture_type;

        let version: Version = desc.data_version.as_str().into();

        let fixture_type_id = FixtureTypeId::from_str(&ft.fixture_type_id).unwrap();

        let reference_fixture_type_id =
            ft.ref_ft.as_deref().and_then(|s| FixtureTypeId::from_str(s).ok());

        let thumbnail = Thumbnail {
            resources: bundle
                .resources()
                .entries()
                .filter(|r| r.kind == bundle::ResourceKind::Thumbnail)
                .map(|r| r.key.clone())
                .collect(),
            offset_x: ft.thumbnail_offset_x.unwrap_or(0),
            offset_y: ft.thumbnail_offset_y.unwrap_or(0),
        };

        let activation_groups: HashMap<String, attr::ActivationGroup> = desc
            .fixture_type
            .attribute_definitions
            .activation_groups
            .as_ref()
            .map(|ags| {
                ags.activation_groups
                    .iter()
                    .map(|ag| {
                        let name = ag.name.as_str();
                        let ag = attr::ActivationGroup::from_str(name).unwrap();
                        (name.to_owned(), ag)
                    })
                    .collect()
            })
            .unwrap_or_default();

        let feature_groups: HashMap<String, attr::FeatureGroup> = desc
            .fixture_type
            .attribute_definitions
            .feature_groups
            .feature_groups
            .clone()
            .into_iter()
            .map(|fg| {
                let fg: attr::FeatureGroup = fg.into();
                (fg.name.to_string(), fg)
            })
            .collect();

        let attributes: HashMap<String, attr::Attribute> = desc
            .fixture_type
            .attribute_definitions
            .attributes
            .attributes
            .clone()
            .into_iter()
            .map(|attr| {
                let attr: attr::Attribute = attr.into();
                (attr.name.to_string(), attr)
            })
            .collect();

        let wheels: HashMap<String, wheel::Wheel> = desc
            .fixture_type
            .wheels
            .clone()
            .map(|wheels| {
                wheels
                    .wheels
                    .into_iter()
                    .map(|wheel| {
                        let wheel: wheel::Wheel = wheel.into();
                        (wheel.name.to_string(), wheel)
                    })
                    .collect()
            })
            .unwrap_or_default();

        Self {
            version,
            name: ft.name.clone(),
            short_name: ft.short_name.clone(),
            long_name: ft.long_name.clone(),
            manufacturer: ft.manufacturer.clone(),
            description: ft.description.clone(),
            fixture_type_id,
            reference_fixture_type_id,
            thumbnail,
            can_have_children: ft.can_have_children.clone().into(),
            activation_groups,
            feature_groups,
            attributes,
            wheels,
            bundle,
        }
    }
}

impl std::fmt::Debug for Gdtf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gdtf")
            .field("version", &self.version)
            .field("name", &self.name)
            .field("short_name", &self.short_name)
            .field("long_name", &self.long_name)
            .field("manufacturer", &self.manufacturer)
            .field("description", &self.description)
            .field("fixture_type_id", &self.fixture_type_id)
            .field("reference_fixture_type_id", &self.reference_fixture_type_id)
            .field("thumbnail", &self.thumbnail)
            .field("can_have_children", &self.can_have_children)
            .field("activation_groups", &self.activation_groups)
            .field("feature_groups", &self.feature_groups)
            .field("attributes", &self.attributes)
            .field("wheels", &self.wheels)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixtureTypeId(Uuid);

impl FixtureTypeId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for FixtureTypeId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl str::FromStr for FixtureTypeId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(Uuid::from_str(s)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    major: u32,
    minor: u32,
}

impl From<(u32, u32)> for Version {
    fn from((major, minor): (u32, u32)) -> Self {
        Self { major, minor }
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        let mut parts = value.splitn(2, '.');
        let major = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        let minor = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        Self { major, minor }
    }
}

impl Version {
    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Thumbnail {
    pub(crate) resources: Vec<ResourceKey>,
    pub(crate) offset_x: i32,
    pub(crate) offset_y: i32,
}

impl Thumbnail {
    pub fn resources(&self) -> &[ResourceKey] {
        &self.resources
    }

    pub fn offset_x(&self) -> i32 {
        self.offset_x
    }

    pub fn offset_y(&self) -> i32 {
        self.offset_y
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node(Vec<String>);

impl Node {
    pub fn parts(&self) -> &[String] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_dotted_string(&self) -> String {
        self.0.join(".")
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_dotted_string())
    }
}

impl str::FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Node(Vec::new()));
        }
        let parts: Vec<String> = s.split('.').map(|part| part.trim().to_string()).collect();
        Ok(Node(parts))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name(String);

impl Name {
    pub fn new(value: impl Into<String>) -> Self {
        // FIXME: Validate name.
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
