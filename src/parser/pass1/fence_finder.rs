//! Utility functions for finding matching fences.

use regex::{Match, Regex}; // Add Match

/// Finds the matching closing fence for a given opening fence, handling nesting.
///
/// Args:
///   content: The string content to search within.
///   fence_chars: The exact characters of the fence (e.g., "```" or "````").
///   search_start_pos: The byte position in `content` *after* the opening fence line.
///
/// Returns:
///   Option<Match<'a>>: The match object for the correct closing fence line, or None if not found.
pub(crate) fn find_closing_fence<'a>(
    content: &'a str,
    fence_chars: &str,
    search_start_pos: usize,
) -> Option<Match<'a>> {
    // Pattern for the closing fence: start of line, optional whitespace, exact fence chars, optional whitespace, end of line.
    let closing_pattern = format!(r"(?m)^\s*{}\s*$", regex::escape(fence_chars));
    let closing_regex = Regex::new(&closing_pattern).ok()?; // Handle potential regex compilation error

    // Pattern for finding *any* subsequent opening fence of the *same type*.
    // Matches start of line, optional whitespace, exact fence chars, optional language tag, newline.
    // This helps us count nested blocks correctly.
    // Handle optional carriage return for CRLF compatibility
    let opening_pattern = format!(r"(?m)^\s*{}(?:[^\n\r]*)(\r?\n)", regex::escape(fence_chars));
    let opening_regex = Regex::new(&opening_pattern).ok()?;

    let mut level = 1; // Start at level 1 for the initial opening fence
    let mut current_pos = search_start_pos;

    loop {
        // Find the next potential opening and closing fences from the current position
        let next_opening = opening_regex.find_at(content, current_pos);
        let next_closing = closing_regex.find_at(content, current_pos);

        match (next_opening, next_closing) {
            (Some(open_match), Some(close_match)) => {
                // Both found, determine which comes first
                if open_match.start() < close_match.start() {
                    // Opening fence comes first, increment level
                    level += 1;
                    current_pos = open_match.end(); // Continue search after this opening fence
                } else {
                    // Closing fence comes first, decrement level
                    level -= 1;
                    if level == 0 {
                        // This is the matching closing fence for the initial opening fence
                        return Some(close_match);
                    }
                    // It closed a nested block, continue search after this closing fence
                    current_pos = close_match.end();
                }
            }
            (None, Some(close_match)) => {
                // Only a closing fence found
                level -= 1;
                if level == 0 {
                    // This is the matching closing fence
                    return Some(close_match);
                }
                // This case implies unbalanced fences if level != 0, but we continue searching
                // in case there's another closing fence later.
                current_pos = close_match.end();
                if level < 0 {
                    // Should ideally not happen with well-formed markdown, but indicates an issue.
                    // Treat as unclosed for safety.
                    return None;
                }
            }
            (Some(open_match), None) => {
                // Only an opening fence found, means the block is definitely unclosed by the end
                // Increment level and continue search (though it will likely end in None)
                level += 1;
                current_pos = open_match.end();
            }
            (None, None) => {
                // No more opening or closing fences found, the block is unclosed
                return None;
            }
        }
    }
}
