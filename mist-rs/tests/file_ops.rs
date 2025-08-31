use mist_rs::helpers::file_manager::{copy_file, move_file_path, remove_file};
use tempfile::tempdir;
use std::fs;

#[test]
fn file_operations_work() {
    let tmp = tempdir().expect("tmp");
    let src = tmp.path().join("a.txt");
    fs::write(&src, "hello").unwrap();
    let copied = tmp.path().join("b.txt");
    copy_file(&src, &copied).unwrap();
    assert!(copied.exists());
    let moved = tmp.path().join("c.txt");
    move_file_path(&copied, &moved).unwrap();
    assert!(!copied.exists());
    assert!(moved.exists());
    remove_file(&moved).unwrap();
    assert!(!moved.exists());
}
