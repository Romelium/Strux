//! Handles the specific logic for a wrapped 'Create' header in Pass 1.

use crate::core_types::{Action, ActionType};
// Removed unused ParseError import
// use crate::errors::ParseError;
use crate::parser::helpers::ensure_trailing_newline;
use crate::parser::pass1::{fence_finder, utils};
// Import type aliases
use super::types::WrappedActionResult;
use std::collections::HashSet;

/// Handles the specific logic for a wrapped 'Create' header.
/// Looks for the next adjacent code block and associates the header with it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_wrapped_create(
    content_to_parse: &str,
    parse_offset: usize,
    fence_start_pos: usize,
    block_outer_end: usize,
    path: &str,
    potential_header_line: &str,
    processed_code_block_ranges: &mut HashSet<(usize, usize)>,
) -> WrappedActionResult {
    // Use type alias here
    // Look for the *next* code block immediately after this one
    if let Some(next_fence_caps) = utils::find_next_opening_fence(
        content_to_parse,
        block_outer_end, // Start search after *this* md block
        processed_code_block_ranges,
    ) {
        let next_fence_match = next_fence_caps.get(0).unwrap();
        // Check if it's immediately adjacent (allow whitespace lines between)
        let gap = &content_to_parse[block_outer_end..next_fence_match.start()];
        if gap.trim().is_empty() {
            println!(
                "    Found wrapped header '{}' associated with the following code block.",
                potential_header_line
            );

            // Now process the *next* block using the header info we just extracted
            let next_fence_start = next_fence_match.start();
            let next_fence_end = next_fence_match.end();
            let next_fence_chars = next_fence_caps.name("fence").unwrap().as_str();

            if let Some(next_closing_match) =
                fence_finder::find_closing_fence(content_to_parse, next_fence_chars, next_fence_end)
            {
                let next_content_start = next_fence_end;
                let next_content_end = next_closing_match.start();
                let next_outer_end = next_closing_match.end();

                let mut block_data =
                    content_to_parse[next_content_start..next_content_end].to_string();
                ensure_trailing_newline(&mut block_data);

                // Create the action using the wrapped header info
                let action = Action {
                    action_type: ActionType::Create,
                    path: path.to_string(), // Clone path here
                    content: Some(block_data),
                    // Use original_pos of the markdown header block
                    original_pos: fence_start_pos + parse_offset,
                };
                let next_block_range = (next_fence_start, next_outer_end);
                println!(
                    "     -> Added CREATE action for '{}' from wrapped header.",
                    path // Use original path for println
                );
                return Ok(Some((action, fence_start_pos, next_block_range)));
            } else {
                eprintln!("Warning: Found wrapped Create header '{}' but the following code block is unclosed. Skipping.", potential_header_line);
            }
        } else {
            eprintln!("Warning: Found wrapped Create header '{}' but it's not immediately followed by a code block (gap='{}'). Skipping.", potential_header_line, gap.escape_debug());
        }
    } else {
        eprintln!("Warning: Found wrapped Create header '{}' but no subsequent code block found. Skipping.", potential_header_line);
    }
    Ok(None)
}
