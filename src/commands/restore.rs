use crate::quarantine;
use std::path::Path;
use std::io;

pub fn run_restore(dir: &Path) -> io::Result<()> {
    let quarantine_dir = quarantine::get_quarantine_dir(dir);
    match quarantine::restore_quarantined(&quarantine_dir, dir) {
        Ok(_) => {
            println!("✅ Quarantined files restored.");
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to restore quarantined files: {}", e);
            Err(e)
        },
    }
}
