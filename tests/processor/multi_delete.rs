//! Tests for processing multiple delete actions in one run.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir_with_files};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_delete_multiple_files() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("file1.txt", "1"),
        ("dir/file2.txt", "2"),
        ("dir/subdir/file3.txt", "3"),
        ("another.log", "4"),
        ("keep_me.txt", "5"), // This one is NOT deleted
    ]);

    let md = "\n## Deleted File: file1.txt\n**Deleted File: dir/file2.txt**\n## Deleted File: dir/subdir/file3.txt\n**Deleted File: another.log**\n\n## File: new_file.txt\n```\nThis should still be created.\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    // Check deleted files
    temp_dir
        .child("file1.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("dir/file2.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("dir/subdir/file3.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("another.log")
        .assert(predicate::path::missing());

    // Check file that was NOT deleted
    temp_dir.child("keep_me.txt").assert("5");

    // Check file that was created
    temp_dir
        .child("new_file.txt")
        .assert("This should still be created.\n");

    assert_summary(&summary, 1, 0, 4, 0, 0, 0, 0, 0, 0, 0); // 1 created, 4 deleted
}
