use crate::mvr::{
    self as mvr,
    geo::Geometry,
    layer::{ScaleHandling, Source},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) id: mvr::NodeId<Class>,
}

impl Class {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> mvr::NodeId<Class> {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub(crate) name: String,
    pub(crate) id: mvr::NodeId<Position>,
}

impl Position {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> mvr::NodeId<Position> {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symdef {
    pub(crate) name: String,
    pub(crate) id: mvr::NodeId<Symdef>,

    pub(crate) geometries: Vec<Geometry>,
}

impl Symdef {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> mvr::NodeId<Symdef> {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MappingDefinition {
    pub(crate) name: String,
    pub(crate) id: mvr::NodeId<MappingDefinition>,

    pub(crate) size_x: u32,
    pub(crate) size_y: u32,
    pub(crate) source: Source,
    pub(crate) scale_handling: ScaleHandling,
}

impl MappingDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> mvr::NodeId<MappingDefinition> {
        self.id
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
