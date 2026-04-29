pub mod bundle;

use std::{collections::HashMap, path::PathBuf, str::FromStr};

use uuid::Uuid;

pub mod aux;
mod builder;
pub mod geo;
pub mod layer;

use crate::mvr::{
    aux::{Class, MappingDefinition, Position, Symdef},
    layer::Layer,
};

pub struct Mvr {
    bundle: bundle::Bundle,

    version: Version,
    provider: Provider,

    symdefs: HashMap<Uuid, Symdef>,
    classes: HashMap<Uuid, Class>,
    mapping_definitions: HashMap<Uuid, MappingDefinition>,
    positions: HashMap<Uuid, Position>,

    layers: HashMap<Uuid, Layer>,
}

impl Mvr {
    pub fn new(bundle: bundle::Bundle) -> Self {
        builder::MvrBuilder::new(bundle).build()
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

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    pub fn symdefs(&self) -> impl Iterator<Item = &Symdef> {
        self.symdefs.values()
    }

    pub fn symdef(&self, uuid: Uuid) -> Option<&Symdef> {
        self.symdefs.get(&uuid)
    }

    pub fn classes(&self) -> impl Iterator<Item = &Class> {
        self.classes.values()
    }

    pub fn class(&self, uuid: Uuid) -> Option<&Class> {
        self.classes.get(&uuid)
    }

    pub fn positions(&self) -> impl Iterator<Item = &Position> {
        self.positions.values()
    }

    pub fn position(&self, uuid: Uuid) -> Option<&Position> {
        self.positions.get(&uuid)
    }

    pub fn mapping_definitions(&self) -> impl Iterator<Item = &MappingDefinition> {
        self.mapping_definitions.values()
    }

    pub fn mapping_definition(&self, uuid: Uuid) -> Option<&MappingDefinition> {
        self.mapping_definitions.get(&uuid)
    }

    pub fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.layers.values()
    }

    pub fn layer(&self, uuid: Uuid) -> Option<&Layer> {
        self.layers.get(&uuid)
    }
}

impl std::fmt::Debug for Mvr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mvr")
            .field("version", &self.version)
            .field("provider", &self.provider)
            .field("symdefs", &self.symdefs)
            .field("classes", &self.classes)
            .field("positions", &self.positions)
            .field("layers", &self.layers)
            .finish()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provider {
    name: String,
    version: String,
}

impl Provider {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId<T> {
    uuid: Uuid,
    _marker: std::marker::PhantomData<T>,
}

impl<T> NodeId<T> {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid, _marker: std::marker::PhantomData }
    }

    pub fn as_uuid(&self) -> Uuid {
        self.uuid
    }
}

impl<T> From<Uuid> for NodeId<T> {
    fn from(uuid: Uuid) -> Self {
        Self { uuid, _marker: std::marker::PhantomData }
    }
}

impl<T> FromStr for NodeId<T> {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(Uuid::from_str(s)?))
    }
}
