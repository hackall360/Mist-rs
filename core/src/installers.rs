use std::path::Path;

use crate::helpers::download_manager::DownloadManager;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Installer {
    pub id: usize,
    pub name: String,
    pub url: String,
}

/// Return a list of available installers.
/// In the future this should query Apple's catalogs. Currently uses a static list for demonstration.
///
/// # Examples
/// ```
/// use mist_core::installers::list_installers;
/// let installers = list_installers();
/// assert!(installers.len() >= 1);
/// ```
pub fn list_installers() -> Vec<Installer> {
    vec![
        Installer {
            id: 1,
            name: "Rust Logo".to_string(),
            url: "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string(),
        },
        Installer {
            id: 2,
            name: "Cargo Favicon".to_string(),
            url: "https://doc.rust-lang.org/cargo/favicon.png".to_string(),
        },
    ]
}

/// Download the installer with the specified `id` to `dest`.
///
/// # Examples
/// ```no_run
/// use std::path::Path;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// mist_core::installers::download_installer(1, Path::new("installer.pkg")).await?;
/// # Ok(())
/// # }
/// ```
pub async fn download_installer(id: usize, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let installer = list_installers()
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| "installer not found".to_string())?;
    let manager = DownloadManager::new();
    manager.download(&installer.url, dest).await
}

/// Download the installer with retry options.
///
/// # Examples
/// ```no_run
/// use std::path::Path;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// mist_core::installers::download_installer_with_retry(1, Path::new("installer.pkg"), 3, 5).await?;
/// # Ok(())
/// # }
/// ```
pub async fn download_installer_with_retry(
    id: usize,
    dest: &Path,
    retries: usize,
    delay: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let installer = list_installers()
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| "installer not found".to_string())?;
    let manager = DownloadManager::new();
    manager
        .download_with_retry(&installer.url, dest, retries, delay)
        .await
}
