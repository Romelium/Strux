//! Handles adding a successfully parsed action to the list or logging skips in Pass 1.

use crate::core_types::Action;
use std::collections::HashSet;

/// Adds the action if found, sets its original position, and marks the header as processed.
/// Logs a skip message if no action was found and the block wasn't skipped for other reasons.
#[allow(clippy::too_many_arguments)]
pub(crate) fn add_action_or_log_skip(
    current_action: Option<Action>,
    header_line_start_pos_rel: Option<usize>,
    action_source: &str,
    parse_offset: usize,
    block_content_start: usize,
    original_block_start: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &mut HashSet<usize>,
) {
    if let (Some(mut action), Some(header_start_rel)) = (current_action, header_line_start_pos_rel)
    {
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
    } else if action_source == "unknown" {
        // Only log skip if no action was attempted from any source.
        // Check if the block was skipped because of an *ignored* internal header
        // (which would have been marked in processed_header_starts by handle_internal_header).
        let first_line_start_original = block_content_start + parse_offset;
        if !processed_header_starts.contains(&first_line_start_original) {
            println!(
                "    Code block at original pos {} has no associated action header (checked external, wrapped, internal). Skipping.",
                original_block_start
            );
        }
    }
    // If action_source is known but current_action is None, it means a potential header
    // was found but ignored (e.g., invalid path, wrapped header without next block),
    // and the warning/info message was already printed by the respective handler.
}
