use crate::types::StagingEntry;
use crate::types::{FileEntry, HistoryEntry};
use crate::utility::encrypt;
use aes_gcm::KeyInit;
use rand::RngCore;
use rand::rngs::OsRng;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;

pub fn setup_fake_arc(write_staging: bool, write_chunk: bool) -> PathBuf {
    let arc_path = PathBuf::from(".arc");
    let state = arc_path.join("state");
    let chunks = state.join("chunks");
    let history = arc_path.join("history");

    // Create required dirs
    fs::create_dir_all(&chunks).expect("❌ Failed to create chunks dir");
    fs::create_dir_all(&history).expect("❌ Failed to create history dir");

    // Write AES key
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    fs::write(arc_path.join("secret.key"), &key_bytes).expect("❌ Failed to write secret.key");

    if write_staging {
        let staging = vec![StagingEntry {
            path: "dummy.txt".into(),
            hash: "deadbeef".into(),
        }];
        let staging_path = state.join("staging.json");

        fs::create_dir_all(staging_path.parent().unwrap())
            .expect("❌ Failed to create staging dir");
        fs::write(&staging_path, serde_json::to_string(&staging).unwrap())
            .expect("❌ Failed to write staging.json");
        println!("✅ Wrote staging.json");
    }

    // Add this to setup_fake_arc right after writing staging
    let raw = fs::read_to_string(".arc/state/staging.json").unwrap();
    println!("📦 staging.json: {}", raw);
    let parsed: Vec<crate::types::StagingEntry> = serde_json::from_str(&raw).unwrap();
    assert!(!parsed.is_empty(), "❌ staging.json parsed to empty vec");

    if write_chunk {
        let chunk_path = chunks.join("deadbeef");

        // ✅ Ensure parent dir exists
        if let Some(parent) = chunk_path.parent() {
            fs::create_dir_all(parent).expect("❌ Failed to create chunk parent dir");
        }

        if let Some(parent) = chunk_path.parent() {
            fs::create_dir_all(parent).expect("❌ Failed to create parent dir for chunk");
        }
        fs::write(&chunk_path, b"dummydata").expect("❌ Failed to write dummy chunk");
        println!("✅ Wrote dummy chunk");
    }

    arc_path
}

pub fn setup_fake_arc_pull_source(base: &Path) -> aes_gcm::Aes256Gcm {
    let arc_path = base.join(".arc");
    fs::create_dir_all(arc_path.join("state/chunks")).unwrap();
    fs::create_dir_all(arc_path.join("history")).unwrap();

    let key_bytes = [0u8; 32];
    let key = aes_gcm::Aes256Gcm::new_from_slice(&key_bytes).unwrap();
    fs::write(arc_path.join("secret.key"), &key_bytes).unwrap();

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

    // Fake encrypted chunk
    let chunk = encrypt::encrypt(&key, b"restored contents");
    fs::write(arc_path.join("state/chunks/deadbeef"), chunk).unwrap();

    // Fake encrypted manifest
    let manifest = HistoryEntry {
        timestamp: "now".into(),
        snapshot_hash: "somehash".into(),
        files: vec![FileEntry {
            path: "recovered.txt".into(),
            hash: "deadbeef".into(),
        }],
    };
    let manifest_json = serde_json::to_string(&manifest).unwrap();
    let encrypted_manifest = encrypt::encrypt(&key, manifest_json.as_bytes());

    fs::write(arc_path.join("history/latest.json"), encrypted_manifest).unwrap();

    key
}

pub fn setup_fake_arc_unlock_manifest(dir: &Path) {
    let arc = dir.join(".arc");
    fs::create_dir_all(arc.join("state/chunks")).unwrap();

    // 🔑 Write secret.key
    let key_bytes = [0u8; 32];
    fs::write(arc.join("secret.key"), &key_bytes).unwrap();
    let cipher = aes_gcm::Aes256Gcm::new_from_slice(&key_bytes).unwrap();

    // 📦 Encrypt chunk file contents
    let encrypted = crate::utility::encrypt::encrypt(&cipher, b"restored contents");
    fs::write(arc.join("state/chunks/deadbeef"), encrypted).unwrap();

    // 📜 Write manifest with hash matching the chunk
    let manifest = crate::types::HistoryEntry {
        timestamp: "now".into(),
        snapshot_hash: "hash123".into(),
        files: vec![crate::types::FileEntry {
            path: "restored.txt".into(),
            hash: "deadbeef".into(),
        }],
    };

    let manifest_path = arc.join("state/latest_manifest.json");
    fs::write(manifest_path, serde_json::to_string(&manifest).unwrap()).unwrap();
}
