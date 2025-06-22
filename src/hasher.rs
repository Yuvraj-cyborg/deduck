use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::collections::HashMap;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;
use sha2::{Sha256, Digest};
use blake3;
use twox_hash::XxHash64;
use std::hash::Hasher;

use indicatif::{ProgressBar};

#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    Sha256,
    Blake3,
    XxHash,
}

pub fn hash_file(path: &PathBuf, algo: &HashAlgorithm) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    match algo {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            std::io::copy(&mut reader, &mut hasher)?;
            Ok(format!("{:x}", hasher.finalize()))
        }

        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            hasher.update_reader(&mut reader)?;
            Ok(hasher.finalize().to_hex().to_string())
        }

        HashAlgorithm::XxHash => {
            let mut hasher = XxHash64::with_seed(0);
            let mut buffer = [0u8; 8192];
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.write(&buffer[..count]);
            }
            Ok(format!("{:x}", hasher.finish()))
        }
    }
}

pub fn hash_files(files: Vec<PathBuf>,algo: HashAlgorithm,pb: ProgressBar,) -> HashMap<String, Vec<PathBuf>> {
    files
        .par_iter()
        .progress_with(pb)
        .filter_map(|file| {
            match hash_file(file, &algo) {
            Ok(hash) => Some((hash, file.clone())),
            Err(_) => None,
            }
     })
        .fold(HashMap::new, add_to_map)
        .reduce(HashMap::new, merge_maps)
}


fn add_to_map( mut acc: HashMap<String, Vec<PathBuf>>,item: (String, PathBuf)) -> HashMap<String, Vec<PathBuf>> {
    let (hash, path) = item;
    acc.entry(hash).or_default().push(path);
    acc
}

fn merge_maps( mut acc: HashMap<String, Vec<PathBuf>>, map: HashMap<String, Vec<PathBuf>>) -> HashMap<String, Vec<PathBuf>> {
    for (hash, paths) in map {
        acc.entry(hash).or_default().extend(paths);
    }
    acc
}
