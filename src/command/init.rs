use crate::utility::encrypt;
use std::fs;
use std::path::Path;

pub fn run() {
    if Path::new(".arc").exists() {
        println!(".arc already exists.");
        return;
    }
    fs::create_dir_all(".arc/state/chunks").unwrap();
    fs::create_dir_all(".arc/history").unwrap();

    encrypt::generate_key(Path::new(".arc/secret.key"));

    let key = encrypt::load_key(Path::new(".arc/secret.key"));

    let config = crate::types::Config {
        arc_version: "1.0".to_string(),
        archive_name: "MyArc".to_string(),
        description: "Local Arc MVP".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        creator_public_key: "SOME-FAKE-KEY-FOR-NOW".to_string(),
    };

    let config_json = serde_json::to_string_pretty(&config).unwrap();
    encrypt::encrypt_to_file(&key, config_json.as_bytes(), Path::new(".arc/config.json"));

    println!("Initialized Arc with encryption.");
}
