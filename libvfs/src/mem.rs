use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

use prefix_tree::PrefixMap;

use crate::file::File;
use crate::FileBackend;

enum InMemFsEntry {
    File(InMemFileData),
    Dir,
}

pub struct InMemoryBackend {
    files: Mutex<PrefixMap<u8, InMemFsEntry>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            files: Mutex::new(PrefixMap::new()),
        }
    }

    fn find_data(&self, path: &Path) -> Option<InMemFileData> {
        let guard = self.files.lock().unwrap();
        let entry = match guard.get(path.to_str().unwrap()) {
            None => return None,
            Some(e) => e,
        };
        let file_entry = match entry {
            InMemFsEntry::File(e) => e,
            InMemFsEntry::Dir => return None,
        };
        Some(file_entry.clone())
    }
}

impl FileBackend for InMemoryBackend {
    fn open(&self, path: &Path) -> std::io::Result<File> {
        let data = match self.find_data(path) {
            None => return Err(Error::new(ErrorKind::NotFound, "file not found")),
            Some(d) => d,
        };
        Ok(File::new(Box::new(InMemFile::from(data))))
    }

    fn create(&self, path: &Path) -> std::io::Result<File> {
        let mut files = self.files.lock().unwrap();
        let p = path.to_str().unwrap();
        if files.contains_key(p) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "the file already exists",
            ));
        }

        let data = InMemFileData::new(Mutex::new(vec![]));
        let file = File::new(Box::new(InMemFile::from(data.clone())));
        files.insert(p, InMemFsEntry::File(data));

        Ok(file)
    }

    fn create_dir(&self, path: &Path) -> std::io::Result<()> {
        let mut files = self.files.lock().unwrap();
        let p = path.to_str().unwrap();
        if files.contains_key(p) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "the directory already exists",
            ));
        }

        files.insert(p, InMemFsEntry::Dir);

        Ok(())
    }

    fn r#move(&self, from: &Path, to: &Path) -> std::io::Result<()> {
        let mut files = self.files.lock().unwrap();
        let f = from.to_str().unwrap();
        let t = to.to_str().unwrap();
        if !files.contains_key(f) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "the file or directory does not exist",
            ));
        }
        let entry = files.remove(f).unwrap();
        files.insert(t, entry);

        Ok(())
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        let mut files = self.files.lock().unwrap();
        let p = path.to_str().unwrap();
        if !files.contains_key(p) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "the file does not exist",
            ));
        }
        match files.get(p).unwrap() {
            InMemFsEntry::File(_) => files.remove(p),
            InMemFsEntry::Dir => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "the file is a directory",
                ))
            }
        };

        Ok(())
    }

    fn remove_dir(&self, path: &Path) -> std::io::Result<()> {
        let mut files = self.files.lock().unwrap();
        let p = path.to_str().unwrap();
        if !files.contains_key(p) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "the directory does not exist",
            ));
        }
        match files.get(p).unwrap() {
            InMemFsEntry::File(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "the directory is a file",
                ))
            }
            InMemFsEntry::Dir => files.remove(p),
        };

        Ok(())
    }
}

type InMemFileData = Arc<Mutex<Vec<u8>>>;

struct InMemFile {
    pointer: u64,
    data: InMemFileData,
}

impl From<InMemFileData> for InMemFile {
    fn from(data: InMemFileData) -> Self {
        Self { data, pointer: 0 }
    }
}

impl Read for InMemFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let data = self.data.lock().unwrap();
        let ptr = self.pointer as usize;
        let buf_len = buf.len();
        buf.copy_from_slice(&data[ptr..ptr + buf_len]);
        Ok(buf_len)
    }
}

impl Write for InMemFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut data = self.data.lock().unwrap();
        let required_length = self.pointer as usize + buf.len();
        let data_len = data.len();
        if data_len < required_length {
            data.reserve(required_length - data_len);
        }
        data[self.pointer as usize..].copy_from_slice(&buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Seek for InMemFile {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let mut current_pointer = self.pointer as i64;
        match pos {
            SeekFrom::Start(pointer) => current_pointer = pointer as i64,
            SeekFrom::End(pointer) => {
                current_pointer = self.data.lock().unwrap().len() as i64 + pointer
            }
            SeekFrom::Current(pointer) => current_pointer += pointer,
        };
        if current_pointer < 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "negative offset"));
        }

        self.pointer = current_pointer as u64;
        Ok(self.pointer)
    }
}
