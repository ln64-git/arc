use std::{fs, path::Path};

use crate::{command::commit, utility::setup_fake_arc::setup_fake_arc};

#[test]
fn test_commit_creates_history() {
    let tempdir = tempfile::tempdir().expect("❌ Failed to create temp dir");
    std::env::set_current_dir(tempdir.path()).unwrap();

    let _arc_path = setup_fake_arc(true, true);

    let secret_key = Path::new(".arc/secret.key");
    assert!(secret_key.exists(), "❌ secret.key not created");

    commit::run();

    let latest_json = Path::new(".arc/history/latest.json");
    assert!(
        latest_json.exists(),
        "❌ No latest.json created at {:?}",
        latest_json
    );

    let metadata = fs::metadata(&latest_json).expect("❌ Cannot stat latest.json");
    assert!(
        metadata.len() > 16,
        "❌ latest.json too small, likely empty or failed encryption"
    );

    println!("✅ Test passed: Encrypted latest.json created");
}
