use std::process::Command as std_Command;
use assert_cmd::Command as assert_Command;

#[test]
fn runs_ls() {
    let mut command = std_Command::new("ls");
    let response = command.output();
    assert!(response.is_ok());
}

#[test]
fn runs_hello() {
    let mut command = assert_Command::cargo_bin("hello").unwrap();
    command.assert().success();
}

#[test]
fn hello_prints_hello() {
    let mut command = assert_Command::cargo_bin("hello").unwrap();
    command.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut command = assert_Command::cargo_bin("true").unwrap();
    command.assert().success();
}

#[test]
fn false_not_ok() {
    let mut command = assert_Command::cargo_bin("false").unwrap();
    command.assert().failure();
}