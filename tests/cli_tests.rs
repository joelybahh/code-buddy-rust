use assert_cmd::Command;

#[test]
fn test_cli_commit() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    let assert = cmd.arg("commit").assert();
    assert.success();
}
