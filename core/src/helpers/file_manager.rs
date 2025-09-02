use std::error::Error;
use std::fs;
use std::path::Path;

/// Copy a file from `src` to `dest`.
pub fn copy_file(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    fs::copy(src, dest)?;
    Ok(())
}

/// Move a file from `src` to `dest`.
pub fn move_file_path(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    fs::rename(src, dest)?;
    Ok(())
}

/// Remove a file at `path` if it exists.
pub fn remove_file(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Create a directory and all missing parents at `path`.
pub fn create_dir_all(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(path)?;
    Ok(())
}

/// Recursively remove a directory at `path` if it exists.
pub fn remove_dir_all(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}
