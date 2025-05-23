//! Tests for handling duplicate actions targeting the same path.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir, setup_temp_dir_with_files};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_create_duplicate_last_wins_with_force() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: duplicate.txt\n```\nFirst content\n```\n\n## File: duplicate.txt\n```\nSecond content\n```\n";

    // Run with overwrite = true to ensure the second create overwrites the first
    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed");

    temp_dir
        .child("duplicate.txt")
        .assert(predicate::path::is_file());
    temp_dir.child("duplicate.txt").assert("Second content\n"); // Last action's content
                                                                // The first action creates, the second overwrites.
    assert_summary(&summary, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_create_duplicate_no_force_skips() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: duplicate_skip.txt\n```\nFirst content\n```\n\n## File: duplicate_skip.txt\n```\nSecond content\n```\n";

    // Run with overwrite = false (default)
    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("duplicate_skip.txt")
        .assert(predicate::path::is_file());
    temp_dir
        .child("duplicate_skip.txt")
        .assert("First content\n"); // First action's content remains
                                    // The first action creates, the second is skipped because it exists.
    assert_summary(&summary, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_create_then_delete_last_wins() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: create_then_delete.txt\n```\nShould be deleted\n```\n\n**Deleted File: create_then_delete.txt**\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("create_then_delete.txt")
        .assert(predicate::path::missing()); // Last action (delete) wins
    assert_summary(&summary, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_delete_then_create_last_wins() {
    let temp_dir = setup_temp_dir_with_files(&[("delete_then_create.txt", "Initial content")]);
    let md = "\n**Deleted File: delete_then_create.txt**\n\n## File: delete_then_create.txt\n```\nShould exist\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("delete_then_create.txt")
        .assert(predicate::path::is_file());
    temp_dir
        .child("delete_then_create.txt")
        .assert("Should exist\n"); // Last action (create) wins
                                   // The first action deletes, the second creates.
    assert_summary(&summary, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_delete_then_create_no_initial_file() {
    let temp_dir = setup_temp_dir(); // No initial file
    let md = "\n**Deleted File: delete_then_create_missing.txt**\n\n## File: delete_then_create_missing.txt\n```\nShould exist\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("delete_then_create_missing.txt")
        .assert(predicate::path::is_file());
    temp_dir
        .child("delete_then_create_missing.txt")
        .assert("Should exist\n"); // Last action (create) wins
                                   // The first action skips (not found), the second creates.
    assert_summary(&summary, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}
