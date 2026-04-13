use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use crate::mvr::bundle::{
    Bundle, GSD_FILE_NAME, LoadOptions, ResourceEntry, ResourceKey, ResourceKind, ResourceMap,
};

pub(crate) enum BundleSource {
    Folder { root: PathBuf },
    Archive { path: PathBuf },
}

pub(crate) trait Source {
    fn load_bundle(&self, source: BundleSource) -> Bundle;
}

pub(crate) struct FolderSource {
    pub path: PathBuf,
    pub options: LoadOptions,
}

impl Source for FolderSource {
    fn load_bundle(&self, source: BundleSource) -> Bundle {
        let description = quick_xml::de::from_reader(BufReader::new(
            File::open(self.path.join(GSD_FILE_NAME)).unwrap(),
        ))
        .unwrap();

        let mut resources = ResourceMap::new();

        if self.options.root_only {
            if let Ok(read_dir) = fs::read_dir(&self.path) {
                for entry in read_dir.flatten() {
                    let p = entry.path();

                    if !p.is_file() {
                        continue;
                    }

                    let relative = match p.strip_prefix(&self.path) {
                        Ok(r) => r,
                        Err(_) => continue,
                    };

                    let key_str = if self.options.sanitize_paths {
                        crate::sanetize_path(relative)
                    } else {
                        relative.to_string_lossy().to_string()
                    };

                    let key = ResourceKey::new(key_str);
                    let kind = ResourceKind::from_path(&p);
                    resources.insert(ResourceEntry { key, kind });
                }
            }
        } else {
            let mut stack = vec![self.path.clone()];
            while let Some(dir) = stack.pop() {
                let Ok(read_dir) = fs::read_dir(&dir) else {
                    continue;
                };

                for entry in read_dir.flatten() {
                    let p = entry.path();

                    if p.is_dir() {
                        stack.push(p);
                        continue;
                    }

                    if !p.is_file() {
                        continue;
                    }

                    let relative = match p.strip_prefix(&self.path) {
                        Ok(r) => r,
                        Err(_) => continue,
                    };

                    let key_str = if self.options.sanitize_paths {
                        crate::sanetize_path(relative)
                    } else {
                        relative.to_string_lossy().to_string()
                    };

                    let key = ResourceKey::new(key_str);
                    let kind = ResourceKind::from_path(&p);
                    resources.insert(ResourceEntry { key, kind });
                }
            }
        }

        Bundle::new(description, resources, source)
    }
}

pub(crate) struct ArchiveSource {
    pub _path: PathBuf,
    pub _options: LoadOptions,
}

impl Source for ArchiveSource {
    fn load_bundle(&self, _source: BundleSource) -> Bundle {
        todo!();
    }
}
