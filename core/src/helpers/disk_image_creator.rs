use std::error::Error;
use std::path::Path;

#[cfg(target_os = "macos")]
pub fn create_disk_image(src: &Path, dmg: &Path) -> Result<(), Box<dyn Error>> {
    use std::process::Command;
    let status = Command::new("hdiutil")
        .arg("create")
        .arg("-srcfolder")
        .arg(src)
        .arg(dmg)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err("failed to create disk image".into())
    }
}

#[cfg(not(target_os = "macos"))]
pub fn create_disk_image(src: &Path, dmg: &Path) -> Result<(), Box<dyn Error>> {
    use fs_extra::dir::{copy, CopyOptions};
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    copy(src, dmg, &options)?;
    Ok(())
}
