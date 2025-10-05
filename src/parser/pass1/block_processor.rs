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
    all_code_block_ranges: &mut HashSet<(usize, usize)>,
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
    let action_found = determination_result.is_some();

    if let Some((action, header_pos_rel, action_source)) = determination_result {
        action_adder::add_action(
            action,
            header_pos_rel,
            action_source,
            parse_offset,
            actions_with_pos,
            processed_header_starts,
        );
    } else {
        // The block was unassociated in this pass. Log it.
        println!(
            "    Code block at original pos {} has no associated action header (checked external, wrapped, internal). Leaving for Pass 2.",
            original_block_start
        );
    }

    // Always record that this range is a code block.
    all_code_block_ranges.insert((fence_start_pos, block_outer_end));

    // ONLY record the block range if an action was found for it in this pass.
    // Wrapped actions mark their content blocks inside the determiner, so this
    // correctly handles external/internal actions.
    if action_found && !processed_code_block_ranges.contains(&(fence_start_pos, block_outer_end)) {
        processed_code_block_ranges.insert((fence_start_pos, block_outer_end));
    }

    Ok(())
}

// --- Moved to action_determiner.rs ---
// determine_block_action
