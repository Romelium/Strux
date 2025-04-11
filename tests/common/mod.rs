//! Common helper functions for integration and CLI tests.

use assert_fs::prelude::*;
use assert_fs::TempDir;
use markdown_processor::{parse_markdown, process_actions, Action, AppError, Summary};
use std::path::Path;

/// Sets up a temporary directory and populates it with initial files if needed.
pub fn setup_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temporary directory")
}

/// Runs the core processing logic (parse + process) within a temporary directory.
/// Returns the summary and the TempDir handle for further assertions.
#[allow(dead_code)]
pub fn run_processor(
    markdown_content: &str,
    base_dir: &TempDir,
    overwrite: bool,
) -> Result<(Summary, Vec<Action>), AppError> {
    // 1. Parse
    let actions = parse_markdown(markdown_content)?;

    // 2. Process
    let summary = process_actions(base_dir.path(), actions.clone(), overwrite)?; // Clone actions for return

    Ok((summary, actions))
}

/// Helper to read file content, returning None if not found or error.
#[allow(dead_code)]
pub fn read_file_content(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

// Example of how you might add initial files to the temp dir if needed for tests
#[allow(dead_code)] // Keep available for future tests
pub fn setup_temp_dir_with_files(files: &[(&str, &str)]) -> TempDir {
    let temp = setup_temp_dir();
    for (path, content) in files {
        temp.child(path)
            .write_str(content)
            .expect("Failed to write initial file");
    }
    temp
}
