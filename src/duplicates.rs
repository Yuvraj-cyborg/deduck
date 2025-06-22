use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{scanner,filters,hasher::{hash_files, HashAlgorithm},quarantine};

pub fn duplicates(dir: &Path,scan_choice: usize,quarantine_flag: bool) -> io::Result<usize> {
    let files = scanner::scan_directory(dir)?;
    let files_found = files.len();

    if files.is_empty() {
        println!("❌ No files found in the directory.");
        return Ok(0);
    }

    let allowed_exts = ["pdf", "png", "txt", "doc", "xlsx", "jpeg", "jpg"];
    let batches = filters::batch(files, &allowed_exts)?;

    if batches.is_empty() {
        println!("⚠️ No files matching allowed extensions found.");
        return Ok(files_found);
    }

    let algo = match scan_choice {
        0 => HashAlgorithm::XxHash,
        1 => HashAlgorithm::Blake3,
        2 => HashAlgorithm::Sha256,
        _ => unreachable!(),
    };

    let all_files: Vec<PathBuf> = batches.values().flat_map(|v| v.clone()).collect();

    let pb = ProgressBar::new(all_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("=> "),
    );
    pb.set_message("🔍 Hashing all files...");

    let hash_map: HashMap<String, Vec<PathBuf>> = hash_files(all_files, algo, pb.clone());

    pb.finish_with_message("✅ Finished hashing all files");

    let mut duplicates_found = false;

    for (hash, paths) in &hash_map {
        if paths.len() > 1 {
            duplicates_found = true;
            println!("\n🔁 Duplicate Hash: {}", hash);
            for path in paths {
                println!("    {}", path.display());
            }

            if quarantine_flag {
                let to_quarantine: Vec<PathBuf> = paths.iter().skip(1).cloned().collect();
                if !to_quarantine.is_empty() {
                    let quarantine_dir = quarantine::get_quarantine_dir(dir);
                    if let Err(e) = quarantine::quarantine_duplicates(to_quarantine, &quarantine_dir) {
                        eprintln!("❌ Failed to quarantine: {}", e);
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
