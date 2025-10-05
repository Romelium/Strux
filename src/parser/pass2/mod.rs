//! Logic for Pass 2 of markdown parsing: Finding unassociated content headers and linking them forward.

use crate::core_types::Action;
use crate::errors::ParseError;
use crate::parser::header_utils::{extract_header_action_details, get_action_type};
use crate::parser::helpers::ensure_trailing_newline;
use crate::parser::pass1::{fence_finder, utils as pass1_utils}; // Reuse utils from pass1
use crate::parser::regex::HEADER_REGEX;
use std::collections::HashSet;

/// Executes Pass 2: Find unassociated headers for content actions (`File`, `Append`, `Prepend`)
/// and link them to the next available code block.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_pass2(
    content_to_parse: &str,
    parse_offset: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &mut HashSet<usize>,
    processed_code_block_ranges: &mut HashSet<(usize, usize)>,
) -> Result<(), ParseError> {
    for caps in HEADER_REGEX.captures_iter(content_to_parse) {
        let header_match = caps.get(0).unwrap(); // The whole match
        let header_start_rel = header_match.start();
        let original_header_pos = header_start_rel + parse_offset;

        // Skip if this header was already processed in Pass 1
        if processed_header_starts.contains(&original_header_pos) {
            continue;
        }

        // Skip if this header is inside a code block that was processed in Pass 1
        if pass1_utils::is_already_processed(header_start_rel, processed_code_block_ranges) {
            continue;
        }

        if let Some(details) = extract_header_action_details(&caps) {
            if let Some(action_type) = get_action_type(&details.action_word) {
                match action_type {
                    crate::core_types::ActionType::Create
                    | crate::core_types::ActionType::Append
                    | crate::core_types::ActionType::Prepend => {
                        // This is an orphaned content header. Let's find its block.
                        println!(
                            "  - Found unassociated content header for '{}'. Searching for next code block...",
                            details.path
                        );

                        // Find the next available code block after this header
                        if let Some(next_fence_caps) = pass1_utils::find_next_opening_fence(
                            content_to_parse,
                            header_match.end(), // Start search after the header
                            processed_code_block_ranges,
                        ) {
                            // We found a candidate block.
                            let next_fence_match = next_fence_caps.get(0).unwrap();
                            let next_fence_start = next_fence_match.start();
                            let next_fence_end = next_fence_match.end();
                            let next_fence_chars = next_fence_caps.name("fence").unwrap().as_str();

                            if let Some(next_closing_match) = fence_finder::find_closing_fence(
                                content_to_parse,
                                next_fence_chars,
                                next_fence_end,
                            ) {
                                // Successfully found a complete, unprocessed block. Associate it.
                                println!(
                                    "    -> Associated with code block at relative pos {}",
                                    next_fence_start
                                );

                                let next_content_start = next_fence_end;
                                let next_content_end = next_closing_match.start();
                                let next_outer_end = next_closing_match.end();

                                let mut block_data = content_to_parse
                                    [next_content_start..next_content_end]
                                    .to_string();
                                ensure_trailing_newline(&mut block_data);

                                let action = Action {
                                    action_type,
                                    path: details.path,
                                    dest_path: None,
                                    content: Some(block_data),
                                    original_pos: original_header_pos,
                                };
                                actions_with_pos.push((original_header_pos, action));

                                // Mark both as processed so they aren't picked up again
                                processed_header_starts.insert(original_header_pos);
                                processed_code_block_ranges
                                    .insert((next_fence_start, next_outer_end));
                            } else {
                                // Found an opening fence but it was unclosed.
                                eprintln!(
                                    "Warning: Found header '{}' for path '{}' but the next code block was unclosed. Skipping.",
                                    header_match.as_str().trim(), details.path
                                );
                            }
                        } else {
                            // No subsequent code block found for this header.
                            eprintln!(
                                "Warning: Found header '{}' for path '{}' at original pos {} without an associated code block. Skipping.",
                                header_match.as_str().trim(), details.path, original_header_pos
                            );
                        }
                    }
                    // Delete and Move actions will be handled in Pass 3
                    crate::core_types::ActionType::Delete | crate::core_types::ActionType::Move => {
                    }
                }
            }
        }
    }
    Ok(())
}
