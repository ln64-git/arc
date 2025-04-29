use crate::types::{Config, FileEntry, HistoryEntry};
use crate::utility::encrypt;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn run(source_arc_path: &str, optional_file: Option<String>) {
    println!("🔄 Pulling from ARC at {}", source_arc_path);

    initialize_local_arc_structure_if_needed();
    copy_secret_key_and_config(source_arc_path);

    let key = encrypt::load_secret_key();
    let manifest = load_and_decrypt_manifest(source_arc_path, &key);

    match optional_file.as_deref() {
        Some(".") | Some("/") => {
            pull_all_files(&manifest, source_arc_path, &key);
        }
        Some(path) => {
            pull_smart(&manifest, source_arc_path, path, &key);
        }
        _ => {
            println!("📜 Available files in ARC:");
            display_manifest_summary(&manifest);
            println!("⚡ Use `arc pull <source> <file or dir>` to pull.");
        }
    }

    println!("🎉 Pull complete.");
}

fn pull_smart(
    manifest: &HistoryEntry,
    source_arc_path: &str,
    path: &str,
    key: &aes_gcm::Aes256Gcm,
) {
    let exact = manifest.files.iter().find(|f| f.path == path);
    let recursive: Vec<&FileEntry> = manifest
        .files
        .iter()
        .filter(|f| f.path.starts_with(path))
        .collect();

    match (exact, recursive.len()) {
        (Some(file), _) => {
            println!("🚚 Pulling file: {}", path);
            pull_and_restore_file(file, source_arc_path, key);
        }
        (_, 0) => {
            eprintln!("❌ No match found for '{}'", path);
        }
        (_, n) if n > 0 => {
            println!(
                "📁 Pulling directory `{}` with {} files",
                path,
                recursive.len()
            );
            for entry in recursive {
                pull_and_restore_file(entry, source_arc_path, key);
            }
        }
        _ => {
            eprintln!("❌ Unknown pull state");
        }
    }
}

fn initialize_local_arc_structure_if_needed() {
    let arc_dir = Path::new(".arc");
    if !arc_dir.exists() {
        println!("🛠 No local .arc/ found. Initializing...");
        fs::create_dir_all(".arc/state/chunks").expect("Failed to create chunks dir");
        fs::create_dir_all(".arc/history").expect("Failed to create history dir");
    }
}

fn copy_secret_key_and_config(source_arc_path: &str) {
    let source_arc = Path::new(source_arc_path).join(".arc");
    let dest_arc = Path::new(".arc");

    let config_path = source_arc.join("config.json");

    if !config_path.exists() {
        eprintln!("❌ Missing config.json in source ARC");
        return;
    }

    let config_data = fs::read_to_string(&config_path).expect("Failed to read config.json");
    let config: Config = serde_json::from_str(&config_data).expect("Invalid config.json format");

    // Always copy config.json
    copy_if_missing(&config_path, &dest_arc.join("config.json"), "config.json");

    // Conditionally copy secret.key
    if config.export_secret_key {
        copy_if_missing(
            &source_arc.join("secret.key"),
            &dest_arc.join("secret.key"),
            "secret.key",
        );
    } else {
        println!("🔒 Not copying secret.key (export_secret_key = false)");
    }
}

fn copy_if_missing(src: &Path, dst: &Path, label: &str) {
    if src.exists() && !dst.exists() {
        match fs::copy(src, dst) {
            Ok(_) => println!("✅ Copied {}", label),
            Err(e) => eprintln!("❌ Failed to copy {}: {}", label, e),
        }
    }
}

fn load_and_decrypt_manifest(source_arc_path: &str, key: &aes_gcm::Aes256Gcm) -> HistoryEntry {
    let manifest_path = Path::new(source_arc_path).join(".arc/history/latest.json");
    let manifest_bytes = encrypt::decrypt_from_file(key, &manifest_path);

    let manifest: HistoryEntry =
        serde_json::from_slice(&manifest_bytes).expect("Invalid manifest format");

    // ⬇️ NEW: Save plaintext manifest locally
    let local_manifest_path = Path::new(".arc/state/latest_manifest.json");
    if let Ok(json) = serde_json::to_string_pretty(&manifest) {
        if let Err(e) = fs::write(&local_manifest_path, json) {
            eprintln!("❌ Failed to save local manifest: {}", e);
        } else {
            println!("✅ Saved local manifest to .arc/state/latest_manifest.json");
        }
    }

    manifest
}

fn pull_all_files(manifest: &HistoryEntry, source_arc_path: &str, key: &aes_gcm::Aes256Gcm) {
    println!("🚚 Pulling all files...");
    for file_entry in &manifest.files {
        pull_and_restore_file(file_entry, source_arc_path, key);
    }
}

fn pull_and_restore_file(file_entry: &FileEntry, source_arc_path: &str, key: &aes_gcm::Aes256Gcm) {
    let source_chunk = Path::new(source_arc_path)
        .join(".arc/state/chunks")
        .join(&file_entry.hash);
    let dest_chunk = Path::new(".arc/state/chunks").join(&file_entry.hash);

    if !dest_chunk.exists() {
        match fs::copy(&source_chunk, &dest_chunk) {
            Ok(_) => println!("✅ Fetched chunk {}", file_entry.hash),
            Err(e) => {
                eprintln!("❌ Failed to fetch chunk {}: {}", file_entry.hash, e);
                return;
            }
        }
    } else {
        println!("🗂️ Already have chunk {}", file_entry.hash);
    }

    let encrypted_chunk = fs::read(&dest_chunk).expect("Failed to read encrypted chunk");
    let decrypted_contents =
        encrypt::decrypt(key, &encrypted_chunk).expect("Failed to decrypt chunk");

    let output_path = Path::new(&file_entry.path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let mut output_file = fs::File::create(output_path).unwrap();
    output_file.write_all(&decrypted_contents).unwrap();

    println!("✅ Restored {}", output_path.display());
}

fn display_manifest_summary(manifest: &HistoryEntry) {
    for file_entry in &manifest.files {
        println!("📄 {}", file_entry.path);
    }
}
