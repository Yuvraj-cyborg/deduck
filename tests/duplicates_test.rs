use deduck::duplicates::duplicates;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

fn create_dummy_file(dir: &std::path::Path, name: &str, content: &[u8]) {
    let file_path = dir.join(name);
    let mut file = File::create(file_path).unwrap();
    file.write_all(content).unwrap();
}

#[test]
fn test_find_and_process_duplicates_with_quarantine() {
    // 1. Setup
    let temp_dir = tempdir().unwrap();
    let search_dir = temp_dir.path();

    create_dummy_file(search_dir, "file1.txt", b"unique content");
    create_dummy_file(search_dir, "file2.txt", b"duplicate content");
    create_dummy_file(search_dir, "file3.txt", b"duplicate content");
    create_dummy_file(search_dir, "file4.png", b"more duplicate content");
    create_dummy_file(search_dir, "file5.png", b"more duplicate content");

    // 2. Run with quarantine enabled
    let result = duplicates(search_dir, 1, true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5); // 5 files were scanned

    // 3. Assert files were moved to quarantine
    let quarantine_dir = search_dir.join(".deduck_quarantine");
    assert!(quarantine_dir.exists());

    let quarantined_files: Vec<_> = fs::read_dir(&quarantine_dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();

    assert_eq!(quarantined_files.len(), 2);
    assert!(
        (quarantined_files.contains(&"file3.txt".to_string())
            && !search_dir.join("file3.txt").exists())
            || (quarantined_files.contains(&"file2.txt".to_string())
                && !search_dir.join("file2.txt").exists())
    );
    assert!(
        (quarantined_files.contains(&"file5.png".to_string())
            && !search_dir.join("file5.png").exists())
            || (quarantined_files.contains(&"file4.png".to_string())
                && !search_dir.join("file4.png").exists())
    );

    // 4. Assert one of each duplicate pair remains
    assert!(
        search_dir.join("file2.txt").exists() || search_dir.join("file3.txt").exists()
    );
    assert!(
        search_dir.join("file4.png").exists() || search_dir.join("file5.png").exists()
    );
    assert!(search_dir.join("file1.txt").exists());
}

#[test]
fn test_find_and_process_duplicates_no_quarantine() {
    // 1. Setup
    let temp_dir = tempdir().unwrap();
    let search_dir = temp_dir.path();

    create_dummy_file(search_dir, "file1.txt", b"unique content");
    create_dummy_file(search_dir, "file2.txt", b"duplicate content");
    create_dummy_file(search_dir, "file3.txt", b"duplicate content");

    // 2. Run with quarantine disabled
    let result = duplicates(search_dir, 1, false);
    assert!(result.is_ok());

    // 3. Assert no quarantine directory was created
    let quarantine_dir = search_dir.join(".deduck_quarantine");
    assert!(!quarantine_dir.exists());

    // 4. Assert original files still exist
    assert!(search_dir.join("file1.txt").exists());
    assert!(search_dir.join("file2.txt").exists());
    assert!(search_dir.join("file3.txt").exists());
} 