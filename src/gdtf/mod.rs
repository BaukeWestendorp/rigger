use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::{self, FromStr as _},
};

use uuid::Uuid;

use crate::gdtf::bundle::FromBundle as _;

pub mod bundle;

mod attr;
mod model;
mod phys;
mod resource;
mod wheel;

pub use attr::*;
pub use model::*;
pub use phys::*;
pub use resource::*;
pub use wheel::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Gdtf {
    version: Version,

    name: String,
    short_name: Option<String>,
    long_name: Option<String>,
    manufacturer: String,
    description: String,
    fixture_type_id: FixtureTypeId,
    reference_fixture_type_id: Option<FixtureTypeId>,
    thumbnail_offset: glam::I16Vec2,
    can_have_children: bool,

    activation_groups: NodeContainer<attr::ActivationGroup>,
    feature_groups: NodeContainer<attr::FeatureGroup>,
    attributes: NodeContainer<attr::Attribute>,

    wheels: NodeContainer<wheel::Wheel>,

    emitters: NodeContainer<phys::Emitter>,
    filters: NodeContainer<phys::Filter>,
    color_space: Option<phys::ColorSpace>,
    additional_color_spaces: NodeContainer<phys::ColorSpace>,
    gamuts: NodeContainer<phys::Gamut>,
    dmx_profiles: NodeContainer<phys::DmxProfile>,
    cri_groups: Vec<phys::CriGroup>,
    properties: phys::Properties,

    models: NodeContainer<model::Model>,

    resources: Resources,
}

