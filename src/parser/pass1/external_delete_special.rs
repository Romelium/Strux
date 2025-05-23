//! Handles the special external delete header case (## Deleted File: + path in block).

use crate::constants::ACTION_DELETED_FILE;
use crate::core_types::{Action, ActionType};
use crate::errors::ParseError;
use crate::parser::path_utils::validate_path_format;

/// Specific handler for the "## Deleted File:" header + path in code block format.
pub(crate) fn handle_external_delete_special_case(
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
                dest_path: None, // Delete actions don't have a dest_path
                content: None,
                original_pos: 0, // original_pos set later
            }))
        }
    }
}
