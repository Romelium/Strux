//! Unit tests for header_utils.rs functionality.

// Bring items from the specific module being tested into scope
use super::header_utils::*; // Use the specific module name

// Bring items from other modules needed for tests into scope
use crate::constants::{ACTION_DELETED_FILE, ACTION_FILE};
use crate::core_types::ActionType;
use crate::parser::regex::HEADER_REGEX; // Need the regex to generate captures

fn get_captures(text: &str) -> Option<regex::Captures<'_>> {
    let caps = HEADER_REGEX.captures(text);
    caps
}

#[test]
fn test_extract_bold_file() {
    let input = "**File: path/to/file.txt**";
    let caps = get_captures(input).expect("Regex failed to capture bold file");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "path/to/file.txt");
}

#[test]
fn test_extract_bold_file_with_backticks() {
    let input = "**File: `path/in/ticks.txt`**";
    let caps = get_captures(input).expect("Regex failed to capture bold file with backticks");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "path/in/ticks.txt"); // Backticks are stripped
}

#[test]
fn test_extract_hash_deleted_file() {
    let input = "## Deleted File: old_file.log  ";
    let caps = get_captures(input).expect("Regex failed to capture hash deleted file");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_DELETED_FILE);
    assert_eq!(path, "old_file.log"); // Trailing space trimmed from content part
}

#[test]
fn test_extract_hash_deleted_file_with_backticks() {
    let input = "## Deleted File: `another/tick.log`";
    let caps =
        get_captures(input).expect("Regex failed to capture hash deleted file with backticks");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_DELETED_FILE);
    assert_eq!(path, "another/tick.log"); // Backticks stripped
}

#[test]
fn test_extract_backtick_only() {
    let input = "`simple/path.rs`";
    let caps = get_captures(input).expect("Regex failed to capture backtick only");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "simple/path.rs");
}

#[test]
fn test_extract_backtick_only_with_trailing_text() {
    let input = "`simple/path.rs` (some comment)";
    let caps =
        get_captures(input).expect("Regex failed to capture backtick only with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "simple/path.rs"); // Trailing text ignored
}

#[test]
fn test_extract_numbered_backtick() {
    let input = "12. `numbered/path.py`";
    let caps = get_captures(input).expect("Regex failed to capture numbered backtick");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "numbered/path.py");
}

#[test]
fn test_extract_bold_backtick() {
    let input = "**`bold/tick.js`**";
    let caps = get_captures(input).expect("Regex failed to capture bold backtick");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "bold/tick.js");
}

#[test]
fn test_extract_bold_backtick_with_trailing_text() {
    let input = "**`bold/tick.js`** and more";
    let caps =
        get_captures(input).expect("Regex failed to capture bold backtick with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "bold/tick.js"); // Trailing text ignored
}

#[test]
fn test_extract_hash_backtick() {
    let input = "## `hash/tick.css`";
    let caps = get_captures(input).expect("Regex failed to capture hash backtick");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "hash/tick.css");
}

#[test]
fn test_extract_no_match() {
    assert!(get_captures("Just text").is_none());
    assert!(get_captures("**NotAnAction: path**").is_none());
    // This should now match the regex, but extractor should return None
    assert!(extract_action_path_from_captures(&get_captures("## File: ").unwrap()).is_none());
    // Test extractor directly for empty path case
    let caps_empty = get_captures("## File: ").unwrap(); // Regex matches
    assert!(extract_action_path_from_captures(&caps_empty).is_none()); // Extractor rejects empty path
    let caps_empty_ticks = get_captures("## File: ``").unwrap(); // Regex matches
    assert!(extract_action_path_from_captures(&caps_empty_ticks).is_none()); // Extractor rejects empty path
                                                                             // Test extractor directly for only-backticks case
    let caps_only_ticks = get_captures("## File: ```").unwrap(); // Regex matches
    assert!(extract_action_path_from_captures(&caps_only_ticks).is_none()); // Extractor rejects only-backticks path
}

#[test]
fn test_get_action_type_valid() {
    assert_eq!(get_action_type(ACTION_FILE), Some(ActionType::Create));
    assert_eq!(
        get_action_type(ACTION_DELETED_FILE),
        Some(ActionType::Delete)
    );
}

#[test]
fn test_get_action_type_invalid() {
    assert_eq!(get_action_type("Create File"), None);
    assert_eq!(get_action_type(""), None);
    assert_eq!(get_action_type(" Patch File "), None);
}
