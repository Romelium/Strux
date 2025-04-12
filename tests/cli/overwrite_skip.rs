//! CLI tests for overwrite (--force) and skip logic.

use super::common::get_cmd;
// Import setup_temp_dir from the top-level tests/test_common.rs
use crate::test_common::setup_temp_dir;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_overwrite_force() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## File: config.ini\n```\nvalue = new\n```\n")
        .unwrap();
    let output_dir = temp_dir.child("generated");
    output_dir.create_dir_all().unwrap(); // Pre-create for overwrite test
    output_dir
        .child("config.ini")
        .write_str("value = old")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.arg(md_path.path())
        .arg("-o")
        .arg(output_dir.path())
        .arg("--force"); // Use -f or --force

    cmd.assert().success().stdout(predicate::str::contains(
        "Files overwritten (--force):        1",
    ));

    output_dir.child("config.ini").assert("value = new\n");
}

#[test]
fn test_cli_skip_existing_no_force() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## File: config.ini\n```\nvalue = new\n```\n")
        .unwrap();
    let output_dir = temp_dir.child("generated");
    output_dir.create_dir_all().unwrap(); // Pre-create for skip test
    output_dir
        .child("config.ini")
        .write_str("value = old")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.arg(md_path.path()).arg("-o").arg(output_dir.path()); // No --force

    cmd.assert().success().stdout(predicate::str::contains(
        "Skipped (create, exists):           1",
    ));

    output_dir.child("config.ini").assert("value = old"); // Unchanged
}
