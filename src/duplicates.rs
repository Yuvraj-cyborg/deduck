use std::path::PathBuf;
use std::path::Path;

use crate::{scanner, filters, hasher::{hash_files, HashAlgorithm}, quarantine::quarantine_duplicates};

pub fn handle_duplicates(dir: &Path, scan_choice: usize, quarantine: bool) {
    match scanner::scan_directory(dir) {
        Ok(files) => {
            if files.is_empty() {
                println!("❌ No files found in the directory.");
                return;
            }

            let allowed_exts = ["pdf", "png", "txt", "doc"];
            match filters::batch(files, &allowed_exts) {
                Ok(batches) => {
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

                                        if quarantine {
                                            let to_quarantine: Vec<PathBuf> = paths.iter().skip(1).cloned().collect();
                                            let quarantine_dir = dir.join(".deduck_quarantine");
                                            if let Err(e) = quarantine_duplicates(to_quarantine, &quarantine_dir) {
                                                eprintln!("Failed to quarantine: {}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Batching error: {}", e),
            }
        }
        Err(e) => eprintln!("Scan error: {}", e),
    }
}
