//! Logic for Pass 2 of markdown parsing: Finding standalone actions.

use crate::core_types::Action;
use crate::errors::ParseError;
use crate::parser::header_utils::{extract_action_path_from_captures, get_action_type};
use crate::parser::path_utils::validate_path_format;
use crate::parser::regex::HEADER_REGEX;
use std::collections::HashSet;

// Declare submodules for Pass 2
mod standalone_delete;

/// Executes Pass 2: Find standalone Delete headers and warn about orphaned Create.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_pass2(
    content_to_parse: &str,
    parse_offset: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &HashSet<usize>, // Read-only access needed
    processed_code_block_ranges: &HashSet<(usize, usize)>, // Use this to skip headers inside blocks
) -> Result<(), ParseError> {
    for caps in HEADER_REGEX.captures_iter(content_to_parse) {
        let header_match = caps.get(0).unwrap(); // The whole match
        let header_start_rel = header_match.start();
        // Removed header_text debug variable
        let original_header_pos = header_start_rel + parse_offset;

        // Skip if this header's original start position was already processed in Pass 1
        if processed_header_starts.contains(&original_header_pos) {
            continue;
        }

        // Skip if this header falls within a code block range processed by Pass 1
        let is_inside_block =
            processed_code_block_ranges
                .iter()
                .any(|&(block_start, block_end)| {
                    header_start_rel >= block_start && header_start_rel < block_end
                });
        if is_inside_block {
            continue;
        }

        // Try to extract action and path.
        let extraction_result = extract_action_path_from_captures(&caps);

        // Removed debug logs

        if let Some((action_word, path)) = extraction_result {
            // Path extraction succeeded, proceed with validation and action handling for Pass 2.
            if validate_path_format(&path).is_err() {
                eprintln!(
                    "Warning: Invalid path format in standalone header '{}'. Skipping.",
                    header_match.as_str().trim()
                );
                continue;
            }

            if let Some(action_type) = get_action_type(&action_word) {
                match action_type {
                    crate::core_types::ActionType::Delete => {
                        // Only process standalone deletes found here (path must be in header).
                        standalone_delete::handle_standalone_delete(
                            original_header_pos,
                            &path,
                            actions_with_pos,
                        );
                    }
                    crate::core_types::ActionType::Create => {
                        // Warn about orphaned Create headers
                        eprintln!(
                            "Warning: Found header '{}' for path '{}' at original pos {} without an associated code block. Skipping.",
                            header_match.as_str().trim(), path, original_header_pos
                        );
                    }
                }
            }
        } else {
            // Extraction failed or path was empty.
        }
    }
    Ok(())
}
