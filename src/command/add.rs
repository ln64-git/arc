use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn run(file_path: &str) {
    let mut file = File::open(file_path).expect("Cannot open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let hash = hex::encode(hasher.finalize());

    let chunk_path = format!(".Arc/state/chunks/{}", hash);
    if !Path::new(&chunk_path).exists() {
        let mut chunk_file = File::create(chunk_path).unwrap();
        chunk_file.write_all(&contents).unwrap();
    }

    println!("Added {} with hash {}", file_path, hash);
}
