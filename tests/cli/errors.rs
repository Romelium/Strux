//! CLI tests for various error conditions.

use super::common::get_cmd;
// Import setup_temp_dir from the top-level tests/test_common.rs
use crate::test_common::setup_temp_dir;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_missing_input_file() {
    let temp_dir = setup_temp_dir();
    let mut cmd = get_cmd();
    cmd.arg(temp_dir.path().join("nonexistent.md")); // Path that doesn't exist

    // Combine predicates to handle both POSIX and Windows error messages
    let error_message_predicate = predicate::str::contains("Error: I/O error:") // Common part
        .and(
            // Must contain common part AND...
            predicate::str::contains("No such file or directory") // ...either the POSIX message
                .or(predicate::str::contains("cannot find the file specified")), // ...or the Windows message
        );

    cmd.assert().failure().stderr(error_message_predicate); // Use the combined predicate
}

#[test]
fn test_cli_output_dir_is_file() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("## File: test.txt\n```\ncontent\n```")
        .unwrap();
    let output_path = temp_dir.child("output_is_file.txt");
    output_path.write_str("I am a file").unwrap();

    let mut cmd = get_cmd();
    cmd.arg(md_path.path()).arg("-o").arg(output_path.path());

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: Argument error: Base path is not a directory",
    ));
}

#[test]
fn test_cli_unsafe_path() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## File: ../unsafe_file.txt\n```\nShould not be created\n```\n")
        .unwrap();
    let output_dir = temp_dir.child("generated"); // Does not exist initially

    let mut cmd = get_cmd();
    cmd.arg(md_path.path()).arg("-o").arg(output_dir.path());

    cmd.assert()
        .success() // The process itself succeeds but skips the unsafe action
        .stdout(predicate::str::contains(
            "Failed (unsafe/invalid path):       1",
        ));

    // Check the file wasn't created in the parent dir
    let parent_dir = temp_dir.path().parent().unwrap();
    assert!(!parent_dir.join("unsafe_file.txt").exists());
    // Check it wasn't created inside the output dir either
    output_dir
        .child("unsafe_file.txt")
        .assert(predicate::path::missing());
}
