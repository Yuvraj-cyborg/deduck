use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn scan_directory(path: &Path) -> Result<Vec<PathBuf>, walkdir::Error> {
    let mut path_vec: Vec<PathBuf> = Vec::new();
        for entry in WalkDir::new(path) {
            let entry = entry?;
             let entry_path = entry.path();

         if entry_path.is_file() {
            path_vec.push(entry_path.to_path_buf());
        }
    }
    Ok(path_vec)
}
