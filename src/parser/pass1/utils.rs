//! Utility functions specific to Pass 1 of parsing.

use crate::parser::regex::OPENING_FENCE_REGEX; // Add this
use regex::Captures; // Add Captures
use std::collections::HashSet;

/// Checks if a position falls within any already processed block range.
pub(crate) fn is_already_processed(pos: usize, ranges: &HashSet<(usize, usize)>) -> bool {
    ranges.iter().any(|&(start, end)| start <= pos && pos < end)
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
