//! Handles the 'delete' file action.

use crate::core_types::DeleteStatus;
use crate::errors::ProcessError;
use std::fs;
use std::path::Path;

/// Deletes the specified file path. Handles non-existence and non-file types.
pub(crate) fn process_delete(
    resolved_full_path: &Path,
    relative_path_str: &str, // For logging
) -> Result<DeleteStatus, ProcessError> {
    if resolved_full_path.exists() {
        // Use symlink_metadata to check type without following symlinks
        let metadata =
            fs::symlink_metadata(resolved_full_path).map_err(|e| ProcessError::Io { source: e })?;

        if metadata.is_file() {
            // It's a regular file (or a symlink to one, but we delete the link)
            println!("  Deleting file: {}", relative_path_str);
            fs::remove_file(resolved_full_path).map_err(|e| ProcessError::Io { source: e })?;
            Ok(DeleteStatus::Deleted)
        } else if metadata.is_dir() {
            // It's a directory, skip deletion
            eprintln!(
                "Warning: Skipping deletion. Path '{}' is a directory.",
                relative_path_str
            );
            Ok(DeleteStatus::SkippedIsDir)
        } else {
            // It's something else (e.g., a broken symlink). Attempt deletion.
            println!(
                "  Attempting to delete non-file/non-dir path: {}",
                relative_path_str
            );
            match fs::remove_file(resolved_full_path) {
                Ok(_) => {
                    println!("    Successfully deleted non-file/non-dir path.");
                    Ok(DeleteStatus::Deleted)
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Skipping deletion. Path '{}' exists but is not a regular file or directory and could not be deleted: {}",
                        relative_path_str, e
                    );
                    Ok(DeleteStatus::SkippedOtherType)
                }
            }
        }
    } else {
        // File specified for deletion does not exist
        println!("  Skipping deletion: File not found: {}", relative_path_str);
        Ok(DeleteStatus::SkippedNotFound)
    }
}
