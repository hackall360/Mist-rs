use std::error::Error;
use std::path::Path;

#[cfg(not(target_os = "macos"))]
use fs_extra::file::{copy, CopyOptions};
#[cfg(target_os = "macos")]
use tokio::process::Command;

/// Convert a disk image at `src` into an ISO at `dest`.
#[allow(dead_code)]
pub async fn create_iso(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "macos")]
    {
        let status = Command::new("hdiutil")
            .arg("convert")
            .arg(src)
            .arg("-format")
            .arg("UDTO")
            .arg("-o")
            .arg(dest)
            .status()
            .await?;
        if status.success() {
            Ok(())
        } else {
            Err("failed to create ISO".into())
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        copy(src, dest, &CopyOptions::new())?;
        Ok(())
    }
}
