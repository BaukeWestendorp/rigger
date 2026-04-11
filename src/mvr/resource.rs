use std::{
    fmt,
    path::{Path, PathBuf},
};

use crate::gdtf::Gdtf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelHandle(String);

impl ModelHandle {
    pub fn new(relative_path: impl Into<String>) -> Self {
        Self(relative_path.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for ModelHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextureHandle(String);

impl TextureHandle {
    pub fn new(relative_path: impl Into<String>) -> Self {
        Self(relative_path.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for TextureHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GdtfHandle(String);

impl GdtfHandle {
    pub fn new(relative_path: impl Into<String>) -> Self {
        Self(relative_path.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for GdtfHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub struct ModelResource {
    pub(crate) path: PathBuf,
}

impl ModelResource {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn check_path(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| matches!(ext, "gltf" | "glb" | "3ds"))
    }
}

pub struct TextureResource {
    pub(crate) path: PathBuf,
}

impl TextureResource {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn check_path(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| matches!(ext, "png" | "jpg" | "jpeg"))
    }
}

pub struct GdtfResource {
    pub(crate) path: PathBuf,
    pub(crate) gdtf: Gdtf,
}

impl GdtfResource {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn gdtf(&self) -> &Gdtf {
        &self.gdtf
    }

    pub fn check_path(path: &Path) -> bool {
        dbg!(path);
        path.extension().and_then(|ext| ext.to_str()).is_some_and(|ext| matches!(ext, "gdtf"))
    }
}
