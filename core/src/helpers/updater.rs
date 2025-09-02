use std::error::Error;

#[cfg(target_os = "macos")]
use super::sparkle;

/// Check for application updates and download them if available.
///
/// On macOS the existing Sparkle helper is used while other
/// platforms rely on the `self_update` crate to fetch releases
/// from GitHub.
pub fn check_for_updates() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "macos")]
    {
        // Delegate to Sparkle which handles updating the GUI app.
        sparkle::check_for_updates()?;
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Use the cross‑platform self_update crate for CLI builds.
        self_update::backends::github::Update::configure()
            .repo_owner("ninxsoft")
            .repo_name("Mist-rs")
            .bin_name("mist")
            .show_download_progress(true)
            .build()?
            .update()?;
    }

    Ok(())
}
