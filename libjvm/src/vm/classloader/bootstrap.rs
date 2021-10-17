use crate::vm::class::Class;
use crate::vm::classloader::classpath::{ClassPath, ClassPathEntry};
use crate::vm::classloader::ClassLoader;
use std::sync::{Arc, RwLock};
use vfs::FileSystem;

pub struct BootstrapClassLoader {
    fs: Arc<RwLock<dyn FileSystem>>,
    class_path: ClassPath,
    loaded_classes: Vec<Class>,
}

impl BootstrapClassLoader {
    pub fn new(fs: Arc<RwLock<dyn FileSystem>>, class_path: ClassPath) -> Self {
        Self {
            fs,
            class_path,
            loaded_classes: vec![],
        }
    }
}

impl ClassLoader for BootstrapClassLoader {
    fn add_entry(&mut self, entry: ClassPathEntry) {
        self.class_path.add_entry(entry)
    }

    fn find_class<N>(&self, n: N) -> Option<&Class>
    where
        N: AsRef<str>,
    {
        let name = n.as_ref();
        for c in &self.loaded_classes {
            if c.name() == name {
                return Some(c);
            }
        }
        None
    }

    fn find_or_load_class<N>(&mut self, n: N) -> Option<&Class>
    where
        N: AsRef<str>,
    {
        if let Some(class) = self.find_class(n) {
            return Some(class);
        }

        todo!("load class from disk")
    }
}
