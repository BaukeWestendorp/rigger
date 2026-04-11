mod gsd;

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    str::FromStr,
};

pub use gsd::*;
use uuid::Uuid;

const GSD_FILE_NAME: &str = "GeneralSceneDescription.xml";

pub struct Mvr {
    gsd: GeneralSceneDescription,

    models: BTreeMap<FileName, Model>,
    textures: BTreeMap<FileName, Texture>,
}

impl Mvr {
    pub fn from_source<S: source::Source>(source: S) -> Self {
        let gsd = quick_xml::de::from_reader(source.gsd_reader()).unwrap();
        let models = source.models();
        let textures = source.textures();
        Self { gsd, models, textures }
    }

    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        Self::from_source(source::Folder::new(path))
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        Self::from_source(source::Archive::new(path))
    }

    pub fn gsd(&self) -> &GeneralSceneDescription {
        &self.gsd
    }

    pub fn models(&self) -> &BTreeMap<FileName, Model> {
        &self.models
    }

    pub fn textures(&self) -> &BTreeMap<FileName, Texture> {
        &self.textures
    }

    pub fn symdef(&self, uuid: Uuid) -> Option<&Symdef> {
        // FIXME: Use index for this.
        self.gsd
            .scene
            .aux_data
            .as_ref()?
            .symdef
            .iter()
            .find(|symdef| Uuid::from_str(&symdef.uuid).unwrap() == uuid)
    }

    pub fn class(&self, uuid: Uuid) -> Option<&BasicChildListAttribute> {
        // FIXME: Use index for this.
        self.gsd
            .scene
            .aux_data
            .as_ref()?
            .class
            .iter()
            .find(|class| Uuid::from_str(&class.uuid).unwrap() == uuid)
    }

    pub fn mapping_definition(&self, uuid: Uuid) -> Option<&MappingDefinition> {
        // FIXME: Use index for this.
        self.gsd
            .scene
            .aux_data
            .as_ref()?
            .mapping_definition
            .iter()
            .find(|def| Uuid::from_str(&def.uuid).unwrap() == uuid)
    }

    pub fn position(&self, uuid: Uuid) -> Option<&BasicChildListAttribute> {
        // FIXME: Use index for this.
        self.gsd
            .scene
            .aux_data
            .as_ref()?
            .position
            .iter()
            .find(|pos| Uuid::from_str(&pos.uuid).unwrap() == uuid)
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    path: PathBuf,
}

impl Model {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    path: PathBuf,
}

impl Texture {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

pub mod source {
    use std::{
        collections::BTreeMap,
        fs::File,
        io::{self, BufReader},
        path::{Path, PathBuf},
    };

    use super::*;

    pub trait Source {
        fn gsd_reader(&self) -> Box<dyn io::BufRead>;

        fn models(&self) -> BTreeMap<FileName, Model>;

        fn textures(&self) -> BTreeMap<FileName, Texture>;
    }

    fn is_model_file(path: &Path) -> bool {
        path.extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| matches!(ext, "glb" | "gltf" | "3ds"))
    }

    fn is_texture_file(path: &Path) -> bool {
        path.extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| matches!(ext, "png" | "jpg" | "jpeg"))
    }

    pub struct Folder {
        path: PathBuf,
        models: BTreeMap<FileName, Model>,
        textures: BTreeMap<FileName, Texture>,
    }

    impl Folder {
        pub fn new(path: impl Into<PathBuf>) -> Self {
            let path = path.into();

            let mut models = BTreeMap::new();
            let mut textures = BTreeMap::new();

            if let Ok(read_dir) = std::fs::read_dir(&path) {
                for entry in read_dir.flatten() {
                    let p = entry.path();

                    if !p.is_file() {
                        continue;
                    }

                    let file_name = crate::sanetize_file_name(p.file_name().unwrap());

                    if is_model_file(&p) {
                        models.insert(
                            file_name.clone(),
                            Model { path: path.join(&file_name).canonicalize().unwrap() },
                        );
                    } else if is_texture_file(&p) {
                        textures.insert(
                            file_name.clone(),
                            Texture { path: path.join(&file_name).canonicalize().unwrap() },
                        );
                    }
                }
            }

            Self { path, models, textures }
        }

        pub fn path(&self) -> &PathBuf {
            &self.path
        }
    }

    impl Source for Folder {
        fn gsd_reader(&self) -> Box<dyn io::BufRead> {
            let gsd_path = self.path.join(GSD_FILE_NAME);
            Box::new(BufReader::new(File::open(gsd_path).unwrap()))
        }

        fn models(&self) -> BTreeMap<FileName, Model> {
            // FIXME: Cloning here kind of redundant, as we only call this function when creating the Mvr.
            self.models.clone()
        }

        fn textures(&self) -> BTreeMap<FileName, Texture> {
            // FIXME: Cloning here kind of redundant, as we only call this function when creating the Mvr.
            self.textures.clone()
        }
    }

    pub struct Archive {
        path: PathBuf,
    }

    impl Archive {
        pub fn new(path: impl Into<PathBuf>) -> Self {
            Self { path: path.into() }
        }

        pub fn path(&self) -> &PathBuf {
            &self.path
        }
    }

    impl Source for Archive {
        fn gsd_reader(&self) -> Box<dyn io::BufRead> {
            todo!();
        }

        fn models(&self) -> BTreeMap<FileName, Model> {
            todo!();
        }

        fn textures(&self) -> BTreeMap<FileName, Texture> {
            todo!();
        }
    }
}
