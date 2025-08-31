use std::path::Path;

use crate::helpers::download_manager::DownloadManager;

#[derive(Clone, Debug)]
pub struct Installer {
    pub id: usize,
    pub name: String,
    pub url: String,
}

/// Return a list of available installers.
/// In the future this should query Apple's catalogs. Currently uses a static list for demonstration.
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
pub async fn download_installer(id: usize, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let installer = list_installers()
        .into_iter()
        .find(|i| i.id == id)
        .ok_or_else(|| "installer not found".to_string())?;
    let manager = DownloadManager::new();
    manager.download(&installer.url, dest).await
}
