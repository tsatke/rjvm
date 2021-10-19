use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use vfs::{FileSystem, PhysicalFS, VfsPath};

use classloader::classpath::ClassPathEntry;

use crate::vm::area::{Heap, MethodArea};
use crate::vm::classloader::bootstrap::BootstrapClassLoader;
use crate::vm::classloader::classpath::ClassPath;
use crate::vm::thread::Thread;

pub mod area;
pub mod classloader;
pub mod stack;
pub mod thread;
pub mod types;

pub struct VM {
    /// Specified by [`$2.5.3`].
    ///
    /// [`$2.5.3`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.5.3
    heap: Arc<RwLock<Heap>>,
    method_area: Arc<RwLock<MethodArea>>,
    file_system: VfsPath,
    bootstrap_class_loader: BootstrapClassLoader,
}

impl Default for VM {
    fn default() -> Self {
        // default classpath should probably contain the jars in $JAVA_HOME
        Self::new(PhysicalFS::new(PathBuf::new()), ClassPath::from(vec![]))
    }
}

impl VM {
    pub fn new(fs: impl FileSystem, cp: ClassPath) -> Self {
        let file_system: VfsPath = fs.into();
        Self {
            heap: Arc::new(RwLock::new(Heap::new())),
            method_area: Arc::new(RwLock::new(MethodArea::new())),
            bootstrap_class_loader: BootstrapClassLoader::new(file_system.clone(), cp),
            file_system,
        }
    }

    pub fn run_main_class(self, class_name: &'static str) {
        let thread = std::thread::spawn(move || {
            let mut main_thread = Thread::new();
            main_thread.run_method(class_name, "main:([Ljava/lang/String;)V");
        });
        let _ = thread.join();
    }
}
