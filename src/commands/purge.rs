use crate::utils::{delete_quarantine_dir, process_quarantined_files};
use crate::report::Report;
use std::path::Path;
use std::io;

pub fn run_purge(dir: &Path) -> io::Result<()> {
    let quarantine_dir = crate::quarantine::get_quarantine_dir(dir);

    let mut report = Report::new();
    process_quarantined_files(&quarantine_dir, &mut report)?;

    delete_quarantine_dir(&quarantine_dir)?;
    report.display();

    Ok(())
}
