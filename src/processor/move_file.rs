//! Handles the 'move' file action.

use crate::core_types::MoveStatus;
use crate::errors::ProcessError;
use crate::processor::create::ensure_parent_directory; // Re-use for dest parent
use std::fs;
use std::path::Path;

/// Moves a file from a source path to a destination path.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_move(
    resolved_source_path: &Path,
    resolved_dest_path: &Path,
    relative_source_str: &str, // For logging
    relative_dest_str: &str,   // For logging
    resolved_base: &Path,      // For ensure_parent_directory
    overwrite: bool,
) -> Result<MoveStatus, ProcessError> {
    // --- Critical Check: Source and Destination are the same ---
    if resolved_source_path == resolved_dest_path {
        if !resolved_source_path.exists() {
            println!(
                "  Skipping move: Source file not found (source and destination are the same): {}",
                relative_source_str
            );
            return Ok(MoveStatus::SkippedSourceNotFound);
        }
        // If source and dest are same, and source exists:
        if !overwrite {
            println!(
                "  Skipping move: Source and destination are the same and file exists: {} (use --force to 'overwrite')",
                relative_source_str
            );
            return Ok(MoveStatus::SkippedDestinationExists);
        } else {
            // With --force, moving a file to itself is a no-op but considered "done".
            // We'll count it as MovedOverwritten for summary consistency if --force is used.
            println!(
                "  Skipping move (no-op): Source and destination are the same: {}",
                relative_source_str
            );
            return Ok(MoveStatus::MovedOverwritten); // Or MoveStatus::Moved if preferred for no-op
        }
    }

    // 1. Check source path
    if !resolved_source_path.exists() {
        println!(
            "  Skipping move: Source file not found: {}",
            relative_source_str
        );
        return Ok(MoveStatus::SkippedSourceNotFound);
    }

    let source_metadata =
        fs::symlink_metadata(resolved_source_path).map_err(|e| ProcessError::Io { source: e })?;

    if source_metadata.is_dir() {
        eprintln!(
            "Warning: Skipping move. Source path '{}' is a directory, not a file.",
            relative_source_str
        );
        return Ok(MoveStatus::SkippedSourceIsDir);
    }
    if !source_metadata.is_file() {
        // E.g. broken symlink, or other special file type we don't handle for move
        eprintln!(
            "Warning: Skipping move. Source path '{}' is not a regular file.",
            relative_source_str
        );
        return Ok(MoveStatus::SkippedSourceNotFound); // Treat as if not found for simplicity
    }

    // 2. Ensure parent directory of destination exists
    // This reuses the logic from create.rs, which also handles if a parent component is a file.
    ensure_parent_directory(resolved_dest_path, resolved_base)?;

    // 3. Check destination path
    let mut final_status = MoveStatus::Moved; // Optimistic default

    if resolved_dest_path.exists() {
        let dest_metadata =
            fs::symlink_metadata(resolved_dest_path).map_err(|e| ProcessError::Io { source: e })?;

        if dest_metadata.is_dir() {
            eprintln!(
                "Warning: Skipping move. Destination path '{}' exists and is a directory.",
                relative_dest_str
            );
            return Ok(MoveStatus::SkippedDestinationIsDir);
        }

        // Destination exists and is a file (or symlink to one)
        if !overwrite {
            println!(
                "  Skipping move: Destination file '{}' exists (use --force to overwrite).",
                relative_dest_str
            );
            return Ok(MoveStatus::SkippedDestinationExists);
        } else {
            // Overwrite is true, remove existing destination file
            println!(
                "  Destination file '{}' exists. Removing to overwrite.",
                relative_dest_str
            );
            fs::remove_file(resolved_dest_path).map_err(|e| ProcessError::Io { source: e })?;
            final_status = MoveStatus::MovedOverwritten;
        }
    }

    // 4. Perform the rename (move)
    println!(
        "  Moving file: '{}' to '{}'",
        relative_source_str, relative_dest_str
    );
    fs::rename(resolved_source_path, resolved_dest_path)
        .map_err(|e| ProcessError::Io { source: e })?;

    Ok(final_status)
}
