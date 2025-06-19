use crate::prompts;
use crate::scanner;
use crate::filters;
use crate::hasher::{hash_files, HashAlgorithm};
use crate::quarantine;
use crate::report::Report;
use crate::config::load_scan_mode;
use std::path::Path;
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

    let files = scanner::scan_directory(dir)?;

    report.set_files_found(files.len());

    let allowed_exts = ["pdf", "png", "txt", "doc"];
    let batches = filters::batch(files, &allowed_exts)?;

    let algo = match scan_choice {
        0 => HashAlgorithm::XxHash,
        1 => HashAlgorithm::Blake3,
        2 => HashAlgorithm::Sha256,
        _ => unreachable!(),
    };

    for ((ext, size), group) in batches {
        if group.len() > 1 {
            let hash_map = hash_files(group.clone(), algo.clone());
            let has_duplicates = hash_map.values().any(|paths| paths.len() > 1);

            if has_duplicates {
                println!("\nBatch: .{} | {} bytes", ext, size);

                for (hash, paths) in hash_map {
                    if paths.len() > 1 {
                        println!("  Hash: {}", hash);
                        for path in &paths {
                            println!("    {}", path.display());
                        }

                        let duplicates: Vec<_> = paths.iter().skip(1).cloned().collect();

                        for dup in &duplicates {
                            if let Ok(meta) = std::fs::metadata(dup) {
                                report.add_file(dup.clone(), meta.len());
                            }
                        }

                        quarantine::quarantine_duplicates(duplicates, &quarantine_dir)?;
                    }
                }
            }
        }
    }

    if clean_choice == 1 {
        if quarantine_dir.exists() {
            std::fs::remove_dir_all(&quarantine_dir)?;
            println!("🗑️ Quarantine folder deleted: {}", quarantine_dir.display());
        }
        report.display();
    }

    Ok(())
}
