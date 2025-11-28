//! Handles internal headers within code blocks in Pass 1.

use crate::core_types::Action; // Import Action
use crate::errors::ParseError;
use crate::parser::helpers::{is_likely_comment, is_likely_string};
use crate::parser::internal_comment::extract_path_from_internal_comment;
// Import the specific handler function and the context struct
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

    // --- Check for **Action:** or ## Action: ---
    // This handles standard markdown headers like **File:**, ## File:, ## Create, etc.
    if let Some(caps) = HEADER_REGEX.captures(first_line) {
        // Apply heuristics *before* trying to extract path/action
        if is_likely_string(stripped_first_line) {
            println!(
                "    Info: Ignoring potential internal header (matched string heuristic): '{}'",
                stripped_first_line
            );
            return Ok(None); // Ignore
        }

        if is_likely_comment(stripped_first_line) {
            // If it looks like a comment, we generally ignore it to avoid false positives.
            // HOWEVER, if it starts with '#', it matches Markdown header syntax.
            // Since we support ## Action inside code blocks (which are often comments in the host language),
            // we MUST allow lines starting with '#' to proceed to extraction.
            // We filter out other comment types (//, --, etc.) unless they were handled by extract_path_from_internal_comment above.
            if !stripped_first_line.starts_with('#') {
                println!(
                    "    Info: Ignoring potential internal header (matched comment heuristic): '{}'",
                    stripped_first_line
                );
                return Ok(None);
            }
            // If it starts with '#', we proceed. The HEADER_REGEX match implies it has the structure of a header.
            // extract_header_action_details will further validate the path.
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
