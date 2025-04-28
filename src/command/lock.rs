use crate::types::HistoryEntry;
use crate::utility::encrypt;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(file: Option<String>) {
    let cipher = encrypt::load_secret_key();

    match file {
        Some(filepath) => {
            // 🛡 Single file lock
            lock_file(filepath, &cipher);
        }
        _ => {
            // 🛡 Full ARC lock
            lock_all(&cipher);
        }
    }
}

fn lock_file(filepath: String, cipher: &aes_gcm::Aes256Gcm) {
    let path = Path::new(&filepath);

    if !path.exists() {
        println!("File {} does not exist.", filepath);
        return;
    }

    println!("🔒 Encrypting file {}...", filepath);

    let plaintext = fs::read(&path).expect("Failed to read file for locking");
    let encrypted = encrypt::encrypt(cipher, &plaintext);

    fs::write(&path, encrypted).expect("Failed to encrypt file");

    println!("🔒 File {} encrypted successfully.", filepath);
}

fn lock_all(cipher: &aes_gcm::Aes256Gcm) {
    println!("🔒 Locking entire ARC...");

    let arc_dir = Path::new(".arc");

    if !arc_dir.exists() {
        println!("No .arc/ found. Nothing to lock.");
        return;
    }

    let mut files_to_lock = Vec::new();

    visit_dirs(arc_dir, &mut files_to_lock);

    for file_path in files_to_lock {
        if is_already_encrypted(&file_path) {
            println!("Already encrypted: {}", file_path.display());
            continue;
        }

        println!("Encrypting internal {}", file_path.display());

        let plaintext = fs::read(&file_path).expect("Failed to read file for locking");
        let encrypted = encrypt::encrypt(cipher, &plaintext);

        fs::write(&file_path, encrypted).expect("Failed to overwrite file with encrypted data");
    }

    let manifest_path = Path::new(".arc/state/latest_manifest.json");

    if manifest_path.exists() {
        println!("Locking added project files...");

        let manifest_bytes = encrypt::decrypt_from_file(cipher, manifest_path);
        let manifest: HistoryEntry =
            serde_json::from_slice(&manifest_bytes).expect("Invalid manifest");

        for file_entry in manifest.files {
            let file_path = Path::new(&file_entry.path);

            if file_path.exists() {
                println!("Encrypting added file {}", file_path.display());

                let plaintext =
                    fs::read(&file_path).expect("Failed to read added file for locking");
                let encrypted = encrypt::encrypt(cipher, &plaintext);

                fs::write(&file_path, encrypted).expect("Failed to encrypt added file");
            } else {
                println!("Warning: added file missing {}", file_path.display());
            }
        }
    } else {
        println!("Warning: No latest.json manifest found. Cannot lock added project files.");
    }

    println!("🔒 Full ARC lock complete.");
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

fn is_already_encrypted(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    if path_str.contains(".arc/secret.key") || path_str.contains("secret.key") {
        return true; // 🛡️ NEVER encrypt the secret key
    }
    if let Some(ext) = path.extension() {
        if ext == "json" {
            if path_str.contains("/history/") || path.ends_with("config.json") {
                return true;
            }
        }
    }
    false
}