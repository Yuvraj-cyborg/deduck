mod commands;
mod config;
mod prompts;
mod scanner;
mod filters;
mod hasher;
mod quarantine;
mod report;
mod duplicates;

use clap::{Parser, Subcommand};
use std::path::{PathBuf};
use std::process::exit;

use commands::{clean, filter, purge, restore, scan};
use config::get_dir_or_saved;

#[derive(Parser, Debug)]
#[command(name = "deduck", version = "0.1.0", author = "Yuvraj Biswal")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan { #[arg(short, long)] dir: Option<PathBuf> },
    Filter { #[arg(short, long)] dir: Option<PathBuf> },
    Clean { #[arg(short, long)] dir: Option<PathBuf> },
    Restore { #[arg(short, long)] dir: Option<PathBuf> },
    Purge { #[arg(short, long)] dir: Option<PathBuf> },
}

fn main() {
    let cli = Cli::parse();

    let dir_opt = match &cli.command {
        Commands::Scan { dir } => dir,
        Commands::Filter { dir } => dir,
        Commands::Clean { dir } => dir,
        Commands::Restore { dir } => dir,
        Commands::Purge { dir } => dir,
    };

    let dir = get_dir_or_saved(dir_opt);

    let result = match &cli.command {
        Commands::Scan { .. } => scan::run_scan(dir.as_path()),
        Commands::Filter { .. } => filter::run_filter(dir.as_path()),
        Commands::Clean { .. } => clean::run_clean(dir.as_path()),
        Commands::Restore { .. } => restore::run_restore(dir.as_path()),
        Commands::Purge { .. } => purge::run_purge(dir.as_path()),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        exit(1);
    }
}
