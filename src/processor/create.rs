//! Handles the 'create' file action.

use crate::core_types::{Action, CreateStatus};
use crate::errors::ProcessError;
use std::fs;
use std::path::Path;

/// Creates or overwrites a file with the provided content.
pub(crate) fn process_create(
    item: &Action,
    resolved_full_path: &Path,
    relative_path_str: &str, // For logging
    resolved_base: &Path,    // For logging relative paths
    overwrite: bool,
) -> Result<CreateStatus, ProcessError> {
    let content = item
        .content
        .as_ref()
        .ok_or_else(|| ProcessError::Internal("Missing content for create action".to_string()))?;

    // Ensure parent directory exists and is a directory
    ensure_parent_directory(resolved_full_path, resolved_base)?;

    let mut status = CreateStatus::Created; // Default optimistic status

    // Check if the target path itself exists
    if resolved_full_path.exists() {
        if resolved_full_path.is_dir() {
            return Err(ProcessError::TargetIsDirectory {
                path: resolved_full_path.to_path_buf(),
            });
        } else if !overwrite {
            println!(
                "  Skipping existing file: {} (use --force to overwrite)",
                relative_path_str
            );
            return Ok(CreateStatus::SkippedExists);
        } else {
            println!(
                "  Overwriting file: {} ({} bytes)",
                relative_path_str,
                content.len()
            );
            status = CreateStatus::Overwritten;
        }
    } else {
        println!(
            "  Creating file: {} ({} bytes)",
            relative_path_str,
            content.len()
        );
        // Status remains Created
    }

    // Write the file content (as bytes to preserve line endings)
    fs::write(resolved_full_path, content.as_bytes())
        .map_err(|e| ProcessError::Io { source: e })?;

    Ok(status)
}

/// Ensures the parent directory of a path exists, creating it if necessary.
/// Also checks if the parent path itself is unexpectedly a file.
fn ensure_parent_directory(target_path: &Path, resolved_base: &Path) -> Result<(), ProcessError> {
    if let Some(parent_dir) = target_path.parent() {
        if parent_dir == resolved_base || parent_dir.exists() {
            // If parent exists, ensure it's a directory
            if !parent_dir.is_dir() {
                return Err(ProcessError::ParentIsNotDirectory {
                    path: target_path.to_path_buf(),
                    parent_path: parent_dir.to_path_buf(),
                });
            }
        } else {
            // Parent does not exist, create it
            let relative_parent_dir = parent_dir.strip_prefix(resolved_base).unwrap_or(parent_dir);
            println!("  Creating directory: {}", relative_parent_dir.display());
            fs::create_dir_all(parent_dir).map_err(|e| ProcessError::Io { source: e })?;
        }
    }
    // If no parent (e.g., root directory), assume it's okay.
    Ok(())
}
