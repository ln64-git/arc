use crate::types::StagingEntry;
use crate::utility::encrypt;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn run(file_path: &str) {
    let path = Path::new(file_path);

    if path.is_dir() {
        println!("📂 Adding directory {} recursively", file_path);
        add_directory_recursively(path);
    } else if path.is_file() {
        add_single_file(path);
    } else {
        eprintln!("❌ Path {} is not a file or directory.", file_path);
    }
}

fn add_directory_recursively(dir: &Path) {
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        add_single_file(entry.path());
    }
}

fn add_single_file(file_path: &Path) {
    let mut file = File::open(file_path).expect("❌ Cannot open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let cipher = encrypt::load_key(Path::new(".arc/secret.key"));
    let encrypted = encrypt::encrypt(&cipher, &contents);

    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let hash = hex::encode(hasher.finalize());

    let chunk_path = format!(".arc/state/chunks/{}", hash);
    if !Path::new(&chunk_path).exists() {
        let mut chunk_file = File::create(&chunk_path).unwrap();
        chunk_file.write_all(&encrypted).unwrap();
    }

    println!("✅ Added {} with hash {}", file_path.display(), hash);

    let staging_path = ".arc/state/staging.json";
    let mut staging: Vec<StagingEntry> = if Path::new(staging_path).exists() {
        let contents = fs::read_to_string(staging_path).unwrap();
        serde_json::from_str(&contents).unwrap()
    } else {
        Vec::new()
    };

    staging.push(StagingEntry {
        path: file_path.to_string_lossy().to_string(),
        hash,
    });

    let staging_json = serde_json::to_string_pretty(&staging).unwrap();
    fs::write(staging_path, staging_json).unwrap();
}
