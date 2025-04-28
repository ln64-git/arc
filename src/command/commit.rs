use crate::types::{FileEntry, HistoryEntry, StagingEntry};
use crate::utility::encrypt;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub fn run() {
    println!("Committing Arc...");

    let staging_path = ".arc/state/staging.json";
    if !Path::new(staging_path).exists() {
        println!("Nothing to commit.");
        return;
    }

    let staging_contents = fs::read_to_string(staging_path).unwrap();
    let staging: Vec<StagingEntry> = serde_json::from_str(&staging_contents).unwrap();

    let mut state = Sha256::new();

    // Validate that all chunks exist and hash their contents
    for entry in &staging {
        let chunk_path = format!(".arc/state/chunks/{}", entry.hash);
        let mut file = File::open(&chunk_path).expect("Chunk file missing");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        state.update(&contents);
    }

    let snapshot_hash = hex::encode(state.finalize());

    let files: Vec<FileEntry> = staging
        .into_iter()
        .map(|entry| FileEntry {
            path: entry.path,
            hash: entry.hash,
        })
        .collect();

    let history_entry = HistoryEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        snapshot_hash,
        files,
    };

    let history_json = serde_json::to_string_pretty(&history_entry).unwrap();
    let key = encrypt::load_key(Path::new(".arc/secret.key"));

    let history_path = format!(".arc/history/{}.json", history_entry.timestamp);
    encrypt::encrypt_to_file(&key, history_json.as_bytes(), Path::new(&history_path));

    encrypt::encrypt_to_file(
        &key,
        history_json.as_bytes(),
        Path::new(".arc/history/latest.json"),
    );

    fs::remove_file(".arc/state/staging.json").unwrap();

    println!("Committed encrypted state.");
}
