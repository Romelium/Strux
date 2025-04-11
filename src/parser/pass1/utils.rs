//! Utility functions specific to Pass 1 of parsing.

use regex::{Match, Regex};
use std::collections::HashSet;

/// Checks if a position falls within any already processed block range.
pub(crate) fn is_already_processed(pos: usize, ranges: &HashSet<(usize, usize)>) -> bool {
    ranges.iter().any(|&(start, end)| start <= pos && pos < end)
}

/// Finds the closing fence matching the opening fence characters.
pub(crate) fn find_closing_fence<'a>(
    content: &'a str,
    fence_chars: &str,
    search_start_pos: usize,
) -> Option<Match<'a>> {
    let pattern = format!(r"(?m)^\s*{}\s*$", regex::escape(fence_chars));
    // Compile regex here or pass pre-compiled if performance critical
    let closing_regex = Regex::new(&pattern).ok()?;
    closing_regex.find_at(content, search_start_pos)
}
