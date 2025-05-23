//! Tests for processing mixed actions (create, delete, overwrite) in one run.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir_with_files};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_create_and_delete_mixed() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("to_delete_1.log", "delete me"),
        ("data/to_delete_2.tmp", "more deletable data"),
    ]);

    let md = "\n## File: new_script.py\n```python\nprint(\"Hello\")\n```\n\n**Deleted File: to_delete_1.log**\n\n`config.toml`\n```toml\nenabled = true\n```\n\n## Deleted File: data/to_delete_2.tmp\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    // Check created files
    temp_dir.child("new_script.py").assert("print(\"Hello\")\n");
    temp_dir.child("config.toml").assert("enabled = true\n");

    // Check deleted files
    temp_dir
        .child("to_delete_1.log")
        .assert(predicate::path::missing());
    temp_dir
        .child("data/to_delete_2.tmp")
        .assert(predicate::path::missing());

    assert_summary(
        &summary, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_create_and_overwrite_mixed_force() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("config.ini", "[old]\nvalue=1"),
        ("data/params.json", "{\"old\": true}"),
    ]);

    let md = "\n## File: main.go\n```go\npackage main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Go!\") }\n```\n\n**File: config.ini**\n```ini\n[new]\nvalue = 2\n```\n\n`data/params.json`\n```json\n{\n  \"new\": true,\n  \"overwritten\": true\n}\n```\n\n## File: another_new.txt\n```\nJust another file.\n```\n";

    // Run with overwrite = true
    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed");

    // Check new files
    temp_dir
        .child("main.go")
        .assert("package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Go!\") }\n");
    temp_dir
        .child("another_new.txt")
        .assert("Just another file.\n");

    // Check overwritten files
    temp_dir.child("config.ini").assert("[new]\nvalue = 2\n");
    temp_dir
        .child("data/params.json")
        .assert("{\n  \"new\": true,\n  \"overwritten\": true\n}\n");

    assert_summary(
        &summary, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_create_overwrite_delete_complex() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("app.cfg", "old_config"),      // To be overwritten
        ("logs/yesterday.log", "log1"), // To be deleted
    ]);

    let md = "\n## File: src/mod.rs\n```rust\n// Module file\n```\n\n**File: app.cfg**\n```\nnew_config\n```\n\n## Deleted File: logs/yesterday.log\n\n`tests/run.sh`\n```bash\n#!/bin/bash\necho \"Testing...\"\n```\n\n**Deleted File: non_existent.tmp**\n";

    // Run with overwrite = true
    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed");

    // Check created files
    temp_dir.child("src/mod.rs").assert("// Module file\n");
    temp_dir
        .child("tests/run.sh")
        .assert("#!/bin/bash\necho \"Testing...\"\n");

    // Check overwritten file
    temp_dir.child("app.cfg").assert("new_config\n");

    // Check deleted file
    temp_dir
        .child("logs/yesterday.log")
        .assert(predicate::path::missing());

    // Check non-existent file (still missing)
    temp_dir
        .child("non_existent.tmp")
        .assert(predicate::path::missing());

    assert_summary(
        &summary, 2, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_create_delete_move_complex() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("app.cfg", "old_config_for_move"), // To be moved
        ("logs/today.log", "log_content"),  // To be deleted
        ("config/target_for_overwrite.ini", "initial_target_content"), // To be overwritten by move
    ]);

    let md = "\n## File: src/main.rs\n```rust\nfn main() {}\n```\n\n## Moved File: app.cfg to config/new_app.cfg\n\n**Deleted File: logs/today.log**\n\n## Moved File: `config/target_for_overwrite.ini` to `config/target_for_overwrite.ini`\n\n`docs/README.template`\n```\nTemplate content\n```\n";

    // Run with overwrite = true
    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed");

    // Check created files
    temp_dir.child("src/main.rs").assert("fn main() {}\n");
    temp_dir
        .child("docs/README.template")
        .assert("Template content\n");

    // Check moved file (app.cfg -> config/new_app.cfg)
    temp_dir.child("app.cfg").assert(predicate::path::missing()); // Original gone
    temp_dir
        .child("config/new_app.cfg")
        .assert("old_config_for_move"); // New one exists

    // Check deleted file
    temp_dir
        .child("logs/today.log")
        .assert(predicate::path::missing());

    // Check the move-to-self-overwrite case
    temp_dir
        .child("config/target_for_overwrite.ini")
        .assert("initial_target_content");

    // Expected: 2 created, 1 deleted, 1 moved (app.cfg), 1 moved_overwritten (target_for_overwrite.ini to itself with --force)
    assert_summary(
        &summary, 2, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_create_append_prepend_delete_move_complex() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("config.txt", "Initial config\n"),
        ("app.log", "Log started\n"),
        ("old_file.txt", "To be moved"),
        ("to_delete.txt", "Delete this"),
    ]);

    let md = "\n## File: new_file.txt\n```\nNew file content\n```\n\n## Append File: app.log\n```\nLog entry 1\n```\n\n## Prepend File: config.txt\n```\n# Auto-generated header\n```\n\n## Moved File: old_file.txt to archive/moved_file.txt\n\n**Deleted File: to_delete.txt**\n\n## Append File: app.log\n```\nLog entry 2\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    // Create
    temp_dir.child("new_file.txt").assert("New file content\n");
    // Append (twice)
    temp_dir
        .child("app.log")
        .assert("Log started\nLog entry 1\nLog entry 2\n");
    // Prepend
    temp_dir
        .child("config.txt")
        .assert("# Auto-generated header\nInitial config\n");
    // Move
    temp_dir
        .child("old_file.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("archive/moved_file.txt")
        .assert("To be moved");
    // Delete
    temp_dir
        .child("to_delete.txt")
        .assert(predicate::path::missing());

    // Summary: 1 created, 2 appended, 1 prepended, 1 moved, 1 deleted
    assert_summary(
        &summary, 1, 0, 1, 1, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}
