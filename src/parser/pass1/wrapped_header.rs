//! Handles wrapped headers (header inside ```markdown block) in Pass 1.

use crate::core_types::{Action, ActionType};
// Removed unused ParseError import
// use crate::errors::ParseError;
use crate::parser::header_utils::{extract_action_path_from_captures, get_action_type};
use crate::parser::pass1::wrapped_create_handler; // Import new handler
use crate::parser::path_utils::validate_path_format;
use crate::parser::regex::HEADER_REGEX;
// Import type aliases
use super::types::WrappedActionResult;
use std::collections::HashSet;

/// Handles wrapped headers (header inside ```markdown block).
#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_wrapped_header(
    content_to_parse: &str,
    parse_offset: usize,
    fence_start_pos: usize,
    block_content_start: usize,
    block_content_end: usize,
    block_outer_end: usize,
    processed_code_block_ranges: &mut HashSet<(usize, usize)>,
) -> WrappedActionResult {
    // Use type alias here
    // Returns (Action, header_start_pos_rel, next_block_range)
    let md_block_content = &content_to_parse[block_content_start..block_content_end];
    let trimmed_lines: Vec<&str> = md_block_content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    if trimmed_lines.len() == 1 {
        let potential_header_line = trimmed_lines[0];
        println!(
            "    Checking single line inside markdown block: '{}'",
            potential_header_line
        );
        if let Some(header_caps) = HEADER_REGEX.captures(potential_header_line) {
            if let Some((action_word, path)) = extract_action_path_from_captures(&header_caps) {
                if validate_path_format(&path).is_err() {
                    eprintln!(
                        "Warning: Invalid path format in wrapped header '{}'. Skipping.",
                        potential_header_line
                    );
                    return Ok(None);
                }

                if let Some(action_type) = get_action_type(&action_word) {
                    match action_type {
                        ActionType::Create => {
                            // Delegate to specific handler
                            return wrapped_create_handler::handle_wrapped_create(
                                content_to_parse,
                                parse_offset,
                                fence_start_pos,
                                block_outer_end,
                                &path,
                                potential_header_line,
                                processed_code_block_ranges,
                            );
                        }
                        ActionType::Delete => {
                            println!("    Found wrapped standalone DELETE action for: '{}'", path);
                            let action = Action {
                                action_type: ActionType::Delete,
                                path, // Path is moved here
                                content: None,
                                original_pos: fence_start_pos + parse_offset,
                            };
                            // No next block range needed for delete, return dummy range
                            return Ok(Some((action, fence_start_pos, (0, 0))));
                        }
                    }
                } else {
                    println!("    Single line in markdown block ('{}') did not contain a valid action word.", potential_header_line);
                }
            } else {
                println!(
                    "    Single line in markdown block ('{}') did not yield a valid path.",
                    potential_header_line
                );
            }
        } else {
            println!(
                "    Single line in markdown block ('{}') did not match HEADER_REGEX.",
                potential_header_line
            );
        }
    } else {
        println!(
            "    Markdown block does not contain exactly one non-empty line (found {}). Treating as regular content.",
            trimmed_lines.len()
        );
    }
    Ok(None)
}
