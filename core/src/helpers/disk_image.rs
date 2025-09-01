use std::error::Error;
use std::path::Path;

/// Trait for disk image operations.
pub trait DiskImageProvider {
    fn create(&self, src: &Path, dmg: &Path) -> Result<(), Box<dyn Error>>;
}

#[cfg(target_os = "macos")]
pub struct HdiutilDiskImageProvider;

#[cfg(target_os = "macos")]
impl DiskImageProvider for HdiutilDiskImageProvider {
    fn create(&self, src: &Path, dmg: &Path) -> Result<(), Box<dyn Error>> {
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
}

#[cfg(not(target_os = "macos"))]
pub struct CopyDiskImageProvider;

#[cfg(not(target_os = "macos"))]
impl DiskImageProvider for CopyDiskImageProvider {
    fn create(&self, src: &Path, dmg: &Path) -> Result<(), Box<dyn Error>> {
        use fs_extra::dir::{copy, CopyOptions};
        eprintln!("Disk images are not supported on this platform; copying directory instead");
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        copy(src, dmg, &options)?;
        Ok(())
    }
}

/// Returns the default disk image provider for the current platform.
pub fn default_provider() -> impl DiskImageProvider {
    #[cfg(target_os = "macos")]
    {
        HdiutilDiskImageProvider
    }
    #[cfg(not(target_os = "macos"))]
    {
        CopyDiskImageProvider
    }
}

/// Convenience function to create a disk image using the default provider.
pub fn create_disk_image(src: &Path, dmg: &Path) -> Result<(), Box<dyn Error>> {
    let provider = default_provider();
    provider.create(src, dmg)
}
