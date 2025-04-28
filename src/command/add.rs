use crate::types::StagingEntry;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn run(file_path: &str) {
    let mut file = File::open(file_path).expect("Cannot open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let hash = hex::encode(hasher.finalize());

    let chunk_path = format!(".arc/state/chunks/{}", hash);
    if !Path::new(&chunk_path).exists() {
        let mut chunk_file = File::create(&chunk_path).unwrap();
        chunk_file.write_all(&contents).unwrap();
    }

    println!("Added {} with hash {}", file_path, hash);

    // 🛡 Save to staging.json
    let staging_path = ".arc/state/staging.json";
    let mut staging: Vec<StagingEntry> = if Path::new(staging_path).exists() {
        let contents = fs::read_to_string(staging_path).unwrap();
        serde_json::from_str(&contents).unwrap()
    } else {
        Vec::new()
    };

    staging.push(StagingEntry {
        path: file_path.to_string(),
        hash: hash,
    });

    let staging_json = serde_json::to_string_pretty(&staging).unwrap();
    fs::write(staging_path, staging_json).unwrap();
}
