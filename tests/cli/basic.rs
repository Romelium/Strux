//! Basic CLI command tests (help, version, default output).

use crate::common::get_cmd; // Use common helper from the same test module
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::env;
// Import setup_temp_dir from the top-level tests/common
use crate::common::setup_temp_dir;

#[test]
fn test_cli_help() {
    let mut cmd = get_cmd();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: markdown_processor"))
        .stdout(predicate::str::contains("[OPTIONS] <MARKDOWN_FILE>"))
        .stdout(predicate::str::contains("--output-dir <DIR>"))
        .stdout(predicate::str::contains("--force"));
}

#[test]
fn test_cli_version() {
    let mut cmd = get_cmd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_default_output_dir() {
    let temp_dir = setup_temp_dir(); // Use temp dir for isolation
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(temp_dir.path()).unwrap(); // Change CWD

    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("## File: default_out.txt\n```\ndefault\n```")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.arg(md_path.path()); // No -o argument

    // Expect output in ./project-generated relative to CWD (temp_dir)
    let default_output_dir = temp_dir.child("project-generated");

    cmd.assert().success();

    default_output_dir.assert(predicate::path::is_dir());
    default_output_dir
        .child("default_out.txt")
        .assert("default\n");

    // Cleanup: change back to original directory
    env::set_current_dir(original_dir).unwrap();
}
