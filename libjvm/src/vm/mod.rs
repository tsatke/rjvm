use std::sync::{Arc, RwLock};

use classloader::classpath::ClassPathEntry;

use crate::vm::area::{Heap, MethodArea};
use crate::vm::thread::Thread;

pub mod area;
pub mod class;
pub mod classloader;
pub mod opcode;
pub mod stack;
pub mod thread;
pub mod types;

pub struct VMBuilder {
    class_path_entries: Vec<ClassPathEntry>,
}

impl VMBuilder {
    pub fn new() -> Self {
        Self {
            class_path_entries: Vec::new(),
        }
    }

    pub fn build(self) -> VM {
        VM::from(self)
    }

    pub fn add_classpath_entry(mut self, entry: ClassPathEntry) -> Self {
        self.class_path_entries.push(entry);
        self
    }
}

pub struct VM {
    /// Specified by [`$2.5.3`].
    ///
    /// [`$2.5.3`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-2.html#jvms-2.5.3
    heap: Arc<RwLock<Heap>>,
    method_area: Arc<RwLock<MethodArea>>,
}

impl From<VMBuilder> for VM {
    fn from(builder: VMBuilder) -> Self {
        Self {
            heap: Arc::new(RwLock::new(Heap::new())),
            method_area: Arc::new(RwLock::new(MethodArea::new())),
        }
    }
}

impl VM {
    pub fn run_main_class(self, class_name: &'static str) {
        let thread = std::thread::spawn(move || {
            let mut main_thread = Thread::new();
            main_thread.run_method(class_name, "main:([Ljava/lang/String;)V");
        });
        let _ = thread.join();
    }
}
