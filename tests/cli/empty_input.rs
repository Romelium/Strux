//! Tests for CLI handling of empty or non-actionable input files.

use super::common::get_cmd;
// Import setup_temp_dir from the top-level tests/test_common.rs
use crate::test_common::setup_temp_dir;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_empty_markdown_file() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("empty.md");
    md_path.write_str("").unwrap(); // Write an empty file

    let mut cmd = get_cmd();
    cmd.arg(md_path.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Info: No actionable content found.",
        ))
        .stdout(predicate::str::contains("No actions to process."))
        .stdout(predicate::str::contains(
            "Files created:                      0",
        )) // Check summary output
        .stdout(predicate::str::contains(
            "Files deleted:                      0",
        ))
        .stdout(predicate::str::contains(
            "Skipped (create, exists):           0",
        ))
        .stdout(predicate::str::contains(
            "Failed (unsafe/invalid path):       0",
        ));
}

#[test]
fn test_cli_markdown_file_no_actions() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("no_actions.md");
    md_path
        .write_str("# Just a Title\n\nSome regular markdown text.\n\n```rust\nlet x = 5;\n```\n")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.arg(md_path.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Warning: No valid actions extracted. Check formatting.", // Expect warning because ``` exists
        ))
        .stdout(predicate::str::contains("No actions to process."))
        .stdout(predicate::str::contains(
            "Files created:                      0",
        )) // Check summary output
        .stdout(predicate::str::contains(
            "Files deleted:                      0",
        ))
        .stdout(predicate::str::contains(
            "Skipped (create, exists):           0",
        ))
        .stdout(predicate::str::contains(
            "Failed (unsafe/invalid path):       0",
        ));
}
