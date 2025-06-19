use crate::prompts;
use crate::duplicates;
use crate::quarantine;
use crate::report::Report;
use crate::config::load_scan_mode;
use std::path::{Path, PathBuf};
use std::io;

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

    let processor = |paths: &Vec<PathBuf>| {
        let duplicates: Vec<_> = paths.iter().skip(1).cloned().collect();

        for dup in &duplicates {
            if let Ok(meta) = std::fs::metadata(dup) {
                report.add_file(dup.clone(), meta.len());
            }
        }

        if let Err(e) = quarantine::quarantine_duplicates(duplicates, &quarantine_dir) {
            eprintln!("Failed to quarantine: {}", e);
        }
    };

    let files_found = duplicates::find_and_process_duplicates(dir, scan_choice, processor)?;
    report.set_files_found(files_found);

    if clean_choice == 1 {
        if quarantine_dir.exists() {
            std::fs::remove_dir_all(&quarantine_dir)?;
            println!("🗑️ Quarantine folder deleted: {}", quarantine_dir.display());
        }
        report.display();
    }

    Ok(())
}
