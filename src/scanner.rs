use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_directory(dir: &Path) -> io::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Directory does not exist"));
    }

    let files: Vec<PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_file)
        .map(|entry| entry.path().to_path_buf())
        .collect();

    Ok(files)
}

fn is_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
}
