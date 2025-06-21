use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use indicatif::{ProgressBar, ProgressStyle};
use deduck::hasher::{hash_files, HashAlgorithm};

fn create_temp_file(content: &str, filename: &str) -> PathBuf {
    let path = std::env::temp_dir().join(filename);
    let mut file = File::create(&path).expect("Failed to create temp file");
    file.write_all(content.as_bytes()).expect("Failed to write to temp file");
    path
}

#[test]
fn test_hash_files_group_duplicates_with_progress() {
    let file1 = create_temp_file("hello world", "deduck_test_file1.txt");
    let file2 = create_temp_file("hello world", "deduck_test_file2.txt"); 
    let file3 = create_temp_file("different content", "deduck_test_file3.txt");

    let files = vec![file1.clone(), file2.clone(), file3.clone()];
    let pb = ProgressBar::new(files.len() as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("=> "),
    );

    let hash_map = hash_files(files, HashAlgorithm::Sha256, pb.clone());
    pb.finish_with_message("✅ Finished hashing");

    let duplicate_group = hash_map.values()
        .find(|group| group.contains(&file1) && group.contains(&file2))
        .expect("No group found for duplicates");

    assert_eq!(duplicate_group.len(), 2);
    assert!(!duplicate_group.contains(&file3));

    for file in [&file1, &file2, &file3] {
        if let Err(e) = fs::remove_file(file) {
            eprintln!("Failed to delete temp file {}: {}", file.display(), e);
        }
    }
}
