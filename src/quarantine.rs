use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn get_quarantine_dir(base_dir: &Path) -> PathBuf {
    base_dir.join(".deduck_quarantine")
}

pub fn quarantine_duplicates(files: Vec<PathBuf>, quarantine_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(quarantine_dir)?;

    for file in files {
        if let Some(filename) = file.file_name() {
            let dest = quarantine_dir.join(filename);
            fs::rename(&file, dest)?;
        }
    }

    Ok(())
}
pub fn restore_quarantined(quarantine_dir: &Path, target_dir: &Path) -> io::Result<()> {
    if !quarantine_dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "No quarantine directory found."));
    }

    for entry in fs::read_dir(quarantine_dir)? {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap();
        let dest = target_dir.join(file_name);
        fs::rename(file_path, dest)?;
    }

    fs::remove_dir_all(quarantine_dir)?;
    Ok(())
}