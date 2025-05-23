//! Tests for interactions between create and delete actions.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir, setup_temp_dir_with_files};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_create_then_delete() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: temp.txt\n```\nTemporary content\n```\n\n**Deleted File: temp.txt**\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("temp.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_delete_then_create() {
    let temp_dir = setup_temp_dir();
    temp_dir
        .child("recreate.txt")
        .write_str("Old content")
        .unwrap();
    let md = "\n## Deleted File: recreate.txt\n\n## File: recreate.txt\n```\nNew content\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("recreate.txt")
        .assert(predicate::path::is_file());
    temp_dir.child("recreate.txt").assert("New content\n");
    assert_summary(&summary, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_create_then_move() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: source.txt\n```\nContent to move\n```\n\n## Moved File: source.txt to dest.txt\n";
    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("source.txt")
        .assert(predicate::path::missing());
    temp_dir.child("dest.txt").assert("Content to move\n");
    assert_summary(&summary, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_then_delete() {
    let temp_dir = setup_temp_dir_with_files(&[("original_source.txt", "Original")]);
    let md = "\n## Moved File: original_source.txt to intermediate.txt\n\n## Deleted File: intermediate.txt\n";
    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("original_source.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("intermediate.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_then_create_same_path_no_force() {
    let temp_dir = setup_temp_dir_with_files(&[("source_for_move.txt", "Move content")]);
    let md = "\n## Moved File: source_for_move.txt to target.txt\n\n## File: target.txt\n```\nCreate content, should be skipped\n```\n";
    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed"); // No force

    temp_dir
        .child("source_for_move.txt")
        .assert(predicate::path::missing());
    temp_dir.child("target.txt").assert("Move content"); // Moved content wins
    assert_summary(&summary, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_then_create_same_path_with_force() {
    let temp_dir = setup_temp_dir_with_files(&[("source_for_move_force.txt", "Move content")]);
    let md = "\n## Moved File: source_for_move_force.txt to target_force.txt\n\n## File: target_force.txt\n```\nCreate content, should overwrite moved file\n```\n";
    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed"); // With force

    temp_dir
        .child("source_for_move_force.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("target_force.txt")
        .assert("Create content, should overwrite moved file\n"); // Create content wins due to force
    assert_summary(&summary, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}
