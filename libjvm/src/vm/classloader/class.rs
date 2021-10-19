use libjava::bytecode::Op;
use libjava::classfile::{ClassFile, ConstantPoolInfo};
use std::lazy::OnceCell;

pub struct Class {
    /// A cache for the name of this class.
    name: OnceCell<String>,
    /// The parsed class structure of this class, as parsed from the file.
    class_file: ClassFile,
}

impl Class {
    pub fn name(&self) -> &str {
        self.name.get_or_init(|| self.class_file.this_class())
    }
}

impl From<ClassFile> for Class {
    fn from(class_file: ClassFile) -> Self {
        Self {
            name: OnceCell::new(),
            class_file,
        }
    }
}
