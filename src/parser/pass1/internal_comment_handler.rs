//! Handles internal comment headers like `// File: path` or `//path`.

use crate::core_types::{Action, ActionType};
use crate::errors::ParseError;
use crate::parser::helpers::ensure_trailing_newline;
use crate::parser::path_utils::validate_path_format;
use std::collections::HashSet;

/// Context for processing an internal comment header.
// Make the struct pub(crate) so internal_header.rs can use it
pub(crate) struct InternalCommentContext<'a> {
    pub(crate) block_content: &'a str,
    pub(crate) rest_content: &'a str,
    pub(crate) stripped_first_line: &'a str,
    pub(crate) header_original_pos: usize,
    pub(crate) block_content_start: usize,
}

/// Handles internal comment headers like `// File: path` or `//path`.
// Updated signature uses context struct and fewer arguments
pub(crate) fn handle_internal_comment_header(
    path: String,
    include_header: bool,
    context: &InternalCommentContext, // Use context struct
    processed_header_starts: &mut HashSet<usize>,
) -> Result<Option<(Action, usize)>, ParseError> {
    if validate_path_format(&path).is_err() {
        eprintln!(
            "Warning: Invalid path format in internal comment header '{}'. Skipping.",
            context.stripped_first_line // Use context field
        );
        return Ok(None);
    }
    println!(
        "    Found internal comment header: '{}' ({} output)",
        context.stripped_first_line, // Use context field
        if include_header {
            "Included in"
        } else {
            "Excluded from"
        }
    );
    processed_header_starts.insert(context.header_original_pos); // Use context field
    let mut final_content = if include_header {
        context.block_content.to_string() // Use context field
    } else {
        context.rest_content.to_string() // Use context field
    };
    ensure_trailing_newline(&mut final_content);
    let action = Action {
        action_type: ActionType::Create,
        path,
        dest_path: None, // Create actions don't have a dest_path
        content: Some(final_content),
        original_pos: 0, // Set later in pass1 mod
    };
    println!("     -> Added CREATE action for '{}'", action.path);
    // Return the block content start position from the context
    Ok(Some((action, context.block_content_start))) // Use context field
}
