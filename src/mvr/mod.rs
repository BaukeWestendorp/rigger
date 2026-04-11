use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use uuid::Uuid;

mod gsd;
mod resource;
mod source;

pub use gsd::*;
pub use resource::*;
use source::*;

const GSD_FILE_NAME: &str = "GeneralSceneDescription.xml";

pub struct Mvr {
    gsd: GeneralSceneDescription,

    models: HashMap<ModelHandle, ModelResource>,
    textures: HashMap<TextureHandle, TextureResource>,
    gdtfs: HashMap<GdtfHandle, GdtfResource>,
}

impl Mvr {
    pub fn from_folder(path: impl Into<PathBuf>) -> Self {
        FolderSource { path: path.into() }.load()
    }

    pub fn from_archive(path: impl Into<PathBuf>) -> Self {
        ArchiveSource { path: path.into() }.load()
    }

    pub fn gsd(&self) -> &GeneralSceneDescription {
        &self.gsd
    }

    pub fn models(&self) -> &HashMap<ModelHandle, ModelResource> {
        &self.models
    }

    pub fn model(&self, handle: &ModelHandle) -> Option<&ModelResource> {
        self.models.get(handle)
    }

    pub fn model_handle(&self, relative_path_from_source: impl AsRef<Path>) -> Option<ModelHandle> {
        let s = crate::sanetize_path(relative_path_from_source.as_ref());
        let handle = ModelHandle::new(s);
        self.models.contains_key(&handle).then_some(handle)
    }

    pub fn textures(&self) -> &HashMap<TextureHandle, TextureResource> {
        &self.textures
    }

    pub fn texture(&self, handle: &TextureHandle) -> Option<&TextureResource> {
        self.textures.get(handle)
    }

    pub fn texture_handle(
        &self,
        relative_path_from_source: impl AsRef<Path>,
    ) -> Option<TextureHandle> {
        let s = crate::sanetize_path(relative_path_from_source.as_ref());
        let handle = TextureHandle::new(s);
        self.textures.contains_key(&handle).then_some(handle)
    }

    pub fn gdtfs(&self) -> &HashMap<GdtfHandle, GdtfResource> {
        &self.gdtfs
    }

    pub fn gdtf(&self, handle: &GdtfHandle) -> Option<&GdtfResource> {
        self.gdtfs.get(handle)
    }

    pub fn gdtf_handle(&self, relative_path_from_source: impl AsRef<Path>) -> Option<GdtfHandle> {
        let s = crate::sanetize_path(relative_path_from_source.as_ref());
        let handle = GdtfHandle::new(s);
        self.gdtfs.contains_key(&handle).then_some(handle)
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
