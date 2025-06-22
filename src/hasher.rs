use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::collections::HashMap;
use rayon::prelude::*;
use indicatif::{ProgressBar, ParallelProgressIterator};
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

    if let HashAlgorithm::Sha256 = algo {
        let mut hasher = Sha256::new();
        std::io::copy(&mut reader, &mut hasher)?;
        return Ok(format!("{:x}", hasher.finalize()));
    }

    if let HashAlgorithm::Blake3 = algo {
        let mut hasher = blake3::Hasher::new();
        hasher.update_reader(&mut reader)?;
        return Ok(hasher.finalize().to_hex().to_string());
    }

    if let HashAlgorithm::XxHash = algo {
        let mut hasher = XxHash64::with_seed(0);
        let mut buffer = [0u8; 8192];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.write(&buffer[..count]);
        }
        return Ok(format!("{:x}", hasher.finish()));
    }

    Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported hash algorithm"))
}

pub fn hash_files(files: Vec<PathBuf>, algo: HashAlgorithm, pb: ProgressBar) -> HashMap<String, Vec<PathBuf>> {
    let maps: Vec<HashMap<String, Vec<PathBuf>>> = files
        .par_iter()
        .progress_with(pb)
        .filter_map(|file| {
            match hash_file(file, &algo) {
                Ok(hash) => Some((hash, file.clone())),
                Err(_) => None,
            }
        })
        .map(|(hash, path)| {
            let mut map = HashMap::new();
            map.entry(hash).or_insert_with(Vec::new).push(path);
            map
        })
        .collect();

    let mut result = HashMap::new();

    for map in maps {
        for (hash, paths) in map {
            if !result.contains_key(&hash) {
                result.insert(hash.clone(), Vec::new());
            }
            result.get_mut(&hash).unwrap().extend(paths);
        }
    }

    result
}
