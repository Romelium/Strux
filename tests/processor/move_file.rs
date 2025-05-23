//! Tests for file move logic in the processor.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir, setup_temp_dir_with_files};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_move_file_simple() {
    let temp_dir = setup_temp_dir_with_files(&[("source.txt", "Move me")]);
    let md = "\n## Moved File: source.txt to dest.txt\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("source.txt")
        .assert(predicate::path::missing());
    temp_dir.child("dest.txt").assert("Move me");
    assert_summary(&summary, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_file_create_dest_parent_dir() {
    let temp_dir = setup_temp_dir_with_files(&[("source.txt", "Move me too")]);
    let md = "\n## Moved File: source.txt to new_dir/sub_dir/dest.txt\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("source.txt")
        .assert(predicate::path::missing());
    temp_dir.child("new_dir").assert(predicate::path::is_dir());
    temp_dir
        .child("new_dir/sub_dir")
        .assert(predicate::path::is_dir());
    temp_dir
        .child("new_dir/sub_dir/dest.txt")
        .assert("Move me too");
    assert_summary(&summary, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_file_source_not_found() {
    let temp_dir = setup_temp_dir();
    let md = "\n## Moved File: non_existent_source.txt to dest.txt\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("dest.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_file_source_is_dir() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("source_dir").create_dir_all().unwrap();
    let md = "\n## Moved File: source_dir to dest.txt\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("source_dir")
        .assert(predicate::path::is_dir()); // Source unchanged
    temp_dir
        .child("dest.txt")
        .assert(predicate::path::missing()); // Dest not created
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_file_dest_exists_no_force() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("source.txt", "Source content"),
        ("dest.txt", "Existing dest content"),
    ]);
    let md = "\n## Moved File: source.txt to dest.txt\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed"); // No --force

    temp_dir.child("source.txt").assert("Source content"); // Source unchanged
    temp_dir.child("dest.txt").assert("Existing dest content"); // Dest unchanged
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_file_dest_exists_with_force() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("source.txt", "Source content"),
        ("dest.txt", "Old dest content to be overwritten"),
    ]);
    let md = "\n## Moved File: source.txt to dest.txt\n";

    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed"); // With --force

    temp_dir
        .child("source.txt")
        .assert(predicate::path::missing()); // Source gone
    temp_dir.child("dest.txt").assert("Source content"); // Dest overwritten
    assert_summary(&summary, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_move_file_dest_is_dir() {
    let temp_dir = setup_temp_dir_with_files(&[("source.txt", "Source content")]);
    temp_dir.child("dest_dir").create_dir_all().unwrap();
    let md = "\n## Moved File: source.txt to dest_dir\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir.child("source.txt").assert("Source content"); // Source unchanged
    temp_dir.child("dest_dir").assert(predicate::path::is_dir()); // Dest dir unchanged
                                                                  // File not moved into dir
    temp_dir
        .child("dest_dir/source.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0);
}

#[test]
fn test_process_move_file_dest_is_dir_with_force() {
    // --force should not allow replacing a directory with a file
    let temp_dir = setup_temp_dir_with_files(&[("source.txt", "Source content")]);
    temp_dir.child("dest_dir").create_dir_all().unwrap();
    let md = "\n## Moved File: source.txt to dest_dir\n";

    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed"); // With --force

    temp_dir.child("source.txt").assert("Source content");
    temp_dir.child("dest_dir").assert(predicate::path::is_dir());
    temp_dir
        .child("dest_dir/source.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0);
}

#[test]
fn test_process_move_file_unsafe_source() {
    let temp_dir = setup_temp_dir();
    // We don't need to create ../unsafe_source.txt, just try to move it
    let md = "\n## Moved File: ../unsafe_source.txt to safe_dest.txt\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");
    temp_dir
        .child("safe_dest.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0);
}

#[test]
fn test_process_move_file_unsafe_destination() {
    let temp_dir = setup_temp_dir_with_files(&[("safe_source.txt", "content")]);
    let md = "\n## Moved File: safe_source.txt to ../unsafe_dest.txt\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir.child("safe_source.txt").assert("content"); // Source should remain
    let parent_dir = temp_dir.path().parent().unwrap();
    assert!(predicate::path::missing().eval(&parent_dir.join("unsafe_dest.txt")));
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0);
}

#[test]
fn test_process_move_file_source_and_dest_are_same() {
    let temp_dir = setup_temp_dir_with_files(&[("file.txt", "content")]);
    let md = "\n## Moved File: file.txt to file.txt\n";

    // Without --force, this should be skipped as destination exists
    let (summary_no_force, _) = run_processor(md, &temp_dir, false).expect("Processing failed");
    temp_dir.child("file.txt").assert("content");
    assert_summary(
        &summary_no_force,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        0,
        0,
    );

    // With --force, this should be a no-op, counted as MovedOverwritten due to the --force flag.
    let (summary_force, _) = run_processor(md, &temp_dir, true).expect("Processing failed");
    temp_dir.child("file.txt").assert("content");
    assert_summary(
        &summary_force,
        0,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    );
}

#[test]
fn test_process_move_file_source_and_dest_are_same_source_missing() {
    let temp_dir = setup_temp_dir(); // file.txt does not exist
    let md = "\n## Moved File: file.txt to file.txt\n";

    // Without --force
    let (summary_no_force, _) = run_processor(md, &temp_dir, false).expect("Processing failed");
    temp_dir
        .child("file.txt")
        .assert(predicate::path::missing());
    assert_summary(
        &summary_no_force,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ); // SkippedSourceNotFound

    // With --force
    let (summary_force, _) = run_processor(md, &temp_dir, true).expect("Processing failed");
    temp_dir
        .child("file.txt")
        .assert(predicate::path::missing());
    assert_summary(
        &summary_force,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ); // SkippedSourceNotFound
}
