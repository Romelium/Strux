//! Handles internal standard headers like `**File:**` or `## File:`.

use crate::constants::ACTION_DELETED_FILE;
use crate::core_types::{Action, ActionType};
use crate::errors::ParseError;
use crate::parser::header_utils::{extract_header_action_details, get_action_type};
use crate::parser::helpers::ensure_trailing_newline;
use crate::parser::path_utils::validate_path_format;
use std::collections::HashSet;

/// Handles internal standard headers like `**File:**` or `## File:`.
pub(crate) fn handle_internal_standard_header(
    caps: regex::Captures,
    rest_content: &str,
    stripped_first_line: &str,
    header_original_pos: usize,
    block_content_start: usize,
    processed_header_starts: &mut HashSet<usize>,
) -> Result<Option<(Action, usize)>, ParseError> {
    if let Some(details) = extract_header_action_details(&caps) {
        // "Moved File" headers are not valid inside code blocks.
        if details.dest_path.is_some() {
            println!(
                "Info: Ignoring 'Moved File:' header inside code block at original pos {}.",
                header_original_pos
            );
            processed_header_starts.insert(header_original_pos);
            return Ok(None);
        }

        if validate_path_format(&details.path).is_err() {
            eprintln!(
                "Warning: Invalid path format in internal standard header '{}'. Skipping.",
                stripped_first_line
            );
            return Ok(None);
        }

        if let Some(action_type @ ActionType::Create) = get_action_type(&details.action_word) {
            println!(
                "    Found internal standard header: '{}' (Excluded from output)",
                stripped_first_line
            );
            processed_header_starts.insert(header_original_pos);
            let mut block_data = rest_content.to_string();
            ensure_trailing_newline(&mut block_data);
            let action = Action {
                action_type,
                path: details.path,
                dest_path: None, // Create actions don't have dest_path
                content: Some(block_data),
                original_pos: 0,
            };
            println!(
                "     -> Added {} action for '{}'",
                format!("{:?}", action.action_type).to_uppercase(),
                action.path
            );
            return Ok(Some((action, block_content_start)));
        } else if get_action_type(&details.action_word) == Some(ActionType::Delete) {
            println!(
                "Info: Ignoring '{}:' header inside code block at original pos {}.",
                ACTION_DELETED_FILE, header_original_pos
            );
            processed_header_starts.insert(header_original_pos);
            return Ok(None);
        }
    }
    Ok(None)
}
