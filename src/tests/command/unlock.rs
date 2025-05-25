use crate::command::unlock;
use crate::utility::setup_fake_arc::setup_fake_arc_unlock_manifest;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_unlock_restores_file() {
    let temp = tempdir().unwrap();
    setup_fake_arc_unlock_manifest(temp.path());
    std::env::set_current_dir(temp.path()).unwrap();

    unlock::run(None);

    let out_path = Path::new("restored.txt");
    assert!(out_path.exists(), "Restored file not found");

    let contents =
        fs::read_to_string(out_path).expect(&format!("Failed to read output file: {:?}", out_path));
    assert_eq!(contents, "restored contents");
}
