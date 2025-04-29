use crate::command::pull;
use aes_gcm::KeyInit;
use std::fs::{self};
use tempfile::tempdir;

#[test]
fn test_pull_manifest_copies_chunks() {
    let temp_src = tempdir().unwrap();
    let temp_dest = tempdir().unwrap();

    // Setup fake ARC repo
    let arc_path = temp_src.path().join(".arc");
    fs::create_dir_all(arc_path.join("history")).unwrap();
    fs::create_dir_all(arc_path.join("state/chunks")).unwrap();

    // Write fake config + key
    fs::write(
        arc_path.join("config.json"),
        r#"{
            "arc_version": "1.0",
            "archive_name": "TestArc",
            "description": "Test",
            "created_at": "now",
            "creator_public_key": "abc",
            "export_secret_key": true
        }"#,
    )
    .unwrap();
    fs::write(arc_path.join("secret.key"), [0u8; 32]).unwrap();

    // Shared key
    let key = aes_gcm::Aes256Gcm::new_from_slice(&[0u8; 32]).unwrap();

    // Create fake encrypted chunk
    let chunk = crate::utility::encrypt::encrypt(&key, b"restored contents");
    fs::write(arc_path.join("state/chunks/deadbeef"), chunk).unwrap();

    // Create and encrypt manifest
    let manifest = crate::types::HistoryEntry {
        timestamp: "now".into(),
        snapshot_hash: "somehash".into(),
        files: vec![crate::types::FileEntry {
            path: "recovered.txt".into(),
            hash: "deadbeef".into(),
        }],
    };
    let manifest_json = serde_json::to_string(&manifest).unwrap();
    let enc_manifest = crate::utility::encrypt::encrypt(&key, manifest_json.as_bytes());
    fs::write(arc_path.join("history/latest.json"), enc_manifest).unwrap();

    // Run pull with file arg to trigger restore
    std::env::set_current_dir(temp_dest.path()).unwrap();
    pull::run(temp_src.path().to_str().unwrap(), Some(".".to_string()));

    let out_path = temp_dest.path().join("recovered.txt");
    assert!(out_path.exists(), "Pulled file not restored");

    let contents = fs::read_to_string(out_path).unwrap();
    assert_eq!(contents, "restored contents");
}
