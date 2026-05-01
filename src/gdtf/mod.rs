use std::path::PathBuf;

mod description;
mod resource;
mod source;

pub(crate) use description::*;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node(Vec<String>);

impl Node {
    pub fn parts(&self) -> &[String] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_dotted_string(&self) -> String {
        self.0.join(".")
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_dotted_string())
    }
}

impl std::str::FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Node(Vec::new()));
        }
        let parts: Vec<String> = s.split('.').map(|part| part.trim().to_string()).collect();
        Ok(Node(parts))
    }
}
