use crate::basepath::BasePathBackend;
use crate::file::File;
use crate::mem::InMemoryBackend;
use crate::os::OsFileBackend;
use std::path::Path;

pub mod basepath;
pub mod file;
pub mod mem;
pub mod os;

pub trait FileBackend {
    fn open(&self, path: &Path) -> std::io::Result<File>;

    fn exists(&self, path: &Path) -> std::io::Result<bool>;

    fn create(&self, path: &Path) -> std::io::Result<File>;

    fn create_dir(&self, path: &Path) -> std::io::Result<()>;

    fn r#move(&self, from: &Path, to: &Path) -> std::io::Result<()>;

    fn remove_file(&self, path: &Path) -> std::io::Result<()>;

    fn remove_dir(&self, path: &Path) -> std::io::Result<()>;
}

pub struct FileSystem {
    inner: Box<dyn FileBackend>,
}

impl FileSystem {
    pub fn new_os_fs() -> Self {
        Self {
            inner: Box::new(OsFileBackend::new()),
        }
    }

    pub fn new_base_path_fs<P>(underlying: FileSystem, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            inner: Box::new(BasePathBackend::new(underlying, path)),
        }
    }

    pub fn new_in_memory_fs() -> Self {
        Self {
            inner: Box::new(InMemoryBackend::new()),
        }
    }

    pub fn open<P>(&self, path: P) -> std::io::Result<File>
    where
        P: AsRef<Path>,
    {
        self.inner.open(path.as_ref())
    }

    pub fn exists<P>(&self, path: P) -> std::io::Result<bool>
    where
        P: AsRef<Path>,
    {
        self.inner.exists(path.as_ref())
    }

    pub fn create<P>(&self, path: P) -> std::io::Result<File>
    where
        P: AsRef<Path>,
    {
        self.inner.create(path.as_ref())
    }

    pub fn create_dir<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        self.inner.create_dir(path.as_ref())
    }

    pub fn r#move<P>(&self, from: P, to: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        self.inner.r#move(from.as_ref(), to.as_ref())
    }

    pub fn remove_file<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        self.inner.remove_file(path.as_ref())
    }

    pub fn remove_dir<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        self.inner.remove_dir(path.as_ref())
    }
}

impl FileBackend for FileSystem {
    fn open(&self, path: &Path) -> std::io::Result<File> {
        self.open(path)
    }

    fn exists(&self, path: &Path) -> std::io::Result<bool> {
        self.exists(path)
    }

    fn create(&self, path: &Path) -> std::io::Result<File> {
        self.create(path)
    }

    fn create_dir(&self, path: &Path) -> std::io::Result<()> {
        self.create_dir(path)
    }

    fn r#move(&self, from: &Path, to: &Path) -> std::io::Result<()> {
        self.r#move(from, to)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        self.remove_file(path)
    }

    fn remove_dir(&self, path: &Path) -> std::io::Result<()> {
        self.remove_dir(path)
    }
}
