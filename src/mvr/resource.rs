use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use crate::gdtf::Gdtf;

#[derive(Clone, PartialEq, Default)]
pub struct Resources {
    gdtfs: HashMap<ResourceKey, Gdtf>,
    models: HashMap<ResourceKey, ModelResource>,
    textures: HashMap<ResourceKey, TextureResource>,
}

impl Resources {
    pub fn gdtfs(&self) -> impl Iterator<Item = (&ResourceKey, &Gdtf)> {
        self.gdtfs.iter()
    }

    pub fn gdtf(&self, key: &ResourceKey) -> Option<&Gdtf> {
        self.gdtfs.get(key)
    }

    pub fn add_gdtf(&mut self, key: ResourceKey, gdtf: Gdtf) {
        self.gdtfs.insert(key, gdtf);
    }

    pub fn remove_gdtf(&mut self, key: &ResourceKey) -> Option<Gdtf> {
        self.gdtfs.remove(key)
    }

    pub fn models(&self) -> impl Iterator<Item = (&ResourceKey, &ModelResource)> {
        self.models.iter()
    }

    pub fn model(&self, key: &ResourceKey) -> Option<&ModelResource> {
        self.models.get(key)
    }

    pub fn add_model(&mut self, key: ResourceKey, data: ModelResource) {
        self.models.insert(key, data);
    }

    pub fn remove_model(&mut self, key: &ResourceKey) -> Option<ModelResource> {
        self.models.remove(key)
    }

    pub fn textures(&self) -> impl Iterator<Item = (&ResourceKey, &TextureResource)> {
        self.textures.iter()
    }

    pub fn texture(&self, key: &ResourceKey) -> Option<&TextureResource> {
        self.textures.get(key)
    }

    pub fn add_texture(&mut self, key: ResourceKey, data: TextureResource) {
        self.textures.insert(key, data);
    }

    pub fn remove_texture(&mut self, key: &ResourceKey) -> Option<TextureResource> {
        self.textures.remove(key)
    }
}

impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resources")
            .field(
                "gdtfs",
                &self.gdtfs().map(|(k, _)| (k, "<Gdtf>".to_string())).collect::<HashMap<_, _>>(),
            )
            .field("models", &self.models)
            .field("textures", &self.textures)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceKey(PathBuf);

impl ResourceKey {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    pub fn relative_path(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Clone, PartialEq)]
pub struct ModelResource {
    bytes: Vec<u8>,
    kind: ModelKind,
}

impl ModelResource {
    pub fn new(file_name: &Path, bytes: Vec<u8>) -> Self {
        let kind =
            match file_name.extension().and_then(|s| s.to_str()).map(|s| s.to_ascii_lowercase()) {
                Some(ext) if ext == "3ds" => ModelKind::ThreeDs,
                Some(ext) if ext == "gltf" => ModelKind::Gltf,
                Some(ext) if ext == "glb" => ModelKind::Glb,
                Some(ext) => ModelKind::Other { extension: ext },
                None => ModelKind::Other { extension: String::new() },
            };

        Self { bytes, kind }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn kind(&self) -> &ModelKind {
        &self.kind
    }
}

impl fmt::Debug for ModelResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModelResource")
            .field("bytes", &"Vec(...)")
            .field("kind", &self.kind)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelKind {
    ThreeDs,
    Gltf,
    Glb,
    Other { extension: String },
}

impl ModelKind {
    pub fn extension(&self) -> &str {
        match self {
            ModelKind::ThreeDs => "3ds",
            ModelKind::Gltf => "gltf",
            ModelKind::Glb => "glb",
            ModelKind::Other { extension } => extension,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TextureResource {
    bytes: Vec<u8>,
    kind: TextureKind,
}

impl TextureResource {
    pub fn new(file_name: &Path, bytes: Vec<u8>) -> Self {
        let kind =
            match file_name.extension().and_then(|s| s.to_str()).map(|s| s.to_ascii_lowercase()) {
                Some(ext) if ext == "png" => TextureKind::Png,
                Some(ext) if ext == "jpg" => TextureKind::Jpg,
                Some(ext) if ext == "jpeg" => TextureKind::Jpeg,
                Some(ext) if ext == "svg" => TextureKind::Svg,
                Some(ext) => TextureKind::Other { extension: ext },
                None => TextureKind::Other { extension: String::new() },
            };

        Self { bytes, kind }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn kind(&self) -> &TextureKind {
        &self.kind
    }
}

impl fmt::Debug for TextureResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextureResource")
            .field("bytes", &"Vec(...)")
            .field("kind", &self.kind)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureKind {
    Png,
    Jpg,
    Jpeg,
    Svg,
    Other { extension: String },
}

impl TextureKind {
    pub fn extension(&self) -> &str {
        match self {
            TextureKind::Png => "png",
            TextureKind::Jpg => "jpg",
            TextureKind::Jpeg => "jpeg",
            TextureKind::Svg => "svg",
            TextureKind::Other { extension } => extension,
        }
    }
}
