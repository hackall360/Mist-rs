use std::error::Error;
use std::path::Path;

/// Trait defining launchd-related behavior.
pub trait LaunchdProvider {
    /// Load a launchd plist. This is only meaningful on macOS.
    fn load(&self, plist: &Path) -> Result<(), Box<dyn Error>>;
}

#[cfg(target_os = "macos")]
pub struct Launchctl;

#[cfg(target_os = "macos")]
impl LaunchdProvider for Launchctl {
    fn load(&self, plist: &Path) -> Result<(), Box<dyn Error>> {
        use std::process::Command;
        let status = Command::new("launchctl").arg("load").arg(plist).status()?;
        if status.success() {
            Ok(())
        } else {
            Err("failed to load launchd service".into())
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub struct NoopLaunchdProvider;

#[cfg(not(target_os = "macos"))]
impl LaunchdProvider for NoopLaunchdProvider {
    fn load(&self, _plist: &Path) -> Result<(), Box<dyn Error>> {
        eprintln!("launchd is not available on this platform");
        Ok(())
    }
}

/// Returns the default launchd provider for the current platform.
pub fn default_provider() -> impl LaunchdProvider {
    #[cfg(target_os = "macos")]
    {
        Launchctl
    }
    #[cfg(not(target_os = "macos"))]
    {
        NoopLaunchdProvider
    }
}
