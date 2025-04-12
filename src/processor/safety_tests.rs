//! Unit tests specifically for the path safety logic in src/processor/safety.rs
//! These tests live alongside the code they test.

// Use assert_fs for temporary directory and file manipulation in tests
use assert_fs::prelude::*;
use assert_fs::TempDir;
// Use crate::errors for ProcessError within the same crate
use crate::errors::ProcessError;

// Helper function to call the safety check directly using super::
// It now takes TempDir directly for convenience in setting up paths.
fn check_safety(temp_dir: &TempDir, target_rel_path: &str) -> Result<(), ProcessError> {
    let base_path = temp_dir.path();
    let target_path = base_path.join(target_rel_path);
    // Need to canonicalize the base path first for the check function
    let canonical_base = base_path
        .canonicalize()
        .expect("Failed to canonicalize base path for test setup");
    // Call the function in the parent module's safety submodule
    super::safety::ensure_path_safe(&canonical_base, &target_path)
}

// Helper to create a temp dir - replaces setup_temp_dir from test_common
fn setup_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temporary directory for test")
}

#[test]
fn test_safety_target_exists_safe() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("safe_file.txt").write_str("safe").unwrap();
    assert!(check_safety(&temp_dir, "safe_file.txt").is_ok());
}

#[test]
fn test_safety_target_exists_unsafe() {
    let temp_dir = setup_temp_dir();
    // We can't easily create a file outside the temp dir in a portable way.
    // Instead, we test the logic using relative paths that *would* escape.
    // The function canonicalizes, so `base/../sibling` becomes `/path/to/sibling`.
    let result = check_safety(&temp_dir, "../unsafe_file.txt");
    assert!(matches!(result, Err(ProcessError::PathNotSafe { .. })));
}

#[test]
fn test_safety_target_not_exist_parent_safe() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("safe_dir").create_dir_all().unwrap();
    // Target doesn't exist, but parent does and is inside base.
    assert!(check_safety(&temp_dir, "safe_dir/new_file.txt").is_ok());
}

#[test]
fn test_safety_target_not_exist_parent_unsafe() {
    let temp_dir = setup_temp_dir();
    // Parent resolves outside the base directory.
    let result = check_safety(&temp_dir, "../unsafe_dir/new_file.txt");
    assert!(matches!(result, Err(ProcessError::PathNotSafe { .. })));
}

#[test]
fn test_safety_target_not_exist_parent_not_exist_safe() {
    let temp_dir = setup_temp_dir();
    // Neither target nor parent exists, but the logical path is within base.
    assert!(check_safety(&temp_dir, "new_dir/new_file.txt").is_ok());
}

#[test]
fn test_safety_target_not_exist_parent_not_exist_unsafe() {
    let temp_dir = setup_temp_dir();
    // Neither target nor parent exists, and the logical path points outside.
    let result = check_safety(&temp_dir, "../new_unsafe_dir/new_file.txt");
    // This case relies on the non-canonicalized check in check_nonexistent_target_safety
    // or potentially fails during parent canonicalization if the unsafe parent *did* exist.
    // Given the current implementation, it should detect the parent is outside base.
    assert!(matches!(result, Err(ProcessError::PathNotSafe { .. })));
}

#[test]
fn test_safety_intermediate_component_is_file() {
    let temp_dir = setup_temp_dir();
    temp_dir
        .child("file_as_dir")
        .write_str("I am a file")
        .unwrap();
    // Try to create something inside the path where 'file_as_dir' is treated as a directory.
    let result = check_safety(&temp_dir, "file_as_dir/nested_file.txt");

    // This should fail because the parent ('file_as_dir') is not a directory.
    // The error can come from metadata check or canonicalization depending on exact flow.
    assert!(
        matches!(result, Err(ProcessError::ParentIsNotDirectory { .. }))
            || matches!(result, Err(ProcessError::Io { .. })) // Canonicalize might return generic IO error sometimes
            || matches!(result, Err(ProcessError::PathResolution { .. })), // Or path resolution error
        "Expected ParentIsNotDirectory or related error, got {:?}",
        result
    );
}

#[test]
#[cfg(unix)] // Test behavior with symlinks (more relevant on Unix)
fn test_safety_symlink_inside_base() {
    let temp_dir = setup_temp_dir();
    temp_dir.child("target_dir").create_dir_all().unwrap();
    temp_dir
        .child("target_dir/actual_file.txt")
        .write_str("actual")
        .unwrap();
    let link_path = temp_dir.path().join("safe_link");
    std::os::unix::fs::symlink("target_dir/actual_file.txt", &link_path)
        .expect("Failed to create symlink");

    // Checking the link itself should be safe as it resides within base
    assert!(check_safety(&temp_dir, "safe_link").is_ok());
    // Checking a path *through* the link should also be safe if target is safe
    assert!(check_safety(&temp_dir, "safe_link").is_ok()); // ensure_path_safe canonicalizes
}

#[test]
#[cfg(unix)]
fn test_safety_symlink_points_outside_base() {
    let temp_dir = setup_temp_dir();
    let sibling_dir = temp_dir
        .path()
        .parent()
        .unwrap()
        .join("strux_test_sibling_target");
    // Use std::fs or fs_err for consistency within unit tests if needed elsewhere
    std::fs::create_dir_all(&sibling_dir).expect("Failed to create sibling dir");
    std::fs::write(sibling_dir.join("outside_file.txt"), "outside")
        .expect("Failed to write outside file");

    let link_path = temp_dir.path().join("unsafe_link");
    // Create a link pointing outside the temp dir
    std::os::unix::fs::symlink("../strux_test_sibling_target/outside_file.txt", &link_path)
        .expect("Failed to create symlink");

    // Accessing the link itself is okay (it lives inside base)
    // But ensure_path_safe canonicalizes, revealing the unsafe target.
    let result = check_safety(&temp_dir, "unsafe_link");
    assert!(matches!(result, Err(ProcessError::PathNotSafe { .. })));

    // Clean up the sibling directory
    std::fs::remove_dir_all(&sibling_dir).expect("Failed to remove sibling dir");
}
