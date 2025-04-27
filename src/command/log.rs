use std::fs;

pub fn run() {
    let entries = fs::read_dir(".arc/history").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let contents = fs::read_to_string(&path).unwrap();
            let history: crate::types::HistoryEntry = serde_json::from_str(&contents).unwrap();
            println!("{} -> {}", history.timestamp, history.snapshot_hash);
        }
    }
}
