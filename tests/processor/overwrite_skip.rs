//! Tests for processor overwrite and skip logic.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_create_skip_existing_no_force() {
    let temp_dir = setup_temp_dir();
    temp_dir
        .child("existing.txt")
        .write_str("Original")
        .unwrap();
    let md = "\n## File: existing.txt\n```\nNew content, should be ignored\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed"); // overwrite = false

    temp_dir.child("existing.txt").assert("Original");
    assert_summary(&summary, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_create_overwrite_existing_with_force() {
    let temp_dir = setup_temp_dir();
    temp_dir
        .child("existing.txt")
        .write_str("Original")
        .unwrap();
    let md = "\n## File: existing.txt\n```\nNew content, should overwrite\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed"); // overwrite = true

    temp_dir
        .child("existing.txt")
        .assert("New content, should overwrite\n");
    assert_summary(&summary, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_delete_skip_not_found() {
    let temp_dir = setup_temp_dir();
    let md = "\n**Deleted File: missing.txt**\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("missing.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_delete_skip_is_directory() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("a_directory").create_dir_all().unwrap();
    let md = "\n## Deleted File: a_directory\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("a_directory")
        .assert(predicate::path::is_dir());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0);
}
