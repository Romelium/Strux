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
            // Pass the original target_path for error reporting context if needed.
            check_nonexistent_path_safety(target_path, &canonical_base)
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

/// Recursively checks safety for a path that does not necessarily exist
/// by examining its ancestors relative to the canonical base.
fn check_nonexistent_path_safety(
    path_to_check: &Path,
    canonical_base: &Path,
) -> Result<(), ProcessError> {
    // Base case: If the path_to_check *is* the base directory, it's safe by definition.
    // We need to compare canonical paths if possible.
    match path_to_check.canonicalize() {
        Ok(canonical_check) if canonical_check == *canonical_base => return Ok(()),
        Ok(_) => {} // Path exists and is not the base, continue to parent check
        Err(ref e) if e.kind() == ErrorKind::NotFound => {} // Path doesn't exist, continue
        Err(ref e) if e.kind() == ErrorKind::NotADirectory => {
            // An intermediate component is a file.
            return Err(ProcessError::ParentIsNotDirectory {
                path: path_to_check.to_path_buf(), // Report the path we were checking
                parent_path: path_to_check
                    .parent()
                    .unwrap_or(path_to_check)
                    .to_path_buf(),
            });
        }
        Err(e) => {
            // Other canonicalization error (permissions?)
            return Err(ProcessError::PathResolution {
                path: path_to_check.to_path_buf(),
                details: format!("Failed to canonicalize path during safety check: {}", e),
            });
        }
    }

    // Get the parent of the path we are currently checking.
    if let Some(parent) = path_to_check.parent() {
        // If the parent *is* the base directory, the path is safe (it's directly inside).
        // Compare canonical paths if possible for robustness.
        match parent.canonicalize() {
            Ok(canonical_parent) => {
                if canonical_parent == *canonical_base {
                    return Ok(());
                }
                // Parent exists and is not the base. Check if it's *within* the base.
                if !canonical_parent.starts_with(canonical_base) {
                    return Err(ProcessError::PathNotSafe {
                        resolved_path: canonical_parent, // Report the unsafe parent
                        base_path: canonical_base.to_path_buf(),
                    });
                }
                // Parent exists and is within the base. Now, ensure it's actually a directory.
                match fs::metadata(&canonical_parent) {
                    Ok(meta) if meta.is_dir() => {
                        // Parent exists, is safe, and is a directory. Path is safe.
                        Ok(())
                    }
                    Ok(_) => {
                        // Parent exists, is safe, but is NOT a directory.
                        Err(ProcessError::ParentIsNotDirectory {
                            path: path_to_check.to_path_buf(), // The path we were trying to create/check
                            parent_path: parent.to_path_buf(), // The parent that is not a directory
                        })
                    }
                    Err(e) => {
                        // Error getting metadata for the existing canonical parent (permissions?)
                        Err(ProcessError::Io { source: e })
                    }
                }
            }
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                // Parent does not exist. Recursively check the parent's safety.
                check_nonexistent_path_safety(parent, canonical_base)
            }
            Err(ref e) if e.kind() == ErrorKind::NotADirectory => {
                // An intermediate component in the *parent's* path is a file.
                Err(ProcessError::ParentIsNotDirectory {
                    path: path_to_check.to_path_buf(), // The original path being checked
                    parent_path: parent.to_path_buf(), // The parent path that failed
                })
            }
            Err(e) => {
                // Other error canonicalizing the parent (permissions?)
                Err(ProcessError::PathResolution {
                    path: parent.to_path_buf(),
                    details: format!("Failed to canonicalize parent directory: {}", e),
                })
            }
        }
    } else {
        // Cannot get parent (e.g., root path "/" or similar).
        // If we reached here, it means the path wasn't the base itself.
        // This implies an attempt to access something outside the base, potentially the root.
        Err(ProcessError::PathResolution {
            path: path_to_check.to_path_buf(),
            details: "Cannot determine parent directory for safety check (potentially root path)."
                .to_string(),
        })
    }
}

// --- REMOVED old check_nonexistent_target_safety function ---
