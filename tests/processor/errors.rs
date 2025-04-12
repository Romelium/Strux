//! Tests for processor error handling scenarios.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{read_file_content, run_processor, setup_temp_dir};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_create_target_is_directory() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("target_dir").create_dir_all().unwrap();
    let md = "\n## File: target_dir\n```\nThis should fail\n```\n";

    let (summary, _) =
        run_processor(md, &temp_dir, false).expect("Processing should not fail overall");

    temp_dir
        .child("target_dir")
        .assert(predicate::path::is_dir());
    assert!(read_file_content(temp_dir.child("target_dir").path()).is_none());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0); // failed_isdir_create = 1
}

#[test]
fn test_process_create_parent_is_file() {
    let temp_dir = setup_temp_dir();
    temp_dir
        .child("parent_file")
        .write_str("I am a file")
        .unwrap();
    let md = "\n## File: parent_file/nested_file.txt\n```\nThis should fail\n```\n";

    let (summary, _) =
        run_processor(md, &temp_dir, false).expect("Processing should not fail overall");

    temp_dir
        .child("parent_file")
        .assert(predicate::path::is_file());
    temp_dir
        .child("parent_file/nested_file.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0); // failed_parent_isdir = 1
}

#[test]
fn test_process_path_not_safe_relative() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: ../outside_file.txt\n```\nThis should not be created\n```\n";

    let (summary, _) =
        run_processor(md, &temp_dir, false).expect("Processing should not fail overall");

    let parent_dir = temp_dir.path().parent().unwrap();
    assert!(
        !parent_dir.join("outside_file.txt").exists(),
        "File was created outside the base directory!"
    );
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0); // failed_unsafe = 1
}

#[test]
fn test_process_invalid_path_format_in_action() {
    let temp_dir = setup_temp_dir();
    // Manually create an action with an invalid path to test processor directly.
    let actions = vec![markdown_processor::Action {
        action_type: markdown_processor::ActionType::Create,
        path: "bad//path.txt".to_string(),
        content: Some("content".to_string()),
        original_pos: 0,
    }];

    let summary = markdown_processor::process_actions(temp_dir.path(), actions, false)
        .expect("Processing should not fail overall");

    temp_dir
        .child("bad//path.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0); // failed_unsafe = 1
}
