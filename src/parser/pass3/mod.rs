//! Logic for Pass 3 of markdown parsing: Finding standalone actions.

use crate::core_types::Action;
use crate::errors::ParseError;
use crate::parser::header_utils::{extract_header_action_details, get_action_type};
use crate::parser::path_utils::validate_path_format;
use crate::parser::regex::HEADER_REGEX;
use std::collections::HashSet;

// Declare submodules for Pass 3
mod standalone_delete;

/// Executes Pass 3: Find standalone Delete/Move headers.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_pass3(
    content_to_parse: &str,
    parse_offset: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &HashSet<usize>, // Read-only access needed
    all_code_block_ranges: &HashSet<(usize, usize)>, // Use this to skip headers inside blocks
) -> Result<(), ParseError> {
    for caps in HEADER_REGEX.captures_iter(content_to_parse) {
        let header_match = caps.get(0).unwrap(); // The whole match
        let header_start_rel = header_match.start();
        let original_header_pos = header_start_rel + parse_offset;

        if processed_header_starts.contains(&original_header_pos) {
            continue;
        }

        let is_inside_block = all_code_block_ranges
            .iter()
            .any(|&(block_start, block_end)| {
                header_start_rel >= block_start && header_start_rel < block_end
            });
        if is_inside_block {
            continue;
        }

        if let Some(details) = extract_header_action_details(&caps) {
            // Validate primary path
            if validate_path_format(&details.path).is_err() {
                eprintln!(
                    "Warning: Invalid path format in standalone header '{}' (path: '{}'). Skipping.",
                    header_match.as_str().trim(),
                    details.path
                );
                continue;
            }
            // Validate destination path if it's a Move action
            if let Some(ref dest_path_val) = details.dest_path {
                if validate_path_format(dest_path_val).is_err() {
                    eprintln!(
                        "Warning: Invalid destination path format in standalone header '{}' (dest_path: '{}'). Skipping.",
                        header_match.as_str().trim(),
                        dest_path_val
                    );
                    continue;
                }
            }

            if let Some(action_type) = get_action_type(&details.action_word) {
                match action_type {
                    crate::core_types::ActionType::Delete => {
                        standalone_delete::handle_standalone_delete(
                            original_header_pos,
                            &details.path,
                            actions_with_pos,
                        );
                    }
                    crate::core_types::ActionType::Create
                    | crate::core_types::ActionType::Append
                    | crate::core_types::ActionType::Prepend => {
                        // The warning for this is now handled by pass2 if no block is found.
                        // So, we do nothing here.
                    }
                    crate::core_types::ActionType::Move => {
                        // Add Move action
                        println!(
                            "  - Found standalone MOVE action for: '{}' to '{}' at original pos {}",
                            details.path,
                            details.dest_path.as_ref().unwrap_or(&String::new()), // Should always be Some for Move
                            original_header_pos
                        );
                        let action = Action {
                            action_type: crate::core_types::ActionType::Move,
                            path: details.path,
                            dest_path: details.dest_path, // This will be Some if action_type is Move
                            content: None,
                            original_pos: original_header_pos,
                        };
                        actions_with_pos.push((original_header_pos, action));
                    }
                }
            }
        }
    }
    Ok(())
}
