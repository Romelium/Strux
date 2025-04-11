//! Handles internal headers within code blocks in Pass 1.

use crate::constants::ACTION_DELETED_FILE;
use crate::core_types::{Action, ActionType};
use crate::errors::ParseError;
use crate::parser::header_utils::{extract_action_path_from_captures, get_action_type}; // Keep header_utils
use crate::parser::helpers::ensure_trailing_newline;
use crate::parser::internal_comment::extract_path_from_internal_comment;
use crate::parser::path_utils::validate_path_format;
use crate::parser::regex::HEADER_REGEX;
use std::collections::HashSet; // Import HashSet

/// Checks for and handles internal headers within a code block.
/// Applies heuristics to avoid matching comments or strings.
/// Returns an optional Action and the header's start position if found and valid.
/// Also updates the set of processed header start positions.
pub(crate) fn handle_internal_header(
    content: &str,
    block_content_start: usize,
    block_content_end: usize,
    parse_offset: usize,
    processed_header_starts: &mut HashSet<usize>, // Add this argument
) -> Result<Option<(Action, usize)>, ParseError> {
    // Returns (Action, header_start_pos_rel)
    let block_content = &content[block_content_start..block_content_end];
    let first_newline_pos = block_content.find('\n');
    let (first_line, rest_content) = match first_newline_pos {
        Some(pos) => (&block_content[..pos], &block_content[pos + 1..]),
        None => (block_content, ""), // Block is a single line
    };
    let stripped_first_line = first_line.trim();
    let header_original_pos = block_content_start + parse_offset; // Calculate original pos

    // Check for // File: path or // path
    if let Some((path, include_header)) =
        extract_path_from_internal_comment(first_line, stripped_first_line)
    {
        if validate_path_format(&path).is_err() {
            eprintln!(
                "Warning: Invalid path format in internal comment header '{}'. Skipping.",
                stripped_first_line
            );
            return Ok(None);
        }
        println!(
            "    Found internal comment header: '{}' ({} output)",
            stripped_first_line,
            if include_header {
                "Included in"
            } else {
                "Excluded from"
            }
        );
        processed_header_starts.insert(header_original_pos); // Mark header as processed
        let mut final_content = if include_header {
            block_content.to_string()
        } else {
            rest_content.to_string()
        };
        ensure_trailing_newline(&mut final_content);
        let action = Action {
            action_type: ActionType::Create,
            path,
            content: Some(final_content),
            /* Removed patch_content */ original_pos: 0,
        };
        println!("     -> Added CREATE action for '{}'", action.path);
        return Ok(Some((action, block_content_start)));
    }

    // Check for **Action:** or ## Action:
    if let Some(caps) = HEADER_REGEX.captures(first_line) {
        // --- Heuristic Check ---
        // Apply heuristics *before* trying to extract path/action
        // Use stripped_first_line for heuristic checks as leading whitespace is irrelevant
        if crate::parser::helpers::is_likely_comment(stripped_first_line)
            || crate::parser::helpers::is_likely_string(stripped_first_line)
        {
            println!(
                "    Info: Ignoring potential internal header (matched comment/string heuristic): '{}'",
                stripped_first_line
            );
            return Ok(None); // Ignore, do not mark as processed
        }
        // Match non-trimmed line
        if let Some((action_word, path)) = extract_action_path_from_captures(&caps) {
            if validate_path_format(&path).is_err() {
                eprintln!(
                    "Warning: Invalid path format in internal standard header '{}'. Skipping.",
                    stripped_first_line
                );
                return Ok(None);
            }
            if let Some(action_type @ ActionType::Create) = get_action_type(&action_word) {
                // Removed Patch check
                println!(
                    "    Found internal standard header: '{}' (Excluded from output)",
                    stripped_first_line
                );
                processed_header_starts.insert(header_original_pos); // Mark header as processed *only if valid action created*
                let mut block_data = rest_content.to_string();
                ensure_trailing_newline(&mut block_data);
                let action = Action {
                    action_type,
                    path,
                    content: Some(block_data),
                    /* Removed patch_content */ original_pos: 0,
                };
                println!(
                    "     -> Added {} action for '{}'",
                    format!("{:?}", action.action_type).to_uppercase(),
                    action.path
                );
                return Ok(Some((action, block_content_start)));
            // Removed Patch handling block
            } else if get_action_type(&action_word) == Some(ActionType::Delete) {
                println!(
                    "Info: Ignoring '{}:' header inside code block at original pos {}.",
                    ACTION_DELETED_FILE, header_original_pos
                );
                // Mark header as processed even though we ignore the action, as it was explicitly matched
                processed_header_starts.insert(header_original_pos);
                // Return Ok(None) because no action is generated
                return Ok(None);
            }
        }
    }

    Ok(None)
}
