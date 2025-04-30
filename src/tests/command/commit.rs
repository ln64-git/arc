use crate::command::commit;
use crate::types::StagingEntry;
use std::fs::{self};
use tempfile::tempdir;

#[test]
fn test_commit_creates_history() {
    let temp = tempdir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    // Create base .arc directory
    fs::create_dir_all(".arc/state").unwrap();
    fs::create_dir_all(".arc/history").unwrap();
    fs::create_dir_all(".arc/state/chunks").unwrap(); // must happen AFTER state

    // Write dummy secret key
    let key = [0u8; 32];
    fs::write(".arc/secret.key", &key).unwrap();

    // Write dummy staging
    let staging = vec![StagingEntry {
        path: "dummy.txt".into(),
        hash: "deadbeef".into(),
    }];
    fs::write(
        ".arc/state/staging.json",
        serde_json::to_string(&staging).unwrap(),
    )
    .unwrap();

    // Write dummy chunk file
    fs::write(".arc/state/chunks/deadbeef", b"dummydata").unwrap();

    // Run commit
    commit::run();

    // Check latest.json was created
    let latest_exists = fs::read_dir(".arc/history")
        .unwrap()
        .any(|e| e.unwrap().file_name() == "latest.json");
    assert!(latest_exists, "No latest.json created");
}
