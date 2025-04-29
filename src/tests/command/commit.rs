use std::fs::{self};
use crate::types::StagingEntry;
use crate::command::commit;
use tempfile::tempdir;

#[test]
fn test_commit_creates_history() {
    let temp = tempdir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    // Fake .arc structure
    fs::create_dir_all(".arc/state/chunks").unwrap();
    fs::create_dir_all(".arc/history").unwrap();
    fs::create_dir_all(".arc/state").unwrap();

    // Fake secret.key
    let key = [0u8; 32];
    fs::create_dir_all(".arc").unwrap();
    fs::write(".arc/secret.key", &key).unwrap();

    // Fake staging.json
    let staging = vec![StagingEntry {
        path: "dummy.txt".into(),
        hash: "deadbeef".into(),
    }];
    let staging_json = serde_json::to_string(&staging).unwrap();
    fs::write(".arc/state/staging.json", staging_json).unwrap();

    // Fake chunk file
    fs::write(".arc/state/chunks/deadbeef", b"dummydata").unwrap();

    // Run commit
    commit::run();

    // Assert latest.json exists
    let latest = fs::read_dir(".arc/history")
        .unwrap()
        .any(|e| e.unwrap().file_name() == "latest.json");
    assert!(latest, "No latest.json created");
}
