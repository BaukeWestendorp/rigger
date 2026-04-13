const GSD_FILE_NAME: &str = "GeneralSceneDescription.xml";

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

mod description;
mod resource;
mod source;

pub(crate) use description::*;
pub use resource::*;

use source::{BundleSource, FolderSource, Source};

/// Options controlling how a `Bundle` is loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadOptions {
    /// If true (default), only scan the root of the folder/archive for resources.
    ///
    /// The MVR spec states all resources should be in the root, but some producers might not.
    pub root_only: bool,

    /// If true (default), normalize paths before indexing.
    ///
    /// This is needed because some files might have non-standard characters in their name.
    pub sanitize_paths: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self { root_only: true, sanitize_paths: true }
    }
}

/// Representation of an MVR bundle.
///
/// This stays close to the serialized `GeneralSceneDescription.xml` and the
/// files contained in the bundle (folder/zip).
pub struct Bundle {
    description: GeneralSceneDescription,

    /// All discovered files inside the bundle, indexed by bundle-relative path.
    resources: ResourceMap,

    /// Where the bundle was loaded from.
    source: BundleSource,
}

impl Bundle {
    pub(crate) fn new(
        description: GeneralSceneDescription,
        resources: ResourceMap,
        source: BundleSource,
    ) -> Self {
        Self { description, resources, source }
    }

    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        Self::from_folder_with_options(path, LoadOptions::default())
    }

    pub fn from_folder_with_options(path: impl Into<PathBuf>, options: LoadOptions) -> Self {
        let path = path.into();
        FolderSource { path: path.clone(), options }
            .load_bundle(BundleSource::Folder { root: path })
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        Self::from_archive_with_options(path, LoadOptions::default())
    }

    pub fn from_archive_with_options(_path: impl Into<PathBuf>, _options: LoadOptions) -> Self {
        todo!();
    }

    pub fn description(&self) -> &GeneralSceneDescription {
        &self.description
    }

    pub fn resources(&self) -> &ResourceMap {
        &self.resources
    }

    /// Returns the on-disk root folder if this bundle was loaded from a folder.
    ///
    /// For archive-backed bundles this returns `None` (until extraction/caching is implemented).
    pub fn root_folder(&self) -> Option<&Path> {
        match &self.source {
            BundleSource::Folder { root } => Some(root.as_path()),
            BundleSource::Archive { .. } => None,
        }
    }

    /// Resolves a bundle-relative key to a filesystem path if the bundle is folder-backed.
    ///
    /// This is the recommended way for applications that need an actual OS path.
    pub fn resolve_path(&self, key: &ResourceKey) -> Option<PathBuf> {
        self.root_folder().map(|root| root.join(key.as_path()))
    }

    /// Backwards-compatible name for `resolve_path`.
    pub fn resource_path(&self, key: &ResourceKey) -> Option<PathBuf> {
        self.resolve_path(key)
    }

    /// Loads the raw bytes of a resource.
    pub fn resource_bytes(&self, key: &ResourceKey) -> Option<Vec<u8>> {
        match &self.source {
            BundleSource::Folder { root } => {
                let path = root.join(key.as_path());
                let mut f = File::open(path).ok()?;
                let mut buf = Vec::new();
                f.read_to_end(&mut buf).ok()?;
                Some(buf)
            }
            BundleSource::Archive { .. } => todo!(),
        }
    }
}
