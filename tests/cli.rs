// File: tests/cli.rs

//! End-to-end tests for the command-line interface.

// Declare the common module defined in tests/common/mod.rs
mod common;
use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use common::setup_temp_dir;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

fn get_cmd() -> Command {
    Command::cargo_bin("markdown_processor").expect("Failed to find binary")
}

#[test]
fn test_cli_help() {
    let mut cmd = get_cmd();
    cmd.arg("--help");
    cmd.assert()
        .success()
        // Make assertion more general for cross-platform compatibility (.exe)
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
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION"))); // Check for current version
}

#[test]
fn test_cli_basic_create() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str(
            r#"
## File: output/hello.txt
```
Hello from CLI!
```
"#,
        )
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

#[test]
fn test_cli_basic_delete() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str(
            r#"
**Deleted File: old_stuff.log**
"#,
        )
        .unwrap();
    let output_dir = temp_dir.child("generated");
    output_dir.create_dir_all().unwrap(); // Pre-create for delete test
    output_dir
        .child("old_stuff.log")
        .write_str("delete me")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.arg(md_path.path()).arg("-o").arg(output_dir.path());

    cmd.assert().success().stdout(predicate::str::contains(
        "Files deleted:                      1",
    ));

    output_dir
        .child("old_stuff.log")
        .assert(predicate::path::missing());
}

#[test]
fn test_cli_overwrite_force() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str(
            r#"
## File: config.ini
```
value = new
```
"#,
        )
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
        .write_str(
            r#"
## File: config.ini
```
value = new
```
"#,
        )
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

#[test]
fn test_cli_missing_input_file() {
    let temp_dir = setup_temp_dir();
    let mut cmd = get_cmd();
    cmd.arg(temp_dir.path().join("nonexistent.md")); // Path that doesn't exist

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: I/O error:"))
        // Adjust assertion for Windows error message fragment
        .stderr(predicate::str::contains("cannot find the file specified"));
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
        .write_str(
            r#"
## File: ../unsafe_file.txt
```
Should not be created
```
"#,
        )
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

#[test]
fn test_cli_default_output_dir() {
    let temp_dir = setup_temp_dir(); // Use temp dir for isolation
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap(); // Change CWD to temp dir

    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("## File: default_out.txt\n```\ndefault\n```")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.arg(md_path.path()); // No -o argument

    // Expect output in ./project-generated relative to CWD (which is temp_dir)
    let default_output_dir = temp_dir.child("project-generated"); // Does not exist initially

    cmd.assert().success();

    default_output_dir.assert(predicate::path::is_dir());
    default_output_dir
        .child("default_out.txt")
        .assert("default\n");

    // Cleanup: change back to original directory
    std::env::set_current_dir(original_dir).unwrap();
}
