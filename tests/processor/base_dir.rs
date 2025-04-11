//! Tests related to base directory handling in the processor.

use assert_fs::prelude::*;
use markdown_processor::errors::AppError;
use predicates::prelude::*;
// Use helpers from the top-level common module
use crate::common::{run_processor, setup_temp_dir};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_creates_base_directory() {
    let base_temp_dir = setup_temp_dir(); // Parent for the actual base
    let target_base = base_temp_dir.path().join("new_project_dir");

    assert!(!target_base.exists());

    let md = "\n## File: inside.txt\n```\ncontent\n```\n";
    // Use run_processor which handles parse+process
    let (summary, _) = run_processor(md, &base_temp_dir.child("new_project_dir"), false)
        .expect("Processing failed");

    // Assert directory existence using standard Path methods
    assert!(target_base.is_dir());

    // Assert file existence and content using assert_fs::ChildPath
    base_temp_dir
        .child("new_project_dir/inside.txt")
        .assert(predicate::path::is_file());
    base_temp_dir
        .child("new_project_dir/inside.txt")
        .assert("content\n");

    assert_summary(&summary, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_errors_if_base_is_file() {
    let temp_dir = setup_temp_dir();
    let base_path_as_file = temp_dir.child("base_is_file.txt");
    base_path_as_file.write_str("I am a file").unwrap();

    let md = "\n## File: should_fail.txt\n```\ncontent\n```\n";
    let actions = markdown_processor::parse_markdown(md).unwrap();
    let result = markdown_processor::process_actions(base_path_as_file.path(), actions, false);

    assert!(result.is_err());
    match result.err().unwrap() {
        AppError::Argument(msg) => {
            assert!(msg.contains("Base path is not a directory"));
        }
        _ => panic!("Expected Argument error"),
    }
}
