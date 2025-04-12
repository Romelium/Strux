//! Processes a single code block and its potential associated header in Pass 1.

use crate::core_types::Action;
use crate::errors::ParseError;
// Import the new action_determiner module
use crate::parser::pass1::{action_adder, action_determiner};
// Import type aliases - adjust the return type alias usage
use super::types::BlockActionInfo; // IMPORT THE TYPE ALIASES
use std::collections::HashSet;

/// Processes a single code block: determines action type and adds it or logs skip.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_single_block(
    content_to_parse: &str,
    parse_offset: usize,
    fence_start_pos: usize,
    block_content_start: usize,
    block_content_end: usize,
    block_outer_end: usize,
    lang: &str,
    original_block_start: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &mut HashSet<usize>,
    processed_code_block_ranges: &mut HashSet<(usize, usize)>,
) -> Result<(), ParseError> {
    // Determine the action type and source associated with this block
    // Call determine_block_action from the new module
    let determination_result: Option<BlockActionInfo> = action_determiner::determine_block_action(
        content_to_parse,
        parse_offset,
        fence_start_pos,
        block_content_start,
        block_content_end,
        block_outer_end,
        lang,
        processed_header_starts,
        processed_code_block_ranges,
    )?; // Use ? here

    // Process the result
    match determination_result {
        // Destructuring the tuple works the same with the type alias
        Some((action, header_pos_rel, action_source)) => {
            // Add action if found and log skips
            action_adder::add_action_or_log_skip(
                Some(action), // Pass action
                Some(header_pos_rel),
                action_source,
                parse_offset,
                block_content_start,
                original_block_start,
                actions_with_pos,
                processed_header_starts,
            );
        }
        None => {
            // No action associated with this block from any known source
            action_adder::add_action_or_log_skip(
                None,
                None,
                "unknown", // Mark source as unknown
                parse_offset,
                block_content_start,
                original_block_start,
                actions_with_pos,
                processed_header_starts,
            );
        }
    }

    // Always record the block range if we successfully found opening and closing fences,
    // unless it was already added by the wrapped header logic pairing it with *this* block.
    if !processed_code_block_ranges.contains(&(fence_start_pos, block_outer_end)) {
        processed_code_block_ranges.insert((fence_start_pos, block_outer_end));
    }

    Ok(())
}

// --- Moved to action_determiner.rs ---
// determine_block_action
