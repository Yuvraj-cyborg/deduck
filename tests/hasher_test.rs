use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use deduck::hasher::{hash_files, HashAlgorithm};

fn create_temp_file(content: &str, filename: &str) -> PathBuf {
    let path = std::env::temp_dir().join(filename);
    let mut file = File::create(&path).expect("Failed to create temp file");
    file.write_all(content.as_bytes()).expect("Failed to write to temp file");
    path
}

#[test]
fn test_hash_files_group_duplicates() {
    let file1 = create_temp_file("hello world", "file1.txt");
    let file2 = create_temp_file("hello world", "file2.txt"); 
    let file3 = create_temp_file("different content", "file3.txt");

    let files = vec![file1.clone(), file2.clone(), file3.clone()];
    let hash_map = hash_files(files, HashAlgorithm::Sha256);

    let duplicate_group = hash_map.values()
        .find(|group| group.contains(&file1) && group.contains(&file2))
        .expect("No group found for duplicates");

    assert_eq!(duplicate_group.len(), 2);
    assert!(!duplicate_group.contains(&file3));

    let _ = fs::remove_file(file1);
    let _ = fs::remove_file(file2);
    let _ = fs::remove_file(file3);
}
