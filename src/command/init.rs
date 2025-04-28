use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;

pub fn run() {
    if Path::new(".arc").exists() {
        println!(".arc already exists.");
        return;
    }
    fs::create_dir(".arc").unwrap();
    fs::create_dir(".arc/state").unwrap();
    fs::create_dir(".arc/state/chunks").unwrap();
    fs::create_dir(".arc/history").unwrap();

    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let public_key_hex = hex::encode(verifying_key.to_bytes());

    let config = crate::types::Config {
        arc_version: "1.0".to_string(),
        archive_name: "MyArc".to_string(),
        description: "Local Arc MVP".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        creator_public_key: public_key_hex,
    };

    let config_json = serde_json::to_string_pretty(&config).unwrap();
    fs::write(".arc/config.json", config_json).unwrap();

    println!("Initialized Arc.");
}
