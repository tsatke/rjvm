use crate::file::File;
use crate::{FileBackend, FileSystem};
use std::path::{Path, PathBuf};

pub(crate) struct BasePathBackend {
    base_path: PathBuf,
    underlying: FileSystem,
}

impl BasePathBackend {
    pub fn new<P>(underlying: FileSystem, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            underlying,
            base_path: PathBuf::from(path.as_ref()),
        }
    }

    fn relativize<P>(&self, path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let mut actual = self.base_path.clone();
        actual.push(path);
        actual
    }
}

impl FileBackend for BasePathBackend {
    fn open(&self, path: &Path) -> std::io::Result<File> {
        self.underlying.open(self.relativize(path))
    }

    fn create(&self, path: &Path) -> std::io::Result<File> {
        self.underlying.create(self.relativize(path))
    }

    fn create_dir(&self, path: &Path) -> std::io::Result<()> {
        self.underlying.create_dir(self.relativize(path))
    }

    fn r#move(&self, from: &Path, to: &Path) -> std::io::Result<()> {
        self.underlying.r#move(from, to)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        self.underlying.remove_file(path)
    }

    fn remove_dir(&self, path: &Path) -> std::io::Result<()> {
        self.underlying.remove_dir(path)
    }
}
