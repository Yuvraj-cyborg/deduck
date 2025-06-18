mod scanner;
mod filters;
use clap::{command, Parser,Subcommand};
use std::path::PathBuf;

#[derive(Parser,Debug)]
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
        dir: PathBuf,
    },
    Filter {
        #[arg(short, long)]
        dir: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan { dir } => {
            match scanner::scan_directory(dir) {
                Ok(files) => {
                    if files.is_empty() {
                        println!("No files found in the specified directory.");
                    } else {
                        println!("Found {} files in the directory:", files.len());
                        for file in files {
                            println!("{}", file.display());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error scanning directory: {}", e);
                }
            }
        }

        Commands::Filter { dir } => {
            match scanner::scan_directory(dir) {
                Ok(files) => {
                    let allowed_exts = ["pdf", "png","txt","doc"]; 
                 match filters::batch(files, &allowed_exts) {
                     Ok(batches) => {
                      for ((ext, size), group) in batches {
                        if group.len() > 1 {
                            println!("Group: .{} | {} bytes", ext, size);
                            for file in group {
                                println!("  {}", file.display());
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error during batching: {}", e);
                }
              }
            }
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
            }
        }
    }
  }
}