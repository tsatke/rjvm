use libjvm::vm::classloader::classpath::ClassPathEntry;
use libjvm::vm::VMBuilder;

#[test]
pub fn test_simple_vm() {
    let vm = VMBuilder::new()
        .add_classpath_entry(ClassPathEntry::from("tests/simple"))
        .build();
    vm.run_main_class("Main");
}
