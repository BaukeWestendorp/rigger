use uuid::Uuid;

use crate::mvr::{
    self as mvr, Node, NodeId, bundle,
    geo::Geometry,
    layer::{ScaleHandling, Source},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    name: String,
    id: NodeId<Class>,
}

impl bundle::FromBundle for Class {
    type Source = bundle::BasicChildListAttribute;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        let id: NodeId<Self> = Uuid::parse_str(&source.uuid).unwrap().into();
        Self { name: source.name.clone(), id }
    }
}

impl Class {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Node for Class {
    fn id(&self) -> NodeId<Self> {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    name: String,
    id: NodeId<Position>,
}

impl bundle::FromBundle for Position {
    type Source = bundle::BasicChildListAttribute;

    fn from_bundle(source: &Self::Source, _bundle: &bundle::Bundle) -> Self {
        let id: NodeId<Self> = Uuid::parse_str(&source.uuid).unwrap().into();
        Self { name: source.name.clone(), id }
    }
}

impl Position {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Node for Position {
    fn id(&self) -> NodeId<Self> {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symdef {
    name: String,
    id: NodeId<Symdef>,

    geometries: Vec<Geometry>,
}

impl Symdef {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn geometries(&self) -> &[Geometry] {
        &self.geometries
    }
}

impl Node for Symdef {
    fn id(&self) -> NodeId<Self> {
        self.id
    }
}

impl bundle::FromBundle for Symdef {
    type Source = bundle::Symdef;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let name = source.name.to_string();
        let id: NodeId<Self> = Uuid::parse_str(&source.uuid).unwrap().into();
        let geometries = mvr::build_geometries(
            &source.child_list.geometry_3d,
            &source.child_list.symbol,
            bundle,
        );

        Self { name, id, geometries }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MappingDefinition {
    name: String,
    id: NodeId<MappingDefinition>,

    size_x: u32,
    size_y: u32,
    source: Source,
    scale_handling: ScaleHandling,
}

impl bundle::FromBundle for MappingDefinition {
    type Source = bundle::MappingDefinition;

    fn from_bundle(source: &Self::Source, bundle: &bundle::Bundle) -> Self {
        let id: NodeId<Self> = Uuid::parse_str(&source.uuid).unwrap().into();

        Self {
            name: source.name.clone(),
            id,
            size_x: source.size_x as u32,
            size_y: source.size_y as u32,
            source: Source::from_bundle(&source.source, bundle),
            scale_handling: source
                .scale_handeling
                .as_ref()
                .map(|sh| ScaleHandling::from_bundle(&sh, bundle))
                .unwrap_or_default(),
        }
    }
}

impl MappingDefinition {
    pub fn name(&self) -> &str {
        &self.name
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

impl Node for MappingDefinition {
    fn id(&self) -> NodeId<Self> {
        self.id
    }
}
