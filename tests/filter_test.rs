use deduck::filters::batch;
use std::io::Write;
use std::path::{PathBuf};
use std::fs::{File};

fn create_dummy_file(path: &PathBuf, size: usize) {
    let mut file = File::create(path).unwrap();
    file.write_all(&vec![0u8; size]).unwrap();
}

#[test]
fn test_batch() {
    let temp = tempfile::tempdir().unwrap();
    let dir_path = temp.path();

    let file1 = dir_path.join("report1.pdf");
    create_dummy_file(&file1, 2_000_000);

    let file2 = dir_path.join("report2.pdf");
    create_dummy_file(&file2, 2_000_000);

    let file3 = dir_path.join("image.png");
    create_dummy_file(&file3, 1_000_000);

    let file4 = dir_path.join("ignore.txt");
    create_dummy_file(&file4, 1_000_000);

    let all_files = vec![file1, file2, file3, file4];
    let allowed_exts = ["pdf", "png"];

    let result = batch(all_files, &allowed_exts).unwrap();

    assert_eq!(result.len(), 2);

    let pdf_group = result.get(&("pdf".to_string(), 2_000_000)).unwrap();
    assert_eq!(pdf_group.len(), 2);

    let png_group = result.get(&("png".to_string(), 1_000_000)).unwrap();
    assert_eq!(png_group.len(), 1);

    assert!(result.get(&("txt".to_string(), 1_000_000)).is_none());
}
