use deduck::duplicates::duplicates;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn create_dummy_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
    let file_path = dir.join(name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content).unwrap();
    file_path
}

#[test]
fn test_find_and_process_duplicates_with_quarantine() {
    let temp_dir = tempdir().unwrap();
    let search_dir = temp_dir.path();

    create_dummy_file(search_dir, "file1.txt", b"duplicate content");
    create_dummy_file(search_dir, "file2.txt", b"duplicate content");
    create_dummy_file(search_dir, "file3.txt", b"unique content");
    create_dummy_file(search_dir, "file4.png", b"same image");
    create_dummy_file(search_dir, "file5.png", b"same image");

    // Run with quarantine enabled
    let result = duplicates(search_dir, 1, true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);

    let quarantine_dir = search_dir.join(".deduck_quarantine");
    assert!(quarantine_dir.exists());

    let quarantined: HashSet<_> = fs::read_dir(&quarantine_dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();

    let remaining: HashSet<_> = fs::read_dir(search_dir)
        .unwrap()
        .filter_map(|e| {
            let path = e.unwrap().path();
            if path.is_file() {
                Some(path.file_name().unwrap().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    let all_files: HashSet<_> = quarantined.union(&remaining).cloned().collect();
    let expected: HashSet<_> = [
        "file1.txt",
        "file2.txt",
        "file3.txt",
        "file4.png",
        "file5.png",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(all_files, expected);

    // Check exactly one duplicate from each group was quarantined
    let txt_dupes = ["file1.txt", "file2.txt"];
    let img_dupes = ["file4.png", "file5.png"];

    let quarantined_txt_dupes: Vec<_> = txt_dupes.iter().filter(|f| quarantined.contains(**f)).collect();
    let remaining_txt_dupes: Vec<_> = txt_dupes.iter().filter(|f| remaining.contains(**f)).collect();
    assert_eq!(quarantined_txt_dupes.len(), 1, "One .txt duplicate should be quarantined");
    assert_eq!(remaining_txt_dupes.len(), 1, "One .txt duplicate should remain");

    let quarantined_img_dupes: Vec<_> = img_dupes.iter().filter(|f| quarantined.contains(**f)).collect();
    let remaining_img_dupes: Vec<_> = img_dupes.iter().filter(|f| remaining.contains(**f)).collect();
    assert_eq!(quarantined_img_dupes.len(), 1, "One .png duplicate should be quarantined");
    assert_eq!(remaining_img_dupes.len(), 1, "One .png duplicate should remain");

    assert!(remaining.contains("file3.txt")); // unique
}

#[test]
fn test_find_and_process_duplicates_no_quarantine() {
    let temp_dir = tempdir().unwrap();
    let search_dir = temp_dir.path();

    create_dummy_file(search_dir, "file1.txt", b"duplicate content");
    create_dummy_file(search_dir, "file2.txt", b"duplicate content");

    let result = duplicates(search_dir, 1, false);
    assert!(result.is_ok());

    let quarantine_dir = search_dir.join(".deduck_quarantine");
    assert!(!quarantine_dir.exists());

    assert!(search_dir.join("file1.txt").exists());
    assert!(search_dir.join("file2.txt").exists());
}
