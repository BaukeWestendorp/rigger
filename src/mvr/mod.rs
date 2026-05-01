pub mod bundle;

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::PathBuf,
    str::FromStr,
};

use uuid::Uuid;

use crate::gdtf;

pub mod aux;
mod builder;
pub mod geo;
pub mod layer;

use crate::mvr::{
    aux::{Class, MappingDefinition, Position, Symdef},
    layer::{Layer, Object, SceneObject},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectPath {
    layer_ix: usize,
    indices: Vec<usize>,
}

impl ObjectPath {
    pub fn new(layer_ix: usize, indices: Vec<usize>) -> Self {
        Self { layer_ix, indices }
    }

    pub fn layer_ix(&self) -> usize {
        self.layer_ix
    }

    pub fn indices(&self) -> &[usize] {
        &self.indices
    }
}

pub struct ObjectsRecursive<'a> {
    mvr: &'a Mvr,
    stack: Vec<ObjectPath>,
}

impl<'a> ObjectsRecursive<'a> {
    fn new(mvr: &'a Mvr) -> Self {
        let mut stack = Vec::new();

        for (layer_ix, layer) in mvr.layers.iter().enumerate().rev() {
            for object_ix in (0..layer.objects.len()).rev() {
                stack.push(ObjectPath::new(layer_ix, vec![object_ix]));
            }
        }

        Self { mvr, stack }
    }
}

impl<'a> Iterator for ObjectsRecursive<'a> {
    type Item = (ObjectPath, &'a Object);

    fn next(&mut self) -> Option<Self::Item> {
        let mvr = self.mvr;
        let path = self.stack.pop()?;
        let object = mvr.object_by_path(&path)?;

        if let Some(children) = object.children() {
            for child_ix in (0..children.len()).rev() {
                let mut child_path = path.indices.clone();
                child_path.push(child_ix);
                self.stack.push(ObjectPath::new(path.layer_ix, child_path));
            }
        }

        Some((path, object))
    }
}

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
    // [layers, objects]
    objects_ix: HashMap<Uuid, [usize; 2]>,
    objects_path_ix: HashMap<Uuid, ObjectPath>,
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
        if let Some([layer_ix, object_ix]) = self.objects_ix.get(&id).copied() {
            return Some(&self.layers[layer_ix].objects[object_ix]);
        }

