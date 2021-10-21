use std::io::{Read, Seek, SeekFrom, Write};

pub trait ReadWriteSeek: Read + Write + Seek {}

impl<U> ReadWriteSeek for U where U: Read + Write + Seek {}

pub struct File {
    inner: Box<dyn ReadWriteSeek>,
}

impl File {
    pub fn new(inner: Box<dyn ReadWriteSeek>) -> Self {
        Self { inner }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}
