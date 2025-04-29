use crate::types::HistoryEntry;
use std::fs;
use std::path::Path;

pub fn run(full: bool) {
    let manifest_path = Path::new(".arc/state/latest_manifest.json");

    if !manifest_path.exists() {
        println!("⚠️ No local manifest found at .arc/state/latest_manifest.json");
        println!("ℹ️ Try running `arc pull <source>` to fetch a manifest first.");
        return;
    }

    let manifest_data =
        fs::read_to_string(manifest_path).expect("❌ Failed to read local manifest");

    let manifest: HistoryEntry =
        serde_json::from_str(&manifest_data).expect("❌ Manifest JSON invalid");

    println!("📜 Files in ARC:");

    for file in manifest.files {
        if full {
            println!(" — 📄 {}  —  hash: {}", file.path, file.hash);
        } else {
            println!(" — 📄 {}", file.path);
        }
    }
}
