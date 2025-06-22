use crate::report::Report;
use std::fs;
use std::io;
use std::path::Path;

pub fn process_quarantined_files(quarantine_dir: &Path,report: &mut Report) -> io::Result<()> {
    if !quarantine_dir.exists() {
        return Ok(());
    }

    let entries = fs::read_dir(quarantine_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Ok(metadata) = fs::metadata(&path) {
            report.add_file(path, metadata.len());
        }
    }

    Ok(())
}

pub fn delete_quarantine_dir(path: &Path) -> io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
        println!("🗑️ Quarantine folder deleted: {}", path.display());
    }
    Ok(())
} 