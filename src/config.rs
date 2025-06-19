use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use dirs;

const CONFIG_FILE_NAME: &str = "last_dir.txt";
const SCAN_MODE_FILE_NAME: &str = "scan_mode.txt";

fn get_config_path(file: &str) -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".deduck").join(file))
}

pub fn save_last_dir(dir: &Path) -> io::Result<()> {
    if let Some(path) = get_config_path(CONFIG_FILE_NAME) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        writeln!(file, "{}", dir.display())?;
    }
    Ok(())
}

pub fn load_last_dir() -> Option<PathBuf> {
    if let Some(path) = get_config_path(CONFIG_FILE_NAME) {
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
    if let Some(path) = get_config_path(SCAN_MODE_FILE_NAME) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        writeln!(file, "{}", mode)?;
    }
    Ok(())
}

pub fn load_scan_mode() -> Option<usize> {
    if let Some(path) = get_config_path(SCAN_MODE_FILE_NAME) {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(mode) = content.trim().parse::<usize>() {
                return Some(mode);
            }
        }
    }
    None
}
