use uuid::Uuid;

use crate::mvr::{
    geo::Geometry,
    layer::{ScaleHandling, Source},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) uuid: Uuid,
}

impl Class {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub(crate) name: String,
    pub(crate) uuid: Uuid,
}

impl Position {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symdef {
    pub(crate) name: String,
    pub(crate) uuid: Uuid,

    pub(crate) geometries: Vec<Geometry>,
}

impl Symdef {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MappingDefinition {
    pub(crate) name: String,
    pub(crate) uuid: Uuid,

    pub(crate) size_x: u32,
    pub(crate) size_y: u32,
    pub(crate) source: Source,
    pub(crate) scale_handling: ScaleHandling,
}

impl MappingDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn size_x(&self) -> u32 {
        self.size_x
    }

    pub fn size_y(&self) -> u32 {
        self.size_y
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn scale_handling(&self) -> ScaleHandling {
        self.scale_handling
    }
}
