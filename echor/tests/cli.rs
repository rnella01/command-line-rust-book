use assert_cmd::Command;
use predicates::prelude::*;
// use std::fs;
use std::process::Command as std_Command;
use std::str;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn fails_with_no_args() {
    let mut command = Command::cargo_bin("echor").unwrap();
    command
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs_with_one_arg() {
    let mut command = Command::cargo_bin("echor").unwrap();
    command.arg("hello").assert().success();
}

#[test]
fn runs_with_optional_arg() {
    let mut command = Command::cargo_bin("echor").unwrap();
    command.arg("-n").arg("hello").assert().success();
}

#[test]
fn fails_with_only_optional_arg() {
    let mut command = Command::cargo_bin("echor").unwrap();
    command.arg("-n").assert().failure();
}

#[test]
fn matches_native_echo_command() -> TestResult {
    let mut echo_command = std_Command::new("echo");
    // echo_command.arg("Hello there");
    echo_command.arg("Hello").arg("there");
    let echo_command_output = echo_command.output().unwrap();
    let expected_str = str::from_utf8(&echo_command_output.stdout).unwrap();
    let mut echor_command = Command::cargo_bin("echor")?;
    echor_command
        .arg("Hello there")
        .assert()
        .success()
        .stdout(predicate::eq(expected_str));
    Ok(())
}

fn run_commands_and_compare(args: &[&str]) -> TestResult {
    let mut native_command = std_Command::new("echo");
    let native_command_output = native_command.args(args).output().unwrap();
    let expected_str = str::from_utf8(&native_command_output.stdout).unwrap();
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(expected_str));
    Ok(())
}

#[test]
fn matches_native_echo_command_with_args() -> TestResult {
    run_commands_and_compare(&["hello", "world"])
}

#[test]
fn matches_native_echo_command_with_optional_args_1() -> TestResult {
    run_commands_and_compare(&["-n", "hello", "world"])
}

#[test]
fn matches_native_echo_command_with_optional_args_2() -> TestResult {
    run_commands_and_compare(&["-n", "hello world"])
}
