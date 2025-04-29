use crate::types::{Config, FileEntry, HistoryEntry};
use serde_json;

#[test]
fn test_config_serialization() {
    let config = Config {
        arc_version: "1.0".to_string(),
        archive_name: "TestArc".to_string(),
        description: "Testing Arc config".to_string(),
        created_at: "2025-04-01T00:00:00Z".to_string(),
        creator_public_key: "TEST-KEY".to_string(),
        export_secret_key: true,
    };

    let json = serde_json::to_string(&config).unwrap();
    let deserialized: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(config.arc_version, deserialized.arc_version);
    assert_eq!(config.archive_name, deserialized.archive_name);
}

#[test]
fn test_history_entry_serialization() {
    let history = HistoryEntry {
        timestamp: "2025-04-01T00:00:00Z".to_string(),
        snapshot_hash: "abc123".to_string(),
        files: vec![FileEntry {
            path: "file1.txt".to_string(),
            hash: "hash1".to_string(),
        }],
    };

    let json = serde_json::to_string(&history).unwrap();
    let deserialized: HistoryEntry = serde_json::from_str(&json).unwrap();

    assert_eq!(history.timestamp, deserialized.timestamp);
    assert_eq!(history.snapshot_hash, deserialized.snapshot_hash);
    assert_eq!(history.files.len(), deserialized.files.len());
}
