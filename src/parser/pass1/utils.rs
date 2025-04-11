//! Utility functions specific to Pass 1 of parsing.

use crate::parser::regex::OPENING_FENCE_REGEX; // Add this
use regex::{Captures, Match, Regex}; // Add Captures
use std::collections::HashSet;

/// Checks if a position falls within any already processed block range.
pub(crate) fn is_already_processed(pos: usize, ranges: &HashSet<(usize, usize)>) -> bool {
    ranges.iter().any(|&(start, end)| start <= pos && pos < end)
}

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
                                                    // println!(
                                                    //     "    [Fence Finder] Found nested opening at {}, level={}",
                                                    //     open_match.start(),
                                                    //     level
                                                    // );
                } else {
                    // Closing fence comes first, decrement level
                    level -= 1;
                    // println!(
                    //     "    [Fence Finder] Found closing at {}, level={}",
                    //     close_match.start(),
                    //     level
                    // );
                    if level == 0 {
                        // This is the matching closing fence for the initial opening fence
                        // println!("    [Fence Finder] Matched closing fence found!");
                        return Some(close_match);
                    }
                    // It closed a nested block, continue search after this closing fence
                    current_pos = close_match.end();
                }
            }
            (None, Some(close_match)) => {
                // Only a closing fence found
                level -= 1;
                // println!(
                //     "    [Fence Finder] Found closing (no more openings) at {}, level={}",
                //     close_match.start(),
                //     level
                // );
                if level == 0 {
                    // This is the matching closing fence
                    // println!("    [Fence Finder] Matched closing fence found!");
                    return Some(close_match);
                }
                // This case implies unbalanced fences if level != 0, but we continue searching
                // in case there's another closing fence later.
                current_pos = close_match.end();
                if level < 0 {
                    // Should ideally not happen with well-formed markdown, but indicates an issue.
                    // Treat as unclosed for safety.
                    // eprintln!("Warning: Fence nesting level went below zero. Assuming unclosed block.");
                    return None;
                }
            }
            (Some(open_match), None) => {
                // Only an opening fence found, means the block is definitely unclosed by the end
                // Increment level and continue search (though it will likely end in None)
                level += 1;
                current_pos = open_match.end();
                // println!(
                //     "    [Fence Finder] Found nested opening (no more closings) at {}, level={}",
                //     open_match.start(),
                //     level
                // );
            }
            (None, None) => {
                // No more opening or closing fences found, the block is unclosed
                // println!("    [Fence Finder] No more fences found, block unclosed.");
                return None;
            }
        }
    }
}

/// Finds the next opening fence that starts at or after `search_start_pos`,
/// skipping any fences that fall within already processed ranges.
pub(crate) fn find_next_opening_fence<'a>(
    content: &'a str,
    search_start_pos: usize,
    processed_ranges: &HashSet<(usize, usize)>,
) -> Option<Captures<'a>> {
    // Iterate through potential matches starting from the search position
    for caps in OPENING_FENCE_REGEX
        .captures_iter(content)
        .skip_while(|c| c.get(0).unwrap().start() < search_start_pos)
    {
        let fence_match = caps.get(0).unwrap();
        let fence_start = fence_match.start();

        // Check if this fence starts within an already processed block
        if !is_already_processed(fence_start, processed_ranges) {
            // Found the next valid opening fence
            return Some(caps);
        }
        // Otherwise, continue searching
    }
    // No subsequent valid opening fence found
    None
}
