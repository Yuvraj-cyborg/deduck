use crate::prompts;
use crate::duplicates;
use crate::report::Report;
use crate::config::load_scan_mode;

use std::fs;
use std::io;
use std::path::{Path};

pub fn run_clean(dir: &Path) -> io::Result<()> {
    let scan_choice = match load_scan_mode() {
        Some(mode) => mode,
        None => {
            eprintln!("❌ No scan mode found. Please run `deduck filter` first.");
            return Err(io::Error::new(io::ErrorKind::Other, "No scan mode saved"));
        }
    };

    let clean_choice = prompts::prompt_clean_choice()?;
    let quarantine_dir = dir.join(".deduck_quarantine");

    let mut report = Report::new();

    let files_found = duplicates::find_and_process_duplicates(dir,scan_choice,true)?;

    report.set_files_found(files_found);
    process_quarantined_files(&quarantine_dir, &mut report)?;

    if clean_choice == 1 {
        delete_quarantine_dir(&quarantine_dir)?;
        report.display();
    }

    Ok(())
}

fn process_quarantined_files(quarantine_dir: &Path,report: &mut Report,) -> io::Result<()> {
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

fn delete_quarantine_dir(path: &Path) -> io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
        println!("🗑️ Quarantine folder deleted: {}", path.display());
    }
    Ok(())
}
