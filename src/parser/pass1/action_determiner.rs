//! Determines the action type (external, wrapped, internal) for a given block in Pass 1.

use crate::parser::pass1::{external_header, internal_header, wrapped_header};
// Import type aliases
use super::types::DeterminationResult;
use std::collections::HashSet;

/// Determines the action type (external, wrapped, internal) for a given block.
/// Returns the Action, its relative start position, and its source ("external", "wrapped", "internal").
/// Returns Ok(None) if no action is associated with this block.
#[allow(clippy::too_many_arguments)]
pub(crate) fn determine_block_action(
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
        processed_header_starts,
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
