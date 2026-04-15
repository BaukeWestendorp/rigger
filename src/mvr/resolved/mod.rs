use std::{collections::HashMap, path::PathBuf, sync::Arc};

use uuid::Uuid;

mod aux;
mod builder;
mod geo;
mod layer;

pub use aux::*;
pub use geo::*;
pub use layer::*;

use crate::mvr::bundle;

pub struct Mvr {
    bundle: bundle::Bundle,

    version: Version,
    provider: Provider,

    symdefs: HashMap<Uuid, Arc<Symdef>>,
    classes: HashMap<Uuid, Arc<Class>>,
    // FIXME: Implement MappingDefinition mapping_definitions: HashMap<Uuid, Arc<MappingDefinition>>,
    positions: HashMap<Uuid, Arc<Position>>,

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
        self.symdefs.values().map(|v| &**v)
    }

    pub fn symdef(&self, uuid: Uuid) -> Option<&Symdef> {
        self.symdefs.get(&uuid).map(|v| &**v)
    }

    pub fn classes(&self) -> impl Iterator<Item = &Class> {
        self.classes.values().map(|v| &**v)
    }

    pub fn class(&self, uuid: Uuid) -> Option<&Class> {
        self.classes.get(&uuid).map(|v| &**v)
    }

    pub fn positions(&self) -> impl Iterator<Item = &Position> {
        self.positions.values().map(|v| &**v)
    }

    pub fn position(&self, uuid: Uuid) -> Option<&Position> {
        self.positions.get(&uuid).map(|v| &**v)
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
    major: i32,
    minor: i32,
}

impl Version {
    pub fn major(&self) -> i32 {
        self.major
    }

    pub fn minor(&self) -> i32 {
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
