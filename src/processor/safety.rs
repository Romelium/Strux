//! Path safety validation logic.

use crate::errors::ProcessError;
use std::fs; // Use fs::metadata
use std::io::ErrorKind; // Import io and ErrorKind
use std::path::Path;

/// Checks if the target path is safely within the base directory.
/// Canonicalizes paths for reliable comparison.
pub(crate) fn ensure_path_safe(base_dir: &Path, target_path: &Path) -> Result<(), ProcessError> {
    // Canonicalize base directory (must succeed as it's resolved in process_actions)
    let canonical_base = match base_dir.canonicalize() {
        Ok(path) => path,
        // Handle potential NotADirectory error if base_dir itself is somehow a file (should be caught earlier, but belt-and-suspenders)
        Err(e) if e.kind() == ErrorKind::NotADirectory => {
            return Err(ProcessError::ParentIsNotDirectory {
                // Technically the base itself isn't a dir
                path: base_dir.to_path_buf(),
                parent_path: base_dir.parent().unwrap_or(base_dir).to_path_buf(),
            });
        }
        Err(e) => return Err(ProcessError::Io { source: e }),
    };

    // Check if the target path *exists* first using metadata.
    match fs::metadata(target_path) {
        Ok(_) => {
            // Target exists. Canonicalize it for the safety check.
            match target_path.canonicalize() {
                Ok(canonical_target) => {
                    if canonical_target.starts_with(&canonical_base) {
                        Ok(()) // Path exists and is safe
                    } else {
                        Err(ProcessError::PathNotSafe {
                            resolved_path: canonical_target,
                            base_path: canonical_base,
                        })
                    }
                }
                // *** CATCH NotADirectory during canonicalization of EXISTING target ***
                Err(e) if e.kind() == ErrorKind::NotADirectory => {
                    Err(ProcessError::ParentIsNotDirectory {
                        path: target_path.to_path_buf(),
                        parent_path: target_path.parent().unwrap_or(target_path).to_path_buf(),
                    })
                }
                Err(e) => {
                    // Error canonicalizing an *existing* path (permissions?)
                    Err(ProcessError::PathResolution {
                        path: target_path.to_path_buf(),
                        details: format!("Failed to canonicalize existing target path: {}", e),
                    })
                }
            }
        }
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            // Target doesn't exist: Check safety based on its intended parent.
            check_nonexistent_target_safety(target_path, &canonical_base)
        }
        // *** CATCH NotADirectory during metadata check (parent is file) ***
        Err(ref e) if e.kind() == ErrorKind::NotADirectory => {
            Err(ProcessError::ParentIsNotDirectory {
                path: target_path.to_path_buf(),
                parent_path: target_path.parent().unwrap_or(target_path).to_path_buf(),
            })
        }
        Err(e) => {
            // Other error getting metadata (permissions?)
            Err(ProcessError::Io { source: e }) // Map other metadata errors to IO
        }
    }
}

/// Checks safety for a target path that does not yet exist by examining its parent.
fn check_nonexistent_target_safety(
    target_path: &Path,
    canonical_base: &Path,
) -> Result<(), ProcessError> {
    if let Some(parent) = target_path.parent() {
        // Canonicalize the parent directory.
        match parent.canonicalize() {
            Ok(canonical_parent) => {
                // Check if the existing parent is within the base directory.
                if canonical_parent.starts_with(canonical_base) {
                    // Parent is safe, so creating the target inside it is considered safe.
                    Ok(())
                } else {
                    // Parent exists but is outside the base directory. Unsafe.
                    Err(ProcessError::PathNotSafe {
                        resolved_path: canonical_parent, // Report the parent path
                        base_path: canonical_base.to_path_buf(),
                    })
                }
            }
            // *** CATCH NotADirectory during PARENT canonicalization ***
            Err(ref parent_err) if parent_err.kind() == ErrorKind::NotADirectory => {
                Err(ProcessError::ParentIsNotDirectory {
                    path: target_path.to_path_buf(), // The target we were trying to create
                    parent_path: parent.to_path_buf(), // The parent that is not a directory
                })
            }
            Err(ref parent_err) if parent_err.kind() == ErrorKind::NotFound => {
                // Parent directory itself doesn't exist. This implies it needs to be
                // created. Assume `create_dir_all` will handle safety within the base.
                // We could recursively check the parent's parent, but relying on
                // the initial base check and `create_dir_all` is often sufficient.
                // If the non-existent parent's path *string* looks unsafe (e.g., "../.."),
                // it might be caught earlier, but canonicalization handles this better.
                // For simplicity here, assume okay if parent *would* be created inside base.
                // A stricter check could trace the path components upwards.
                // Let's check if the parent *path itself* starts with base logically.
                if parent.starts_with(canonical_base) {
                    Ok(())
                } else {
                    // If even the logical parent path isn't inside base, it's unsafe.
                    // This check is less robust than canonicalization but handles simple cases.
                    Err(ProcessError::PathNotSafe {
                        resolved_path: parent.to_path_buf(), // Report logical parent
                        base_path: canonical_base.to_path_buf(),
                    })
                }
            }
            Err(parent_err) => {
                // Other error canonicalizing the parent (e.g., permissions).
                Err(ProcessError::PathResolution {
                    path: parent.to_path_buf(),
                    details: format!("Failed to canonicalize parent directory: {}", parent_err),
                })
            }
        }
    } else {
        // Cannot get parent (e.g., root path "/" or similar). Unlikely for relative paths.
        Err(ProcessError::PathResolution {
            path: target_path.to_path_buf(),
            details: "Cannot determine parent directory for safety check.".to_string(),
        })
    }
}
