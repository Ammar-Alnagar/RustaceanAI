use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_cli_openai() {
    let mut cmd = Command::cargo_bin("rust_ai").unwrap();
    cmd.arg("cli")
        .arg("--provider")
        .arg("openai")
        .arg("--prompt")
        .arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is a response from OpenAI."));
}

#[test]
fn test_cli_gemini() {
    let mut cmd = Command::cargo_bin("rust_ai").unwrap();
    cmd.arg("cli")
        .arg("--provider")
        .arg("gemini")
        .arg("--prompt")
        .arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is a response from Gemini."));
}

#[test]
fn test_cli_openrouter() {
    let mut cmd = Command::cargo_bin("rust_ai").unwrap();
    cmd.arg("cli")
        .arg("--provider")
        .arg("openrouter")
        .arg("--prompt")
        .arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is a response from OpenRouter."));
}

#[test]
fn test_cli_huggingface() {
    let mut cmd = Command::cargo_bin("rust_ai").unwrap();
    cmd.arg("cli")
        .arg("--provider")
        .arg("huggingface")
        .arg("--prompt")
        .arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is a response from HuggingFace."));
}

#[test]
fn test_cli_local() {
    let mut cmd = Command::cargo_bin("rust_ai").unwrap();
    cmd.arg("cli")
        .arg("--provider")
        .arg("local")
        .arg("--prompt")
        .arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is a response from a local model."));
}
