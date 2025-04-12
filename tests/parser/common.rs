//! Common helpers for parser integration tests.

use strux::core_types::{Action, ActionType};

// --- Test Helpers ---

pub fn assert_action(
    action: Option<&Action>,
    expected_type: ActionType,
    expected_path: &str,
    expected_content: Option<&str>,
) {
    let action = action.expect("Expected an action, but found None");
    assert_eq!(action.action_type, expected_type, "Action type mismatch");
    assert_eq!(action.path, expected_path, "Action path mismatch");
    assert_eq!(
        action.content.as_deref(),
        expected_content,
        "Action content mismatch"
    );
}
