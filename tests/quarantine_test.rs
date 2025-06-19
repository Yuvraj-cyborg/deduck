use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

use deduck::quarantine::{quarantine_duplicates, restore_quarantined, purge_quarantine};

#[test]
fn test_quarantine_and_restore() {
    let temp_dir = tempdir().unwrap();
    let orig_dir = temp_dir.path().join("original");
    let quarantine_dir = temp_dir.path().join("quarantine");
    let restore_dir = temp_dir.path().join("restore");

    fs::create_dir_all(&orig_dir).unwrap();
    fs::create_dir_all(&restore_dir).unwrap();

    let file1_path = orig_dir.join("file1.txt");
    let file2_path = orig_dir.join("file2.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    let mut file2 = File::create(&file2_path).unwrap();

    writeln!(file1, "hello world").unwrap();
    writeln!(file2, "hello rust").unwrap();

    // Quarantine the files
    quarantine_duplicates(vec![file1_path.clone(), file2_path.clone()], &quarantine_dir).unwrap();

    // Files should be moved from original dir to quarantine dir
    assert!(!file1_path.exists());
    assert!(!file2_path.exists());
    assert!(quarantine_dir.join("file1.txt").exists());
    assert!(quarantine_dir.join("file2.txt").exists());

    restore_quarantined(&quarantine_dir, &restore_dir).unwrap();

    assert!(!quarantine_dir.exists());

    assert!(restore_dir.join("file1.txt").exists());
    assert!(restore_dir.join("file2.txt").exists());
}

#[test]
fn test_purge_quarantine() {
    let temp_dir = tempdir().unwrap();
    let quarantine_dir = temp_dir.path().join("quarantine");

    fs::create_dir_all(&quarantine_dir).unwrap();

    let dummy_file = quarantine_dir.join("dummy.txt");
    let mut file = File::create(&dummy_file).unwrap();
    writeln!(file, "dummy content").unwrap();

    assert!(dummy_file.exists());

    purge_quarantine(&quarantine_dir).unwrap();

    assert!(!quarantine_dir.exists());
}
