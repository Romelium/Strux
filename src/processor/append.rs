//! Handles the 'append' file action.

use crate::core_types::{Action, AppendStatus};
use crate::errors::ProcessError;
use crate::processor::create::ensure_parent_directory; // Re-use for dest parent
use std::fs;
use std::io::{ErrorKind, Write}; // Import Write for append mode
use std::path::Path;

/// Appends content to a file. If the file does not exist, it's created.
pub(crate) fn process_append(
    item: &Action,
    resolved_full_path: &Path,
    relative_path_str: &str, // For logging
    resolved_base: &Path,    // For ensure_parent_directory
) -> Result<AppendStatus, ProcessError> {
    let content_to_append = item
        .content
        .as_ref()
        .ok_or_else(|| ProcessError::Internal("Missing content for append action".to_string()))?;

    // Ensure parent directory exists
    ensure_parent_directory(resolved_full_path, resolved_base)?;

    // Check if the target path itself exists
    match fs::metadata(resolved_full_path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Err(ProcessError::TargetIsDirectoryForAppend {
                    path: resolved_full_path.to_path_buf(),
                });
            }
            // File exists, open in append mode
            println!(
                "  Appending to file: {} ({} bytes)",
                relative_path_str,
                content_to_append.len()
            );
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(resolved_full_path)
                .map_err(|e| ProcessError::Io { source: e })?;
            file.write_all(content_to_append.as_bytes())
                .map_err(|e| ProcessError::Io { source: e })?;
            Ok(AppendStatus::Appended)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // File does not exist, create it
            println!(
                "  File not found, creating and writing: {} ({} bytes)",
                relative_path_str,
                content_to_append.len()
            );
            fs::write(resolved_full_path, content_to_append.as_bytes())
                .map_err(|e| ProcessError::Io { source: e })?;
            Ok(AppendStatus::Created)
        }
        Err(e) => Err(ProcessError::Io { source: e }), // Other metadata error
    }
}
