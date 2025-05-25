use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_arc_init_creates_structure() {
    let dir = tempdir().unwrap();
    let arc_path = dir.path().join(".arc/state/chunks");

    let mut cmd = Command::cargo_bin("arc").unwrap();
    cmd.current_dir(&dir); // <-- set working directory for the command
    cmd.arg("init").assert().success();

    assert!(arc_path.exists());
}
