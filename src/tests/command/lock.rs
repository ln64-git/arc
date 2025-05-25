use crate::command::lock;
use std::fs::{self};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_lock_encrypts_and_removes_original() {
    let temp = tempdir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    fs::create_dir_all(".arc/state/chunks").unwrap();
    fs::create_dir_all(".arc/state").unwrap();

    // Fake secret.key
    let key = [0u8; 32];
    fs::write(".arc/secret.key", &key).unwrap();

    // Create file and manifest
    let file_path = temp.path().join("important.txt");
    fs::write(&file_path, b"topsecret").unwrap();

    let manifest = crate::types::HistoryEntry {
        timestamp: "now".into(),
        snapshot_hash: "somehash".into(),
        files: vec![crate::types::FileEntry {
            path: file_path.to_string_lossy().to_string(),
            hash: "deadbeef".into(),
        }],
    };
    fs::write(
        ".arc/state/latest_manifest.json",
        serde_json::to_string(&manifest).unwrap(),
    )
    .unwrap();

    // Ensure the chunks directory exists before running the lock command
    fs::create_dir_all(".arc/state/chunks").unwrap();
    lock::run(None);

    assert!(!file_path.exists(), "Original file still exists after lock");
    assert!(
        Path::new(".arc/state/chunks/deadbeef").exists(),
        "Chunk file not created"
    );
}
