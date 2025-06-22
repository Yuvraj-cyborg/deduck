use crate::prompts;
use crate::duplicates;
use crate::report::Report;
use crate::config::load_scan_mode;
use std::io;
use std::path::Path;
use crate::utils::{delete_quarantine_dir, process_quarantined_files};

pub fn run_clean(dir: &Path) -> io::Result<()> {
    let scan_choice = match load_scan_mode() {
        Some(mode) => mode,
        None => {
            eprintln!("❌ No scan mode found. Please run `deduck filter` first.");
            return Err(io::Error::new(io::ErrorKind::Other, "No scan mode saved"));
        }
    };

    let clean_choice = prompts::prompt_clean_choice()?;
    let quarantine_dir = crate::quarantine::get_quarantine_dir(dir);

    let mut report = Report::new();

    let files_found = duplicates::duplicates(dir,scan_choice,true)?;

    report.set_files_found(files_found);
    process_quarantined_files(&quarantine_dir, &mut report)?;

    if clean_choice == 1 {
        delete_quarantine_dir(&quarantine_dir)?;
        report.display();
    }

    Ok(())
}
