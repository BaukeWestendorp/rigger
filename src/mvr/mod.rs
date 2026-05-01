use std::{collections::HashMap, fmt::Debug, path::PathBuf, str::FromStr};

use uuid::Uuid;

use crate::gdtf;

pub mod aux;
pub mod bundle;
pub mod geo;
pub mod layer;

mod builder;

pub use aux::{Class, MappingDefinition, Position, Symdef};
pub use geo::Geometry;
pub use layer::{GdtfInfo, Layer, Object, ObjectKind};

pub struct Mvr {
    bundle: bundle::Bundle,

    version: Version,
    provider: Provider,

    symdefs: HashMap<NodeId<Symdef>, Symdef>,
    classes: HashMap<NodeId<Class>, Class>,
    mapping_definitions: HashMap<NodeId<MappingDefinition>, MappingDefinition>,
    positions: HashMap<NodeId<Position>, Position>,

    layers: Vec<Layer>,
    layers_ix: HashMap<NodeId<Layer>, usize>,
    objects_path_ix: HashMap<NodeId<Object>, ObjectPath>,

    gdtfs: HashMap<bundle::ResourceKey, gdtf::Gdtf>,
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

    pub fn symdef(&self, id: NodeId<Symdef>) -> Option<&Symdef> {
        self.symdefs.get(&id)
    }

    pub fn classes(&self) -> impl Iterator<Item = &Class> {
        self.classes.values()
    }

    pub fn class(&self, id: NodeId<Class>) -> Option<&Class> {
        self.classes.get(&id)
    }

    pub fn positions(&self) -> impl Iterator<Item = &Position> {
        self.positions.values()
    }

    pub fn position(&self, id: NodeId<Position>) -> Option<&Position> {
        self.positions.get(&id)
    }

    pub fn mapping_definitions(&self) -> impl Iterator<Item = &MappingDefinition> {
        self.mapping_definitions.values()
    }

    pub fn mapping_definition(&self, id: NodeId<MappingDefinition>) -> Option<&MappingDefinition> {
        self.mapping_definitions.get(&id)
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    pub fn layer(&self, id: NodeId<Layer>) -> Option<&Layer> {
        let layer_ix = *self.layers_ix.get(&id)?;
        Some(&self.layers[layer_ix])
    }

    pub fn object(&self, id: NodeId<Object>) -> Option<&Object> {
        let path = self.objects_path_ix.get(&id)?;
        self.object_by_path(path)
    }

    pub(crate) fn object_path(&self, id: NodeId<Object>) -> Option<&ObjectPath> {
        self.objects_path_ix.get(&id)
    }

    pub(crate) fn object_by_path(&self, path: &ObjectPath) -> Option<&Object> {
        let layer_ix = *self.layers_ix.get(&path.layer_id)?;
        let layer = self.layers.get(layer_ix)?;

        let mut indices = path.indices.iter();
        let first = *indices.next()?;
        let mut object = layer.objects.get(first)?;

        for &ix in indices {
            let child_objects = object.child_objects()?;
            object = child_objects.get(ix)?;
        }

        Some(object)
    }

    pub fn object_world_transform(&self, id: NodeId<Object>) -> Option<glam::Affine3A> {
        let path = self.object_path(id)?;
        self.object_world_transform_by_path(path)
    }

    fn object_world_transform_by_path(&self, path: &ObjectPath) -> Option<glam::Affine3A> {
        let layer_ix = *self.layers_ix.get(&path.layer_id)?;
        let layer = self.layers.get(layer_ix)?;

        let mut indices = path.indices.iter();
        let first = *indices.next()?;
        let mut object = layer.objects.get(first)?;
        let mut transform = *layer.local_transform() * *object.local_transform();

        for &ix in indices {
            let child_objects = object.child_objects()?;
            object = child_objects.get(ix)?;
            transform = transform * *object.local_transform();
        }

        Some(transform)
    }

    pub fn object_geometries_world<'a>(
        &'a self,
        id: NodeId<Object>,
    ) -> Option<impl Iterator<Item = (&'a geo::Geometry, glam::Affine3A)> + 'a> {
        let path = self.object_path(id)?;
        let world = self.object_world_transform_by_path(path)?;
        let object = self.object_by_path(path)?;
        let geometries = object.geometries()?;

        Some(geometries.iter().map(move |g| (g, world * g.local_transform())))
    }

    pub fn gdtfs(&self) -> impl Iterator<Item = (&bundle::ResourceKey, &gdtf::Gdtf)> {
        self.gdtfs.iter()
    }

    pub fn gdtf(&self, file_name: &str) -> Option<&gdtf::Gdtf> {
        let key = self.gdtf_resource_key(file_name)?;
        self.gdtfs.get(&key)
    }

    fn gdtf_resource_key(&self, gdtf_spec: &str) -> Option<bundle::ResourceKey> {
        let key = bundle::ResourceKey::new(gdtf_spec);
        if self.bundle.resources().contains_key(&key) {
            return Some(key);
        }

        for entry in self.bundle.resources().entries() {
            if entry.key().relative_path().file_name()?.to_str()? == gdtf_spec {
                return Some(entry.key().clone());
            }
        }

        None
    }

    pub fn models(&self) -> impl Iterator<Item = &bundle::ResourceEntry> {
        self.bundle.resources().entries().filter(|e| e.kind() == bundle::ResourceKind::Model)
    }

    pub fn textures(&self) -> impl Iterator<Item = &bundle::ResourceEntry> {
        self.bundle.resources().entries().filter(|e| e.kind() == bundle::ResourceKind::Texture)
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

#[derive(Debug)]
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

impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        Self { uuid: self.uuid, _marker: std::marker::PhantomData }
    }
}

impl<T> Copy for NodeId<T> {}

impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<T> Eq for NodeId<T> {}

impl<T> std::hash::Hash for NodeId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ObjectPath {
    layer_id: NodeId<Layer>,
    indices: Vec<usize>,
}

impl ObjectPath {
    pub fn new(layer_id: NodeId<Layer>, indices: Vec<usize>) -> Self {
        Self { layer_id, indices }
    }
}
