use std::cmp::min;
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

    fn exists(&self, path: &Path) -> std::io::Result<bool> {
        Ok(self
            .files
            .lock()
            .unwrap()
            .contains_key(path.to_str().unwrap()))
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
        let to_read = min(buf_len, data.len() - ptr);
        buf[0..to_read].copy_from_slice(&data[ptr..ptr + to_read]);
        self.pointer += to_read as u64;
        Ok(to_read)
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
        buf.iter().for_each(|&b| data.push(b));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_write_open_read_file() {
        let fs = InMemoryBackend::new();
        let path = Path::new("test.txt");
        {
            let mut f = fs.create(path).unwrap();
            writeln!(&mut f, "some number: {}", 7);
        }
        let mut f = fs.open(path).unwrap();
        let mut actual = String::new();
        f.read_to_string(&mut actual).unwrap();
        assert_eq!("some number: 7\n", actual);
    }

    #[test]
    fn test_exists_remove_file() {
        let fs = InMemoryBackend::new();
        let path = Path::new("test.txt");
        assert!(!fs.exists(path).unwrap());
        fs.create(path).unwrap();
        assert!(fs.exists(path).unwrap());
        fs.remove_file(path).unwrap();
        assert!(!fs.exists(path).unwrap());
    }

    #[test]
    fn test_move() {
        let fs = InMemoryBackend::new();
        let old_path = Path::new("old.txt");
        let new_path = Path::new("new.txt");
        assert!(!fs.exists(old_path).unwrap());
        assert!(!fs.exists(new_path).unwrap());

        fs.create(old_path).unwrap();
        assert!(fs.exists(old_path).unwrap());
        assert!(!fs.exists(new_path).unwrap());

        fs.r#move(old_path, new_path).unwrap();
        assert!(!fs.exists(old_path).unwrap());
        assert!(fs.exists(new_path).unwrap());
    }
}
