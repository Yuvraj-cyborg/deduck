use std::path::PathBuf;

#[derive(Default)]
pub struct Report {
    pub files_found: usize,
    pub files_deleted: usize,
    pub space_freed: u64, 
    pub deleted_files: Vec<PathBuf>,
}

impl Report {
    pub fn new() -> Self {
        Report::default()
    }

    pub fn add_file(&mut self, path: PathBuf, size: u64) {
        self.files_deleted += 1;
        self.space_freed += size;
        self.deleted_files.push(path);
    }

    pub fn set_files_found(&mut self, count: usize) {
        self.files_found = count;
    }

    pub fn display(&self) {
        println!("\n📊 Cleanup Report:");
        println!("  Files found     : {}", self.files_found);
        println!("  Files deleted   : {}", self.files_deleted);
        println!("  Space cleaned   : {:.2} MB", self.space_freed as f64 / (1024.0 * 1024.0));

        if !self.deleted_files.is_empty() {
            println!("  Deleted files:");
            for file in &self.deleted_files {
                println!("    {}", file.display());
            }
        }
    }
}
