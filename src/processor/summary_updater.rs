//! Helper functions to update the Summary struct based on processing outcomes.

use crate::core_types::{CreateStatus, DeleteStatus, Summary}; // Import enums directly
use crate::errors::ProcessError;

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

pub(crate) fn update_summary_error(summary: &mut Summary, error: ProcessError) {
    match error {
        ProcessError::Io { .. } | ProcessError::PathResolution { .. } => summary.failed_io += 1,
        ProcessError::PathNotSafe { .. } | ProcessError::InvalidPathFormat { .. } => {
            summary.failed_unsafe += 1
        }
        ProcessError::TargetIsDirectory { .. } => summary.failed_isdir_create += 1, // Renamed field
        ProcessError::ParentIsNotDirectory { .. } => summary.failed_parent_isdir += 1,
        ProcessError::UnknownAction | ProcessError::Internal(_) => summary.error_other += 1,
    }
}

// --- Tests moved to summary_updater_tests.rs ---
// #[cfg(test)]
// mod tests { ... }
