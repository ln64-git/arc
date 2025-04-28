use std::fs;

pub fn run() {
    println!("Restoring Archive...");
    for entry in fs::read_dir(".Arc/state/chunks").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            println!("Found chunk: {}", path.display());
        }
    }
}
