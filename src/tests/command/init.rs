use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_arc_init_creates_structure() {
    let dir = tempdir().unwrap();
    env::set_current_dir(&dir).unwrap();

    let mut cmd = Command::cargo_bin("arc").unwrap();
    cmd.arg("init").assert().success();

    assert!(dir.path().join(".arc/state/chunks").exists());
}
