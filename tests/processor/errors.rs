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
    // failed_isdir_create is the 2nd "failed" param (16th overall)
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    );
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
    // failed_parent_isdir is the 3rd "failed" param (17th overall)
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    );
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
    // failed_unsafe is the 4th "failed" param (18th overall)
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    );
}

#[test]
fn test_process_invalid_path_format_in_action() {
    let temp_dir = setup_temp_dir();
    // Manually create an action with an invalid path to test processor directly.
    let actions = vec![strux::Action {
        action_type: strux::ActionType::Create,
        path: "bad//path.txt".to_string(),
        dest_path: None,
        content: Some("content".to_string()),
        original_pos: 0,
    }];

    let summary = strux::process_actions(temp_dir.path(), actions, false)
        .expect("Processing should not fail overall");

    temp_dir
        .child("bad//path.txt")
        .assert(predicate::path::missing());
    // failed_unsafe is the 4th "failed" param (18th overall)
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    );
}

#[test]
fn test_process_empty_path_string() {
    let temp_dir = setup_temp_dir();
    // Manually create an action with an empty path string.
    let actions = vec![strux::Action {
        action_type: strux::ActionType::Create,
        path: "".to_string(),
        dest_path: None,
        content: Some("content".to_string()),
        original_pos: 0,
    }];

    let summary = strux::process_actions(temp_dir.path(), actions, false)
        .expect("Processing should not fail overall");

    // The main check is that the summary correctly recorded the failure.
    // failed_unsafe is the 4th "failed" param (18th overall)
    assert_summary(
        &summary, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    );
}
