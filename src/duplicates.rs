use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    scanner,
    filters,
    hasher::{hash_files, HashAlgorithm},
    quarantine,
    similar::similar_images,
};

pub fn duplicates(dir: &Path, scan_choice: usize, quarantine_flag: bool) -> io::Result<usize> {
    let files = scanner::scan_directory(dir)?;
    let files_found = files.len();

    if files.is_empty() {
        println!("❌ No files found in the directory.");
        return Ok(0);
    }

    let doc_exts = ["pdf", "txt", "doc", "xlsx"];
    let image_exts = ["png", "jpg", "jpeg"];

    let (doc_files, image_files, all_files): (Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>) = if scan_choice == 2 {
        let mut docs = Vec::new();
        let mut images = Vec::new();

        for file in &files {
            if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
                let ext = ext.to_lowercase();
                if doc_exts.contains(&ext.as_str()) {
                    docs.push(file.clone());
                } else if image_exts.contains(&ext.as_str()) {
                    images.push(file.clone());
                }
            }
        }

        let all = docs.iter().chain(images.iter()).cloned().collect();
        (docs, images, all)
    } else {
        let allowed_exts = ["pdf", "txt", "doc", "xlsx", "png", "jpeg", "jpg"];
        let batches = filters::batch(files, &allowed_exts)?;

        if batches.is_empty() {
            println!("⚠️ No files matching allowed extensions found.");
            return Ok(files_found);
        }

        let flat: Vec<PathBuf> = batches.values().flat_map(|v| v.clone()).collect();
        (flat.clone(), vec![], flat)
    };

    let algo = match scan_choice {
        0 => HashAlgorithm::XxHash,
        1 => HashAlgorithm::Blake3,
        2 => HashAlgorithm::Sha256, 
        _ => unreachable!(),
    };

    let pb = ProgressBar::new(
        if scan_choice == 2 {
            doc_files.len()
        } else {
            all_files.len()
        } as u64
    );

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("=> "),
    );
    pb.set_message("🔍 Hashing files...");

    let hash_map: HashMap<String, Vec<PathBuf>> = if scan_choice == 2 {
        hash_files(doc_files.clone(), HashAlgorithm::Sha256, pb.clone())
    } else {
        hash_files(all_files.clone(), algo, pb.clone())
    };

    pb.finish_with_message("✅ Finished hashing files");

    let mut duplicates_found = false;
    let mut similar_found = false;
    let mut to_quarantine: Vec<PathBuf> = Vec::new();

    if scan_choice == 2 {
        println!("🔍 Performing image similarity scan...");
        let similar_map = similar_images(image_files.clone(), 10);

        for (base, similars) in &similar_map {
            if !similars.is_empty() {
                similar_found = true;
                println!("\n🖼️ Visually similar images:");
                println!("   Base: {}", base.display());
                for path in similars {
                    println!("   ↳ {}", path.display());
                }

                let mut similar_group = vec![base.clone()];
                similar_group.extend(similars.clone());
                to_quarantine.extend(similar_group.into_iter().skip(1));
            }
        }
    }

    for (hash, paths) in &hash_map {
        if paths.len() > 1 {
            duplicates_found = true;
            println!("\n🔁 Duplicate Hash: {}", hash);
            for path in paths {
                println!("    {}", path.display());
            }

            to_quarantine.extend(paths.iter().skip(1).cloned());
        }
    }

    if quarantine_flag && !to_quarantine.is_empty() {
        let quarantine_dir = quarantine::get_quarantine_dir(dir);
        if let Err(e) = quarantine::quarantine_duplicates(to_quarantine, &quarantine_dir) {
            eprintln!("❌ Failed to quarantine files: {}", e);
        }
    }

    if !duplicates_found && !similar_found {
        println!("✅ No duplicate or similar files found.");
    }

    Ok(files_found)
}
