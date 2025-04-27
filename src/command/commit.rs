use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Read;

pub fn run() {
    let mut state = Sha256::new();

    for entry in fs::read_dir(".arc/state/chunks").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(&path).unwrap();
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();
            state.update(&contents);
        }
    }

    let snapshot_hash = hex::encode(state.finalize());

    let entry = crate::types::HistoryEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        snapshot_hash,
    };

    let history_path = format!(".arc/history/{}.json", entry.timestamp);
    let entry_json = serde_json::to_string_pretty(&entry).unwrap();
    fs::write(history_path, entry_json).unwrap();

    println!("Committed state.");
}
