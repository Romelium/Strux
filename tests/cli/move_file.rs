//! CLI tests related to file moving.

use super::common::get_cmd;
use crate::test_common::setup_temp_dir; // For setting up temp CWD
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_basic_move() {
    let temp_dir = setup_temp_dir(); // This will be our CWD for the test
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Moved File: old_name.txt to new_name.txt\n")
        .unwrap();

    let output_dir_name = "generated_output";
    let output_dir = temp_dir.child(output_dir_name); // Relative to temp_dir (CWD)

    // Create the source file inside the output directory that will be targeted
    output_dir.create_dir_all().unwrap();
    output_dir
        .child("old_name.txt")
        .write_str("content to move")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path()); // Run `strux` from within temp_dir
    cmd.arg(md_path.path()) // Path to markdown file (now relative to CWD)
        .arg("-o")
        .arg(output_dir_name); // Output dir relative to CWD

    cmd.assert().success().stdout(predicate::str::contains(
        "Files moved:                        1",
    ));

    output_dir
        .child("old_name.txt")
        .assert(predicate::path::missing());
    output_dir.child("new_name.txt").assert("content to move");
}

#[test]
fn test_cli_move_overwrite_dest_with_force() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Moved File: source.txt to dest.txt\n")
        .unwrap();

    let output_dir_name = "generated_output";
    let output_dir = temp_dir.child(output_dir_name);
    output_dir.create_dir_all().unwrap();
    output_dir
        .child("source.txt")
        .write_str("new hotness")
        .unwrap();
    output_dir
        .child("dest.txt")
        .write_str("old and busted")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path())
        .arg("-o")
        .arg(output_dir_name)
        .arg("--force");

    cmd.assert().success().stdout(predicate::str::contains(
        "Files moved (dest overwritten):     1",
    ));

    output_dir
        .child("source.txt")
        .assert(predicate::path::missing());
    output_dir.child("dest.txt").assert("new hotness");
}

#[test]
fn test_cli_move_skip_dest_exists_no_force() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Moved File: source.txt to dest.txt\n")
        .unwrap();

    let output_dir_name = "generated_output";
    let output_dir = temp_dir.child(output_dir_name);
    output_dir.create_dir_all().unwrap();
    output_dir
        .child("source.txt")
        .write_str("source data")
        .unwrap();
    output_dir
        .child("dest.txt")
        .write_str("destination data")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path()).arg("-o").arg(output_dir_name); // No --force

    cmd.assert().success().stdout(predicate::str::contains(
        "Skipped (move, dest exists):        1",
    ));

    output_dir.child("source.txt").assert("source data"); // Unchanged
    output_dir.child("dest.txt").assert("destination data"); // Unchanged
}

#[test]
fn test_cli_move_source_not_found() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Moved File: missing_source.txt to any_dest.txt\n")
        .unwrap();

    let output_dir_name = "generated_output";
    // output_dir doesn't need to exist for this test if source is missing

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path()).arg("-o").arg(output_dir_name);

    cmd.assert().success().stdout(predicate::str::contains(
        "Skipped (move, src not found):      1",
    ));

    temp_dir
        .child(output_dir_name)
        .child("any_dest.txt")
        .assert(predicate::path::missing());
}

#[test]
fn test_cli_move_dest_is_dir() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Moved File: source_file.txt to dest_dir\n")
        .unwrap();

    let output_dir_name = "generated_output";
    let output_dir = temp_dir.child(output_dir_name);
    output_dir.create_dir_all().unwrap();
    output_dir
        .child("source_file.txt")
        .write_str("file content")
        .unwrap();
    output_dir.child("dest_dir").create_dir_all().unwrap(); // Dest is a directory

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path()).arg("-o").arg(output_dir_name);

    cmd.assert().success().stdout(predicate::str::contains(
        "Skipped (move, dest is dir):        1",
    ));

    output_dir.child("source_file.txt").assert("file content"); // Source remains
    output_dir
        .child("dest_dir")
        .assert(predicate::path::is_dir()); // Dest dir remains
}
