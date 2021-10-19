use libjvm::vm::classloader::class::Class;
use libjvm::vm::classloader::classpath::ClassPathEntry;
use libjvm::vm::VM;
use std::path::PathBuf;
use vfs::PhysicalFS;

#[test]
pub fn test_simple_vm() {
    let fs = PhysicalFS::new(PathBuf::new());
    let cp = vec![ClassPathEntry::from("test/simple")];

    let vm = VM::new(fs, cp.into());
    vm.run_main_class("Main");
}
