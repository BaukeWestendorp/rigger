use std::path::PathBuf;

mod description;
mod resource;
mod source;

pub use description::*;
pub use resource::*;
use source::*;

const DESCRIPTION_FILE_NAME: &str = "description.xml";

pub struct Gdtf {
    description: GdtfDescription,
}

impl Gdtf {
    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        FolderSource { path: path.into() }.load()
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        ArchiveSource { path: path.into() }.load()
    }

    pub fn description(&self) -> &GdtfDescription {
        &self.description
    }
}
