//! Helper functions for the markdown parser.

use crate::core_types::{Action, ActionType};
use once_cell::sync::Lazy; // For static regex
use regex::Regex;
// Removed unused ParseError
use std::collections::HashMap;

// --- Moved to header_utils.rs ---
// extract_action_path_from_captures
// get_action_type

// --- Moved to path_utils.rs ---
// validate_path_format

// --- Moved to internal_comment.rs ---
// extract_path_from_internal_comment

/// Regex to find the last closing ``` fence on its own line, possibly with whitespace.
static LAST_CLOSING_FENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*```\s*$").expect("Failed to compile LAST_CLOSING_FENCE_REGEX")
});

/// Handles the initial check for and potential stripping of ```markdown blocks.
/// Returns the content slice to parse and the starting offset.
pub(crate) fn preprocess_markdown(markdown_content: &str) -> (&str, usize) {
    // Check if the content starts with ```markdown (potentially after whitespace)
    // and ends with ``` (potentially before whitespace)
    let trimmed_content = markdown_content.trim();
    if trimmed_content.starts_with("```markdown") && trimmed_content.ends_with("```") {
        // More thorough check: find the first newline after ```markdown
        if let Some(first_newline_idx_after_trim_start) = trimmed_content.find('\n') {
            let content_after_first_line =
                &trimmed_content[first_newline_idx_after_trim_start + 1..];
            // Find the *last* potential closing fence within the trimmed content
            if let Some(last_fence_match) = LAST_CLOSING_FENCE_REGEX
                .find_iter(content_after_first_line)
                .last()
            {
                // Check if the closing fence is indeed at the very end of the inner content
                if last_fence_match.end() == content_after_first_line.len() {
                    println!("Info: Input appears fully wrapped in '```markdown ... ```', ignoring content.");
                    return ("", markdown_content.len()); // Ignore everything
                }
            }
        } else if trimmed_content == "```markdown" {
            // Handle case where file *only* contains ```markdown
            println!("Info: Input file only contained '```markdown'.");
            return ("", markdown_content.len());
        }
        // If it starts with ```markdown but doesn't seem fully wrapped, fall through to old logic
    }

    // Original logic: Check only the very first line if not fully wrapped
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
