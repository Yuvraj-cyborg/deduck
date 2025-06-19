use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use dirs;

const CONFIG_DIR_NAME: &str = ".deduck";
const LAST_DIR_FILE: &str = "last_dir.txt";
const SCAN_MODE_FILE: &str = "scan_mode.txt";

fn config_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(CONFIG_DIR_NAME))
}

fn config_path(filename: &str) -> Option<PathBuf> {
    config_dir().map(|dir| dir.join(filename))
}

pub fn save_last_dir(dir: &Path) -> io::Result<()> {
    if let Some(path) = config_path(LAST_DIR_FILE) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        writeln!(file, "{}", dir.display())?;
    }
    Ok(())
}

pub fn load_last_dir() -> Option<PathBuf> {
    if let Some(path) = config_path(LAST_DIR_FILE) {
        if let Ok(content) = fs::read_to_string(path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return Some(PathBuf::from(trimmed));
            }
        }
    }
    None
}

pub fn save_scan_mode(mode: usize) -> io::Result<()> {
    if let Some(path) = config_path(SCAN_MODE_FILE) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        writeln!(file, "{}", mode)?;
    }
    Ok(())
}

pub fn load_scan_mode() -> Option<usize> {
    if let Some(path) = config_path(SCAN_MODE_FILE) {
        if let Ok(content) = fs::read_to_string(path) {
            return content.trim().parse::<usize>().ok();
        }
    }
    None
}

pub fn get_dir_or_saved(dir_opt: &Option<PathBuf>) -> PathBuf {
    match dir_opt {
        Some(dir) => {
            if let Err(e) = save_last_dir(dir) {
                eprintln!("Warning: failed to save last directory: {}", e);
            }
            dir.clone()
        }
        None => match load_last_dir() {
            Some(saved_dir) => saved_dir,
            None => {
                eprintln!("Error: No directory specified and no saved directory found. Please specify --dir.");
                std::process::exit(1);
            }
        },
    }
}
