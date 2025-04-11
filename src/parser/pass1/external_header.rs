//! Handles external headers preceding code blocks in Pass 1.

use crate::constants::ACTION_DELETED_FILE;
use crate::core_types::{Action, ActionType};
use crate::errors::ParseError;
use crate::parser::header_utils::{extract_action_path_from_captures, get_action_type};
use crate::parser::helpers::ensure_trailing_newline;
use crate::parser::path_utils::validate_path_format;
use crate::parser::regex::HEADER_REGEX;

/// Checks for and handles an external header preceding a code block.
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
        return Ok(None);
    } // No line before fence

    let prev_line_end_rel = fence_line_start_rel - 1;
    let prev_line_start_rel = content[..prev_line_end_rel]
        .rfind('\n')
        .map_or(0, |n| n + 1);
    let prev_line_content = &content[prev_line_start_rel..prev_line_end_rel];
    let stripped_prev_line = prev_line_content.trim();

    if let Some(caps) = HEADER_REGEX.captures(stripped_prev_line) {
        // Check for the specific "Deleted File" action word FIRST.
        let action_word_from_capture = caps
            .name("action_word_hash")
            .map(|m| m.as_str())
            .or_else(|| caps.name("action_word_bold").map(|m| m.as_str()));

        if action_word_from_capture == Some(ACTION_DELETED_FILE) {
            // If the keyword matches, handle the special case where the path is in the block.
            // This bypasses extract_action_path_from_captures which would fail here.
            println!(
                "    Detected external '{}:' header, invoking special handler.",
                ACTION_DELETED_FILE
            );
            return handle_external_delete_special_case(
                content,
                block_content_start,
                block_content_end,
                prev_line_start_rel,
                parse_offset,
            )
            .map(|opt_action| opt_action.map(|a| (a, prev_line_start_rel)));
        }

        // If it wasn't the special delete keyword, proceed with normal extraction.
        if let Some((action_word, path)) = extract_action_path_from_captures(&caps) {
            if validate_path_format(&path).is_err() {
                eprintln!(
                    "Warning: Invalid path format in external header '{}'. Skipping.",
                    stripped_prev_line
                );
                return Ok(None);
            }
            // We already handled Delete above, so only check for Create here.
            if let Some(ActionType::Create) = get_action_type(&action_word) {
                println!("    Found external header: '{}'", stripped_prev_line);
                let mut block_data = content[block_content_start..block_content_end].to_string();
                ensure_trailing_newline(&mut block_data);
                let action = Action {
                    action_type: ActionType::Create,
                    path,
                    content: Some(block_data),
                    original_pos: 0,
                };
                println!(
                    "     -> Added {} action for '{}'",
                    format!("{:?}", action.action_type).to_uppercase(),
                    action.path
                );
                return Ok(Some((action, prev_line_start_rel)));
            }
        }
        // If path extraction failed or action wasn't Create, fall through to Ok(None)
    }
    Ok(None)
}

/// Specific handler for the "## Deleted File:" header + path in code block format.
fn handle_external_delete_special_case(
    content: &str,
    block_content_start: usize,
    block_content_end: usize,
    header_start_rel: usize,
    parse_offset: usize,
) -> Result<Option<Action>, ParseError> {
    println!(
        "    Found external '{}:' header. Checking code block for path...",
        ACTION_DELETED_FILE
    );
    let block_raw_content = &content[block_content_start..block_content_end];
    let block_lines: Vec<&str> = block_raw_content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();

    if block_lines.is_empty() {
        eprintln!(
            "Warning: '{}:' header at original pos {} followed by empty block. Skipping.",
            ACTION_DELETED_FILE,
            header_start_rel + parse_offset
        );
        Ok(None) // Return Ok(None) so no action is added
    } else {
        let path_from_block = block_lines[0].to_string();
        if block_lines.len() > 1 {
            eprintln!("Warning: Code block for '{}:' at original pos {} has multiple lines. Using first: '{}'.", ACTION_DELETED_FILE, header_start_rel + parse_offset, path_from_block);
        }
        if validate_path_format(&path_from_block).is_err() {
            eprintln!("Warning: Invalid path format '{}' in code block for external '{}:' header. Skipping.", path_from_block, ACTION_DELETED_FILE);
            Ok(None) // Return Ok(None) so no action is added
        } else {
            println!("      -> Path from code block: '{}'", path_from_block);
            Ok(Some(Action {
                action_type: ActionType::Delete,
                path: path_from_block,
                content: None,
                /* Removed patch_content */ original_pos: 0, // original_pos set later
            }))
        }
    }
}
