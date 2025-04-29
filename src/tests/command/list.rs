use crate::command::list;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_list_manifest_files() {
    let temp = tempdir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    fs::create_dir_all(".arc/state").unwrap();

    let manifest = crate::types::HistoryEntry {
        timestamp: "now".into(),
        snapshot_hash: "somehash".into(),
        files: vec![
            crate::types::FileEntry {
                path: "file1.txt".into(),
                hash: "hash1".into(),
            },
            crate::types::FileEntry {
                path: "file2.txt".into(),
                hash: "hash2".into(),
            },
        ],
    };
    fs::write(
        ".arc/state/latest_manifest.json",
        serde_json::to_string(&manifest).unwrap(),
    )
    .unwrap();

    list::run(false);
    // This only prints, for better test we would capture stdout.
}
