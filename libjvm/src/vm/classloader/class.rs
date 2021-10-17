use libjava::classfile::ClassFile;

pub struct Class {
    class_file: ClassFile,
}

impl Class {
    pub fn new(class_file: ClassFile) -> Self {
        Self { class_file }
    }
}
