use std::error::Error;
use std::path::Path;

#[cfg(not(target_os = "macos"))]
use fs_extra::dir::{copy, CopyOptions};
#[cfg(target_os = "macos")]
use tokio::process::Command;

/// Create a bootable installer by invoking `createinstallmedia` or a fallback.
#[allow(dead_code)]
pub async fn create_bootable_installer(
    createinstallmedia: &Path,
    volume: &Path,
) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "macos")]
    {
        let status = Command::new(createinstallmedia)
            .arg("--volume")
            .arg(volume)
            .status()
            .await?;
        if status.success() {
            Ok(())
        } else {
            Err("createinstallmedia failed".into())
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        copy(createinstallmedia, volume, &options)?;
        Ok(())
    }
}
