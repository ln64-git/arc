use crate::types::{FileEntry, HistoryEntry, StagingEntry};
use crate::utility::encrypt;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub fn run() {
    println!("Committing Arc...");

    let staging_path = Path::new(".arc/state/staging.json");

    if !staging_path.exists() {
        println!("Nothing to commit.");
        return;
    }

    let staging_contents = match fs::read_to_string(staging_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read staging.json: {}", e);
            return;
        }
    };

    let staging: Vec<StagingEntry> = match serde_json::from_str(&staging_contents) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("❌ Invalid staging.json format: {}", e);
            return;
        }
    };

    println!("Raw staging contents: {}", staging_contents);
    println!("Parsed staging entries: {:?}", staging);

    if staging.is_empty() {
        println!("Nothing to commit (staging empty).");
        return;
    }

    let mut state = Sha256::new();

    for entry in &staging {
        let chunk_path = format!(".arc/state/chunks/{}", entry.hash);
        let mut file = File::open(&chunk_path)
            .unwrap_or_else(|_| panic!("Chunk file missing: {}", chunk_path));
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

    // Ensure .arc/history exists
    fs::create_dir_all(".arc/history").expect("❌ Failed to create .arc/history");

    let safe_ts = history_entry.timestamp.replace(':', "-");
    let history_path = format!(".arc/history/{}.json", safe_ts);
    encrypt::encrypt_to_file(&key, history_json.as_bytes(), Path::new(&history_path));

    encrypt::encrypt_to_file(
        &key,
        history_json.as_bytes(),
        Path::new(".arc/history/latest.json"),
    );

    let _ = fs::remove_file(staging_path);

    println!("✅ Committed encrypted state.");
}
