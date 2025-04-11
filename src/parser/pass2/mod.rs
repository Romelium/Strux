//! Logic for Pass 2 of markdown parsing: Finding standalone actions.

use crate::core_types::Action;
use crate::errors::ParseError;
use crate::parser::header_utils::{extract_action_path_from_captures, get_action_type};
use crate::parser::path_utils::validate_path_format;
use crate::parser::regex::HEADER_REGEX;
use std::collections::HashSet;

// Declare submodules for Pass 2
mod standalone_delete;

/// Executes Pass 2: Find standalone Delete headers and warn about orphaned Create. // Removed Patch
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_pass2(
    content_to_parse: &str,
    parse_offset: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &HashSet<usize>, // Read-only access needed
    _processed_code_block_ranges: &HashSet<(usize, usize)>, // Revert: No longer needed, prefix with _
) -> Result<(), ParseError> {
    for caps in HEADER_REGEX.captures_iter(content_to_parse) {
        let header_match = caps.get(0).unwrap(); // The whole match
        let header_start_rel = header_match.start();
        // Removed header_end_rel as it's not needed
        let original_header_pos = header_start_rel + parse_offset;

        // Skip if this header's original start position was already processed in Pass 1
        if processed_header_starts.contains(&original_header_pos) {
            continue;
        }

        // Revert: Remove the check for is_inside_processed_block

        // DEBUG: Log attempt to extract (Remove this line)
        // println!("  [Pass 2] Checking header at pos {}: '{}'", original_header_pos, header_match.as_str().trim());

        // Try to extract action and path. If extraction fails (returns None),
        // it means the path was empty or it's a special case handled by Pass 1.
        // Pass 2 should ignore these cases entirely.
        if let Some((action_word, path)) = extract_action_path_from_captures(&caps) {
            // Path extraction succeeded, proceed with validation and action handling for Pass 2.
            // DEBUG: Log success (Remove this line)
            // println!("    [Pass 2] Extracted action='{}', path='{}'", action_word, path);
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
                        // Removed Patch
                        // Warn about orphaned Create headers
                        eprintln!(
                            "Warning: Found header '{}' for path '{}' at original pos {} without an associated code block. Skipping.",
                            header_match.as_str().trim(), path, original_header_pos
                        );
                    }
                }
            }
        } else {
            // DEBUG: Log extraction failure (Remove this line)
            // println!("    [Pass 2] Path extraction failed or path was empty. Ignoring header.");
        }
        // If extract_action_path_from_captures returned None, do nothing in Pass 2.
    }
    Ok(())
}
