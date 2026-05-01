const DESCRIPTION_FILE_NAME: &str = "Description.xml";

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

mod description;
mod resource;
mod source;

pub use description::*;
pub use resource::*;

use source::{ArchiveSource, BundleSource, FolderSource, SourceLoader as _};

/// Representation of an GDTF bundle.
///
/// This stays close to the serialized `Description.xml` and the
/// files contained in the bundle (folder/zip).
pub struct Bundle {
    description: GdtfDescription,
    resources: ResourceMap,
    source: BundleSource,
}

impl Bundle {
    pub(crate) fn new(
        description: GdtfDescription,
        resources: ResourceMap,
        source: BundleSource,
    ) -> Self {
        Self { description, resources, source }
    }

    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        FolderSource::new(path.clone()).load_bundle(BundleSource::Folder { root: path })
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        ArchiveSource::new(path)
            .load_bundle(BundleSource::Archive { temp_dir: tempfile::TempDir::new().unwrap() })
    }

    pub fn description(&self) -> &GdtfDescription {
        &self.description
    }

    pub fn resources(&self) -> &ResourceMap {
        &self.resources
    }

    pub fn root_folder(&self) -> &Path {
        self.source.root_folder()
    }

    pub fn resolve_path(&self, key: &ResourceKey) -> PathBuf {
        self.root_folder().join(key.path())
    }

    pub fn open_resource(&self, key: &ResourceKey) -> Option<impl Read> {
        let path = self.resolve_path(key);
        File::open(path).ok()
    }

    pub fn resource_bytes(&self, key: &ResourceKey) -> Option<Vec<u8>> {
        let mut reader = self.open_resource(key)?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).ok()?;
        Some(buf)
    }
}
