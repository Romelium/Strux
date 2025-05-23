//! Common helpers for parser integration tests.

use strux::core_types::{Action, ActionType};

// --- Test Helpers ---

#[allow(clippy::too_many_arguments)]
pub fn assert_action(
    action: Option<&Action>,
    expected_type: ActionType,
    expected_path: &str,              // Source path for Move, target for others
    expected_dest_path: Option<&str>, // Destination path for Move
    expected_content: Option<&str>,
) {
    let action = action.expect("Expected an action, but found None");
    assert_eq!(
        action.action_type, expected_type,
        "Action type mismatch for path: {}",
        action.path
    );
    assert_eq!(
        action.path, expected_path,
        "Action path (source for Move, target for others) mismatch"
    );
    assert_eq!(
        action.dest_path.as_deref(),
        expected_dest_path,
        "Action destination path mismatch for source/target: {}",
        action.path
    );
    assert_eq!(
        action.content.as_deref(),
        expected_content,
        "Action content mismatch for path: {}",
        action.path
    );
}
