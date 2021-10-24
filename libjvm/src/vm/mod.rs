use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use classloader::classpath::ClassPathEntry;
use libvfs::FileSystem;

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
    file_system: Rc<FileSystem>,
    bootstrap_class_loader: BootstrapClassLoader,
}

impl Default for VM {
    fn default() -> Self {
        // default classpath should probably contain the jars in $JAVA_HOME
        Self::new(FileSystem::new_os_fs(), ClassPath::from(vec![]))
    }
}

impl VM {
    pub fn new(fs: FileSystem, cp: ClassPath) -> Self {
        Self {
            heap: Arc::new(RwLock::new(Heap::new())),
            method_area: Arc::new(RwLock::new(MethodArea::new())),
            bootstrap_class_loader: BootstrapClassLoader::new(fs.clone(), cp),
            file_system: Rc::new(fs),
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
