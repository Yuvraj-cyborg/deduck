use dialoguer::Select;
use std::io;

pub fn prompt_scan_mode() -> io::Result<usize> {
    let scan_modes = &["Quick Scan", "Normal Scan", "Deep Scan"];
    Select::new()
        .with_prompt("Select scan mode")
        .default(1)
        .items(scan_modes)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

pub fn prompt_clean_choice() -> io::Result<usize> {
    let clean_modes = &[
        "Separate",
        "Separate + Clean",
    ];
    Select::new()
        .with_prompt("Choose cleaning action")
        .default(0)
        .items(clean_modes)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}
