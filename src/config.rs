//Here config.rs handles saving file paths temporarily so after filter command is run, the last directory and scan mode are saved for future use.
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use dirs;

const CONFIG_DIR_NAME: &str = ".deduck";
const LAST_DIR_FILE: &str = "last_dir.txt";
const SCAN_MODE_FILE: &str = "scan_mode.txt";

fn config_dir() -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    Some(home_dir.join(CONFIG_DIR_NAME))
}

fn config_path(filename: &str) -> Option<PathBuf> {
    let config_dir = config_dir()?;
    Some(config_dir.join(filename))
}

pub fn save_last_dir(dir: &Path) -> io::Result<()> {
    let config_file = config_path(LAST_DIR_FILE);

    if let Some(path) = config_file {
        let parent_dir = path.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(parent_dir)?;

        let mut file = fs::File::create(path)?;
        writeln!(file, "{}", dir.display())?;
    }

    Ok(())
}

pub fn load_last_dir() -> Option<PathBuf> {
    let path = config_path(LAST_DIR_FILE)?;

    let content = fs::read_to_string(path).ok()?;
    let trimmed = content.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(PathBuf::from(trimmed))
    }
}

pub fn save_scan_mode(mode: usize) -> io::Result<()> {
    let path = config_path(SCAN_MODE_FILE);

    if let Some(path) = path {
        let parent = path.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(parent)?;

        let mut file = fs::File::create(path)?;
        writeln!(file, "{}", mode)?;
    }

    Ok(())
}

pub fn load_scan_mode() -> Option<usize> {
    let path = config_path(SCAN_MODE_FILE)?;
    let content = fs::read_to_string(path).ok()?;
    content.trim().parse::<usize>().ok()
}

pub fn get_dir_or_saved(dir_opt: &Option<PathBuf>) -> PathBuf {
    if let Some(dir) = dir_opt {
        if let Err(err) = save_last_dir(dir) {
            eprintln!("Warning: failed to save last directory: {}", err);
        }
        return dir.clone();
    }

    if let Some(saved_dir) = load_last_dir() {
        return saved_dir;
    }

    eprintln!("❌ Error: No directory specified and no saved directory found.\n➡️ Please specify --dir.");
    std::process::exit(1);
}
