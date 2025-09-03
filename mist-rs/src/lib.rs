use clap::ValueEnum;
use mist_core::{helpers::file_manager, installers};
use std::path::{Path, PathBuf};

/// Output formats for listing installers.
#[derive(Clone, Debug, ValueEnum)]
pub enum Format {
    /// Plain text output
    Text,
    /// JSON formatted output
    Json,
    /// YAML formatted output
    Yaml,
}

/// Export installers as a string in the given format, optionally filtered by name.
///
/// # Examples
/// ```
/// use mist_rs::{export_installers, Format};
/// let data = export_installers(None, Format::Json).unwrap();
/// assert!(data.contains("Rust Logo"));
/// ```
pub fn export_installers(
    filter: Option<&str>,
    format: Format,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut list = installers::list_installers();
    if let Some(f) = filter {
        let f = f.to_lowercase();
        list.retain(|i| i.name.to_lowercase().contains(&f));
    }
    Ok(match format {
        Format::Text => list
            .into_iter()
            .map(|i| format!("{}: {}", i.id, i.name))
            .collect::<Vec<_>>()
            .join("\n"),
        Format::Json => serde_json::to_string_pretty(&list)?,
        Format::Yaml => serde_yaml::to_string(&list)?,
    })
}

/// Download an installer by `id` with retry options.
///
/// # Examples
/// ```no_run
/// use std::path::Path;
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// mist_rs::download_firmware(1, Path::new("installer.pkg"), 3, 5).await?;
/// # Ok(())
/// # }
/// ```
pub async fn download_firmware(
    id: usize,
    dest: &Path,
    retries: usize,
    delay: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    installers::download_installer_with_retry(id, dest, retries, delay).await
}

/// Remove the cache directory.
///
/// # Examples
/// ```
/// use std::path::Path;
/// let dir = std::env::temp_dir().join("mist-cache-doc");
/// std::fs::create_dir_all(&dir).unwrap();
/// mist_rs::clear_cache(Some(&dir)).unwrap();
/// assert!(!dir.exists());
/// ```
pub fn clear_cache(path: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    let cache_path: PathBuf = match path {
        Some(p) => p.to_path_buf(),
        None => {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".cache/mist")
        }
    };
    file_manager::remove_dir_all(&cache_path)?;
    Ok(())
}
