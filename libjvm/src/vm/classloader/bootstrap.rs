use crate::vm::classloader::class::Class;
use crate::vm::classloader::classpath::{ClassPath, ClassPathEntry};
use crate::vm::classloader::ClassLoader;
use libjava::classfile::ClassFile;
use std::io::{BufReader, Read};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use vfs::impls::altroot::AltrootFS;
use vfs::{FileSystem, SeekAndRead, VfsPath, VfsResult};

pub struct BootstrapClassLoader {
    fs: VfsPath,
    class_path: ClassPath,
    loaded_classes: Vec<Rc<Class>>,
}

impl BootstrapClassLoader {
    pub fn new(fs: VfsPath, class_path: ClassPath) -> Self {
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

    fn find_class<N>(&self, n: N) -> Option<Rc<Class>>
    where
        N: AsRef<str>,
    {
        let path = n.as_ref();
        self.loaded_classes
            .iter()
            .find(|c| c.name() == path)
            .cloned()
    }

    fn find_or_load_class<N>(&mut self, n: N) -> Option<Rc<Class>>
    where
        N: AsRef<str>,
    {
        let mut path = String::from(n.as_ref());
        path.push_str(".class");

        let file = {
            let mut file_opt: Option<Box<dyn SeekAndRead>> = None;
            for entry in self.class_path.entries() {
                let entry_path = match entry {
                    ClassPathEntry::Dir(s) => s.clone(),
                    ClassPathEntry::JarFile(_) => unimplemented!("jar class loading"),
                };

                let entry = self
                    .fs
                    .join(entry_path.as_str())
                    .unwrap()
                    .join(path.as_str())
                    .unwrap();
                if entry.exists().unwrap_or(false) {
                    file_opt = Some(entry.open_file().expect("unable to open file"));
                }
            }
            if file_opt.is_none() {
                // no matching file found in the classpath
                return None;
            }
            file_opt.unwrap()
        };

        let mut rd = BufReader::new(file);
        let parse_result = ClassFile::parse(&mut rd);
        let class_file = match parse_result {
            Ok(f) => f,
            Err(_) => return None,
        };

        let class = Class::from(class_file);
        let rc = Rc::new(class);
        self.loaded_classes.push(rc.clone());
        Some(rc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vfs::{MemoryFS, PhysicalFS};

    #[test]
    fn test_load_class() {
        let base: VfsPath = PhysicalFS::new("".into()).into();
        let mut class_loader = BootstrapClassLoader::new(
            base,
            ClassPath::from(vec![ClassPathEntry::Dir(
                "tests/resources/vm/classloader".into(),
            )]),
        );
        let res = class_loader.find_or_load_class("Test1");
        assert!(res.is_some());
        let class = res.unwrap();
        assert_eq!("Test1", class.name());
    }
}
