use libvfs::FileSystem;
use std::io::Read;

#[test]
fn test_os_fs() {
    let fs = FileSystem::new_os_fs();
    let mut f = fs.open("tests/resources/file1").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    assert_eq!("hello world", content);
}

#[test]
fn test_base_path_fs() {
    let fs = FileSystem::new_base_path_fs(FileSystem::new_os_fs(), "tests/resources");
    let mut f = fs.open("file1").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    assert_eq!("hello world", content);
}
