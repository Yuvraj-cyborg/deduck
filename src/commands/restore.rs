use crate::quarantine;
use std::path::Path;
use std::io;

pub fn run_restore(dir: &Path) -> io::Result<()> {
    let quarantine_dir = dir.join(".deduck_quarantine");
    match quarantine::restore_quarantined(&quarantine_dir, dir) {
        Ok(_) => {
            println!("✅ Quarantined files restored.");
            Ok(())
        }
        Err(e) => Err(e),
    }
}
