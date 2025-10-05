//! Handles adding a successfully parsed action to the list.

use crate::core_types::Action;
use std::collections::HashSet;

/// Adds the action, sets its final original position, and marks the header as processed.
pub(crate) fn add_action(
    mut action: Action,
    header_start_rel: usize,
    action_source: &str,
    parse_offset: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &mut HashSet<usize>,
) {
    let original_pos = header_start_rel + parse_offset;
    // Ensure original_pos wasn't already set by wrapped header logic
    if action.original_pos == 0 {
        action.original_pos = original_pos;
    }
    println!(
        "    -> Adding action from source '{}' with original_pos {}",
        action_source, action.original_pos
    );
    actions_with_pos.push((action.original_pos, action)); // Use final original_pos for sorting
    processed_header_starts.insert(original_pos); // Mark header associated with action
}
