use crate::types::HistoryEntry;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn run(source_arc_path: &str) {
    println!("Pulling from ARC at {}", source_arc_path);

    let source_arc_dir = Path::new(source_arc_path).join(".arc");
    let dest_arc_dir = Path::new(".arc");

    // 🛡 Ensure necessary directories exist
    let source_chunks_path = source_arc_dir.join("state/chunks");
    let dest_chunks_path = dest_arc_dir.join("state/chunks");

    if !dest_chunks_path.exists() {
        fs::create_dir_all(&dest_chunks_path).expect("Failed to create local chunks directory");
    }

    if !source_chunks_path.exists() {
        println!("Error: Source archive chunks path not found.");
        return;
    }

    // 🛡 Copy .arc/config.json
    let source_config = source_arc_dir.join("config.json");
    let dest_config = dest_arc_dir.join("config.json");
    if let Err(e) = fs::copy(&source_config, &dest_config) {
        println!("Warning: Failed to update config.json: {}", e);
    } else {
        println!("Updated config.json");
    }

    // 🛡 Load manifest
    let manifest_path = source_arc_dir.join("history/latest.json");

    let manifest_contents = match fs::read_to_string(&manifest_path) {
        Ok(contents) => contents,
        Err(_) => {
            println!("Error: Could not read manifest file at {:?}", manifest_path);
            return;
        }
    };

    let manifest: HistoryEntry = match serde_json::from_str(&manifest_contents) {
        Ok(m) => m,
        Err(_) => {
            println!("Error: Invalid manifest format.");
            return;
        }
    };

    // 🛡 For each file in manifest
    for file_entry in manifest.files {
        let chunk_filename = &file_entry.hash;
        let source_chunk_path = source_chunks_path.join(chunk_filename);
        let dest_chunk_path = dest_chunks_path.join(chunk_filename);

        // Copy chunk if missing
        if !dest_chunk_path.exists() {
            match fs::copy(&source_chunk_path, &dest_chunk_path) {
                Ok(_) => println!("Fetched chunk {}", chunk_filename),
                Err(e) => {
                    println!("Failed to copy chunk {}: {}", chunk_filename, e);
                    continue;
                }
            }
        } else {
            println!("Already have chunk {}", chunk_filename);
        }

        // Rebuild file under correct path + filename
        let output_file_path = Path::new(&file_entry.path);
        if let Some(parent) = output_file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let chunk_data = fs::read(&dest_chunk_path).expect("Failed to read chunk data");
        let mut output_file =
            fs::File::create(output_file_path).expect("Failed to create output file");
        output_file
            .write_all(&chunk_data)
            .expect("Failed to write file");

        println!("Restored file {}", output_file_path.display());
    }

    println!("Pull complete.");
}
