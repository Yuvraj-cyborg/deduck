use std::collections::HashMap;
use std::path::PathBuf;
use image::io::Reader as ImageReader;
use imagehash::{perceptual_hash};

pub fn similar_images(files: Vec<PathBuf>, threshold: u32) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut hashes = Vec::new();

    for path in &files {
        if let Ok(reader) = ImageReader::open(path) {
            if let Ok(img) = reader.decode() {
                let hash = perceptual_hash(&img);
                hashes.push((path.clone(), hash));
            } else {
                eprintln!("⚠️ Could not decode image: {}", path.display());
            }
        } else {
            eprintln!("⚠️ Could not open image: {}", path.display());
        }
    }

    let mut groups: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();

    for (i, (path_i, hash_i)) in hashes.iter().enumerate() {
        for (path_j, hash_j) in hashes.iter().skip(i + 1) {
            let dist = hamming_distance(&hash_i.bits, &hash_j.bits);
            if dist <= threshold {
                groups.entry(path_i.clone()).or_default().push(path_j.clone());
            }
        }
    }

    groups
}

fn hamming_distance(a: &Vec<bool>, b: &Vec<bool>) -> u32 {
    a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() as u32
}
