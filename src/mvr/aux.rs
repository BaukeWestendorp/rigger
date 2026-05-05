use crate::mvr::{
    self as mvr, bundle,
    geo::Geometry,
    layer::{ScaleHandling, Source},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) id: mvr::NodeId<Class>,
}

impl From<&bundle::BasicChildListAttribute> for Class {
    fn from(value: &bundle::BasicChildListAttribute) -> Self {
        let id: mvr::NodeId<Class> = uuid::Uuid::parse_str(&value.uuid).unwrap().into();
        Self { name: value.name.clone(), id }
    }
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

impl From<&bundle::BasicChildListAttribute> for Position {
    fn from(value: &bundle::BasicChildListAttribute) -> Self {
        let id: mvr::NodeId<Position> = uuid::Uuid::parse_str(&value.uuid).unwrap().into();
        Self { name: value.name.clone(), id }
    }
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

impl From<&bundle::MappingDefinition> for MappingDefinition {
    fn from(value: &bundle::MappingDefinition) -> Self {
        let id: mvr::NodeId<MappingDefinition> = uuid::Uuid::parse_str(&value.uuid).unwrap().into();

        Self {
            name: value.name.clone(),
            id,
            size_x: value.size_x as u32,
            size_y: value.size_y as u32,
            source: (&value.source).into(),
            scale_handling: value
                .scale_handeling
                .as_ref()
                .map(|sh| (&sh.r#enum).into())
                .unwrap_or_default(),
        }
    }
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
