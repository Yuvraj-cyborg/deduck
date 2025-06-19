use crate::quarantine;
use crate::report::Report;
use std::path::Path;
use std::io;

pub fn run_purge(dir: &Path) -> io::Result<()> {
    let quarantine_dir = dir.join(".deduck_quarantine");

    let mut report = Report::new();
    if let Ok(entries) = std::fs::read_dir(&quarantine_dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                report.add_file(entry.path(), meta.len());
            }
        }
    }

    match quarantine::purge_quarantine(&quarantine_dir) {
        Ok(_) => {
            println!("🗑️ Quarantine folder deleted.");
            report.display();
            Ok(())
        }
        Err(e) => Err(e),
    }
}
