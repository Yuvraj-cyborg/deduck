use crate::scanner;
use std::io;
use std::path::Path;

pub fn run_scan(dir: &Path) -> io::Result<()> {
    match scanner::scan_directory(dir) {
        Ok(files) => {
            if files.is_empty() {
                println!("❌ No files found in the specified directory.");
            } else {
                println!("📂 Found {} files:", files.len());
                for file in files {
                    println!("{}", file.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
