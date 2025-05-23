//! Common helpers for processor integration tests.

use strux::core_types::Summary;

// --- Test Helpers ---

#[allow(clippy::too_many_arguments)]
pub fn assert_summary(
    summary: &Summary,
    created: u32,
    overwritten: u32,
    deleted: u32,
    moved: u32,
    moved_overwritten: u32,
    skipped_exists: u32,
    skipped_not_found: u32, // For Delete actions
    skipped_isdir_delete: u32,
    skipped_move_src_not_found: u32,
    skipped_move_src_is_dir: u32,
    skipped_move_dst_exists: u32,
    skipped_move_dst_isdir: u32,
    failed_unsafe: u32,
    failed_isdir_create: u32, // For Create actions
    failed_parent_isdir: u32,
    failed_io: u32,
    // Add other skipped/failed fields if they become distinct for Move
) {
    assert_eq!(summary.created, created, "Summary: created mismatch");
    assert_eq!(
        summary.overwritten, overwritten,
        "Summary: overwritten mismatch"
    );
    assert_eq!(summary.deleted, deleted, "Summary: deleted mismatch");
    assert_eq!(summary.moved, moved, "Summary: moved mismatch");
    assert_eq!(
        summary.moved_overwritten, moved_overwritten,
        "Summary: moved_overwritten mismatch"
    );
    assert_eq!(
        summary.skipped_exists, skipped_exists,
        "Summary: skipped_exists (create) mismatch"
    );
    assert_eq!(
        summary.skipped_not_found, skipped_not_found,
        "Summary: skipped_not_found (delete) mismatch"
    );
    assert_eq!(
        summary.skipped_isdir_delete, skipped_isdir_delete,
        "Summary: skipped_isdir_delete mismatch"
    );
    assert_eq!(
        summary.skipped_move_src_not_found, skipped_move_src_not_found,
        "Summary: skipped_move_src_not_found mismatch"
    );
    assert_eq!(
        summary.skipped_move_src_is_dir, skipped_move_src_is_dir,
        "Summary: skipped_move_src_is_dir mismatch"
    );
    assert_eq!(
        summary.skipped_move_dst_exists, skipped_move_dst_exists,
        "Summary: skipped_move_dst_exists mismatch"
    );
    assert_eq!(
        summary.skipped_move_dst_isdir, skipped_move_dst_isdir,
        "Summary: skipped_move_dst_isdir mismatch"
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
    // Assert other fields like summary.skipped_other_type and summary.error_other if needed
    // For example, if they should always be 0 for these specific tests:
    assert_eq!(
        summary.skipped_other_type, 0,
        "Summary: skipped_other_type should be 0"
    );
    assert_eq!(summary.error_other, 0, "Summary: error_other should be 0");
}
