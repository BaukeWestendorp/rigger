use uuid::Uuid;

use crate::mvr::Geometry;

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
