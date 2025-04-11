//! Logic for Pass 1 of markdown parsing: Associating code blocks with headers.

use crate::core_types::Action;
use crate::errors::ParseError;
use crate::parser::regex::OPENING_FENCE_REGEX;
use std::collections::HashSet;

// Declare submodules for Pass 1
mod external_header;
mod internal_header;
mod utils;

/// Executes Pass 1: Find code blocks and associate Create/Patch/Delete(special) actions.
#[allow(clippy::too_many_arguments)] // Necessary complexity for state passing
pub(crate) fn run_pass1(
    content_to_parse: &str,
    parse_offset: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &mut HashSet<usize>,
    processed_code_block_ranges: &mut HashSet<(usize, usize)>,
) -> Result<(), ParseError> {
    // Use captures_iter to get named groups
    for caps in OPENING_FENCE_REGEX.captures_iter(content_to_parse) {
        // Get the full match for positional info
        let full_match = caps.get(0).unwrap();
        let fence_start_pos = full_match.start(); // Relative to content_to_parse

        if utils::is_already_processed(fence_start_pos, processed_code_block_ranges) {
            continue;
        }

        let fence_end_pos = full_match.end();
        // Get the captured fence characters using the name
        let fence_chars = caps.name("fence").unwrap().as_str();

        let closing_match = match utils::find_closing_fence(
            content_to_parse,
            fence_chars,
            fence_end_pos,
        ) {
            Some(m) => m,
            None => {
                let original_pos = fence_start_pos + parse_offset;
                eprintln!("Warning: Opening fence '{}' at original pos {} has no closing fence. Skipping.", fence_chars, original_pos);
                continue; // Skip this block entirely
            }
        };

        let block_content_start = fence_end_pos;
        let block_content_end = closing_match.start();
        let block_outer_end = closing_match.end();
        let original_block_start = fence_start_pos + parse_offset;

        println!(
            "  - Found code block: '{}' from original pos {} to {}",
            fence_chars,
            original_block_start,
            block_outer_end + parse_offset
        );

        let mut current_action: Option<Action> = None;
        let mut header_line_start_pos_rel: Option<usize> = None;
        let mut processed_this_block = false;

        // --- Check for External Header ---
        if let Some((action, header_pos)) = external_header::handle_external_header(
            content_to_parse,
            fence_start_pos,
            block_content_start,
            block_content_end,
            parse_offset,
        )? {
            current_action = Some(action);
            header_line_start_pos_rel = Some(header_pos);
            processed_this_block = true;
            // Mark header as processed (done below if action is Some)
        }

        // --- Check for Internal Headers if not handled externally ---
        if !processed_this_block {
            // Pass processed_header_starts to internal handler
            if let Some((action, header_pos)) = internal_header::handle_internal_header(
                content_to_parse,
                block_content_start,
                block_content_end,
                parse_offset,
                processed_header_starts, // Pass the mutable set here
            )? {
                current_action = Some(action);
                header_line_start_pos_rel = Some(header_pos);
                // Don't set processed_this_block=true here, handled by action presence below
                // Header processing (marking) is done inside handle_internal_header now
            }
            // Note: handle_internal_header now updates processed_header_starts directly
            // even if it returns Ok(None) for an ignored internal delete.
        }

        // --- Add action if found ---
        if let (Some(mut action), Some(header_start_rel)) =
            (current_action, header_line_start_pos_rel)
        {
            let original_pos = header_start_rel + parse_offset;
            action.original_pos = original_pos;
            actions_with_pos.push((original_pos, action));
            processed_header_starts.insert(original_pos); // Mark header associated with action
        } else if !processed_this_block {
            // Check if the block was skipped because of an *ignored* internal header
            // (which would have been marked in processed_header_starts by handle_internal_header)
            let first_line_start = block_content_start;
            if !processed_header_starts.contains(&(first_line_start + parse_offset)) {
                println!(
                    "    Code block at original pos {} has no associated action header. Skipping.",
                    original_block_start
                );
            }
        }

        // Always record the block range if we successfully found opening and closing fences
        processed_code_block_ranges.insert((fence_start_pos, block_outer_end));
    }
    Ok(())
}
