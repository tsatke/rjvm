use std::path::Path;

use mockall::automock;

use crate::basepath::BasePathBackend;
use crate::copy_on_write::CopyOnWriteBackend;
use crate::file::File;
use crate::mem::InMemoryBackend;
use crate::os::OsFileBackend;

mod basepath;
mod copy_on_write;
mod file;
mod mem;
mod os;

#[automock]
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

    pub fn new_copy_on_write_fs(underlying: FileSystem) -> Self {
        Self {
            inner: Box::new(CopyOnWriteBackend::new(underlying)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_open() {
        let path = Path::new("test.txt");
        let mut mock = MockFileBackend::new();
        mock.expect_open()
            .with(eq(path))
            .times(1)
            .returning(|x| Err(Error::new(ErrorKind::Unsupported, "test-error")));

        let fs = FileSystem {
            inner: Box::new(mock),
        };
        let _ = fs.open(path);
    }

    #[test]
    fn test_create() {
        let path = Path::new("test.txt");
        let mut mock = MockFileBackend::new();
        mock.expect_create()
            .with(eq(path))
            .times(1)
            .returning(|x| Err(Error::new(ErrorKind::Unsupported, "test-error")));

        let fs = FileSystem {
            inner: Box::new(mock),
        };
        let _ = fs.create(path);
    }

    #[test]
    fn test_create_dir() {
        let path = Path::new("testdir");
        let mut mock = MockFileBackend::new();
        mock.expect_create_dir()
            .with(eq(path))
            .times(1)
            .returning(|x| Ok(()));

        let fs = FileSystem {
            inner: Box::new(mock),
        };
        let _ = fs.create_dir(path);
    }

    #[test]
    fn test_exists() {
        let path = Path::new("test.txt");
        let mut mock = MockFileBackend::new();
        mock.expect_exists()
            .with(eq(path))
            .times(1)
            .returning(|x| Ok(false));

        let fs = FileSystem {
            inner: Box::new(mock),
        };
        let _ = fs.exists(path);
    }

    #[test]
    fn test_move() {
        let old = Path::new("old.txt");
        let new = Path::new("new.txt");
        let mut mock = MockFileBackend::new();
        mock.expect_move()
            .with(eq(old), eq(new))
            .times(1)
            .returning(|x, y| Ok(()));

        let fs = FileSystem {
            inner: Box::new(mock),
        };
        let _ = fs.r#move(old, new);
    }

    #[test]
    fn test_remove_file() {
        let path = Path::new("test.txt");
        let mut mock = MockFileBackend::new();
        mock.expect_remove_file()
            .with(eq(path))
            .times(1)
            .returning(|x| Err(Error::new(ErrorKind::Unsupported, "test-error")));

        let fs = FileSystem {
            inner: Box::new(mock),
        };
        let _ = fs.remove_file(path);
    }

    #[test]
    fn test_remove_dir() {
        let path = Path::new("testdir");
        let mut mock = MockFileBackend::new();
        mock.expect_remove_dir()
            .with(eq(path))
            .times(1)
            .returning(|x| Ok(()));

        let fs = FileSystem {
            inner: Box::new(mock),
        };
        let _ = fs.remove_dir(path);
    }
}
