use std::{collections::HashMap, fmt, hash, str};

use uuid::Uuid;

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

impl<T> hash::Hash for NodeId<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<T> From<Uuid> for NodeId<T> {
    fn from(uuid: Uuid) -> Self {
        Self { uuid, _marker: std::marker::PhantomData }
    }
}

impl<T> str::FromStr for NodeId<T> {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(Uuid::from_str(s)?))
    }
}

impl<T> fmt::Debug for NodeId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId<{}>({:?})", std::any::type_name::<T>(), self.uuid)
    }
}

pub trait Node {
    fn id(&self) -> NodeId<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeContainer<T> {
    items: HashMap<NodeId<T>, T>,
}

impl<T: Node> NodeContainer<T> {
    pub fn add(&mut self, node: T) {
        // FIXME: Return error if it already exists.

        self.items.insert(node.id(), node);
    }

    pub fn get(&self, id: NodeId<T>) -> Option<&T> {
        self.items.get(&id)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.values_mut()
    }
}

impl<T> Default for NodeContainer<T> {
    fn default() -> Self {
        Self { items: HashMap::default() }
    }
}

impl<T> IntoIterator for NodeContainer<T> {
    type Item = T;
    type IntoIter = std::collections::hash_map::IntoValues<NodeId<T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_values()
    }
}

impl<'a, T> IntoIterator for &'a NodeContainer<T> {
    type Item = &'a T;
    type IntoIter = std::collections::hash_map::Values<'a, NodeId<T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.values()
    }
}

impl<'a, T> IntoIterator for &'a mut NodeContainer<T> {
    type Item = &'a mut T;
    type IntoIter = std::collections::hash_map::ValuesMut<'a, NodeId<T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.values_mut()
    }
}
