//! Handles the 'prepend' file action.

use crate::core_types::{Action, PrependStatus};
use crate::errors::ProcessError;
use crate::processor::create::ensure_parent_directory; // Re-use for dest parent
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

/// Prepends content to a file. If the file does not exist, it's created.
pub(crate) fn process_prepend(
    item: &Action,
    resolved_full_path: &Path,
    relative_path_str: &str, // For logging
    resolved_base: &Path,    // For ensure_parent_directory
) -> Result<PrependStatus, ProcessError> {
    let content_to_prepend = item
        .content
        .as_ref()
        .ok_or_else(|| ProcessError::Internal("Missing content for prepend action".to_string()))?;

    // Ensure parent directory exists
    ensure_parent_directory(resolved_full_path, resolved_base)?;

    // Check if the target path itself exists
    match fs::metadata(resolved_full_path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Err(ProcessError::TargetIsDirectoryForPrepend {
                    path: resolved_full_path.to_path_buf(),
                });
            }
            // File exists, read existing, prepend, then write
            println!(
                "  Prepending to file: {} ({} bytes)",
                relative_path_str,
                content_to_prepend.len()
            );
            let existing_content = fs::read_to_string(resolved_full_path)
                .map_err(|e| ProcessError::Io { source: e })?;
            let new_content = format!("{}{}", content_to_prepend, existing_content);
            fs::write(resolved_full_path, new_content.as_bytes())
                .map_err(|e| ProcessError::Io { source: e })?;
            Ok(PrependStatus::Prepended)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // File does not exist, create it
            println!(
                "  File not found, creating and writing: {} ({} bytes)",
                relative_path_str,
                content_to_prepend.len()
            );
            fs::write(resolved_full_path, content_to_prepend.as_bytes())
                .map_err(|e| ProcessError::Io { source: e })?;
            Ok(PrependStatus::Created)
        }
        Err(e) => Err(ProcessError::Io { source: e }), // Other metadata error
    }
}
