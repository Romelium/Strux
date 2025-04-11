//! Integration tests for the `process_actions` function.

mod common;
use assert_fs::prelude::*;
use common::{read_file_content, run_processor, setup_temp_dir}; // Now use items from common
use markdown_processor::core_types::Summary;
use markdown_processor::errors::AppError;
use predicates::prelude::*;

// --- Test Helpers ---

#[allow(clippy::too_many_arguments)]
fn assert_summary(
    summary: &Summary,
    created: u32,
    overwritten: u32,
    deleted: u32,
    skipped_exists: u32,
    skipped_not_found: u32,
    skipped_isdir_delete: u32,
    failed_unsafe: u32,
    failed_isdir_create: u32,
    failed_parent_isdir: u32,
    failed_io: u32,
) {
    assert_eq!(summary.created, created, "Summary: created mismatch");
    assert_eq!(
        summary.overwritten, overwritten,
        "Summary: overwritten mismatch"
    );
    assert_eq!(summary.deleted, deleted, "Summary: deleted mismatch");
    assert_eq!(
        summary.skipped_exists, skipped_exists,
        "Summary: skipped_exists mismatch"
    );
    assert_eq!(
        summary.skipped_not_found, skipped_not_found,
        "Summary: skipped_not_found mismatch"
    );
    assert_eq!(
        summary.skipped_isdir_delete, skipped_isdir_delete,
        "Summary: skipped_isdir_delete mismatch"
    );
    assert_eq!(
        summary.failed_unsafe, failed_unsafe,
        "Summary: failed_unsafe mismatch"
    );
    assert_eq!(
        summary.failed_isdir_create_patch, failed_isdir_create,
        "Summary: failed_isdir_create mismatch"
    ); // Note field name
    assert_eq!(
        summary.failed_parent_isdir, failed_parent_isdir,
        "Summary: failed_parent_isdir mismatch"
    );
    assert_eq!(summary.failed_io, failed_io, "Summary: failed_io mismatch");
    // Add checks for other fields if they become relevant
}

// --- Basic Create/Delete ---

#[test]
fn test_process_create_file() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: new_file.txt\n```\nThis is a new file.\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("new_file.txt")
        .assert(predicate::path::is_file());
    temp_dir
        .child("new_file.txt")
        .assert("This is a new file.\n");
    assert_summary(&summary, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_create_with_parent_dir() {
    let temp_dir = setup_temp_dir();
    let md = "\n`src/app/main.rs`\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir.child("src").assert(predicate::path::is_dir());
    temp_dir.child("src/app").assert(predicate::path::is_dir());
    temp_dir
        .child("src/app/main.rs")
        .assert(predicate::path::is_file());
    temp_dir
        .child("src/app/main.rs")
        .assert("fn main() {\n    println!(\"Hello\");\n}\n");
    assert_summary(&summary, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

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
    assert_summary(&summary, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0);
}

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

// --- Overwrite / Skip Tests ---

#[test]
fn test_process_create_skip_existing_no_force() {
    let temp_dir = setup_temp_dir();
    temp_dir
        .child("existing.txt")
        .write_str("Original")
        .unwrap();
    let md = "\n## File: existing.txt\n```\nNew content, should be ignored\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed"); // overwrite = false

    temp_dir.child("existing.txt").assert("Original"); // Content unchanged
    assert_summary(&summary, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0); // skipped_exists = 1
}

#[test]
fn test_process_create_overwrite_existing_with_force() {
    let temp_dir = setup_temp_dir();
    temp_dir
        .child("existing.txt")
        .write_str("Original")
        .unwrap();
    let md = "\n## File: existing.txt\n```\nNew content, should overwrite\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed"); // overwrite = true

    temp_dir
        .child("existing.txt")
        .assert("New content, should overwrite\n"); // Content changed
    assert_summary(&summary, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0); // overwritten = 1
}

#[test]
fn test_process_delete_skip_not_found() {
    let temp_dir = setup_temp_dir();
    let md = "\n**Deleted File: missing.txt**\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("missing.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0); // skipped_not_found = 1
}

#[test]
fn test_process_delete_skip_is_directory() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("a_directory").create_dir_all().unwrap();
    let md = "\n## Deleted File: a_directory\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("a_directory")
        .assert(predicate::path::is_dir()); // Directory still exists
    assert_summary(&summary, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0); // skipped_isdir_delete = 1
}

// --- Error Handling Tests ---

#[test]
fn test_process_create_target_is_directory() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("target_dir").create_dir_all().unwrap();
    let md = "\n## File: target_dir\n```\nThis should fail\n```\n";

    // We expect process_actions to succeed overall, but record the failure in the summary
    let (summary, _) =
        run_processor(md, &temp_dir, false).expect("Processing should not fail overall");

    temp_dir
        .child("target_dir")
        .assert(predicate::path::is_dir()); // Still a directory
                                            // Check that no file was created *inside* it accidentally
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
        .assert(predicate::path::is_file()); // Still a file
    temp_dir
        .child("parent_file/nested_file.txt")
        .assert(predicate::path::missing());
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0); // failed_parent_isdir = 1
}

#[test]
fn test_process_path_not_safe_relative() {
    let temp_dir = setup_temp_dir();
    // This path attempts to go outside the temp directory
    let md = "\n## File: ../outside_file.txt\n```\nThis should not be created\n```\n";

    let (summary, _) =
        run_processor(md, &temp_dir, false).expect("Processing should not fail overall");

    // Assert the file was NOT created anywhere obvious (especially outside the temp dir)
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
    // Parser should skip this, but if it didn't, processor should catch it.
    // Let's manually create an action with an invalid path to test processor directly.
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
        .assert(predicate::path::missing()); // File not created
    assert_summary(&summary, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0); // failed_unsafe = 1 (due to invalid format check)
}

// --- Base Directory Handling ---

#[test]
fn test_process_creates_base_directory() {
    let base_temp_dir = setup_temp_dir(); // Parent for the actual base
    let target_base = base_temp_dir.path().join("new_project_dir"); // Get PathBuf for processing

    assert!(
        !target_base.exists(),
        "Target base dir should not exist yet"
    );

    let md = "\n## File: inside.txt\n```\ncontent\n```\n";
    let actions = markdown_processor::parse_markdown(md).unwrap();
    let summary = markdown_processor::process_actions(&target_base, actions, false) // Pass PathBuf here
        .expect("Processing failed");

    // Assert directory existence using standard Path methods
    assert!(
        target_base.is_dir(),
        "Target base directory was not created"
    );

    // Assert file existence and content using assert_fs::ChildPath
    base_temp_dir // Start from the TempDir
        .child("new_project_dir/inside.txt") // Get the ChildPath relative to TempDir
        .assert(predicate::path::is_file()); // Now assert() works

    // Optionally, assert content as well:
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