        let path = self.objects_path_ix.get(&id).cloned()?;
        self.object_by_path(&path)
    }

    pub fn scene_object(&self, id: NodeId<Object>) -> Option<&SceneObject> {
        let object = self.object(id)?;
        match object.kind() {
            layer::ObjectKind::SceneObject(scene_object) => Some(scene_object),
            _ => None,
        }
    }

    pub fn group_object(&self, id: NodeId<Object>) -> Option<&layer::GroupObject> {
        let object = self.object(id)?;
        match object.kind() {
            layer::ObjectKind::GroupObject(group_object) => Some(group_object),
            _ => None,
        }
    }

    pub fn focus_point_object(&self, id: NodeId<Object>) -> Option<&layer::FocusPointObject> {
        let object = self.object(id)?;
        match object.kind() {
            layer::ObjectKind::FocusPoint(focus_point_object) => Some(focus_point_object),
            _ => None,
        }
    }

    pub fn fixture_object(&self, id: NodeId<Object>) -> Option<&layer::FixtureObject> {
        let object = self.object(id)?;
        match object.kind() {
            layer::ObjectKind::Fixture(fixture_object) => Some(fixture_object),
            _ => None,
        }
    }

    pub fn support_object(&self, id: NodeId<Object>) -> Option<&layer::SupportObject> {
        let object = self.object(id)?;
        match object.kind() {
            layer::ObjectKind::Support(support_object) => Some(support_object),
            _ => None,
        }
    }

    pub fn truss_object(&self, id: NodeId<Object>) -> Option<&layer::TrussObject> {
        let object = self.object(id)?;
        match object.kind() {
            layer::ObjectKind::Truss(truss_object) => Some(truss_object),
            _ => None,
        }
    }

    pub fn video_screen_object(&self, id: NodeId<Object>) -> Option<&layer::VideoScreenObject> {
        let object = self.object(id)?;
        match object.kind() {
            layer::ObjectKind::VideoScreen(video_screen_object) => Some(video_screen_object),
            _ => None,
        }
    }

    pub fn projector_object(&self, id: NodeId<Object>) -> Option<&layer::ProjectorObject> {
        let object = self.object(id)?;
        match object.kind() {
            layer::ObjectKind::Projector(projector_object) => Some(projector_object),
            _ => None,
        }
    }

    pub fn resources(&self) -> &bundle::ResourceMap {
        self.bundle.resources()
    }

    pub fn root_folder(&self) -> &std::path::Path {
        self.bundle.root_folder()
    }

    pub fn resolve_path(&self, key: &bundle::ResourceKey) -> PathBuf {
        self.bundle.resolve_path(key)
    }

    pub fn open_resource(&self, key: &bundle::ResourceKey) -> Option<std::fs::File> {
        std::fs::File::open(self.bundle.resolve_path(key)).ok()
    }

    pub fn resource_bytes(&self, key: &bundle::ResourceKey) -> Option<Vec<u8>> {
        self.bundle.resource_bytes(key)
    }

    pub fn layer_ix(&self, id: NodeId<Layer>) -> Option<usize> {
        self.layers_ix.get(&id).copied()
    }

    pub fn layer_at(&self, layer_ix: usize) -> Option<&Layer> {
        self.layers.get(layer_ix)
    }

    pub fn layer_by_name(&self, name: &str) -> Option<&Layer> {
        self.layers.iter().find(|l| l.name() == name)
    }

    pub fn layers_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Layer> + 'a {
        self.layers.iter().filter(move |l| l.name() == name)
    }

    pub fn layer_ids(&self) -> impl Iterator<Item = NodeId<Layer>> + '_ {
        self.layers.iter().map(|l| l.id())
    }

    pub fn objects(&self) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &Object)> + '_ {
        self.layers.iter().flat_map(|layer| {
            let layer_id = layer.id();
            layer.objects().iter().map(move |object| (layer_id, object.id(), object))
        })
    }

    pub fn objects_recursive(&self) -> ObjectsRecursive<'_> {
        ObjectsRecursive::new(self)
    }

    pub fn object_path(&self, id: NodeId<Object>) -> Option<ObjectPath> {
        self.objects_path_ix.get(&id).cloned()
    }

    pub fn object_any(&self, id: NodeId<Object>) -> Option<&Object> {
        let path = self.object_path(id)?;
        self.object_by_path(&path)
    }

    pub fn object_layer_any(&self, id: NodeId<Object>) -> Option<&Layer> {
        let path = self.object_path(id)?;
        self.layers.get(path.layer_ix)
    }

    pub fn object_with_layer_any(&self, id: NodeId<Object>) -> Option<(&Layer, &Object)> {
        let path = self.object_path(id)?;
        self.object_with_layer_by_path(&path)
    }

    pub fn object_with_layer_by_path(&self, path: &ObjectPath) -> Option<(&Layer, &Object)> {
        let layer = self.layers.get(path.layer_ix)?;
        let object = self.object_by_path(path)?;
        Some((layer, object))
    }

    pub fn object_by_path(&self, path: &ObjectPath) -> Option<&Object> {
        let layer = self.layers.get(path.layer_ix)?;
        let mut indices = path.indices.iter();
        let first = *indices.next()?;
        let mut object = layer.objects.get(first)?;

        for &ix in indices {
            let children = object.children()?;
            object = children.get(ix)?;
        }

        Some(object)
    }

    pub fn object_world_transform(&self, id: NodeId<Object>) -> Option<glam::Affine3A> {
        let path = self.object_path(id)?;
        self.object_world_transform_by_path(&path)
    }

    pub fn object_world_transform_by_path(&self, path: &ObjectPath) -> Option<glam::Affine3A> {
        let layer = self.layers.get(path.layer_ix)?;
        let mut indices = path.indices.iter();
        let first = *indices.next()?;
        let mut object = layer.objects.get(first)?;
        let mut transform = layer.local_transform * object.local_transform;

        for &ix in indices {
            let children = object.children()?;
            object = children.get(ix)?;
            transform = transform * object.local_transform;
        }

        Some(transform)
    }

    pub fn object_geometries_world<'a>(
        &'a self,
        id: NodeId<Object>,
    ) -> Option<impl Iterator<Item = (&'a geo::Geometry, glam::Affine3A)> + 'a> {
        let path = self.object_path(id)?;
        let world = self.object_world_transform_by_path(&path)?;
        let object = self.object_by_path(&path)?;
        let geometries = object.geometries()?;

        Some(geometries.iter().map(move |g| (g, world * g.local_transform())))
    }

    pub fn scene_objects_recursive(
        &self,
    ) -> impl Iterator<Item = (ObjectPath, &layer::SceneObject)> + '_ {
        self.objects_recursive().filter_map(|(path, object)| match object.kind() {
            layer::ObjectKind::SceneObject(v) => Some((path, v)),
            _ => None,
        })
    }

    pub fn group_objects_recursive(
        &self,
    ) -> impl Iterator<Item = (ObjectPath, &layer::GroupObject)> + '_ {
        self.objects_recursive().filter_map(|(path, object)| match object.kind() {
            layer::ObjectKind::GroupObject(v) => Some((path, v)),
            _ => None,
        })
    }

    pub fn focus_point_objects_recursive(
        &self,
    ) -> impl Iterator<Item = (ObjectPath, &layer::FocusPointObject)> + '_ {
        self.objects_recursive().filter_map(|(path, object)| match object.kind() {
            layer::ObjectKind::FocusPoint(v) => Some((path, v)),
            _ => None,
        })
    }

    pub fn fixture_objects_recursive(
        &self,
    ) -> impl Iterator<Item = (ObjectPath, &layer::FixtureObject)> + '_ {
        self.objects_recursive().filter_map(|(path, object)| match object.kind() {
            layer::ObjectKind::Fixture(v) => Some((path, v)),
            _ => None,
        })
    }

    pub fn support_objects_recursive(
        &self,
    ) -> impl Iterator<Item = (ObjectPath, &layer::SupportObject)> + '_ {
        self.objects_recursive().filter_map(|(path, object)| match object.kind() {
            layer::ObjectKind::Support(v) => Some((path, v)),
            _ => None,
        })
    }

    pub fn truss_objects_recursive(
        &self,
    ) -> impl Iterator<Item = (ObjectPath, &layer::TrussObject)> + '_ {
        self.objects_recursive().filter_map(|(path, object)| match object.kind() {
            layer::ObjectKind::Truss(v) => Some((path, v)),
            _ => None,
        })
    }

    pub fn video_screen_objects_recursive(
        &self,
    ) -> impl Iterator<Item = (ObjectPath, &layer::VideoScreenObject)> + '_ {
        self.objects_recursive().filter_map(|(path, object)| match object.kind() {
            layer::ObjectKind::VideoScreen(v) => Some((path, v)),
            _ => None,
        })
    }

    pub fn projector_objects_recursive(
        &self,
    ) -> impl Iterator<Item = (ObjectPath, &layer::ProjectorObject)> + '_ {
        self.objects_recursive().filter_map(|(path, object)| match object.kind() {
            layer::ObjectKind::Projector(v) => Some((path, v)),
            _ => None,
        })
    }

    pub fn objects_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = (ObjectPath, &'a Object)> + 'a {
        self.objects_recursive().filter(move |(_, object)| object.name() == name)
    }

    pub fn objects_named_top_level<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &'a Object)> + 'a {
        self.objects().filter(move |(_, _, object)| object.name() == name)
    }

    pub fn objects_in_class<'a>(
        &'a self,
        class_id: NodeId<Class>,
    ) -> impl Iterator<Item = (ObjectPath, &'a Object)> + 'a {
        self.objects_recursive().filter(move |(_, object)| object.class_id() == Some(class_id))
    }

    pub fn objects_with_gdtf_spec<'a>(
        &'a self,
        gdtf_spec: &'a str,
    ) -> impl Iterator<Item = (ObjectPath, &'a Object)> + 'a {
        self.objects_recursive().filter(move |(_, object)| {
            object.gdtf_info().is_some_and(|g| g.gdtf_spec() == gdtf_spec)
        })
    }

    pub fn objects_with_gdtf_mode<'a>(
        &'a self,
        gdtf_mode: &'a str,
    ) -> impl Iterator<Item = (ObjectPath, &'a Object)> + 'a {
        self.objects_recursive().filter(move |(_, object)| {
            object.gdtf_info().is_some_and(|g| g.gdtf_mode() == gdtf_mode)
        })
    }

    pub fn objects_with_dmx_address<'a>(
        &'a self,
        break_: u32,
        absolute_value: u32,
    ) -> impl Iterator<Item = (ObjectPath, &'a Object)> + 'a {
        self.objects_recursive().filter(move |(_, object)| {
            object.dmx_addresses().is_some_and(|addrs| {
                addrs.iter().any(|a| a.break_() == break_ && a.absolute_value() == absolute_value)
            })
        })
    }

    pub fn objects_with_multipatch<'a>(
        &'a self,
        multipatch: Uuid,
    ) -> impl Iterator<Item = (ObjectPath, &'a Object)> + 'a {
        self.objects_recursive().filter(move |(_, object)| {
            object.identifier().is_some_and(|id| match id {
                layer::ObjectIdentifier::Multipatch(u) => *u == multipatch,
                _ => false,
            })
        })
    }

    pub fn class_by_name(&self, name: &str) -> Option<&Class> {
        self.classes.values().find(|c| c.name() == name)
    }

    pub fn classes_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Class> + 'a {
        self.classes.values().filter(move |c| c.name() == name)
    }

    pub fn position_by_name(&self, name: &str) -> Option<&Position> {
        self.positions.values().find(|p| p.name() == name)
    }

    pub fn positions_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Position> + 'a {
        self.positions.values().filter(move |p| p.name() == name)
    }

    pub fn symdef_by_name(&self, name: &str) -> Option<&Symdef> {
        self.symdefs.values().find(|s| s.name() == name)
    }

    pub fn symdefs_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Symdef> + 'a {
        self.symdefs.values().filter(move |s| s.name() == name)
    }

    pub fn mapping_definition_by_name(&self, name: &str) -> Option<&MappingDefinition> {
        self.mapping_definitions.values().find(|md| md.name() == name)
    }

    pub fn mapping_definitions_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a MappingDefinition> + 'a {
        self.mapping_definitions.values().filter(move |md| md.name() == name)
    }

    pub fn fixture_objects(
        &self,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &layer::FixtureObject)> + '_ {
        self.objects().filter_map(|(layer_id, object_id, object)| match object.kind() {
            layer::ObjectKind::Fixture(v) => Some((layer_id, object_id, v)),
            _ => None,
        })
    }

    pub fn scene_objects(
        &self,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &layer::SceneObject)> + '_ {
        self.objects().filter_map(|(layer_id, object_id, object)| match object.kind() {
            layer::ObjectKind::SceneObject(v) => Some((layer_id, object_id, v)),
            _ => None,
        })
    }

    pub fn group_objects(
        &self,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &layer::GroupObject)> + '_ {
        self.objects().filter_map(|(layer_id, object_id, object)| match object.kind() {
            layer::ObjectKind::GroupObject(v) => Some((layer_id, object_id, v)),
            _ => None,
        })
    }

    pub fn focus_point_objects(
        &self,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &layer::FocusPointObject)> + '_ {
        self.objects().filter_map(|(layer_id, object_id, object)| match object.kind() {
            layer::ObjectKind::FocusPoint(v) => Some((layer_id, object_id, v)),
            _ => None,
        })
    }

    pub fn support_objects(
        &self,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &layer::SupportObject)> + '_ {
        self.objects().filter_map(|(layer_id, object_id, object)| match object.kind() {
            layer::ObjectKind::Support(v) => Some((layer_id, object_id, v)),
            _ => None,
        })
    }

    pub fn truss_objects(
        &self,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &layer::TrussObject)> + '_ {
        self.objects().filter_map(|(layer_id, object_id, object)| match object.kind() {
            layer::ObjectKind::Truss(v) => Some((layer_id, object_id, v)),
            _ => None,
        })
    }

    pub fn video_screen_objects(
        &self,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &layer::VideoScreenObject)> + '_ {
        self.objects().filter_map(|(layer_id, object_id, object)| match object.kind() {
            layer::ObjectKind::VideoScreen(v) => Some((layer_id, object_id, v)),
            _ => None,
        })
    }

    pub fn projector_objects(
        &self,
    ) -> impl Iterator<Item = (NodeId<Layer>, NodeId<Object>, &layer::ProjectorObject)> + '_ {
        self.objects().filter_map(|(layer_id, object_id, object)| match object.kind() {
            layer::ObjectKind::Projector(v) => Some((layer_id, object_id, v)),
            _ => None,
        })
    }

    pub fn gdtf_resource_key(&self, gdtf_spec: &str) -> Option<bundle::ResourceKey> {
        let key = bundle::ResourceKey::new(gdtf_spec);
        if self.bundle.resources().contains_key(&key) {
            return Some(key);
        }

        for entry in self.bundle.resources().entries() {
            if entry.kind != bundle::ResourceKind::Gdtf {
                continue;
            }

            if entry.key.path().file_name()?.to_str()? == gdtf_spec {
                return Some(entry.key.clone());
            }
        }

        None
    }

    pub fn gdtf_path(&self, gdtf_spec: &str) -> Option<PathBuf> {
        let key = self.gdtf_resource_key(gdtf_spec)?;
        Some(self.bundle.resolve_path(&key))
    }

    pub fn gdtf(&self, gdtf_spec: &str) -> Option<gdtf::Gdtf> {
        let path = self.gdtf_path(gdtf_spec)?;
        Some(gdtf::Gdtf::from_archive(path))
    }

    pub fn gdtf_for_object(&self, id: NodeId<Object>) -> Option<gdtf::Gdtf> {
        let object = self.object_any(id)?;
        let info = object.gdtf_info()?;
        self.gdtf(info.gdtf_spec())
    }

    pub fn with_gdtf<R>(
        &self,
        info: &layer::GdtfInfo,
        f: impl FnOnce(&gdtf::Gdtf) -> R,
    ) -> Option<R> {
        let g = self.gdtf(info.gdtf_spec())?;
        Some(f(&g))
    }

    pub fn with_gdtf_for_object<R>(
        &self,
        id: NodeId<Object>,
        f: impl FnOnce(&gdtf::Gdtf) -> R,
    ) -> Option<R> {
        let object = self.object_any(id)?;
        let info = object.gdtf_info()?;
        self.with_gdtf(info, f)
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

impl<T> Deref for NodeId<T> {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.uuid
    }
}

impl<T> DerefMut for NodeId<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.uuid
    }
}
