use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_directory(dir: &Path) -> io::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Directory does not exist"));
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_a_file) 
        {
        files.push(entry.path().to_path_buf());
        }

    Ok(files)
}

fn is_a_file(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
}