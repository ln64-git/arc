use crate::{command::pull, utility::setup_fake_arc::setup_fake_arc_pull_source};
use std::{
    fs::{self},
    path::Path,
};
use tempfile::tempdir;

#[test]
fn test_pull_manifest_copies_chunks() {
    let temp_src = tempdir().unwrap();
    let temp_dest = tempdir().unwrap();

    // Setup .arc in source
    setup_fake_arc_pull_source(temp_src.path());

    // Set cwd to destination dir
    std::env::set_current_dir(temp_dest.path()).unwrap();

    // Pull from source ARC
    pull::run(temp_src.path().to_str().unwrap(), Some(".".to_string()));

    let out_path = Path::new("recovered.txt");
    assert!(out_path.exists(), "Pulled file not restored");

    let contents = fs::read_to_string(out_path).unwrap();
    assert_eq!(contents, "restored contents");
}
