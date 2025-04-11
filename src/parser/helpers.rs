//! Helper functions for the markdown parser.

use crate::core_types::{Action, ActionType};
// Removed unused import: once_cell::sync::Lazy;
// Removed unused import: regex::Regex;
// Removed unused ParseError
use std::collections::HashMap; // Keep HashMap for check_action_conflicts

// --- Moved to header_utils.rs ---
// extract_action_path_from_captures
// get_action_type

// --- Moved to path_utils.rs ---
// validate_path_format

// --- Moved to internal_comment.rs ---
// extract_path_from_internal_comment

// --- REMOVED UNUSED REGEX ---
// /// Regex to find the last closing ``` fence on its own line, possibly with whitespace.
// static LAST_CLOSING_FENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r"(?m)^\s*```\s*$").expect("Failed to compile LAST_CLOSING_FENCE_REGEX")
// });

/// Handles the initial check for and potential stripping of ```markdown blocks.
/// Returns the content slice to parse and the starting offset.
pub(crate) fn preprocess_markdown(markdown_content: &str) -> (&str, usize) {
    // Simplified logic: Only check if the *very first non-whitespace line* is exactly ```markdown
    // This avoids incorrectly consuming the whole file if it happens to end with ``` later.
    // The regular parser logic (Pass 1) should handle ```markdown blocks correctly anyway,
    // including wrapped headers. This preprocessing step was likely causing more harm than good.
    // We keep the check for the *very first line* being ```markdown just in case someone
    // explicitly starts their file that way intending it as a non-actionable wrapper.
    // Check only the very first line

    if let Some(first_newline_idx) = markdown_content.find('\n') {
        let first_line = &markdown_content[..first_newline_idx];
        if first_line.trim() == "```markdown" {
            println!("Info: Ignoring first line '```markdown'.");
            let parse_offset = first_newline_idx + 1;
            let content_to_parse = &markdown_content[parse_offset..];
            println!("  (Parsing content starting from offset {parse_offset})");
            return (content_to_parse, parse_offset);
        }
    }
    // Also handle the case where the *entire file* is just "```markdown"
    if markdown_content.trim() == "```markdown" {
        println!("Info: Input file only contained '```markdown'.");
        return ("", markdown_content.len());
    }

    // No wrapper detected at the start, parse everything
    (markdown_content, 0)
}

/// Checks the final sorted list of actions for potential conflicts on the same path.
pub(crate) fn check_action_conflicts(final_actions: &[Action]) {
    let mut paths_seen: HashMap<String, (ActionType, usize)> = HashMap::new();
    println!("Checking action sequence...");
    for (i, action) in final_actions.iter().enumerate() {
        let path = &action.path;
        let current_act_type = action.action_type.clone();
        if let Some((prev_act_type, prev_idx)) = paths_seen.get(path) {
            println!(
                "  Info: Action '{:?}' for path '{}' (item {}) follows action '{:?}' (item {}). Ensure sequence is intended.",
                current_act_type, path, i + 1, prev_act_type, prev_idx + 1
            );
        }
        paths_seen.insert(path.clone(), (current_act_type, i));
    }
}

/// Helper to add a trailing newline if needed.
pub(crate) fn ensure_trailing_newline(content: &mut String) {
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
}

/// Heuristic check if a line likely starts with a common single-line comment marker.
pub(crate) fn is_likely_comment(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("//")
        || trimmed.starts_with('#')
        || trimmed.starts_with("--") // SQL, Haskell, Lua
        || trimmed.starts_with(';') // Lisp, Assembly, INI
        || trimmed.starts_with('%') // TeX, Prolog
        || trimmed.starts_with("/*") // Start of block comment (less likely on its own line, but possible)
        || trimmed.starts_with("<!--") // HTML/XML comment
}

/// Heuristic check if a line likely represents a simple string literal assignment or declaration.
/// Focuses on lines starting and ending with common delimiters.
pub(crate) fn is_likely_string(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < 2 {
        return false; // Too short to be a delimited string
    }
    // Check common pairs
    (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        || (trimmed.starts_with('`') && trimmed.ends_with('`'))
    // Optional: Check for common assignment patterns (simple cases)
    // || (trimmed.contains('=') && (trimmed.ends_with(';') || trimmed.ends_with(','))) // e.g., var x = "...";
}
