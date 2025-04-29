use assert_cmd::Command;

#[test]
fn test_arc_init_creates_structure() {
    let mut cmd = Command::cargo_bin("arc").unwrap();
    let assert = cmd.arg("init").assert();
    assert.success();
}
