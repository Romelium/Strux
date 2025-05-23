//! CLI tests related to file appending and prepending.

use super::common::get_cmd;
use crate::test_common::setup_temp_dir; // For setting up temp CWD
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_basic_append() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Append File: data.log\n```\nAppended line.\n```\n")
        .unwrap();

    let output_dir_name = "generated_output";
    let output_dir = temp_dir.child(output_dir_name);

    // Create the initial file
    output_dir.create_dir_all().unwrap();
    output_dir
        .child("data.log")
        .write_str("Initial line.\n")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path()).arg("-o").arg(output_dir_name);

    cmd.assert().success().stdout(predicate::str::contains(
        "Files appended:                     1",
    ));

    output_dir
        .child("data.log")
        .assert("Initial line.\nAppended line.\n");
}

#[test]
fn test_cli_append_creates_new_file() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Append File: new_log.txt\n```\nFirst entry.\n```\n")
        .unwrap();

    let output_dir_name = "generated_output";
    // output_dir.create_dir_all().unwrap(); // Don't create output_dir, let strux do it

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path()).arg("-o").arg(output_dir_name);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Files created:                      1", // Appending to non-existent creates it
        ))
        .stdout(predicate::str::contains(
            "Files appended:                     0", // Not counted as append if created
        ));

    temp_dir
        .child(output_dir_name)
        .child("new_log.txt")
        .assert("First entry.\n");
}

#[test]
fn test_cli_basic_prepend() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Prepend File: config.ini\n```\n# Header\n```\n")
        .unwrap();

    let output_dir_name = "generated_output";
    let output_dir = temp_dir.child(output_dir_name);
    output_dir.create_dir_all().unwrap();
    output_dir
        .child("config.ini")
        .write_str("key=value\n")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path()).arg("-o").arg(output_dir_name);

    cmd.assert().success().stdout(predicate::str::contains(
        "Files prepended:                    1",
    ));

    output_dir
        .child("config.ini")
        .assert("# Header\nkey=value\n");
}

#[test]
fn test_cli_prepend_creates_new_file() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Prepend File: new_script.sh\n```\n#!/bin/bash\n```\n")
        .unwrap();

    let output_dir_name = "generated_output";

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path()).arg("-o").arg(output_dir_name);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Files created:                      1", // Prepending to non-existent creates it
        ))
        .stdout(predicate::str::contains(
            "Files prepended:                    0", // Not counted as prepend if created
        ));

    temp_dir
        .child(output_dir_name)
        .child("new_script.sh")
        .assert("#!/bin/bash\n");
}

#[test]
fn test_cli_append_target_is_dir() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n## Append File: a_dir\n```\ncontent\n```\n")
        .unwrap();

    let output_dir_name = "generated_output";
    let output_dir = temp_dir.child(output_dir_name);
    output_dir.create_dir_all().unwrap();
    output_dir.child("a_dir").create_dir_all().unwrap(); // Create 'a_dir' as a directory

    let mut cmd = get_cmd();
    cmd.current_dir(temp_dir.path());
    cmd.arg(md_path.path()).arg("-o").arg(output_dir_name);

    cmd.assert().success().stdout(predicate::str::contains(
        "Failed (append, target is dir):     1",
    ));
}
