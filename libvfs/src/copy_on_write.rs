use crate::file::File;
use crate::mem::InMemoryBackend;
use crate::{FileBackend, FileSystem};
use prefix_tree::PrefixSet;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Mutex, RwLock};

pub struct CopyOnWriteBackend {
    deleted_paths: RwLock<PrefixSet<u8>>,
    layer: Rc<Mutex<InMemoryBackend>>,
    fallback: FileSystem,
}

impl CopyOnWriteBackend {
    pub fn new(underlying: FileSystem) -> Self {
        Self {
            deleted_paths: RwLock::new(PrefixSet::new()),
            layer: Rc::new(Mutex::new(InMemoryBackend::new())),
            fallback: underlying,
        }
    }
}

impl FileBackend for CopyOnWriteBackend {
    fn open(&self, path: &Path) -> std::io::Result<File> {
        if self
            .deleted_paths
            .read()
            .unwrap()
            .contains(path.to_str().unwrap())
        {
            return Err(Error::new(ErrorKind::NotFound, "file was not found"));
        }

        if self.layer.lock().unwrap().exists(path)? {
            let f = self.layer.lock().unwrap().open(path)?;
            return Ok(File::new(Box::new(CopyOnWriteFile::new_from_layer(
                self.layer.clone(),
                f,
                path,
            ))));
        }

        let f = self.fallback.open(path)?;
        Ok(File::new(Box::new(CopyOnWriteFile::new_from_underlying(
            self.layer.clone(),
            f,
            path,
        ))))
    }

    fn exists(&self, path: &Path) -> std::io::Result<bool> {
        let p = path.to_str().unwrap();
        let is_deleted = self.deleted_paths.read().unwrap().contains(p);
        Ok((!is_deleted)
            && (self.layer.lock().unwrap().exists(path)? || self.fallback.exists(path)?))
    }

    fn create(&self, path: &Path) -> std::io::Result<File> {
        if self.exists(path)? {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "the file already exists",
            ));
        }

        self.deleted_paths
            .write()
            .unwrap()
            .remove(path.to_str().unwrap());
        self.layer.lock().unwrap().create(path)
    }

    fn create_dir(&self, path: &Path) -> std::io::Result<()> {
        if self.exists(path)? {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "the directory already exists",
            ));
        }

        self.layer.lock().unwrap().create_dir(path)
    }

    fn r#move(&self, from: &Path, to: &Path) -> std::io::Result<()> {
        todo!("move")
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        todo!("only attempt to delete in layer if it exists in layer, and also check fallback FS for the path to exist");
        self.layer.lock().unwrap().remove_file(path)?;
        self.deleted_paths
            .write()
            .unwrap()
            .insert(path.to_str().unwrap());
        Ok(())
    }

    fn remove_dir(&self, path: &Path) -> std::io::Result<()> {
        todo!("only attempt to delete in layer if it exists in layer, and also check fallback FS for the path to exist");
        self.layer.lock().unwrap().remove_dir(path)?;
        self.deleted_paths
            .write()
            .unwrap()
            .insert(path.to_str().unwrap());
        Ok(())
    }
}

struct CopyOnWriteFile {
    layer: Rc<Mutex<InMemoryBackend>>,
    underlying: File,
    path: PathBuf,
    writable: bool,
}

impl CopyOnWriteFile {
    fn new_from_underlying(layer: Rc<Mutex<InMemoryBackend>>, file: File, path: &Path) -> Self {
        Self {
            layer,
            underlying: file,
            path: PathBuf::from(path),
            writable: false,
        }
    }

    fn new_from_layer(layer: Rc<Mutex<InMemoryBackend>>, file: File, path: &Path) -> Self {
        Self {
            layer,
            underlying: file,
            path: PathBuf::from(path),
            writable: true,
        }
    }

    fn copy_into_layer(&mut self) {
        let position = self.underlying.seek(SeekFrom::Current(0)).unwrap();

        let mut destination = self
            .layer
            .lock()
            .unwrap()
            .create(self.path.as_path())
            .unwrap();
        self.underlying.seek(SeekFrom::Start(0));
        std::io::copy(&mut self.underlying, &mut destination);
        destination.seek(SeekFrom::Start(position));
        self.underlying = destination;
    }
}

impl Read for CopyOnWriteFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.underlying.read(buf)
    }
}

impl Write for CopyOnWriteFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if !self.writable {
            self.copy_into_layer();
        }

        self.underlying.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if !self.writable {
            self.copy_into_layer();
        }

        self.underlying.flush()
    }
}

impl Seek for CopyOnWriteFile {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.underlying.seek(pos)
    }
}
