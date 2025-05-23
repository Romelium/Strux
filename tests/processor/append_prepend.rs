//! Tests for append and prepend file logic in the processor.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir, setup_temp_dir_with_files};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_append_to_non_existent_file() {
    let temp_dir = setup_temp_dir();
    let md = "\n## Append File: new_log.txt\n```\nFirst log entry.\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("new_log.txt")
        .assert(predicate::path::is_file());
    temp_dir.child("new_log.txt").assert("First log entry.\n");
    // Appending to non-existent file counts as 1 created
    assert_summary(
        &summary, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_prepend_to_non_existent_file() {
    let temp_dir = setup_temp_dir();
    let md = "\n## Prepend File: new_config.ini\n```\n# Initial header\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("new_config.ini")
        .assert(predicate::path::is_file());
    temp_dir
        .child("new_config.ini")
        .assert("# Initial header\n");
    // Prepending to non-existent file counts as 1 created
    assert_summary(
        &summary, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_append_to_existing_file() {
    let temp_dir = setup_temp_dir_with_files(&[("app.log", "Existing line 1\n")]);
    let md = "\n## Append File: app.log\n```\nAppended line 2\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("app.log")
        .assert("Existing line 1\nAppended line 2\n");
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_prepend_to_existing_file() {
    let temp_dir = setup_temp_dir_with_files(&[("notes.txt", "Original note.\n")]);
    let md = "\n## Prepend File: notes.txt\n```\nImportant prefix: \n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("notes.txt")
        .assert("Important prefix: \nOriginal note.\n");
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_append_to_empty_file() {
    let temp_dir = setup_temp_dir_with_files(&[("empty.txt", "")]);
    let md = "\n## Append File: empty.txt\n```\nContent for empty file.\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("empty.txt")
        .assert("Content for empty file.\n");
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_prepend_to_empty_file() {
    let temp_dir = setup_temp_dir_with_files(&[("empty.txt", "")]);
    let md = "\n## Prepend File: empty.txt\n```\nContent for empty file.\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("empty.txt")
        .assert("Content for empty file.\n");
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}

#[test]
fn test_process_append_target_is_directory() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("target_dir").create_dir_all().unwrap();
    let md = "\n## Append File: target_dir\n```\nThis should fail\n```\n";

    let (summary, _) =
        run_processor(md, &temp_dir, false).expect("Processing should not fail overall");

    temp_dir
        .child("target_dir")
        .assert(predicate::path::is_dir()); // Directory remains
                                            // Check summary for failed_isdir_append
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    );
}

#[test]
fn test_process_prepend_target_is_directory() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("target_dir").create_dir_all().unwrap();
    let md = "\n## Prepend File: target_dir\n```\nThis should also fail\n```\n";

    let (summary, _) =
        run_processor(md, &temp_dir, false).expect("Processing should not fail overall");

    temp_dir
        .child("target_dir")
        .assert(predicate::path::is_dir()); // Directory remains
                                            // Check summary for failed_isdir_prepend
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    );
}

#[test]
fn test_process_multiple_appends_and_prepends() {
    let temp_dir = setup_temp_dir_with_files(&[("test.txt", "middle\n")]);
    let md = "\n## Prepend File: test.txt\n```\nfirst\n```\n\n## Append File: test.txt\n```\nthird\n```\n\n## Prepend File: test.txt\n```\nvery_first\n```\n\n## Append File: test.txt\n```\nvery_last\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("test.txt")
        .assert("very_first\nfirst\nmiddle\nthird\nvery_last\n");
    // 2 appends, 2 prepends
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    );
}
