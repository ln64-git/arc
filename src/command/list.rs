use crate::types::HistoryEntry;
use std::fs;
use std::path::Path;

/// Lists all files in the ARC manifest.
pub fn run() {
    let manifest_path = Path::new(".arc/state/latest_manifest.json");

    if !manifest_path.exists() {
        println!("⚠️ No local manifest found at .arc/state/latest_manifest.json");
        println!("ℹ️ Try running `arc pull <source>` to fetch a manifest first.");
        return;
    }

    let manifest_contents = match fs::read_to_string(&manifest_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("❌ Failed to read manifest: {}", e);
            return;
        }
    };

    let manifest: HistoryEntry = match serde_json::from_str(&manifest_contents) {
        Ok(manifest) => manifest,
        Err(e) => {
            eprintln!("❌ Invalid manifest format: {}", e);
            return;
        }
    };

    println!("📜 Files in ARC:");
    for file in manifest.files {
        println!("📄 {}", file.path);
    }
}
