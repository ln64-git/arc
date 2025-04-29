use crate::types::HistoryEntry;
use crate::utility::encrypt;
use aes_gcm::Aes256Gcm;
use std::fs;
use std::path::Path;

pub fn run(target: Option<String>) {
    println!("🔓 Unlocking ARC...");

    let cipher = encrypt::load_secret_key();
    let manifest = load_manifest();

    let files = match target {
        Some(ref path) => manifest
            .files
            .iter()
            .filter(|entry| entry.path.starts_with(path))
            .cloned()
            .collect(),
        _ => manifest.files,
    };

    if files.is_empty() {
        println!("❌ No matching files to unlock.");
        return;
    }

    for file_entry in files {
        unlock_file(&file_entry.path, &file_entry.hash, &cipher);
    }

    println!("🎉 Unlock complete.");
}

fn load_manifest() -> HistoryEntry {
    let manifest_path = Path::new(".arc/state/latest_manifest.json");
    let manifest_data = fs::read_to_string(manifest_path).expect("Failed to read manifest");
    serde_json::from_str(&manifest_data).expect("Invalid manifest format")
}

fn unlock_file(path_str: &str, hash: &str, cipher: &Aes256Gcm) {
    let chunk_path = Path::new(".arc/state/chunks").join(hash);

    if !chunk_path.exists() {
        println!("⚠️ Missing chunk for: {}", path_str);
        return;
    }

    println!("🔓 Unlocking file: {}", path_str);

    let encrypted = fs::read(&chunk_path).expect("Failed to read encrypted chunk");
    let decrypted = encrypt::decrypt(cipher, &encrypted).expect("Failed to decrypt");

    let output_path = Path::new(path_str);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(output_path, decrypted).expect("Failed to restore file");
}
