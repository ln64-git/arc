use crate::types::{FileEntry, HistoryEntry};
use std::fs;
use std::path::Path;

pub fn create_manifest(staging_dir: &Path) -> HistoryEntry {
    let mut files = Vec::new();

    for entry in fs::read_dir(staging_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let hash = filename.clone(); // Since chunks are named by their hash

            files.push(FileEntry {
                path: filename,
                hash,
            });
        }
    }

    HistoryEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        files,
    }
}
