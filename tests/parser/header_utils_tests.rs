//! Unit tests for header_utils.rs functionality.

// Bring items from the specific module being tested into scope
use super::header_utils::*; // Use the specific module name

// Bring items from other modules needed for tests into scope
use crate::constants::{ACTION_DELETED_FILE, ACTION_FILE};
use crate::core_types::ActionType;
use crate::parser::regex::HEADER_REGEX; // Need the regex to generate captures

fn get_captures(text: &str) -> Option<regex::Captures<'_>> {
    let caps = HEADER_REGEX.captures(text);
    // DEBUG: Print if capture fails for a specific input
    // if caps.is_none() {
    //     println!("WARN: No capture for input: '{}'", text);
    // }
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
    assert_eq!(path, "path/in/ticks.txt"); // Backticks are stripped by extractor
}

#[test]
fn test_extract_hash_deleted_file() {
    let input = "## Deleted File: old_file.log  "; // Trailing space handled by regex/extractor
    let caps = get_captures(input).expect("Regex failed to capture hash deleted file");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_DELETED_FILE);
    assert_eq!(path, "old_file.log"); // Trailing space trimmed by extractor
}

#[test]
fn test_extract_hash_deleted_file_with_backticks() {
    let input = "## Deleted File: `another/tick.log`";
    let caps =
        get_captures(input).expect("Regex failed to capture hash deleted file with backticks");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_DELETED_FILE);
    assert_eq!(path, "another/tick.log"); // Backticks stripped by extractor
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
fn test_extract_bold_file_with_trailing_text_outside() {
    let input = "**File: path/to/file.txt** (description)";
    // Note: The simplified bold regex `.+` captures `path/to/file.txt** (description)`
    // The extractor logic then needs to handle this.
    let caps = get_captures(input).expect("Regex failed to capture bold file with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "path/to/file.txt"); // Trailing text ignored by extractor
}

#[test]
fn test_extract_hash_file_with_trailing_text() {
    let input = "## File: path/to/file.txt # comment";
    // Note: The simplified hash regex `.*` captures `path/to/file.txt # comment`
    // The extractor logic then needs to handle this.
    let caps = get_captures(input).expect("Regex failed to capture hash file with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "path/to/file.txt"); // Trailing text ignored by extractor
}

#[test]
fn test_extract_hash_file_with_backticks_and_trailing_text() {
    let input = "## File: `path/in/ticks.txt` (description)";
    // Note: The simplified hash regex `.*` captures `` `path/in/ticks.txt` (description)``
    // The extractor logic then needs to handle this (find backticks first).
    let caps = get_captures(input)
        .expect("Regex failed to capture hash file with ticks and trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "path/in/ticks.txt"); // Backticks stripped, trailing text ignored
}

#[test]
fn test_extract_backtick_only_with_trailing_text_outside() {
    let input = "`simple/path.rs` (some comment)";
    // Note: The backtick regex itself handles the trailing text optionally.
    let caps =
        get_captures(input).expect("Regex failed to capture backtick only with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "simple/path.rs"); // Trailing text ignored
}

// This test was already correct, but adding a comment for clarity
#[test]
fn test_extract_backtick_only_with_trailing_text() {
    let input = "`simple/path.rs` (some comment)";
    let caps =
        get_captures(input).expect("Regex failed to capture backtick only with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "simple/path.rs"); // Trailing text ignored
} // This test case is effectively duplicated by the one above now.

#[test]
fn test_extract_numbered_backtick() {
    let input = "12. `numbered/path.py`";
    let caps = get_captures(input).expect("Regex failed to capture numbered backtick");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "numbered/path.py");
}

#[test]
fn test_extract_numbered_backtick_with_trailing_text() {
    let input = "12. `numbered/path.py` # comment";
    let caps =
        get_captures(input).expect("Regex failed to capture numbered backtick with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "numbered/path.py"); // Trailing text ignored
}

#[test]
fn test_extract_bold_backtick() {
    let input = "**`bold/tick.js`**";
    let caps = get_captures(input).expect("Regex failed to capture bold backtick");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "bold/tick.js");
}

// This test was already correct, but adding a comment for clarity
#[test]
fn test_extract_bold_backtick_with_trailing_text() {
    let input = "**`bold/tick.js`** and more";
    let caps =
        get_captures(input).expect("Regex failed to capture bold backtick with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "bold/tick.js"); // Trailing text ignored
} // This test case is effectively duplicated by the one above now.

#[test]
fn test_extract_hash_backtick() {
    let input = "## `hash/tick.css`";
    let caps = get_captures(input).expect("Regex failed to capture hash backtick");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "hash/tick.css");
}

#[test]
fn test_extract_hash_backtick_with_trailing_text() {
    let input = "## `hash/tick.css` (style file)";
    let caps =
        get_captures(input).expect("Regex failed to capture hash backtick with trailing text");
    let (action, path) = extract_action_path_from_captures(&caps).unwrap();
    assert_eq!(action, ACTION_FILE);
    assert_eq!(path, "hash/tick.css"); // Trailing text ignored
}

#[test]
fn test_extract_no_match() {
    assert!(get_captures("Just text").is_none());
    assert!(get_captures("**NotAnAction: path**").is_none());
    // This should now match the regex, but extractor should return None because path is empty
    let caps_empty_hash = get_captures("## File: ").unwrap();
    assert!(extract_action_path_from_captures(&caps_empty_hash).is_none());
    let caps_empty_bold = get_captures("**File:**").unwrap(); // No space after colon
    assert!(extract_action_path_from_captures(&caps_empty_bold).is_none());
    // Test extractor directly for empty path case within content
    let caps_empty_ticks_hash = get_captures("## File: ``").unwrap(); // Regex matches
    assert!(extract_action_path_from_captures(&caps_empty_ticks_hash).is_none()); // Extractor rejects empty path
    let caps_empty_ticks_bold = get_captures("**File: ``**").unwrap(); // Regex matches
    assert!(extract_action_path_from_captures(&caps_empty_ticks_bold).is_none()); // Extractor rejects empty path
                                                                                  // Test extractor directly for only-backticks case
    let caps_only_ticks_hash = get_captures("## File: ```").unwrap(); // Regex matches
    assert!(extract_action_path_from_captures(&caps_only_ticks_hash).is_none()); // Extractor rejects only-backticks path
    let caps_only_ticks_bold = get_captures("**File: ```**").unwrap(); // Regex matches
    assert!(extract_action_path_from_captures(&caps_only_ticks_bold).is_none());
    // Extractor rejects only-backticks path
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
