use fs_extra::file::{copy, move_file, remove, CopyOptions};
use std::error::Error;
use std::path::Path;

pub fn copy_file(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    copy(src, dest, &CopyOptions::new())?;
    Ok(())
}

pub fn move_file_path(src: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    move_file(src, dest, &CopyOptions::new())?;
    Ok(())
}

pub fn remove_file(path: &Path) -> Result<(), Box<dyn Error>> {
    remove(path)?;
    Ok(())
}
