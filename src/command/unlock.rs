use crate::utility::encrypt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn run(target: Option<String>) {
    println!("🔓 Unlocking ARC...");

    let cipher = encrypt::load_secret_key();

    match target {
        Some(file_path) => {
            unlock_single_file(&cipher, &file_path);
        }
        _ => {
            unlock_entire_arc(&cipher);
        }
    }

    println!("🎉 Unlock complete.");
}

fn unlock_single_file(cipher: &aes_gcm::Aes256Gcm, file_path: &str) {
    let path = Path::new(file_path);

    if !path.exists() {
        println!("❌ File not found: {}", file_path);
        return;
    }

    println!("🔍 Trying to unlock: {}", file_path); // 🧙 PRINT WHICH FILE FIRST

    let encrypted = match fs::read(path) {
        Ok(data) => data,
        Err(_) => {
            println!("❌ Failed to read file: {}", file_path);
            return;
        }
    };

    println!("🔎 Read {} bytes from {}", encrypted.len(), file_path);

    if encrypted.len() < 12 {
        println!(
            "⚠️ File too small to be encrypted ({} bytes). Skipping: {}",
            encrypted.len(),
            file_path
        );
        return;
    }

    match encrypt::decrypt(cipher, &encrypted) {
        Ok(decrypted) => {
            let mut output = fs::File::create(path).expect("Failed to overwrite file");
            output.write_all(&decrypted).unwrap();
            println!("✅ Unlocked file {}", file_path);
        }
        Err(_) => {
            println!(
                "⚠️ Warning: Could not decrypt file: {}. Possibly already unlocked or corrupted.",
                file_path
            );
        }
    }
}

fn unlock_entire_arc(cipher: &aes_gcm::Aes256Gcm) {
    let arc_dir = Path::new(".arc");

    if !arc_dir.exists() {
        println!("No .arc/ found. Nothing to unlock.");
        return;
    }

    let mut files_to_unlock = Vec::new();
    visit_dirs(arc_dir, &mut files_to_unlock);

    for file_path in files_to_unlock {
        unlock_single_file(cipher, file_path.to_str().unwrap());
    }

    // 🛡 Now unlock project files from manifest
    let manifest_path = Path::new(".arc/state/latest_manifest.json");

    if manifest_path.exists() {
        println!("🔎 Unlocking added project files...");

        let manifest_contents = fs::read_to_string(manifest_path).expect("Failed to read manifest");
        let manifest: crate::types::HistoryEntry =
            serde_json::from_str(&manifest_contents).expect("Invalid manifest");

        for file_entry in manifest.files {
            println!("🔓 Trying to unlock project file: {}", file_entry.path);
            unlock_single_file(cipher, &file_entry.path);
        }
    } else {
        println!("⚠️ No manifest found. Skipping added project files unlock.");
    }
}

fn visit_dirs(dir: &Path, files: &mut Vec<PathBuf>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files);
            } else {
                files.push(path);
            }
        }
    }
}
