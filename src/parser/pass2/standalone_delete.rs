//! Handles standalone delete actions found in Pass 2.

// Removed unused import: crate::constants::ACTION_DELETED_FILE;
use crate::core_types::{Action, ActionType};
// Removed unused imports: OPENING_FENCE_REGEX, HashSet

/// Handles logic for adding a standalone delete action found in Pass 2.
pub(crate) fn handle_standalone_delete(
    // Removed unused arguments related to the flawed check
    original_header_pos: usize,
    path: &str,
    actions_with_pos: &mut Vec<(usize, Action)>,
) {
    // DEBUG: Log function entry (Remove this line)
    // println!("    [Standalone Delete] Handling pos={}, path='{}'", original_header_pos, path);

    // Pass 2 simply adds any valid standalone delete header found that wasn't processed in Pass 1.
    // The special "## Deleted File:" + block case is fully handled in Pass 1 now.
    // Check for duplicates before adding.
    if !actions_with_pos
        .iter()
        .any(|(_, a)| a.action_type == ActionType::Delete && a.path == path)
    {
        println!(
            "  - Found standalone DELETE action for: '{}' at original pos {}",
            path, original_header_pos
        );
        actions_with_pos.push((
            original_header_pos,
            Action {
                action_type: ActionType::Delete,
                path: path.to_string(),
                content: None,
                original_pos: original_header_pos,
            },
        ));
        // Mark header as processed? No, Pass 2 iterates once.
    } else {
        println!("  - Info: Duplicate standalone DELETE action found for '{}' at original pos {}. Ignoring.", path, original_header_pos);
    }
}
