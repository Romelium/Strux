//! Common helpers for processor integration tests.

use markdown_processor::core_types::Summary;

// --- Test Helpers ---

#[allow(clippy::too_many_arguments)]
pub fn assert_summary(
    summary: &Summary,
    created: u32,
    overwritten: u32,
    deleted: u32,
    skipped_exists: u32,
    skipped_not_found: u32,
    skipped_isdir_delete: u32,
    failed_unsafe: u32,
    failed_isdir_create: u32,
    failed_parent_isdir: u32,
    failed_io: u32,
) {
    assert_eq!(summary.created, created, "Summary: created mismatch");
    assert_eq!(
        summary.overwritten, overwritten,
        "Summary: overwritten mismatch"
    );
    assert_eq!(summary.deleted, deleted, "Summary: deleted mismatch");
    assert_eq!(
        summary.skipped_exists, skipped_exists,
        "Summary: skipped_exists mismatch"
    );
    assert_eq!(
        summary.skipped_not_found, skipped_not_found,
        "Summary: skipped_not_found mismatch"
    );
    assert_eq!(
        summary.skipped_isdir_delete, skipped_isdir_delete,
        "Summary: skipped_isdir_delete mismatch"
    );
    assert_eq!(
        summary.failed_unsafe, failed_unsafe,
        "Summary: failed_unsafe mismatch"
    );
    assert_eq!(
        summary.failed_isdir_create, failed_isdir_create,
        "Summary: failed_isdir_create mismatch"
    );
    assert_eq!(
        summary.failed_parent_isdir, failed_parent_isdir,
        "Summary: failed_parent_isdir mismatch"
    );
    assert_eq!(summary.failed_io, failed_io, "Summary: failed_io mismatch");
}