impl Gdtf {
    pub fn new(
        name: impl Into<String>,
        manufacturer: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            version: Version::new(1, 2),
            name: name.into(),
            short_name: None,
            long_name: None,
            manufacturer: manufacturer.into(),
            description: description.into(),
            fixture_type_id: FixtureTypeId::new(Uuid::new_v4()),
            reference_fixture_type_id: None,
            thumbnail_offset: glam::I16Vec2::new(0, 0),
            can_have_children: false,
            activation_groups: NodeContainer::new(),
            feature_groups: NodeContainer::new(),
            attributes: NodeContainer::new(),
            wheels: NodeContainer::new(),
            emitters: NodeContainer::new(),
            filters: NodeContainer::new(),
            color_space: None,
            additional_color_spaces: NodeContainer::new(),
            gamuts: NodeContainer::new(),
            dmx_profiles: NodeContainer::new(),
            cri_groups: Vec::new(),
            properties: Properties::new(),
            models: NodeContainer::new(),
            resources: Resources::default(),
        }
    }

    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        Self::from(&bundle::Bundle::from_folder(path))
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        Self::from(&bundle::Bundle::from_archive(path))
    }

    pub fn from_archive_bytes(bytes: &[u8]) -> Self {
        Self::from(&bundle::Bundle::from_archive_bytes(bytes))
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }

    pub fn set_short_name(&mut self, short_name: Option<impl Into<String>>) {
        self.short_name = short_name.map(|s| s.into());
    }

    pub fn long_name(&self) -> Option<&str> {
        self.long_name.as_deref()
    }

    pub fn set_long_name(&mut self, long_name: Option<impl Into<String>>) {
        self.long_name = long_name.map(|s| s.into());
    }

    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    pub fn set_manufacturer(&mut self, manufacturer: impl Into<String>) {
        self.manufacturer = manufacturer.into();
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.description = description.into();
    }

    pub fn fixture_type_id(&self) -> &FixtureTypeId {
        &self.fixture_type_id
    }

    pub fn set_fixture_type_id(&mut self, fixture_type_id: impl Into<FixtureTypeId>) {
        self.fixture_type_id = fixture_type_id.into();
    }

    pub fn reference_fixture_type_id(&self) -> Option<&FixtureTypeId> {
        self.reference_fixture_type_id.as_ref()
    }

    pub fn set_reference_fixture_type_id(
        &mut self,
        reference_fixture_type_id: Option<impl Into<FixtureTypeId>>,
    ) {
        self.reference_fixture_type_id = reference_fixture_type_id.map(Into::into);
    }

    pub fn thumbnail_offset(&self) -> glam::I16Vec2 {
        self.thumbnail_offset
    }

    pub fn set_thumbnail_offset(&mut self, thumbnail_offset: glam::I16Vec2) {
        self.thumbnail_offset = thumbnail_offset;
    }

    pub fn can_have_children(&self) -> bool {
        self.can_have_children
    }

    pub fn set_can_have_children(&mut self, can_have_children: bool) {
        self.can_have_children = can_have_children;
    }

    pub fn activation_groups(&self) -> &NodeContainer<attr::ActivationGroup> {
        &self.activation_groups
    }

    pub fn activation_groups_mut(&mut self) -> &mut NodeContainer<attr::ActivationGroup> {
        &mut self.activation_groups
    }

    pub fn feature_groups(&self) -> &NodeContainer<attr::FeatureGroup> {
        &self.feature_groups
    }

    pub fn feature_groups_mut(&mut self) -> &mut NodeContainer<attr::FeatureGroup> {
        &mut self.feature_groups
    }

    pub fn attributes(&self) -> &NodeContainer<attr::Attribute> {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut NodeContainer<attr::Attribute> {
        &mut self.attributes
    }

    pub fn wheels(&self) -> &NodeContainer<wheel::Wheel> {
        &self.wheels
    }

    pub fn wheels_mut(&mut self) -> &mut NodeContainer<wheel::Wheel> {
        &mut self.wheels
    }

    pub fn emitters(&self) -> &NodeContainer<phys::Emitter> {
        &self.emitters
    }

    pub fn emitters_mut(&mut self) -> &mut NodeContainer<phys::Emitter> {
        &mut self.emitters
    }

    pub fn filters(&self) -> &NodeContainer<phys::Filter> {
        &self.filters
    }

    pub fn filters_mut(&mut self) -> &mut NodeContainer<phys::Filter> {
        &mut self.filters
    }

    pub fn color_space(&self) -> Option<&phys::ColorSpace> {
        self.color_space.as_ref()
    }

    pub fn set_color_space(&mut self, color_space: Option<ColorSpace>) {
        self.color_space = color_space
    }

    pub fn additional_color_spaces(&self) -> &NodeContainer<phys::ColorSpace> {
        &self.additional_color_spaces
    }

    pub fn additional_color_spaces_mut(&mut self) -> &mut NodeContainer<phys::ColorSpace> {
        &mut self.additional_color_spaces
    }

    pub fn gamuts(&self) -> &NodeContainer<phys::Gamut> {
        &self.gamuts
    }

    pub fn gamuts_mut(&mut self) -> &mut NodeContainer<phys::Gamut> {
        &mut self.gamuts
    }

    pub fn dmx_profiles(&self) -> &NodeContainer<phys::DmxProfile> {
        &self.dmx_profiles
    }

    pub fn dmx_profiles_mut(&mut self) -> &mut NodeContainer<phys::DmxProfile> {
        &mut self.dmx_profiles
    }

    pub fn cri_groups(&self) -> &[phys::CriGroup] {
        &self.cri_groups
    }

    pub fn cri_groups_mut(&mut self) -> &mut Vec<phys::CriGroup> {
        &mut self.cri_groups
    }

    pub fn properties(&self) -> &phys::Properties {
        &self.properties
    }

    pub fn properties_mut(&mut self) -> &mut phys::Properties {
        &mut self.properties
    }

    pub fn models(&self) -> &NodeContainer<model::Model> {
        &self.models
    }

    pub fn models_mut(&mut self) -> &mut NodeContainer<model::Model> {
        &mut self.models
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
}

impl From<&bundle::Bundle> for Gdtf {
    fn from(bundle: &bundle::Bundle) -> Self {
        let ft = &bundle.description().fixture_type;

        let mut gdtf = Gdtf::new(&ft.name, &ft.manufacturer, &ft.description);

        gdtf.set_version(bundle.description().data_version.as_str().into());

        gdtf.set_long_name(ft.long_name.as_ref());
        gdtf.set_short_name(ft.short_name.as_ref());

        gdtf.set_fixture_type_id(FixtureTypeId::from_str(&ft.fixture_type_id).unwrap());

        let ref_ft = ft.ref_ft.as_deref().and_then(|s| FixtureTypeId::from_str(s).ok());
        gdtf.set_reference_fixture_type_id(ref_ft);

        gdtf.set_thumbnail_offset(glam::I16Vec2::new(
            ft.thumbnail_offset_x.unwrap_or(0) as i16,
            ft.thumbnail_offset_y.unwrap_or(0) as i16,
        ));

        gdtf.set_can_have_children(ft.can_have_children == bundle::YesNo::Yes);

        if let Some(ags) = &ft.attribute_definitions.activation_groups {
            for ag in &ags.activation_groups {
                gdtf.activation_groups_mut().add(attr::ActivationGroup::from_bundle(ag, bundle));
            }
        }

        for fg in &ft.attribute_definitions.feature_groups.feature_groups {
            gdtf.feature_groups_mut().add(attr::FeatureGroup::from_bundle(fg, bundle));
        }

        for attr in &ft.attribute_definitions.attributes.attributes {
            gdtf.attributes_mut().add(attr::Attribute::from_bundle(attr, bundle));
        }

        if let Some(ws) = &ft.wheels {
            for wheel in &ws.wheels {
                gdtf.wheels_mut().add(wheel::Wheel::from_bundle(wheel, bundle));
            }
        };

        if let Some(pd) = &ft.physical_descriptions {
            if let Some(e) = &pd.emitters {
                for emitter in &e.emitters {
                    gdtf.emitters_mut().add(phys::Emitter::from_bundle(emitter, bundle));
                }
            }
        }

        if let Some(pd) = &ft.physical_descriptions {
            if let Some(f) = &pd.filters {
                for filter in &f.filters {
                    gdtf.filters_mut().add(phys::Filter::from_bundle(filter, bundle));
                }
            }
        }

        let color_space = ft.physical_descriptions.as_ref().and_then(|pd| {
            pd.color_space.as_ref().map(|cs| phys::ColorSpace::from_bundle(cs, bundle))
        });
        gdtf.set_color_space(color_space);

        if let Some(pd) = &ft.physical_descriptions {
            if let Some(acs) = &pd.additional_color_spaces {
                for cs in &acs.color_spaces {
                    gdtf.additional_color_spaces_mut()
                        .add(phys::ColorSpace::from_bundle(cs, bundle));
                }
            }
        }

        if let Some(pd) = &ft.physical_descriptions {
            if let Some(gs) = &pd.gamuts {
                for g in &gs.gamuts {
                    gdtf.gamuts_mut().add(phys::Gamut::from_bundle(g, bundle));
                }
            }
        }

        if let Some(pd) = &ft.physical_descriptions {
            if let Some(dps) = &pd.dmx_profiles {
                for dp in &dps.dmx_profiles {
                    gdtf.dmx_profiles_mut().add(phys::DmxProfile::from_bundle(dp, bundle));
                }
            }
        }

        if let Some(pd) = &ft.physical_descriptions {
            if let Some(cris) = &pd.cr_is {
                for cri in &cris.cri_groups {
                    gdtf.cri_groups_mut().push(phys::CriGroup::from_bundle(cri, bundle));
                }
            }
        }

        let properties = ft
            .physical_descriptions
            .as_ref()
            .and_then(|pd| pd.properties.as_ref().map(|p| phys::Properties::from_bundle(p, bundle)))
            .unwrap_or_default();
        *gdtf.properties_mut() = properties;

        if let Some(ms) = &ft.models {
            for model in &ms.models {
                gdtf.models_mut().add(model::Model::from_bundle(model, bundle));
            }
        };

        if let Some(thumbnail_png) = bundle.resources().get(Path::new("thumbnail.png")) {
            gdtf.resources_mut().set_thumbnail_png(ThumbnailResource::new(thumbnail_png.clone()));
        }

        if let Some(thumbnail_svg) = bundle.resources().get(Path::new("thumbnail.svg")) {
            gdtf.resources_mut().set_thumbnail_svg(ThumbnailResource::new(thumbnail_svg.clone()));
        }

        gdtf
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

impl Version {
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodePath(Vec<String>);

impl NodePath {
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

impl std::fmt::Display for NodePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_dotted_string())
    }
}

impl str::FromStr for NodePath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(NodePath(Vec::new()));
        }
        let parts: Vec<String> = s.split('.').map(|part| part.trim().to_string()).collect();
        Ok(NodePath(parts))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct NodeContainer<T: Node> {
    items: Vec<T>,
    index: HashMap<Name, usize>,
}

impl<T: Node> NodeContainer<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, node: T) {
        // FIXME: Return error if it already exists.

        let index = self.items.len();

        // We can only add this to the index if it has a name.
        // If it does not have a name, it also cannot be found by name.
        if let Some(name) = node.name() {
            self.index.insert(name.to_owned(), index);
        };

        self.items.push(node);
    }

    pub fn get(&self, name: &Name) -> Option<&T> {
        let ix = self.index.get(name)?;
        self.items.get(*ix)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn names(&self) -> impl Iterator<Item = &Name> {
        self.index.keys()
    }
}

impl<T: Node> Default for NodeContainer<T> {
    fn default() -> Self {
        Self { items: Vec::default(), index: HashMap::default() }
    }
}

pub trait Node {
    fn name(&self) -> Option<Name>;
}
