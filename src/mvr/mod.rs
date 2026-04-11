mod gsd;

use std::{ops, path::PathBuf, str::FromStr};

pub use gsd::*;
use uuid::Uuid;

const GSD_FILE_NAME: &str = "GeneralSceneDescription.xml";

pub struct Mvr<S: source::Source> {
    gsd: GeneralSceneDescription,

    source: S,
}

impl<S: source::Source> Mvr<S> {
    pub fn from_source(source: S) -> Self {
        let gsd = quick_xml::de::from_reader(source.gsd_reader()).unwrap();
        Self { gsd, source }
    }

    pub fn gsd(&self) -> &GeneralSceneDescription {
        &self.gsd
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

impl Mvr<source::Folder> {
    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        Self::from_source(source::Folder::new(path))
    }
}

impl Mvr<source::Archive> {
    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        Self::from_source(source::Archive::new(path))
    }
}

impl<S: source::Source> ops::Deref for Mvr<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<S: source::Source> ops::DerefMut for Mvr<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
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
        type Model: Clone;
        type Texture: Clone;

        fn gsd_reader(&self) -> Box<dyn io::BufRead>;

        fn models(&self) -> &BTreeMap<FileName, Self::Model>;

        fn model_from_file_name(&self, file_name: &str) -> Self::Model;

        fn textures(&self) -> &BTreeMap<FileName, Self::Texture>;

        fn texture_from_file_name(&self, file_name: &str) -> Self::Texture;
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
        models: BTreeMap<FileName, PathBuf>,
        textures: BTreeMap<FileName, PathBuf>,
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
                            path.join(&file_name).canonicalize().unwrap(),
                        );
                    } else if is_texture_file(&p) {
                        textures.insert(
                            file_name.clone(),
                            path.join(&file_name).canonicalize().unwrap(),
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
        type Model = PathBuf;
        type Texture = PathBuf;

        fn gsd_reader(&self) -> Box<dyn io::BufRead> {
            let gsd_path = self.path.join(GSD_FILE_NAME);
            Box::new(BufReader::new(File::open(gsd_path).unwrap()))
        }

        fn models(&self) -> &BTreeMap<FileName, Self::Model> {
            &self.models
        }

        fn model_from_file_name(&self, file_name: &str) -> Self::Model {
            self.path.join(file_name)
        }

        fn textures(&self) -> &BTreeMap<FileName, Self::Texture> {
            &self.textures
        }

        fn texture_from_file_name(&self, file_name: &str) -> Self::Texture {
            self.path.join(file_name)
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
        type Model = ();
        type Texture = ();

        fn gsd_reader(&self) -> Box<dyn io::BufRead> {
            todo!();
        }

        fn models(&self) -> &BTreeMap<FileName, Self::Model> {
            todo!();
        }

        fn model_from_file_name(&self, _file_name: &str) -> Self::Model {
            todo!();
        }

        fn textures(&self) -> &BTreeMap<FileName, Self::Texture> {
            todo!();
        }

        fn texture_from_file_name(&self, _file_name: &str) -> Self::Texture {
            todo!();
        }
    }
}
