use crate::types::HistoryEntry;
use crate::utility::encrypt;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn run(source_arc_path: &str) {
    println!("🔄 Pulling from ARC at {}", source_arc_path);

    let source_arc = Path::new(source_arc_path).join(".arc");
    let dest_arc = Path::new(".arc");

    initialize_arc_structure();

    copy_if_missing(
        &source_arc.join("secret.key"),
        &dest_arc.join("secret.key"),
        "secret.key",
    );
    copy_if_missing(
        &source_arc.join("config.json"),
        &dest_arc.join("config.json"),
        "config.json",
    );

    let manifest_path = source_arc.join("history/latest.json");
    if !manifest_path.exists() {
        println!("⚠️ No latest.json found. Nothing to pull.");
        return;
    }

    let key = encrypt::load_secret_key();
    let manifest_bytes = encrypt::decrypt_from_file(&key, &manifest_path);
    let manifest: HistoryEntry = serde_json::from_slice(&manifest_bytes).expect("Invalid manifest");

    pull_chunks_and_restore_files(
        &manifest,
        &source_arc.join("state/chunks"),
        &dest_arc.join("state/chunks"),
        &key,
    );

    save_manifest_plaintext(&manifest);
    println!("🎉 Pull complete.");
}

fn initialize_arc_structure() {
    for dir in [".arc", ".arc/state/chunks", ".arc/history"] {
        fs::create_dir_all(dir).expect(&format!("Failed to create {}", dir));
    }
    println!("🛠 Initialized ARC directory structure.");

    // Secret Key Cleanup
    let base_key = Path::new("secret.key");
    if base_key.exists() {
        println!("🧹 Detected duplicate secret.key in base folder. Removing redundant copy...");
        fs::remove_file(base_key).expect("Failed to delete redundant secret.key in base dir");
        println!("✅ Removed duplicate secret.key from base directory.");
    }
}

fn copy_if_missing(src: &Path, dst: &Path, label: &str) {
    if src.exists() && !dst.exists() {
        fs::copy(src, dst).expect(&format!("Failed to copy {}", label));
        println!("✅ Copied {}", label);
    } else if !src.exists() {
        println!("⚠️ Warning: {} not found in source ARC.", label);
    }
}

fn pull_chunks_and_restore_files(
    manifest: &HistoryEntry,
    source_chunks: &Path,
    dest_chunks: &Path,
    key: &aes_gcm::Aes256Gcm,
) {
    for file_entry in &manifest.files {
        let chunk = &file_entry.hash;
        let src_chunk = source_chunks.join(chunk);
        let dst_chunk = dest_chunks.join(chunk);

        if !dst_chunk.exists() {
            match fs::copy(&src_chunk, &dst_chunk) {
                Ok(_) => println!("✅ Fetched chunk {}", chunk),
                Err(e) => {
                    println!("❌ Failed to fetch chunk {}: {}", chunk, e);
                    continue;
                }
            }
        } else {
            println!("🗂️ Already have chunk {}", chunk);
        }

        let encrypted_chunk = fs::read(&dst_chunk).expect("Failed to read chunk");
        let decrypted = encrypt::decrypt(key, &encrypted_chunk).expect("Failed to decrypt chunk");

        let output_path = Path::new(&file_entry.path);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let mut output = fs::File::create(output_path).unwrap();
        output.write_all(&decrypted).unwrap();
        println!("✅ Restored {}", output_path.display());
    }
}

fn save_manifest_plaintext(manifest: &HistoryEntry) {
    let manifest_path = Path::new(".arc/state/latest_manifest.json");
    let serialized = serde_json::to_string_pretty(manifest).expect("Failed to serialize manifest");
    fs::write(&manifest_path, serialized).expect("Failed to write local manifest");
    println!("✅ Saved local plaintext manifest.");
}
