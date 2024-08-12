use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_execution() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("humor.yaml");

    let config_content = r#"
    commands:
      echo:
        hello:
          world: echo "Hello, World!"
    "#;
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = Command::cargo_bin("humor").unwrap();
    cmd.arg("-c")
        .arg(config_path)
        .arg("echo")
        .arg("hello")
        .arg("world");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn test_cli_invalid_command() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("humor.yaml");

    let config_content = r#"
    commands:
      echo:
        hello:
          world: echo "Hello, World!"
    "#;
    fs::write(&config_path, config_content).unwrap();

    let mut cmd = Command::cargo_bin("humor").unwrap();
    cmd.arg("-c").arg(config_path).arg("invalid").arg("command");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Command not found"));
}
