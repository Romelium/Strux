//! Utilities for parsing markdown header lines (e.g., **Action: path**).

use crate::constants::{ACTION_DELETED_FILE, ACTION_FILE, ACTION_MOVED_FILE};
use crate::core_types::ActionType;
use regex::Captures;

/// Represents the details extracted from a header line.
pub(crate) struct ParsedHeaderAction {
    pub action_word: String,
    pub path: String, // Source path for Move, target path for File/Delete
    pub dest_path: Option<String>, // Destination path for Move
}

/// Extracts action word, primary path, and optional destination path from HEADER_REGEX captures.
pub(crate) fn extract_header_action_details(caps: &Captures) -> Option<ParsedHeaderAction> {
    let mut action_word_opt: Option<String> = None;
    let mut raw_content_opt: Option<String> = None;

    // --- Determine Action Word and Raw Content String ---
    // This part prioritizes specific backticked path formats for ACTION_FILE.
    // If those match, raw_content_opt will be the path itself.
    // Otherwise, for general "Action: content" formats, raw_content_opt will be the 'content'.

    if let Some(p) = caps.name("path_hash_backtick") {
        action_word_opt = Some(ACTION_FILE.to_string());
        raw_content_opt = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_backtick_only") {
        action_word_opt = Some(ACTION_FILE.to_string());
        raw_content_opt = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_numbered_backtick") {
        action_word_opt = Some(ACTION_FILE.to_string());
        raw_content_opt = Some(p.as_str().trim().to_string());
    } else if let Some(p) = caps.name("path_bold_backtick") {
        action_word_opt = Some(ACTION_FILE.to_string());
        raw_content_opt = Some(p.as_str().trim().to_string());
    }
    // General "Action: content" formats
    else if let (Some(aw), Some(c)) = (caps.name("action_word_bold"), caps.name("content_bold")) {
        action_word_opt = Some(aw.as_str().to_string());
        raw_content_opt = Some(c.as_str().to_string()); // content_bold is the raw string after "Action: "
    } else if let (Some(aw), Some(c)) = (caps.name("action_word_hash"), caps.name("content_hash")) {
        action_word_opt = Some(aw.as_str().to_string());
        raw_content_opt = Some(c.as_str().to_string()); // content_hash is the raw string after "Action: "
    }

    // --- Parse Raw Content based on Action Word ---
    match (action_word_opt, raw_content_opt) {
        (Some(action_word), Some(raw_content)) => {
            if action_word == ACTION_MOVED_FILE {
                // Parse "source to destination" from raw_content
                if let Some((source, dest)) = parse_move_paths_from_content(&raw_content) {
                    if is_path_valid_for_action(&source) && is_path_valid_for_action(&dest) {
                        Some(ParsedHeaderAction {
                            action_word,
                            path: source,
                            dest_path: Some(dest),
                        })
                    } else {
                        None
                    }
                } else {
                    None // Failed to parse "source to dest"
                }
            } else {
                // For ACTION_FILE (including backtick-only cases) and ACTION_DELETED_FILE
                if let Some(path) = parse_single_path_from_content(&raw_content) {
                    if is_path_valid_for_action(&path) {
                        Some(ParsedHeaderAction {
                            action_word,
                            path,
                            dest_path: None,
                        })
                    } else {
                        None
                    }
                } else {
                    None // Failed to parse single path
                }
            }
        }
        _ => None, // No action word or raw content captured
    }
}

/// Helper to check if a path string (after initial parsing) is valid for an action.
/// Currently checks if it's non-empty and not just backticks.
fn is_path_valid_for_action(path_str: &str) -> bool {
    if path_str.is_empty() {
        return false;
    }
    // Reject if path consists only of backticks or only of a single double quote
    if path_str.chars().all(|c| c == '`') {
        return false;
    }
    if path_str == "\"" {
        return false;
    }
    // Add other similar checks if needed, e.g., for single quotes
    // if path_str == "'" {
    //     return false;
    // }
    true
}

/// Parses a single path from a content string (which might include backticks or trailing comments).
/// Used for "File:" and "Deleted File:" actions, and for backtick-only path headers.
fn parse_single_path_from_content(raw_content: &str) -> Option<String> {
    let trimmed_content = raw_content.trim();

    // Check for path inside backticks first
    if let (Some(start), Some(end)) = (trimmed_content.find('`'), trimmed_content.rfind('`')) {
        if start < end {
            // Ensure `start` is before `end` to form a valid pair
            let path_between_ticks = trimmed_content[start + 1..end].trim();
            return if path_between_ticks.is_empty() {
                None
            } else {
                Some(path_between_ticks.to_string())
            };
        }
    }

    // No valid backticks, or backticks were part of a larger non-backticked path.
    // Find end of path before potential trailing text like " (" or " #".
    let path_end_index = trimmed_content
        .find(" (")
        .or_else(|| trimmed_content.find(" #"))
        .unwrap_or(trimmed_content.len());

    let path = trimmed_content[..path_end_index].trim();
    if path.is_empty() {
        None
    } else {
        Some(path.to_string())
    }
}

/// Parses "source to destination" from a content string for "Moved File:" actions.
/// The raw_content is the string part after "Moved File: ".
fn parse_move_paths_from_content(raw_content: &str) -> Option<(String, String)> {
    // First, strip potential trailing comments from the whole "source to dest" string
    let content_trimmed = raw_content.trim();
    let path_end_index = content_trimmed
        .find(" (") // Trailing text like (description)
        .or_else(|| content_trimmed.find(" #")) // Trailing text like # comment
        .unwrap_or(content_trimmed.len());
    let main_part = content_trimmed[..path_end_index].trim(); // e.g., "path1 to path2" or "`path1` to `path2`"

    let mut split_idx: Option<usize> = None;
    let mut in_backticks = false;

    // Iterate using char_indices to get byte indices for slicing,
    // and char for comparison.
    for (i, c) in main_part.char_indices() {
        if c == '`' {
            in_backticks = !in_backticks;
        }

        if !in_backticks {
            // Check if the current position marks the beginning of " to "
            // This ensures we are checking from the potential start of the delimiter.
            if main_part[i..].starts_with(" to ") {
                split_idx = Some(i);
                break;
            }
        }
    }

    if let Some(idx) = split_idx {
        let source_path_str = main_part[..idx].trim();
        // Calculate start of dest_path_str carefully using length of " to "
        let dest_path_start_idx = idx + " to ".len();
        // Ensure dest_path_start_idx is within bounds before slicing
        if dest_path_start_idx <= main_part.len() {
            let dest_path_str = main_part[dest_path_start_idx..].trim();

            // `parse_single_path_from_content` handles trimming and backtick removal for each part.
            let source_path = parse_single_path_from_content(source_path_str)?;
            let dest_path = parse_single_path_from_content(dest_path_str)?;

            if !source_path.is_empty() && !dest_path.is_empty() {
                Some((source_path, dest_path))
            } else {
                None
            }
        } else {
            // This case implies " to " was found at the very end of main_part,
            // meaning the destination path string would be empty.
            None
        }
    } else {
        None // " to " delimiter not found or malformed
    }
}

/// Maps action word string to ActionType enum.
pub(crate) fn get_action_type(action_word: &str) -> Option<ActionType> {
    match action_word {
        ACTION_FILE => Some(ActionType::Create),
        ACTION_DELETED_FILE => Some(ActionType::Delete),
        ACTION_MOVED_FILE => Some(ActionType::Move), // New mapping
        _ => None,
    }
}

// Test module moved to header_utils_tests.rs
