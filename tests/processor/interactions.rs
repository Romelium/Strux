//! Tests for interactions between create and delete actions.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level common module
use crate::common::{run_processor, setup_temp_dir};
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
    assert_summary(&summary, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0);
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
    assert_summary(&summary, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0);
}
