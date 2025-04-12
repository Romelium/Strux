//! Unit tests for summary_updater.rs functionality.

// Bring items from the specific module being tested into scope
use super::summary_updater::*; // Use the specific module name

// Bring items from other modules needed for tests into scope
use crate::core_types::{CreateStatus, DeleteStatus, Summary};
use crate::errors::ProcessError;
use std::io;
use std::path::PathBuf;

fn empty_summary() -> Summary {
    Summary::default()
}

#[test]
fn test_update_summary_create() {
    let mut summary = empty_summary();
    update_summary_create(&mut summary, CreateStatus::Created);
    assert_eq!(summary.created, 1);
    update_summary_create(&mut summary, CreateStatus::Overwritten);
    assert_eq!(summary.overwritten, 1);
    update_summary_create(&mut summary, CreateStatus::SkippedExists);
    assert_eq!(summary.skipped_exists, 1);
    assert_eq!(summary.deleted, 0); // Ensure others unchanged
}

#[test]
fn test_update_summary_delete() {
    let mut summary = empty_summary();
    update_summary_delete(&mut summary, DeleteStatus::Deleted);
    assert_eq!(summary.deleted, 1);
    update_summary_delete(&mut summary, DeleteStatus::SkippedNotFound);
    assert_eq!(summary.skipped_not_found, 1);
    update_summary_delete(&mut summary, DeleteStatus::SkippedIsDir);
    assert_eq!(summary.skipped_isdir_delete, 1);
    update_summary_delete(&mut summary, DeleteStatus::SkippedOtherType);
    assert_eq!(summary.skipped_other_type, 1);
    assert_eq!(summary.created, 0); // Ensure others unchanged
}

#[test]
fn test_update_summary_error() {
    let mut summary;

    summary = empty_summary();
    update_summary_error(
        &mut summary,
        ProcessError::Io {
            source: io::Error::new(io::ErrorKind::NotFound, "test"),
        },
    );
    assert_eq!(summary.failed_io, 1);

    summary = empty_summary();
    update_summary_error(
        &mut summary,
        ProcessError::PathResolution {
            path: PathBuf::new(),
            details: "".into(),
        },
    );
    assert_eq!(summary.failed_io, 1);

    summary = empty_summary();
    update_summary_error(
        &mut summary,
        ProcessError::PathNotSafe {
            resolved_path: PathBuf::new(),
            base_path: PathBuf::new(),
        },
    );
    assert_eq!(summary.failed_unsafe, 1);

    summary = empty_summary();
    update_summary_error(
        &mut summary,
        ProcessError::InvalidPathFormat { path: "".into() },
    );
    assert_eq!(summary.failed_unsafe, 1);

    summary = empty_summary();
    update_summary_error(
        &mut summary,
        ProcessError::TargetIsDirectory {
            path: PathBuf::new(),
        },
    );
    assert_eq!(summary.failed_isdir_create, 1);

    summary = empty_summary();
    update_summary_error(
        &mut summary,
        ProcessError::ParentIsNotDirectory {
            path: PathBuf::new(),
            parent_path: PathBuf::new(),
        },
    );
    assert_eq!(summary.failed_parent_isdir, 1);

    summary = empty_summary();
    update_summary_error(&mut summary, ProcessError::UnknownAction);
    assert_eq!(summary.error_other, 1);

    summary = empty_summary();
    update_summary_error(
        &mut summary,
        ProcessError::Internal("internal error".into()),
    );
    assert_eq!(summary.error_other, 1);

    // Ensure others unchanged
    assert_eq!(summary.created, 0);
    assert_eq!(summary.deleted, 0);
}
