//! Helper functions to update the Summary struct based on processing outcomes.

use crate::core_types::{CreateStatus, DeleteStatus, MoveStatus, Summary}; // Import enums directly
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

pub(crate) fn update_summary_move(summary: &mut Summary, status: MoveStatus) {
    match status {
        MoveStatus::Moved => summary.moved += 1,
        MoveStatus::MovedOverwritten => summary.moved_overwritten += 1,
        MoveStatus::SkippedSourceNotFound => summary.skipped_move_src_not_found += 1,
        MoveStatus::SkippedSourceIsDir => summary.skipped_move_src_is_dir += 1,
        MoveStatus::SkippedDestinationExists => summary.skipped_move_dst_exists += 1,
        MoveStatus::SkippedDestinationIsDir => summary.skipped_move_dst_isdir += 1,
    }
}

pub(crate) fn update_summary_error(summary: &mut Summary, error: ProcessError) {
    match error {
        ProcessError::Io { .. } | ProcessError::PathResolution { .. } => summary.failed_io += 1,
        ProcessError::PathNotSafe { .. } | ProcessError::InvalidPathFormat { .. } => {
            summary.failed_unsafe += 1
        }
        ProcessError::TargetIsDirectory { .. } => summary.failed_isdir_create += 1,
        ProcessError::ParentIsNotDirectory { .. } => summary.failed_parent_isdir += 1,
        ProcessError::MoveSourceIsDir { .. } => summary.skipped_move_src_is_dir += 1, // Map specific error to skip count
        ProcessError::UnknownAction | ProcessError::Internal(_) => summary.error_other += 1,
    }
}

// --- Tests moved to summary_updater_tests.rs ---
// #[cfg(test)]
// mod tests { ... }
