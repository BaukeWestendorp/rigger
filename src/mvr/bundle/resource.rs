use std::{collections::HashMap, fmt, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Model,
    Texture,
    Gdtf,
    Other,
}

impl ResourceKind {
    pub fn from_path(path: &Path) -> Self {
        let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase())
        else {
            return Self::Other;
        };

        match ext.as_str() {
            "gltf" | "glb" | "3ds" => Self::Model,
            "png" | "jpg" | "jpeg" => Self::Texture,
            "gdtf" => Self::Gdtf,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceKey(String);

impl ResourceKey {
    pub fn new(relative_path: impl Into<String>) -> Self {
        Self(relative_path.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceEntry {
    pub key: ResourceKey,
    pub kind: ResourceKind,
}

impl ResourceEntry {
    pub fn new(key: ResourceKey, kind: ResourceKind) -> Self {
        Self { key, kind }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResourceMap {
    entries: HashMap<ResourceKey, ResourceEntry>,
}

impl ResourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries(&self) -> impl Iterator<Item = &ResourceEntry> {
        self.entries.values()
    }

    pub fn get(&self, key: &ResourceKey) -> Option<&ResourceEntry> {
        self.entries.get(key)
    }

    pub fn contains_key(&self, key: &ResourceKey) -> bool {
        self.entries.contains_key(key)
    }

    pub(crate) fn insert(&mut self, entry: ResourceEntry) {
        self.entries.insert(entry.key.clone(), entry);
    }
}
