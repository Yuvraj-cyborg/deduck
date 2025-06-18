use deduck::scanner::scan_directory;
use std::fs::File;
use tempfile::tempdir;

#[test]
fn test_scan_directory_basic() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");

    File::create(&file1).unwrap();
    File::create(&file2).unwrap();

    let result = scan_directory(dir.path()).unwrap();

    let mut found = result.iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    found.sort();
    assert_eq!(found, vec!["file1.txt", "file2.txt"]);
}
