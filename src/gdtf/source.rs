use std::{fs::File, io::BufReader, path::PathBuf};

use crate::gdtf::{DESCRIPTION_FILE_NAME, Gdtf};

pub trait GdtfSource {
    fn load(&self) -> Gdtf;
}

pub struct FolderSource {
    pub path: PathBuf,
}

impl GdtfSource for FolderSource {
    fn load(&self) -> Gdtf {
        let description = quick_xml::de::from_reader(BufReader::new(
            File::open(self.path.join(DESCRIPTION_FILE_NAME)).unwrap(),
        ))
        .unwrap();

        if let Ok(read_dir) = std::fs::read_dir(&self.path) {
            for entry in read_dir.flatten() {
                let p = entry.path();

                if !p.is_file() {
                    continue;
                }

                let _file_name = p.file_name().unwrap();
            }
        }

        Gdtf { description }
    }
}

pub struct ArchiveSource {
    pub path: PathBuf,
}

impl GdtfSource for ArchiveSource {
    fn load(&self) -> Gdtf {
        todo!();
    }
}
