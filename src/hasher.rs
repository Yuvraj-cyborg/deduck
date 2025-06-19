use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::collections::HashMap;

use rayon::prelude::*;
use sha2::{Sha256, Digest};
use blake3;
use twox_hash::XxHash64;
use std::hash::Hasher;


#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    Sha256,
    Blake3,
    XxHash,
}

pub fn hash_file(path: &PathBuf, algo: &HashAlgorithm) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 8192];

    match algo {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 { break; }
                hasher.update(&buffer[..count]);
            }
            Ok(format!("{:x}", hasher.finalize()))
        }

        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 { break; }
                hasher.update(&buffer[..count]);
            }
            Ok(hasher.finalize().to_hex().to_string())
        }

        HashAlgorithm::XxHash => {
            let mut hasher = XxHash64::with_seed(0);
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 { break; }
                hasher.write(&buffer[..count]);
            }
            Ok(format!("{:x}", hasher.finish()))
        }
    }
}

pub fn hash_files(files: Vec<PathBuf>, algo: HashAlgorithm) -> HashMap<String, Vec<PathBuf>> {
    files
        .into_par_iter()
        .filter_map(|file| {
            match hash_file(&file, &algo) {
                Ok(hash) => Some((hash, file)),
                Err(_) => None,
            }
        })
        .fold(
            || HashMap::<String, Vec<PathBuf>>::new(), // ID
            |mut acc, (hash, path)| {
                acc.entry(hash).or_default().push(path);
                acc
            },
        )
        .reduce(
            || HashMap::new(), // identity for reduce
            |mut acc, map| {
                for (hash, paths) in map {
                    acc.entry(hash).or_default().extend(paths);
                }
                acc
            },
        )
}