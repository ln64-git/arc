use crate::types::HistoryEntry;
use crate::utility::encrypt;
use aes_gcm::Aes256Gcm;
use std::fs;
use std::path::Path;

pub fn run(target: Option<String>) {
    println!("🔒 Locking ARC...");

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
        println!("❌ No matching files to lock.");
        return;
    }

    for file_entry in files {
        lock_file(&file_entry.path, &file_entry.hash, &cipher);
    }

    println!("🔒 Lock complete.");
}

fn load_manifest() -> HistoryEntry {
    let manifest_path = Path::new(".arc/state/latest_manifest.json");
    let manifest_data = fs::read_to_string(manifest_path).expect("Failed to read manifest");
    serde_json::from_str(&manifest_data).expect("Invalid manifest format")
}

fn lock_file(path_str: &str, hash: &str, cipher: &Aes256Gcm) {
    let path = Path::new(path_str);

    if !path.exists() {
        println!("⚠️ Skipping missing file: {}", path.display());
        return;
    }

    println!("🔒 Locking file: {}", path.display());

    let data = fs::read(&path).expect("Failed to read file");
    let encrypted = encrypt::encrypt(cipher, &data);

    let chunk_path = Path::new(".arc/state/chunks").join(hash);
    fs::create_dir_all(chunk_path.parent().unwrap()).unwrap();
    fs::write(chunk_path, encrypted).expect("Failed to write encrypted chunk");

    fs::remove_file(path).expect("Failed to remove original file");

    cleanup_empty_dirs(path.parent());
}

fn cleanup_empty_dirs(mut dir: Option<&Path>) {
    while let Some(path) = dir {
        if path == Path::new(".") || path == Path::new("..") {
            break;
        }
        match fs::read_dir(path) {
            Ok(mut entries) => {
                if entries.next().is_none() {
                    println!("🧹 Removing empty folder: {}", path.display());
                    fs::remove_dir(path).unwrap_or_else(|e| {
                        println!("⚠️ Failed to remove folder {}: {}", path.display(), e);
                    });
                } else {
                    break; // Directory not empty
                }
            }
            _ => break, // Failed to read
        }
        dir = path.parent();
    }
}
