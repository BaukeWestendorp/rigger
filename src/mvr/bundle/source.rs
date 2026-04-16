use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use crate::mvr::bundle::{
    Bundle, GSD_FILE_NAME, ResourceEntry, ResourceKey, ResourceKind, ResourceMap,
};

pub(crate) enum BundleSource {
    Folder { root: PathBuf },
    Archive { temp_dir: tempfile::TempDir },
}

impl BundleSource {
    pub fn root_folder(&self) -> &Path {
        match &self {
            BundleSource::Folder { root } => root.as_path(),
            BundleSource::Archive { temp_dir, .. } => temp_dir.path(),
        }
    }
}

pub(crate) trait Source {
    fn load_bundle(&self, source: BundleSource) -> Bundle;
}

pub(crate) struct FolderSource {
    path: PathBuf,
}

impl FolderSource {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Source for FolderSource {
    fn load_bundle(&self, source: BundleSource) -> Bundle {
        let description = quick_xml::de::from_reader(BufReader::new(
            File::open(source.root_folder().join(GSD_FILE_NAME)).unwrap(),
        ))
        .unwrap();

        let mut resources = ResourceMap::new();

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

                let key = ResourceKey::new(relative.to_string_lossy());
                let kind = ResourceKind::from_path(&p);
                resources.insert(ResourceEntry { key, kind });
            }
        }

        Bundle::new(description, resources, source)
    }
}

pub(crate) struct ArchiveSource {
    path: PathBuf,
}

impl ArchiveSource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn extract_to_root(&self, root: &Path) {
        let file = File::open(self.path()).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let corrected_name = String::from_utf8_lossy(file.name_raw());
            let relative = PathBuf::from(corrected_name.to_string());
            let out_path = {
                let joined = root.join(&relative);
                let out_path = joined.canonicalize().unwrap_or(joined);
                if !out_path.starts_with(root) {
                    panic!("Invalid file path in archive: {:?}", relative);
                }
                out_path
            };

            if file.is_dir() {
                fs::create_dir_all(&out_path).unwrap();
            } else {
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut out_file = File::create(&out_path).unwrap();
                io::copy(&mut file, &mut out_file).unwrap();
            }
        }
    }
}

impl Source for ArchiveSource {
    fn load_bundle(&self, source: BundleSource) -> Bundle {
        let root = source.root_folder().to_path_buf();

        fs::create_dir_all(&root).unwrap();
        self.extract_to_root(&root);

        let folder_loader = FolderSource { path: root.clone() };
        folder_loader.load_bundle(source)
    }
}
