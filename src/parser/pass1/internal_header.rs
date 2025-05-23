//! Handles internal headers within code blocks in Pass 1.

use crate::core_types::Action; // Import Action
use crate::errors::ParseError;
use crate::parser::helpers::{ensure_trailing_newline, is_likely_comment, is_likely_string};
use crate::parser::internal_comment::extract_path_from_internal_comment;
use crate::parser::path_utils::validate_path_format;
// Import the specific handler function and the context struct (if needed, though it's internal to the handler)
use crate::parser::pass1::internal_comment_handler;
use crate::parser::pass1::internal_standard_handler;
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

    // --- Check for // File: path or // path ---
    if let Some((path, include_header)) =
        extract_path_from_internal_comment(first_line, stripped_first_line)
    {
        // Create the context struct for the handler function
        let context = internal_comment_handler::InternalCommentContext {
            block_content,
            rest_content,
            stripped_first_line,
            header_original_pos,
            block_content_start,
        };
        // Call the handler with the context struct
        return internal_comment_handler::handle_internal_comment_header(
            path,
            include_header,
            &context, // Pass context by reference
            processed_header_starts,
        );
    }

    // --- Check for ## File: path or ## File: `path` ---
    // This is a new internal header format, similar to // File: but using ##
    if stripped_first_line.starts_with("## File:") {
        let content_after_prefix = stripped_first_line.strip_prefix("## File:").unwrap().trim();
        let path = if content_after_prefix.len() > 1
            && content_after_prefix.starts_with('`')
            && content_after_prefix.ends_with('`')
        {
            content_after_prefix[1..content_after_prefix.len() - 1]
                .trim()
                .to_string()
        } else {
            content_after_prefix.to_string()
        };

        if path.is_empty() {
            eprintln!(
                "Warning: Internal header '## File:' at original pos {} found with empty path. Skipping.",
                header_original_pos
            );
            return Ok(None);
        }

        if validate_path_format(&path).is_err() {
            eprintln!(
                "Warning: Invalid path format in internal header '{}' at original pos {}. Skipping.",
                stripped_first_line, header_original_pos
            );
            return Ok(None);
        }

        println!(
            "    Found internal header: '{}' (Excluded from output)",
            stripped_first_line
        );
        processed_header_starts.insert(header_original_pos); // Mark header as processed

        let mut final_content = rest_content.to_string();
        ensure_trailing_newline(&mut final_content);

        let action = Action {
            action_type: crate::core_types::ActionType::Create,
            path,
            dest_path: None, // Create actions don't have a dest_path
            content: Some(final_content),
            original_pos: 0, // Set later in pass1 mod
        };
        println!("     -> Added CREATE action for '{}'", action.path);
        // Return the action and the block content start position (relative)
        return Ok(Some((action, block_content_start)));
    }

    // --- Check for **Action:** or ## Action: (but not ## File:) ---
    // This handles standard markdown headers like **File:** if they appear internally
    if let Some(caps) = HEADER_REGEX.captures(first_line) {
        // Apply heuristics *before* trying to extract path/action
        if is_likely_comment(stripped_first_line) || is_likely_string(stripped_first_line) {
            println!(
                "    Info: Ignoring potential internal header (matched comment/string heuristic): '{}'",
                stripped_first_line
            );
            return Ok(None); // Ignore, do not mark as processed
        }
        // Call the standard handler for these formats
        return internal_standard_handler::handle_internal_standard_header(
            caps,
            rest_content,
            stripped_first_line,
            header_original_pos,
            block_content_start,
            processed_header_starts,
        );
    }

    // No internal header format matched on the first line
    Ok(None)
}
