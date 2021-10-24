use libjvm::vm::classloader::class::Class;
use libjvm::vm::classloader::classpath::ClassPathEntry;
use libjvm::vm::VM;
use libvfs::FileSystem;
use std::path::PathBuf;

#[test]
pub fn test_simple_vm() {
    let fs = FileSystem::new_os_fs();
    let cp = vec![ClassPathEntry::from("test/simple")];

    let vm = VM::new(fs, cp.into());
    vm.run_main_class("Main");
}
