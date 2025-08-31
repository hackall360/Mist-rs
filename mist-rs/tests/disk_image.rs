use mist_rs::helpers::disk_image_creator::create_disk_image;
use tempfile::tempdir;
use std::fs::{self, File};

#[cfg(target_os = "macos")]
#[test]
fn creates_disk_image_macos() {
    let tmp = tempdir().expect("tmp");
    let src = tmp.path().join("src");
    fs::create_dir(&src).unwrap();
    File::create(src.join("test.txt")).unwrap();
    let dmg = tmp.path().join("test.dmg");
    create_disk_image(&src, &dmg).expect("create dmg");
    assert!(dmg.exists());
}

#[cfg(not(target_os = "macos"))]
#[test]
fn creates_disk_image_stub() {
    let tmp = tempdir().expect("tmp");
    let src = tmp.path().join("src");
    fs::create_dir(&src).unwrap();
    File::create(src.join("test.txt")).unwrap();
    let dmg = tmp.path().join("test.dmg");
    create_disk_image(&src, &dmg).expect("create dmg");
    assert!(dmg.exists());
    assert!(dmg.is_dir());
    assert!(dmg.join("test.txt").exists());
}
