use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_arc_init() {
    let temp = tempdir().unwrap();
    let arc_dir = temp.path().join(".arc");

    let mut cmd = Command::cargo_bin("arc").unwrap();
    cmd.current_dir(&temp).arg("init").assert().success();

    assert!(arc_dir.exists());
    assert!(arc_dir.join("config.json").exists());
    assert!(arc_dir.join("secret.key").exists());
}

#[test]
fn test_arc_add_commit() {
    let temp = tempdir().unwrap();
    let test_file = temp.path().join("hello.txt");
    fs::write(&test_file, "Hello World!").unwrap();

    // Initialize
    Command::cargo_bin("arc")
        .unwrap()
        .current_dir(&temp)
        .arg("init")
        .assert()
        .success();

    // Add file
    Command::cargo_bin("arc")
        .unwrap()
        .current_dir(&temp)
        .arg("add")
        .arg("hello.txt")
        .assert()
        .success();

    // Commit
    Command::cargo_bin("arc")
        .unwrap()
        .current_dir(&temp)
        .arg("commit")
        .assert()
        .success();

    // Check that chunks and staging are manipulated
    let chunks_dir = temp.path().join(".arc/state/chunks");
    assert!(
        chunks_dir.read_dir().unwrap().next().is_some(),
        "Chunks should exist"
    );
}
