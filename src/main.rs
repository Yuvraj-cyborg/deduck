mod scanner;
mod filters;
mod hasher;
mod quarantine;
mod config;
mod report;

use clap::{command, Parser, Subcommand};
use dialoguer::Select;
use hasher::{hash_files, HashAlgorithm};
use quarantine::quarantine_duplicates;
use std::path::PathBuf;
use std::process::exit;
use config::{save_last_dir, load_last_dir, save_scan_mode, load_scan_mode};
use report::Report;

#[derive(Parser, Debug)]
#[command(name = "deduck", version = "0.1.0", author = "Yuvraj Biswal")]
#[command(
    about = "Scan for duplicate files",
    long_about = "Deduck recursively scans a directory and identifies duplicate files using their hashes.\nYou can use --dir <path> to specify the starting directory."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan {
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    Filter {
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    Clean {
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    Restore {
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    Purge {
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { dir } => {
            let dir = get_dir_or_saved(dir);
            match scanner::scan_directory(&dir) {
                Ok(files) => {
                    if files.is_empty() {
                        println!("❌ No files found in the specified directory.");
                    } else {
                        println!("📂 Found {} files:", files.len());
                        for file in files {
                            println!("{}", file.display());
                        }
                    }
                }
                Err(e) => eprintln!("Error scanning directory: {}", e),
            }
        }

        Commands::Filter { dir } => {
            let dir = get_dir_or_saved(dir);

            let scan_choice = prompt_scan_mode();

            if let Err(e) = save_scan_mode(scan_choice) {
                eprintln!("Warning: failed to save scan mode: {}", e);
            }

            handle_duplicates(&dir, scan_choice, false);
        }

        Commands::Clean { dir } => {
            let dir = get_dir_or_saved(dir);

            let scan_choice = match load_scan_mode() {
                Some(mode) => mode,
                None => {
                    eprintln!("❌ No scan mode found. Please run `deduck filter` first.");
                    exit(1);
                }
            };

            let clean_modes = &[
                "Separate (move duplicates to quarantine)",
                "Separate + Clean (move and delete quarantine folder)",
            ];
            let clean_choice = Select::new()
                .with_prompt("Choose cleaning action")
                .default(0)
                .items(clean_modes)
                .interact()
                .unwrap();

            let quarantine_dir = dir.join(".deduck_quarantine");
            let mut report = Report::new();

            match scanner::scan_directory(&dir) {
                Ok(files) => {
                    report.set_files_found(files.len());

                    let allowed_exts = ["pdf", "png", "txt", "doc"];
                    match filters::batch(files, &allowed_exts) {
                        Ok(batches) => {
                            let algo = match scan_choice {
                                0 => HashAlgorithm::XxHash,
                                1 => HashAlgorithm::Blake3,
                                2 => HashAlgorithm::Sha256,
                                _ => unreachable!(),
                            };

                            for ((ext, size), group) in batches {
                                if group.len() > 1 {
                                    let hash_map = hash_files(group.clone(), algo.clone());
                                    let has_duplicates =
                                        hash_map.values().any(|paths| paths.len() > 1);

                                    if has_duplicates {
                                        println!("\nBatch: .{} | {} bytes", ext, size);

                                        for (hash, paths) in hash_map {
                                            if paths.len() > 1 {
                                                println!("  Hash: {}", hash);
                                                for path in &paths {
                                                    println!("    {}", path.display());
                                                }

                                                let duplicates: Vec<PathBuf> =
                                                    paths.iter().skip(1).cloned().collect();

                                                for dup in &duplicates {
                                                    if let Ok(meta) = std::fs::metadata(dup) {
                                                        report.add_file(dup.clone(), meta.len());
                                                    }
                                                }

                                                if let Err(e) = quarantine_duplicates(
                                                    duplicates,
                                                    &quarantine_dir,
                                                ) {
                                                    eprintln!(
                                                        "Failed to quarantine duplicates: {}",
                                                        e
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if clean_choice == 1 {
                                if quarantine_dir.exists() {
                                    if let Err(e) = std::fs::remove_dir_all(&quarantine_dir) {
                                        eprintln!(
                                            "Failed to delete quarantine folder: {}",
                                            e
                                        );
                                    } else {
                                        println!(
                                            "🗑️ Quarantine folder deleted: {}",
                                            quarantine_dir.display()
                                        );
                                    }
                                }

                                report.display(); // 👈 show report only after deletion
                            }
                        }
                        Err(e) => eprintln!("Batching error: {}", e),
                    }
                }
                Err(e) => eprintln!("Scan error: {}", e),
            }
        }

        Commands::Restore { dir } => {
            let dir = get_dir_or_saved(dir);
            let quarantine_dir = dir.join(".deduck_quarantine");
            match quarantine::restore_quarantined(&quarantine_dir, &dir) {
                Ok(_) => println!("✅ Quarantined files restored."),
                Err(e) => eprintln!("❌ Failed to restore: {}", e),
            }
        }

        Commands::Purge { dir } => {
    let dir = get_dir_or_saved(dir);
    let quarantine_dir = dir.join(".deduck_quarantine");

    let mut report = Report::new();
    if let Ok(entries) = std::fs::read_dir(&quarantine_dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                report.add_file(entry.path(), meta.len());
            }
        }
    }

    match quarantine::purge_quarantine(&quarantine_dir) {
        Ok(_) => {
            println!("🗑️ Quarantine folder deleted.");
            report.display(); // 👈 show report after purge
        }
        Err(e) => eprintln!("❌ Failed to delete quarantine folder: {}", e),
    }
}

    }
}

fn get_dir_or_saved(dir_opt: &Option<PathBuf>) -> PathBuf {
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
                eprintln!(
                    "Error: No directory specified and no saved directory found. Please specify --dir."
                );
                exit(1);
            }
        },
    }
}

fn prompt_scan_mode() -> usize {
    let scan_modes = &["Quick Scan", "Normal Scan", "Deep Scan"];
    Select::new()
        .with_prompt("Select scan mode")
        .default(1)
        .items(scan_modes)
        .interact()
        .unwrap()
}

fn handle_duplicates(dir: &PathBuf, scan_choice: usize, quarantine: bool) {
    match scanner::scan_directory(dir) {
        Ok(files) => {
            if files.is_empty() {
                println!("❌ No files found in the directory.");
                return;
            }

            let allowed_exts = ["pdf", "png", "txt", "doc"];
            match filters::batch(files, &allowed_exts) {
                Ok(batches) => {
                    let algo = match scan_choice {
                        0 => HashAlgorithm::XxHash,
                        1 => HashAlgorithm::Blake3,
                        2 => HashAlgorithm::Sha256,
                        _ => unreachable!(),
                    };

                    for ((ext, size), group) in batches {
                        if group.len() > 1 {
                            let hash_map = hash_files(group.clone(), algo.clone());
                            let has_duplicates =
                                hash_map.values().any(|paths| paths.len() > 1);

                            if has_duplicates {
                                println!("\nBatch: .{} | {} bytes", ext, size);

                                for (hash, paths) in hash_map {
                                    if paths.len() > 1 {
                                        println!("  Hash: {}", hash);
                                        for path in &paths {
                                            println!("    {}", path.display());
                                        }

                                        if quarantine {
                                            let to_quarantine: Vec<PathBuf> =
                                                paths.iter().skip(1).cloned().collect();
                                            let quarantine_dir =
                                                dir.join(".deduck_quarantine");
                                            if let Err(e) = quarantine_duplicates(
                                                to_quarantine,
                                                &quarantine_dir,
                                            ) {
                                                eprintln!(
                                                    "Failed to quarantine: {}",
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Batching error: {}", e),
            }
        }
        Err(e) => eprintln!("Scan error: {}", e),
    }
}
