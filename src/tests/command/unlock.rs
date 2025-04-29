use crate::command::unlock;
use aes_gcm::KeyInit;
use std::fs::{self};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_unlock_restores_file() {
    let temp = tempdir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    fs::create_dir_all(".arc/state/chunks").unwrap();
    fs::create_dir_all(".arc/state").unwrap();

    // Shared fake key
    let key_bytes = [0u8; 32];
    fs::write(".arc/secret.key", &key_bytes).unwrap();
    let key = aes_gcm::Aes256Gcm::new_from_slice(&key_bytes).unwrap();

    // Encrypt fake chunk
    let encrypted = crate::utility::encrypt::encrypt(&key, b"restored contents");
    fs::write(".arc/state/chunks/deadbeef", encrypted).unwrap();

    // Fake manifest
    let manifest = crate::types::HistoryEntry {
        timestamp: "now".into(),
        snapshot_hash: "somehash".into(),
        files: vec![crate::types::FileEntry {
            path: "restored.txt".into(),
            hash: "deadbeef".into(),
        }],
    };
    fs::write(
        ".arc/state/latest_manifest.json",
        serde_json::to_string(&manifest).unwrap(),
    )
    .unwrap();

    unlock::run(None);

    let out_path = Path::new("restored.txt");
    assert!(out_path.exists(), "Restored file not found");
    let contents = fs::read_to_string(out_path).unwrap();
    assert_eq!(contents, "restored contents");
}
