use crate::file::File;
use crate::FileBackend;
use std::path::Path;

pub(crate) struct OsFileBackend {}

impl OsFileBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileBackend for OsFileBackend {
    fn open(&self, path: &Path) -> std::io::Result<File> {
        let f = std::fs::File::open(path)?;
        Ok(File::new(Box::new(f)))
    }

    fn create(&self, path: &Path) -> std::io::Result<File> {
        let f = std::fs::File::create(path)?;
        Ok(File::new(Box::new(f)))
    }

    fn create_dir(&self, path: &Path) -> std::io::Result<()> {
        std::fs::create_dir(path)
    }

    fn r#move(&self, from: &Path, to: &Path) -> std::io::Result<()> {
        std::fs::rename(from, to)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        std::fs::remove_file(path)
    }

    fn remove_dir(&self, path: &Path) -> std::io::Result<()> {
        std::fs::remove_dir(path)
    }
}
