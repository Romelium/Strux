//! CLI tests related to file creation.

use crate::common::get_cmd;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
// Import setup_temp_dir from the top-level tests/common
use crate::common::setup_temp_dir;

#[test]
fn test_cli_basic_create() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## File: output/hello.txt\n```\nHello from CLI!\n```\n")
        .unwrap();
    let output_dir = temp_dir.child("generated"); // Does not exist initially

    let mut cmd = get_cmd();
    cmd.arg(md_path.path()).arg("-o").arg(output_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Processing Summary:"))
        .stdout(predicate::str::contains(
            "Files created:                      1",
        ));

    output_dir.assert(predicate::path::is_dir());
    output_dir.child("output").assert(predicate::path::is_dir());
    output_dir
        .child("output/hello.txt")
        .assert("Hello from CLI!\n");
}
