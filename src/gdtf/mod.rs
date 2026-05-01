use std::{path::PathBuf, str::FromStr};

use uuid::Uuid;

use crate::gdtf::bundle::ResourceKey;

pub mod bundle;

mod builder;

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
}

impl Gdtf {
    pub fn new(bundle: bundle::Bundle) -> Self {
        builder::GdtfBuilder::new(bundle).build()
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

impl FromStr for FixtureTypeId {
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
    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

pub struct Thumbnail {
    pub(crate) resource: ResourceKey,
    pub(crate) offset_x: i32,
    pub(crate) offset_y: i32,
}

impl Thumbnail {
    pub fn resource(&self) -> &ResourceKey {
        &self.resource
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

impl std::str::FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Node(Vec::new()));
        }
        let parts: Vec<String> = s.split('.').map(|part| part.trim().to_string()).collect();
        Ok(Node(parts))
    }
}
