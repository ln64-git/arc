use crate::types::HistoryEntry;
use crate::utility::encrypt;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn run(source_arc_path: &str) {
    println!("🔄 Pulling from ARC at {}", source_arc_path);

    let source_arc_dir = Path::new(source_arc_path).join(".arc");
    let dest_arc_dir = Path::new(".arc");

    // 🛡 Step 1: Auto-Initialize ARC Structure
    initialize_local_arc_structure();

    // 🛡 Step 2: Copy secret.key if missing
    let source_secret = source_arc_dir.join("secret.key");
    let dest_secret = dest_arc_dir.join("secret.key");

    if !dest_secret.exists() && source_secret.exists() {
        fs::copy(&source_secret, &dest_secret).expect("Failed to copy secret.key");
        println!("✅ Copied secret.key");
    }

    // 🛡 Step 3: Load secret key
    let key = encrypt::load_secret_key();

    // 🛡 Step 4: Copy encrypted config.json (if exists)
    let source_config = source_arc_dir.join("config.json");
    let dest_config = dest_arc_dir.join("config.json");

    if source_config.exists() {
        fs::copy(&source_config, &dest_config).expect("Failed to copy config.json");
        println!("✅ Copied encrypted config.json");
    } else {
        println!("⚠️ Warning: No config.json found in source ARC.");
    }

    // 🛡 Step 5: Decrypt and load manifest (latest.json)
    let manifest_path = source_arc_dir.join("history/latest.json");

    if !manifest_path.exists() {
        println!("⚠️ Warning: No latest.json manifest found. Nothing to pull.");
        return;
    }

    let manifest_bytes = encrypt::decrypt_from_file(&key, &manifest_path);
    let manifest: HistoryEntry =
        serde_json::from_slice(&manifest_bytes).expect("Invalid manifest format");

    // 🛡 Step 6: Pull Chunks and Restore Files
    let source_chunks_path = source_arc_dir.join("state/chunks");
    let dest_chunks_path = dest_arc_dir.join("state/chunks");

    for file_entry in &manifest.files {
        let chunk_filename = &file_entry.hash;
        let source_chunk_path = source_chunks_path.join(chunk_filename);
        let dest_chunk_path = dest_chunks_path.join(chunk_filename);

        if !dest_chunk_path.exists() {
            match fs::copy(&source_chunk_path, &dest_chunk_path) {
                Ok(_) => println!("✅ Fetched chunk {}", chunk_filename),
                Err(e) => {
                    println!("❌ Failed to fetch chunk {}: {}", chunk_filename, e);
                    continue;
                }
            }
        } else {
            println!("🗂️ Already have chunk {}", chunk_filename);
        }

        let encrypted_chunk = fs::read(&dest_chunk_path).expect("Failed to read encrypted chunk");
        let decrypted_contents = encrypt::decrypt(&key, &encrypted_chunk).unwrap();

        let output_file_path = Path::new(&file_entry.path);
        if let Some(parent) = output_file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let mut output_file = fs::File::create(output_file_path).unwrap();
        output_file.write_all(&decrypted_contents).unwrap();

        println!("✅ Restored file {}", output_file_path.display());
    }

    // 🛡 Step 7: Save plaintext latest_manifest.json
    let local_manifest_path = Path::new(".arc/state/latest_manifest.json");
    let serialized_manifest =
        serde_json::to_string_pretty(&manifest).expect("Failed to serialize manifest");
    fs::write(&local_manifest_path, serialized_manifest).expect("Failed to save latest manifest");
    println!("✅ Saved local manifest for future lock.");

    println!("🎉 Pull complete.");
}

/// Helper to initialize .arc/ directory structure
fn initialize_local_arc_structure() {
    let arc_dir = Path::new(".arc");

    if !arc_dir.exists() {
        println!("🛠 No local .arc/ found. Initializing...");
        fs::create_dir(".arc").expect("Failed to create .arc/");
    }

    let chunks_dir = arc_dir.join("state/chunks");
    if !chunks_dir.exists() {
        fs::create_dir_all(&chunks_dir).expect("Failed to create state/chunks/");
    }

    let history_dir = arc_dir.join("history");
    if !history_dir.exists() {
        fs::create_dir_all(&history_dir).expect("Failed to create history/");
    }
}
