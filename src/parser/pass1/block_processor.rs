//! Processes a single code block and its potential associated header in Pass 1.

use crate::core_types::Action;
use crate::errors::ParseError;
use crate::parser::pass1::{action_adder, external_header, internal_header, wrapped_header};
// Import type aliases - adjust the return type alias usage
use super::types::{BlockActionInfo, DeterminationResult}; // IMPORT THE TYPE ALIASES
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
    // Call determine_block_action and use ? to handle errors
    // CORRECT THE TYPE ANNOTATION using the imported alias
    let determination_result: Option<BlockActionInfo> = determine_block_action(
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

/// Determines the action type (external, wrapped, internal) for a given block.
/// Returns the Action, its relative start position, and its source ("external", "wrapped", "internal").
/// Returns Ok(None) if no action is associated with this block.
#[allow(clippy::too_many_arguments)]
fn determine_block_action(
    content_to_parse: &str,
    parse_offset: usize,
    fence_start_pos: usize,
    block_content_start: usize,
    block_content_end: usize,
    block_outer_end: usize,
    lang: &str,
    processed_header_starts: &mut HashSet<usize>,
    processed_code_block_ranges: &mut HashSet<(usize, usize)>,
) -> DeterminationResult {
    // APPLY THE DeterminationResult TYPE ALIAS HERE
    // Check External Header FIRST
    if let Some((action, header_pos)) = external_header::handle_external_header(
        content_to_parse,
        fence_start_pos,
        block_content_start,
        block_content_end,
        parse_offset,
    )? {
        return Ok(Some((action, header_pos, "external")));
    }

    // Check Wrapped Header (only if lang is markdown/md)
    if lang == "markdown" || lang == "md" {
        match wrapped_header::handle_wrapped_header(
            content_to_parse,
            parse_offset,
            fence_start_pos,
            block_content_start,
            block_content_end,
            block_outer_end,
            processed_code_block_ranges,
        )? {
            Some((action, header_pos, next_block_range)) => {
                // Mark *both* blocks as processed for wrapped actions
                processed_code_block_ranges.insert((fence_start_pos, block_outer_end));
                if next_block_range != (0, 0) {
                    // Check if it was a create action that needs the next block range marked
                    processed_code_block_ranges.insert(next_block_range);
                }
                return Ok(Some((action, header_pos, "wrapped")));
            }
            None => {
                // If it was a markdown block but didn't result in a wrapped action,
                // mark it as processed so it's not considered again later.
                processed_code_block_ranges.insert((fence_start_pos, block_outer_end));
                // Continue to check internal headers (though unlikely for md blocks)
            }
        }
    }

    // Check Internal Headers (if lang is not markdown)
    if !(lang == "markdown" || lang == "md") {
        if let Some((action, header_pos)) = internal_header::handle_internal_header(
            content_to_parse,
            block_content_start,
            block_content_end,
            parse_offset,
            processed_header_starts,
        )? {
            return Ok(Some((action, header_pos, "internal")));
        }
    }

    // No action found from any source
    Ok(None)
}
