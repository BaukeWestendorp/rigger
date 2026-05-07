use std::{
    path::PathBuf,
    str::{self, FromStr as _},
};

use uuid::Uuid;

use crate::gdtf::bundle::ResourceKey;

pub mod attr;
pub mod bundle;
pub mod phys;
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

    activation_groups: Vec<attr::ActivationGroup>,
    feature_groups: Vec<attr::FeatureGroup>,
    attributes: Vec<attr::Attribute>,
    wheels: Vec<wheel::Wheel>,

    pub(crate) emitters: Vec<phys::Emitter>,
    pub(crate) filters: Vec<phys::Filter>,
    pub(crate) color_space: Option<phys::ColorSpace>,
    pub(crate) additional_color_spaces: Vec<phys::ColorSpace>,
    pub(crate) gamuts: Vec<phys::Gamut>,
    pub(crate) dmx_profiles: Vec<phys::DmxProfile>,
    pub(crate) cri_groups: Vec<phys::CriGroup>,
    pub(crate) properties: Option<phys::Properties>,
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
        self.name.as_str()
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

    pub fn activation_groups(&self) -> &[attr::ActivationGroup] {
        &self.activation_groups
    }

    pub fn activation_group(&self, name: &str) -> Option<&attr::ActivationGroup> {
        self.activation_groups.iter().find(|o| &o.to_string() == name)
    }

    pub fn feature_groups(&self) -> &[attr::FeatureGroup] {
        &self.feature_groups
    }

    pub fn feature_group(&self, name: &str) -> Option<&attr::FeatureGroup> {
        self.feature_groups.iter().find(|o| o.name.as_str() == name)
    }

    pub fn attributes(&self) -> &[attr::Attribute] {
        &self.attributes
    }

    pub fn attribute(&self, name: &str) -> Option<&attr::Attribute> {
        self.attributes.iter().find(|o| &o.name().to_string() == name)
    }

    pub fn wheels(&self) -> &[wheel::Wheel] {
        &self.wheels
    }

    pub fn wheel(&self, name: &str) -> Option<&wheel::Wheel> {
        self.wheels.iter().find(|o| o.name().is_some_and(|n| n.as_str() == name))
    }

    pub fn emitters(&self) -> &[phys::Emitter] {
        &self.emitters
    }

    pub fn emitter(&self, name: &str) -> Option<&phys::Emitter> {
        self.emitters.iter().find(|o| o.name().as_str() == name)
    }

    pub fn filters(&self) -> &[phys::Filter] {
        &self.filters
    }

    pub fn filter(&self, name: &str) -> Option<&phys::Filter> {
        self.filters.iter().find(|o| o.name().as_str() == name)
    }

    pub fn color_space(&self) -> Option<&phys::ColorSpace> {
        self.color_space.as_ref()
    }

    pub fn additional_color_spaces(&self) -> &[phys::ColorSpace] {
        &self.additional_color_spaces
    }

    pub fn additional_color_space(&self, name: &str) -> Option<&phys::ColorSpace> {
        self.additional_color_spaces.iter().find(|o| o.name().is_some_and(|n| n.as_str() == name))
    }

    pub fn gamuts(&self) -> &[phys::Gamut] {
        &self.gamuts
    }

    pub fn gamut(&self, name: &str) -> Option<&phys::Gamut> {
        self.gamuts.iter().find(|o| o.name().is_some_and(|n| n.as_str() == name))
    }

    pub fn dmx_profiles(&self) -> &[phys::DmxProfile] {
        &self.dmx_profiles
    }

    pub fn dmx_profile(&self, name: &str) -> Option<&phys::DmxProfile> {
        self.dmx_profiles.iter().find(|o| o.name().is_some_and(|n| n.as_str() == name))
    }

    pub fn cri_groups(&self) -> &[phys::CriGroup] {
        &self.cri_groups
    }

    pub fn properties(&self) -> Option<&phys::Properties> {
        self.properties.as_ref()
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

        let activation_groups: Vec<attr::ActivationGroup> = desc
            .fixture_type
            .attribute_definitions
            .activation_groups
            .as_ref()
            .map(|ags| {
                ags.activation_groups
                    .iter()
                    .map(|ag| {
                        let name = Name::new(&ag.name);
                        attr::ActivationGroup::from_str(name.as_str()).unwrap()
                    })
                    .collect()
            })
            .unwrap_or_default();

        let feature_groups: Vec<attr::FeatureGroup> = desc
            .fixture_type
            .attribute_definitions
            .feature_groups
            .feature_groups
            .iter()
            .map(Into::into)
            .collect();

        let attributes: Vec<attr::Attribute> = desc
            .fixture_type
            .attribute_definitions
            .attributes
            .attributes
            .iter()
            .map(Into::into)
            .collect();

        let wheels: Vec<wheel::Wheel> = desc
            .fixture_type
            .wheels
            .as_ref()
            .map(|wheels| wheels.wheels.iter().map(Into::into).collect())
            .unwrap_or_default();

        let emitters = ft
            .physical_descriptions
            .as_ref()
            .map(|pd| {
                pd.emitters
                    .as_ref()
                    .map(|e| e.emitters.iter().map(Into::into).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let filters = ft
            .physical_descriptions
            .as_ref()
            .map(|pd| {
                pd.filters
                    .as_ref()
                    .map(|f| f.filters.iter().map(Into::into).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let color_space = ft
            .physical_descriptions
            .as_ref()
            .map(|pd| pd.color_space.as_ref().map(Into::into))
            .unwrap_or_default();

        let additional_color_spaces = ft
            .physical_descriptions
            .as_ref()
            .map(|pd| {
                pd.additional_color_spaces
                    .as_ref()
                    .map(|acs| acs.color_spaces.iter().map(Into::into).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let gamuts = ft
            .physical_descriptions
            .as_ref()
            .map(|pd| {
                pd.gamuts
                    .as_ref()
                    .map(|g| g.gamuts.iter().map(Into::into).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let dmx_profiles = ft
            .physical_descriptions
            .as_ref()
            .map(|pd| {
                pd.dmx_profiles
                    .as_ref()
                    .map(|d| d.dmx_profiles.iter().map(Into::into).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let cri_groups = ft
            .physical_descriptions
            .as_ref()
            .map(|pd| {
                pd.cr_is
                    .as_ref()
                    .map(|c| c.cri_groups.iter().map(Into::into).collect())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        let properties = ft
            .physical_descriptions
            .as_ref()
            .map(|pd| pd.properties.as_ref().map(Into::into))
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

            emitters,
            filters,
            color_space,
            additional_color_spaces,
            gamuts,
            dmx_profiles,
            cri_groups,
            properties,

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
            .field("emitters", &self.emitters)
            .field("filters", &self.filters)
            .field("color_space", &self.color_space)
            .field("additional_color_spaces", &self.additional_color_spaces)
            .field("gamuts", &self.gamuts)
            .field("dmx_profiles", &self.dmx_profiles)
            .field("cri_groups", &self.cri_groups)
            .field("properties", &self.properties)
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

pub(crate) fn parse_optional_name(s: Option<&str>) -> Option<Name> {
    let name = s.as_ref().map(|s| s.trim());
    match &name {
        Some(s) if s.is_empty() => None,
        Some(s) => Some(Name::new(*s)),
        None => None,
    }
}
