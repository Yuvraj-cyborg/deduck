use crate::prompts;
use crate::duplicates::handle_duplicates;
use crate::config::save_scan_mode;
use std::path::Path;
use std::io;

pub fn run_filter(dir: &Path) -> io::Result<()> {
    let scan_choice = prompts::prompt_scan_mode()?;

    if let Err(e) = save_scan_mode(scan_choice) {
        eprintln!("Warning: failed to save scan mode: {}", e);
    }

    handle_duplicates(dir, scan_choice, false);

    Ok(())
}
