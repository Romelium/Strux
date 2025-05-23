//! Logic for Pass 1 of markdown parsing: Associating code blocks with headers.

use crate::core_types::Action;
use crate::errors::ParseError;
use crate::parser::regex::OPENING_FENCE_REGEX;
use std::collections::HashSet;

// Declare submodules for Pass 1
mod action_adder;
mod action_determiner;
mod block_processor;
mod external_delete_special;
mod external_header;
mod fence_finder;
#[cfg(test)] // Conditionally compile the test module
mod fence_finder_tests; // ADDED test module
mod internal_comment_handler;
mod internal_header;
mod internal_standard_handler;
mod types;
mod utils;
mod wrapped_create_handler;
mod wrapped_header;

/// Executes Pass 1: Find code blocks and associate Create/Delete(special) actions.
#[allow(clippy::too_many_arguments)] // Necessary complexity for state passing
pub(crate) fn run_pass1(
    content_to_parse: &str,
    parse_offset: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &mut HashSet<usize>,
    processed_code_block_ranges: &mut HashSet<(usize, usize)>,
) -> Result<(), ParseError> {
    let mut current_search_pos = 0;
    while let Some(caps) = OPENING_FENCE_REGEX.captures_at(content_to_parse, current_search_pos) {
        let full_match = caps.get(0).unwrap();
        let fence_start_pos = full_match.start(); // Relative to content_to_parse

        if utils::is_already_processed(fence_start_pos, processed_code_block_ranges) {
            // This block was already processed (e.g. as content of a wrapped header)
            // or its opening fence was part of a processed block.
            // Advance search position past this opening fence's start to avoid re-matching it.
            current_search_pos = fence_start_pos + 1;
            // println!( // Optional debug
            //     "    Skipping already processed or overlapping fence at relative pos {}. Advancing search to {}.",
            //     fence_start_pos, current_search_pos
            // );
            continue;
        }

        let fence_end_pos = full_match.end(); // End of the opening fence line
        let fence_chars = caps.name("fence").unwrap().as_str();
        let lang = caps.name("lang").map(|m| m.as_str().trim()).unwrap_or("");

        let closing_match_opt =
            fence_finder::find_closing_fence(content_to_parse, fence_chars, fence_end_pos);

        if closing_match_opt.is_none() {
            let original_pos = fence_start_pos + parse_offset;
            eprintln!(
                "Warning: Opening fence '{}' at original pos {} has no closing fence. Skipping.",
                fence_chars, original_pos
            );
            current_search_pos = fence_end_pos; // Advance past this unclosed opening fence line
            continue;
        }
        let closing_match = closing_match_opt.unwrap();

        let block_content_start = fence_end_pos;
        let block_content_end = closing_match.start();
        let block_outer_end = closing_match.end(); // End of the closing fence line
        let original_block_start = fence_start_pos + parse_offset;

        println!(
            "  - Found code block: '{}' (lang: '{}') from original pos {} to {}",
            fence_chars,
            if lang.is_empty() { "none" } else { lang },
            original_block_start,
            block_outer_end + parse_offset
        );

        block_processor::process_single_block(
            content_to_parse,
            parse_offset,
            fence_start_pos,
            block_content_start,
            block_content_end,
            block_outer_end,
            lang,
            original_block_start,
            actions_with_pos,
            processed_header_starts,
            processed_code_block_ranges,
        )?;

        current_search_pos = block_outer_end; // Advance search to after the current block
    }
    Ok(())
}
