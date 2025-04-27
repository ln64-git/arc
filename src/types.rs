use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub arc_version: String,
    pub archive_name: String,
    pub description: String,
    pub created_at: String,
    pub creator_public_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: String,
    pub snapshot_hash: String,
}
