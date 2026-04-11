use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use crate::{
    gdtf::Gdtf,
    mvr::{
        GSD_FILE_NAME, GdtfHandle, GdtfResource, ModelHandle, ModelResource, Mvr, TextureHandle,
        TextureResource,
    },
};

pub trait MvrSource {
    fn load(&self) -> Mvr;
}

pub struct FolderSource {
    pub path: PathBuf,
}

impl MvrSource for FolderSource {
    fn load(&self) -> Mvr {
        let gsd = quick_xml::de::from_reader(BufReader::new(
            File::open(self.path.join(GSD_FILE_NAME)).unwrap(),
        ))
        .unwrap();

        let mut models = HashMap::new();
        let mut textures = HashMap::new();
        let mut gdtfs = HashMap::new();

        if let Ok(read_dir) = std::fs::read_dir(&self.path) {
            for entry in read_dir.flatten() {
                let p = entry.path();

                let relative = match p.strip_prefix(&self.path) {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                let handle_id = crate::sanetize_path(relative);

                let path = self.path.join(&handle_id).into();
                if ModelResource::check_path(&p) {
                    models.insert(ModelHandle::new(handle_id.clone()), ModelResource { path });
                } else if TextureResource::check_path(&p) {
                    textures
                        .insert(TextureHandle::new(handle_id.clone()), TextureResource { path });
                } else if GdtfResource::check_path(&p) {
                    // FIXME: This sould accept from_folder or from_archive (maybe add from_path?).
                    let gdtf = Gdtf::from_folder(&path);
                    gdtfs.insert(GdtfHandle::new(handle_id.clone()), GdtfResource { path, gdtf });
                }
            }
        }

        Mvr { gsd, models, textures, gdtfs }
    }
}

pub struct ArchiveSource {
    pub path: PathBuf,
}

impl MvrSource for ArchiveSource {
    fn load(&self) -> Mvr {
        todo!();
    }
}
