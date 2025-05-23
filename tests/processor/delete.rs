//! Tests for basic file deletion logic in the processor.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_delete_file() {
    let temp_dir = setup_temp_dir();
    temp_dir
        .child("to_delete.log")
        .write_str("Log data")
        .unwrap();
    let md = "\n**Deleted File: to_delete.log**\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("to_delete.log")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}
