use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_arc_add_commit() {
    let temp = tempdir().unwrap();
    let test_file = temp.path().join("hello.txt");

    fs::write(&test_file, b"hello").unwrap();

    Command::cargo_bin("arc")
        .unwrap()
        .current_dir(temp.path())
        .arg("init")
        .assert()
        .success();

    Command::cargo_bin("arc")
        .unwrap()
        .current_dir(temp.path())
        .args(&["add", "hello.txt"])
        .assert()
        .success();
}
