//! Handles external headers preceding code blocks in Pass 1.

use crate::constants::ACTION_DELETED_FILE;
use crate::core_types::{Action, ActionType};
use crate::errors::ParseError;
use crate::parser::header_utils::{extract_header_action_details, get_action_type};
use crate::parser::helpers::ensure_trailing_newline;
use crate::parser::pass1::external_delete_special;
use crate::parser::path_utils::validate_path_format;
use crate::parser::regex::HEADER_REGEX; // Import the new module

/// Checks for and handles an external header preceding a code block.
/// This applies to *any* code block, including ```markdown.
#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_external_header(
    content: &str,
    fence_start_pos: usize,
    block_content_start: usize,
    block_content_end: usize,
    parse_offset: usize,
) -> Result<Option<(Action, usize)>, ParseError> {
    // Returns (Action, header_start_pos_rel)
    let fence_line_start_rel = content[..fence_start_pos].rfind('\n').map_or(0, |n| n + 1);
    if fence_line_start_rel == 0 {
        return Ok(None); // No line before fence
    }

    let prev_line_end_rel = fence_line_start_rel.saturating_sub(1); // Avoid panic on empty content before fence
    let prev_line_start_rel = content[..prev_line_end_rel]
        .rfind('\n')
        .map_or(0, |n| n + 1);
    let prev_line_content = content
        .get(prev_line_start_rel..prev_line_end_rel)
        .unwrap_or(""); // Handle potential slicing errors
    let stripped_prev_line = prev_line_content.trim();

    if let Some(caps) = HEADER_REGEX.captures(stripped_prev_line) {
        // Check for the specific "Deleted File:" action word FIRST.
        // This format requires the path to be in the block content.
        let action_word_from_capture = caps
            .name("action_word_hash")
            .map(|m| m.as_str())
            .or_else(|| caps.name("action_word_bold").map(|m| m.as_str()));

        if action_word_from_capture == Some(ACTION_DELETED_FILE) {
            // If the keyword matches, handle the special case where the path is in the block.
            println!(
                "    Detected external '{}:' header, invoking special handler.",
                ACTION_DELETED_FILE
            );
            return external_delete_special::handle_external_delete_special_case(
                content,
                block_content_start,
                block_content_end,
                prev_line_start_rel,
                parse_offset,
            )
            .map(|opt_action| opt_action.map(|a| (a, prev_line_start_rel)));
        }

        // If it wasn't the special delete keyword, proceed with normal extraction.
        // This handles ## File:, **File:**, `path`, **`path`**, ## `path`, etc.
        // Also handles Append File, Prepend File.
        // "Moved File" headers are standalone and should not be associated with code blocks here.
        if let Some(details) = extract_header_action_details(&caps) {
            if details.dest_path.is_some() {
                // This indicates a "Moved File" header
                println!(
                    "    Info: External header '{}' is a 'Moved File' action, which is standalone. Ignoring for this code block.",
                    stripped_prev_line
                );
                return Ok(None);
            }

            if validate_path_format(&details.path).is_err() {
                eprintln!(
                    "Warning: Invalid path format in external header '{}'. Skipping.",
                    stripped_prev_line
                );
                return Ok(None);
            }

            if let Some(action_type_enum) = get_action_type(&details.action_word) {
                match action_type_enum {
                    ActionType::Create | ActionType::Append | ActionType::Prepend => {
                        println!("    Found external header: '{}'", stripped_prev_line);
                        let mut block_data =
                            content[block_content_start..block_content_end].to_string();
                        ensure_trailing_newline(&mut block_data);
                        let action = Action {
                            action_type: action_type_enum,
                            path: details.path,
                            dest_path: None,
                            content: Some(block_data),
                            original_pos: 0, // Set later in pass1 mod
                        };
                        println!(
                            "     -> Added {} action for '{}'",
                            format!("{:?}", action.action_type).to_uppercase(),
                            action.path
                        );
                        return Ok(Some((action, prev_line_start_rel)));
                    }
                    ActionType::Delete => {
                        // Standalone Delete headers are handled by Pass 2.
                        // The special "Deleted File:" + block case is handled above.
                        // If we reach here with ActionType::Delete, it means it's a standalone
                        // header that shouldn't be associated with this block.
                        println!(
                            "    Info: External header '{}' is a standalone 'Delete' action. Ignoring for this code block.",
                            stripped_prev_line
                        );
                    }
                    ActionType::Move => {
                        // This should have been caught by `details.dest_path.is_some()` check.
                        // If not, it's an error or unexpected state.
                        eprintln!(
                            "    Error: Unexpected 'Move' action type for external header associated with a code block: '{}'. Ignoring.",
                            stripped_prev_line
                        );
                    }
                }
            } else {
                println!(
                    "    Info: External header '{}' matched regex but action type was not recognized. Ignoring.",
                    stripped_prev_line
                );
            }
        } else {
            println!(
                "    Info: External header '{}' matched regex but failed path extraction. Ignoring.",
                stripped_prev_line
            );
        }
        // If path extraction failed or action wasn't Create/Append/Prepend, fall through to Ok(None)
    }
    Ok(None)
}
