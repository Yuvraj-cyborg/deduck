use std::path::PathBuf;
use std::fs::{self};
use std::collections::HashMap;

pub fn batch(files: Vec<PathBuf>, allowed_exts: &[&str]) -> std::io::Result<HashMap<(String, u64), Vec<PathBuf>>> {
    let mut batches: HashMap<(String, u64), Vec<PathBuf>> = HashMap::new();

    for file in files {
        if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
            if allowed_exts.contains(&ext) {
                let metadata = fs::metadata(&file)?;
                let size = metadata.len();
                let key: (String, u64) = (ext.to_string(), size);
                batches.entry(key).or_default().push(file);
            }
        }
    }

    Ok(batches)
}