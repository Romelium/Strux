//! Helper functions to update the Summary struct based on processing outcomes.

use crate::core_types::{CreateStatus, DeleteStatus, Summary}; // Import enums directly
use crate::errors::{/* Removed PatchError */ ProcessError};

pub(crate) fn update_summary_create(summary: &mut Summary, status: CreateStatus) {
    match status {
        CreateStatus::Created => summary.created += 1,
        CreateStatus::Overwritten => summary.overwritten += 1,
        CreateStatus::SkippedExists => summary.skipped_exists += 1,
    }
}

pub(crate) fn update_summary_delete(summary: &mut Summary, status: DeleteStatus) {
    match status {
        DeleteStatus::Deleted => summary.deleted += 1,
        DeleteStatus::SkippedNotFound => summary.skipped_not_found += 1,
        DeleteStatus::SkippedIsDir => summary.skipped_isdir_delete += 1,
        DeleteStatus::SkippedOtherType => summary.skipped_other_type += 1,
    }
}

// Removed: update_summary_patch function

pub(crate) fn update_summary_error(summary: &mut Summary, error: ProcessError) {
    match error {
        ProcessError::Io { .. } | ProcessError::PathResolution { .. } => summary.failed_io += 1,
        ProcessError::PathNotSafe { .. } | ProcessError::InvalidPathFormat { .. } => {
            summary.failed_unsafe += 1
        }
        ProcessError::TargetIsDirectory { .. } => summary.failed_isdir_create_patch += 1, // Renamed field
        ProcessError::ParentIsNotDirectory { .. } => summary.failed_parent_isdir += 1,
        ProcessError::UnknownAction | ProcessError::Internal(_) => summary.error_other += 1,
        // Removed Patch error cases
        // ProcessError::TargetIsNotFile { .. } => summary.failed_patch_target_not_file += 1,
        // ProcessError::Patch { source: PatchError::InvalidFormat(_), .. } => summary.failed_patch_format += 1,
        // ProcessError::Patch { source: PatchError::ContextNotFound, .. } => summary.failed_patch_context += 1,
        // ProcessError::Patch { source: PatchError::AmbiguousContext(_), .. } => summary.failed_patch_ambiguous += 1,
        // ProcessError::Patch { source: PatchError::TargetMissing, .. } => summary.failed_patch_target_missing += 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_types::Summary;
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
        assert_eq!(summary.failed_isdir_create_patch, 1);

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
}
