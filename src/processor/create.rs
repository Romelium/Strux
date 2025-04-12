//! Handles the 'create' file action.

use crate::core_types::{Action, CreateStatus};
use crate::errors::ProcessError;
use std::fs;
use std::io; // Import io for ErrorKind
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
    // This might return ParentIsNotDirectory if parent exists as file or if creation fails
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
    fs::write(resolved_full_path, content.as_bytes()).map_err(|e| {
        // Check if the write failed because the parent path component is a file
        if e.kind() == io::ErrorKind::NotADirectory {
            // Map this specific IO error to our more descriptive error
            let parent_path = resolved_full_path
                .parent()
                .unwrap_or(resolved_full_path)
                .to_path_buf();
            // *** DEBUG LOG ***
            // eprintln!("[DEBUG] fs::write failed with NotADirectory, mapping to ParentIsNotDirectory for path: {}", resolved_full_path.display());
            ProcessError::ParentIsNotDirectory {
                path: resolved_full_path.to_path_buf(),
                parent_path, // Report the parent path
            }
        } else {
            // *** DEBUG LOG ***
            // eprintln!("[DEBUG] fs::write failed with other IO error: {:?}, mapping to Io for path: {}", e.kind(), resolved_full_path.display());
            ProcessError::Io { source: e }
        }
    })?;

    Ok(status)
}

/// Ensures the parent directory of a path exists, creating it if necessary.
/// Also checks if the parent path itself is unexpectedly a file.
fn ensure_parent_directory(target_path: &Path, resolved_base: &Path) -> Result<(), ProcessError> {
    if let Some(parent_dir) = target_path.parent() {
        // Avoid checking the base directory itself if it's the parent
        if parent_dir == resolved_base || parent_dir.as_os_str().is_empty() {
            return Ok(()); // Base directory is guaranteed to exist and be a dir, or path is in root
        }

        match fs::metadata(parent_dir) {
            Ok(metadata) => {
                // Parent exists, check if it's a directory
                if !metadata.is_dir() {
                    // *** DEBUG LOG ***
                    // eprintln!("[DEBUG] Parent metadata exists but is not dir, returning ParentIsNotDirectory for parent: {}", parent_dir.display());
                    return Err(ProcessError::ParentIsNotDirectory {
                        path: target_path.to_path_buf(),
                        parent_path: parent_dir.to_path_buf(),
                    });
                }
                // Parent exists and is a directory, all good.
            }
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                // Parent does not exist, try to create it
                let relative_parent_dir =
                    parent_dir.strip_prefix(resolved_base).unwrap_or(parent_dir);
                println!("  Creating directory: {}", relative_parent_dir.display());

                if let Err(create_err) = fs::create_dir_all(parent_dir) {
                    // Check if the error is specifically "Not a directory"
                    // This often indicates an intermediate path component was a file during creation attempt.
                    if create_err.kind() == io::ErrorKind::NotADirectory {
                        // *** DEBUG LOG ***
                        // eprintln!("[DEBUG] create_dir_all failed with NotADirectory, returning ParentIsNotDirectory for parent: {}", parent_dir.display());
                        // Map this specific IO error to our more descriptive error
                        return Err(ProcessError::ParentIsNotDirectory {
                            path: target_path.to_path_buf(),
                            // Report the parent directory we *failed* to create
                            parent_path: parent_dir.to_path_buf(),
                        });
                    } else {
                        // *** DEBUG LOG ***
                        // eprintln!("[DEBUG] create_dir_all failed with other IO error: {:?}, returning Io for parent: {}", create_err.kind(), parent_dir.display());
                        // Other I/O error during creation
                        return Err(ProcessError::Io { source: create_err });
                    }
                }
                // Creation successful
            }
            Err(e) => {
                // *** DEBUG LOG ***
                // eprintln!("[DEBUG] fs::metadata failed with other IO error: {:?}, returning Io for parent: {}", e.kind(), parent_dir.display());
                // Other error getting metadata (permissions?)
                return Err(ProcessError::Io { source: e });
            }
        }
    }
    // If no parent (e.g., file directly in base), assume it's okay.
    Ok(())
}
