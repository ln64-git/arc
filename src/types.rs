use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub arc_version: String,
    pub archive_name: String,
    pub description: String,
    pub created_at: String,
    pub creator_public_key: String,
    pub export_secret_key: bool,
}

#[derive(Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: String,
    pub snapshot_hash: String,
    pub files: Vec<FileEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StagingEntry {
    pub path: String,
    pub hash: String,
}
