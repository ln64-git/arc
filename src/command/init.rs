use crate::utility::encrypt;
use std::fs;
use std::path::Path;

pub fn run() {
    if Path::new(".arc").exists() {
        println!("⚠️ .arc already exists.");
        return;
    }

    // Create Arc Directory Structure
    fs::create_dir_all(".arc/state/chunks").expect("❌ Failed to create .arc/state/chunks");
    fs::create_dir_all(".arc/history").expect("❌ Failed to create .arc/history");

    // Generate Secret Key
    encrypt::generate_key(Path::new(".arc/secret.key"));

    // Prepare Default Config
    let config = crate::types::Config {
        arc_version: "1.0".to_string(),
        archive_name: "MyArc".to_string(),
        description: "Local Arc MVP".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        creator_public_key: "SOME-FAKE-KEY-FOR-NOW".to_string(),
        auto_pull_chunks: true,
        auto_decrypt_files: true,
        export_secret_key: true,
    };

    // Save Config JSON (plaintext, NOT encrypted)
    let config_json = serde_json::to_string_pretty(&config).expect("❌ Failed to serialize config");
    fs::write(".arc/config.json", config_json).expect("❌ Failed to write config.json");

    println!("✅ Initialized Arc directory with secret key and plaintext config.json.");
}
