//! Utilities for parsing markdown header lines (e.g., **Action: path**).

use crate::constants::{ACTION_DELETED_FILE, ACTION_FILE};
use crate::core_types::ActionType;
use regex::Captures;

/// Extracts action word and path string from HEADER_REGEX captures, ignoring trailing text.
/// Relies on simplified regex capture groups and performs more parsing here.
pub(crate) fn extract_action_path_from_captures(caps: &Captures) -> Option<(String, String)> {
    let mut action_word: Option<String> = None;
    let mut final_path: Option<String> = None;

    // --- Determine Action Word and Raw Content/Path ---

    // Check specific backtick path captures first (cleanest case)
    if let Some(p) = caps.name("path_hash_backtick") {
        action_word = Some(ACTION_FILE.to_string());
        final_path = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_backtick_only") {
        action_word = Some(ACTION_FILE.to_string());
        final_path = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_numbered_backtick") {
        action_word = Some(ACTION_FILE.to_string());
        final_path = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_bold_backtick") {
        action_word = Some(ACTION_FILE.to_string());
        final_path = Some(p.as_str().trim().to_string());
    }
    // Check combined Action: content captures (need parsing)
    else if let (Some(aw), Some(c)) = (caps.name("action_word_bold"), caps.name("content_bold")) {
        action_word = Some(aw.as_str().to_string());
        final_path = parse_content_for_path(c.as_str());
    } else if let (Some(aw), Some(c)) = (caps.name("action_word_hash"), caps.name("content_hash")) {
        action_word = Some(aw.as_str().to_string());
        final_path = parse_content_for_path(c.as_str());
    }

    // --- Validate and Return ---
    match (action_word, final_path) {
        // Ensure final path is not empty AFTER trimming potential backticks and whitespace
        (Some(aw), Some(fp)) => {
            let final_trimmed_path = fp.trim();
            // Add check: reject if path consists ONLY of backticks after trimming
            if !final_trimmed_path.is_empty() && final_trimmed_path.chars().all(|c| c == '`') {
                return None;
            }
            if !final_trimmed_path.is_empty() {
                Some((aw, final_trimmed_path.to_string()))
            } else {
                None
            }
        }
        _ => None, // No action word, or path parsing failed/resulted in empty path
    }
}

/// Parses the raw captured content string to extract the path, ignoring trailing text.
fn parse_content_for_path(raw_content: &str) -> Option<String> {
    let trimmed_content = raw_content.trim();

    // Check for path inside backticks first
    if let (Some(start), Some(end)) = (trimmed_content.find('`'), trimmed_content.rfind('`')) {
        if start < end {
            // Found distinct backticks, extract path from within
            let path_between_ticks = trimmed_content[start + 1..end].trim();
            // Ensure the content BETWEEN the ticks is not empty after trimming
            return if path_between_ticks.is_empty() {
                None
            } else {
                Some(path_between_ticks.to_string())
            };
        }
        // If start >= end, backticks are malformed or nested in a way we don't handle here.
        // Fall through to treat as non-backticked path.
    }

    // No valid backticks found, treat as non-backticked path.
    // Find the end of the path (before potential trailing text).
    // Trailing text starts with " (" or " #".
    let path_end_index = trimmed_content
        .find(" (")
        .or_else(|| trimmed_content.find(" #"))
        .unwrap_or(trimmed_content.len()); // If no marker found, path is the whole string

    let path = trimmed_content[..path_end_index].trim();

    if path.is_empty() {
        None
    } else {
        Some(path.to_string())
    }
}

/// Maps action word string to ActionType enum.
pub(crate) fn get_action_type(action_word: &str) -> Option<ActionType> {
    match action_word {
        ACTION_FILE => Some(ActionType::Create),
        ACTION_DELETED_FILE => Some(ActionType::Delete),
        _ => None,
    }
}

// Test module moved to header_utils_tests.rs
