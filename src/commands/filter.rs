use crate::config::save_scan_mode;
use crate::duplicates;
use crate::prompts;
use std::io;
use std::path::Path;

pub fn run_filter(dir: &Path) -> io::Result<()> {
    let scan_choice = prompts::prompt_scan_mode()?;

    if let Err(e) = save_scan_mode(scan_choice) {
        eprintln!("Warning: failed to save scan mode: {}", e);
    }

    if let Err(e) = duplicates::duplicates(dir, scan_choice, false) {
        eprintln!("❌ An error occurred during filtering: {}", e);
    }

    Ok(())
}
