use std::path::{PathBuf, Path};
use std::io;

use crate::{scanner, filters, hasher::{hash_files, HashAlgorithm}, quarantine};

pub fn find_and_process_duplicates<F>(dir: &Path, scan_choice: usize, mut processor: F) -> io::Result<usize>
where
    F: FnMut(&Vec<PathBuf>),
{
    let files = scanner::scan_directory(dir)?;
    let files_found = files.len();
    if files.is_empty() {
        println!("❌ No files found in the directory.");
        return Ok(files_found);
    }

    let allowed_exts = ["pdf", "png", "txt", "doc"];
    let batches = filters::batch(files, &allowed_exts)?;

    if batches.is_empty() {
        println!("No files matching allowed extensions found.");
        return Ok(files_found);
    }

    let algo = match scan_choice {
        0 => HashAlgorithm::XxHash,
        1 => HashAlgorithm::Blake3,
        2 => HashAlgorithm::Sha256,
        _ => unreachable!(),
    };

    let mut duplicates_found = false;

    for ((ext, size), group) in batches {
        if group.len() > 1 {
            let hash_map = hash_files(group, algo.clone());
            let has_duplicates_in_batch = hash_map.values().any(|paths| paths.len() > 1);

            if has_duplicates_in_batch {
                duplicates_found = true;
                println!("\nBatch: .{} | {} bytes", ext, size);

                for (hash, paths) in hash_map {
                    if paths.len() > 1 {
                        println!("  Hash: {}", hash);
                        for path in &paths {
                            println!("    {}", path.display());
                        }
                        processor(&paths);
                    }
                }
            }
        }
    }

    if !duplicates_found {
        println!("✅ No duplicate files found.");
    }

    Ok(files_found)
}

pub fn handle_duplicates(dir: &Path, scan_choice: usize, quarantine: bool) {
    let processor = |paths: &Vec<PathBuf>| {
        if quarantine {
            let to_quarantine: Vec<PathBuf> = paths.iter().skip(1).cloned().collect();
            if !to_quarantine.is_empty() {
                let quarantine_dir = dir.join(".deduck_quarantine");
                if let Err(e) = quarantine::quarantine_duplicates(to_quarantine, &quarantine_dir) {
                    eprintln!("Failed to quarantine: {}", e);
                }
            }
        }
    };

    if let Err(e) = find_and_process_duplicates(dir, scan_choice, processor) {
        eprintln!("An error occurred: {}", e);
    }
}
