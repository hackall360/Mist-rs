use std::error::Error;

/// Trait for update checking functionality.
pub trait Updater {
    fn check_for_updates(&self) -> Result<(), Box<dyn Error>>;
}

#[cfg(target_os = "macos")]
pub struct SparkleUpdater;

#[cfg(target_os = "macos")]
impl Updater for SparkleUpdater {
    fn check_for_updates(&self) -> Result<(), Box<dyn Error>> {
        // Placeholder for Sparkle update logic on macOS
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
pub struct StubUpdater;

#[cfg(not(target_os = "macos"))]
impl Updater for StubUpdater {
    fn check_for_updates(&self) -> Result<(), Box<dyn Error>> {
        eprintln!("Sparkle updates are not available on this platform");
        Ok(())
    }
}

/// Returns the default updater for the current platform.
pub fn default_updater() -> impl Updater {
    #[cfg(target_os = "macos")]
    {
        SparkleUpdater
    }
    #[cfg(not(target_os = "macos"))]
    {
        StubUpdater
    }
}

/// Convenience function to invoke the updater using the default implementation.
pub fn check_for_updates() -> Result<(), Box<dyn Error>> {
    let updater = default_updater();
    updater.check_for_updates()
}
