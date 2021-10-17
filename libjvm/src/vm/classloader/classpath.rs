use std::path::Path;

pub struct ClassPath {
    items: Vec<ClassPathEntry>,
}

impl ClassPath {
    pub fn entries(&self) -> impl Iterator<Item = &ClassPathEntry> {
        self.items.iter()
    }

    pub fn add_entry(&mut self, entry: ClassPathEntry) {
        self.items.push(entry);
    }
}

impl From<Vec<ClassPathEntry>> for ClassPath {
    fn from(items: Vec<ClassPathEntry>) -> Self {
        Self { items }
    }
}

pub enum ClassPathEntry {
    Dir(String),
    JarFile(String),
}

impl<P> From<P> for ClassPathEntry
where
    P: AsRef<str>,
{
    fn from(p: P) -> Self {
        let path_str = p.as_ref();
        let path = Path::new(path_str);
        if path.is_dir() {
            return Self::Dir(path_str.to_owned());
        }
        Self::JarFile(path_str.to_owned())
    }
}
