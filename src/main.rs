mod commands;
mod config;
mod prompts;
mod scanner;
mod filters;
mod hasher;
mod quarantine;
mod report;
mod duplicates;
mod utils;
mod similar;

use clap::{Parser, Subcommand};
use std::path::{PathBuf};
use std::process::exit;

use commands::{clean, filter, purge, restore, scan};
use config::get_dir_or_saved;

#[derive(Parser, Debug)]
#[command(name = "deduck", version = "0.1.0", author = "Yuvraj Biswal")]
#[command(
    about = "Scan for duplicate files",
    long_about = "Deduck recursively scans a directory and identifies duplicate files using their hashes.\nYou can use --dir <path> to specify the starting directory."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(global = true, short, long)]
    dir: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan,
    Filter,
    Clean,
    Restore,
    Purge,
}

fn main() {
    let cli = Cli::parse();

    let dir = get_dir_or_saved(&cli.dir);

    let result = match &cli.command {
        Commands::Scan => scan::run_scan(dir.as_path()),
        Commands::Filter => filter::run_filter(dir.as_path()),
        Commands::Clean => clean::run_clean(dir.as_path()),
        Commands::Restore => restore::run_restore(dir.as_path()),
        Commands::Purge => purge::run_purge(dir.as_path()),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        exit(1);
    }
}
