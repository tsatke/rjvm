use crate::classfile::ClassFile;
use std::sync::{Arc, Mutex};

use crate::vm::classloader::ClassLoader;

pub struct Class {
    class_file: ClassFile,
}

impl Class {
    pub fn new(class_file: ClassFile) -> Self {
        Self { class_file }
    }
}
