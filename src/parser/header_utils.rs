//! Utilities for parsing markdown header lines (e.g., **Action: path**).

use crate::constants::{ACTION_DELETED_FILE, ACTION_FILE};
use crate::core_types::ActionType;
use regex::Captures;

/// Extracts action word and path string from HEADER_REGEX captures.
pub(crate) fn extract_action_path_from_captures(caps: &Captures) -> Option<(String, String)> {
    let mut action_word: Option<String> = None;
    let mut header_path: Option<String> = None;
    let mut content_str: Option<&str> = None;

    // Extract based on named capture groups
    if let (Some(aw), Some(c)) = (caps.name("action_word_bold"), caps.name("content_bold")) {
        action_word = Some(aw.as_str().to_string());
        content_str = Some(c.as_str());
    } else if let (Some(aw), Some(c)) = (caps.name("action_word_hash"), caps.name("content_hash")) {
        action_word = Some(aw.as_str().to_string());
        content_str = Some(c.as_str());
    } else if let Some(p) = caps.name("path_backtick_only") {
        action_word = Some(ACTION_FILE.to_string());
        header_path = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_numbered_backtick") {
        action_word = Some(ACTION_FILE.to_string());
        header_path = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_bold_backtick") {
        action_word = Some(ACTION_FILE.to_string());
        header_path = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_hash_backtick") {
        action_word = Some(ACTION_FILE.to_string());
        header_path = Some(p.as_str().trim().to_string());
    }

    // Process content_str for Bold/Hash Action formats to extract path
    if let Some(content) = content_str {
        let stripped_content = content.trim();
        // Check if the stripped content is *only* backticks (e.g., `` ` `` or ``` `` ```)
        // If so, treat it as an empty path.
        let is_only_backticks = stripped_content.starts_with('`')
            && stripped_content.ends_with('`')
            && stripped_content
                .chars()
                .skip(1)
                .take(stripped_content.len() - 2)
                .all(|c| c == '`');

        if is_only_backticks {
            header_path = Some("".to_string()); // Treat as empty path explicitly
        } else {
            // Prefer path inside backticks if present within the content part
            header_path = Some(
                if stripped_content.len() > 1
                    && stripped_content.starts_with('`')
                    && stripped_content.ends_with('`')
                {
                    stripped_content[1..stripped_content.len() - 1]
                        .trim()
                        .to_string() // Inside backticks
                } else {
                    stripped_content.to_string() // Whole content as path
                },
            );
        }
    }

    // Validate and return
    match (action_word, header_path) {
        // Ensure the extracted path is not empty AFTER trimming potential backticks and whitespace
        (Some(aw), Some(hp)) => {
            let final_path = hp.trim(); // Trim whitespace from final path string
            if !final_path.is_empty() {
                Some((aw, final_path.to_string()))
            } else {
                None
            }
        }
        _ => None,
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
