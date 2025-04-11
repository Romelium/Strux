//! Utilities for parsing markdown header lines (e.g., **Action: path**).

use crate::constants::{ACTION_DELETED_FILE, ACTION_FILE};
use crate::core_types::ActionType;
use regex::Captures;

/// Extracts action word and path string from HEADER_REGEX captures.
pub(crate) fn extract_action_path_from_captures(caps: &Captures) -> Option<(String, String)> {
    let mut action_word: Option<String> = None;
    // DEBUG: Log raw captures (Remove this line)
    // println!("  [Extractor] Captures: {:?}", caps);
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

    // DEBUG: Log intermediate values (Remove this line)
    // println!("  [Extractor] Intermediate action='{:?}', path='{:?}'", action_word, header_path);

    // Validate and return
    match (action_word, header_path) {
        // Ensure the extracted path is not empty AFTER trimming potential backticks and whitespace
        (Some(aw), Some(hp)) => {
            // DEBUG: Log check (Remove this line)
            // println!("  [Extractor] Checking action='{}', path='{}'", aw, hp);
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

#[cfg(test)]
mod tests {
    // Bring items from parent module (header_utils.rs) into scope
    use super::*;
    // Bring items from other modules needed for tests into scope
    use crate::constants::{ACTION_DELETED_FILE, ACTION_FILE}; // <-- ADDED
    use crate::core_types::ActionType; // <-- ADDED
    use crate::parser::regex::HEADER_REGEX; // Need the regex to generate captures

    fn get_captures(text: &str) -> Option<regex::Captures<'_>> {
        // println!("[get_captures] Input: '{}'", text); // DEBUG
        let caps = HEADER_REGEX.captures(text);
        // println!("[get_captures] Result: {:?}", caps.is_some()); // DEBUG
        caps
    }

    #[test]
    fn test_extract_bold_file() {
        let input = "**File: path/to/file.txt**";
        // println!("\n--- Test: test_extract_bold_file ---"); // DEBUG
        let caps = get_captures(input).expect("Regex failed to capture bold file"); // Added expect message
                                                                                    // Use functions from parent module brought in by `use super::*;`
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_FILE); // Use constant brought in by `use crate::constants::...`
        assert_eq!(path, "path/to/file.txt");
    }

    #[test]
    fn test_extract_bold_file_with_backticks() {
        let input = "**File: `path/in/ticks.txt`**";
        // println!("\n--- Test: test_extract_bold_file_with_backticks ---"); // DEBUG
        let caps = get_captures(input).expect("Regex failed to capture bold file with backticks");
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_FILE);
        assert_eq!(path, "path/in/ticks.txt"); // Backticks are stripped
    }

    #[test]
    fn test_extract_hash_deleted_file() {
        let input = "## Deleted File: old_file.log  ";
        // println!("\n--- Test: test_extract_hash_deleted_file ---"); // DEBUG
        let caps = get_captures(input).expect("Regex failed to capture hash deleted file"); // Added expect message
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_DELETED_FILE); // Use constant
        assert_eq!(path, "old_file.log"); // Trailing space trimmed from content part
    }

    #[test]
    fn test_extract_hash_deleted_file_with_backticks() {
        let input = "## Deleted File: `another/tick.log`";
        // println!("\n--- Test: test_extract_hash_deleted_file_with_backticks ---"); // DEBUG
        let caps =
            get_captures(input).expect("Regex failed to capture hash deleted file with backticks"); // Added expect message
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_DELETED_FILE); // Use constant
        assert_eq!(path, "another/tick.log"); // Backticks stripped
    }

    #[test]
    fn test_extract_backtick_only() {
        let input = "`simple/path.rs`";
        // println!("\n--- Test: test_extract_backtick_only ---"); // DEBUG
        let caps = get_captures(input).expect("Regex failed to capture backtick only");
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_FILE); // Use constant
        assert_eq!(path, "simple/path.rs");
    }

    #[test]
    fn test_extract_backtick_only_with_trailing_text() {
        let input = "`simple/path.rs` (some comment)";
        // println!("\n--- Test: test_extract_backtick_only_with_trailing_text ---"); // DEBUG
        let caps =
            get_captures(input).expect("Regex failed to capture backtick only with trailing text");
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_FILE); // Use constant
        assert_eq!(path, "simple/path.rs"); // Trailing text ignored
    }

    #[test]
    fn test_extract_numbered_backtick() {
        let input = "12. `numbered/path.py`";
        // println!("\n--- Test: test_extract_numbered_backtick ---"); // DEBUG
        let caps = get_captures(input).expect("Regex failed to capture numbered backtick");
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_FILE); // Use constant
        assert_eq!(path, "numbered/path.py");
    }

    #[test]
    fn test_extract_bold_backtick() {
        let input = "**`bold/tick.js`**";
        // println!("\n--- Test: test_extract_bold_backtick ---"); // DEBUG
        let caps = get_captures(input).expect("Regex failed to capture bold backtick");
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_FILE); // Use constant
        assert_eq!(path, "bold/tick.js");
    }

    #[test]
    fn test_extract_bold_backtick_with_trailing_text() {
        let input = "**`bold/tick.js`** and more";
        // println!("\n--- Test: test_extract_bold_backtick_with_trailing_text ---"); // DEBUG
        let caps =
            get_captures(input).expect("Regex failed to capture bold backtick with trailing text");
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_FILE); // Use constant
        assert_eq!(path, "bold/tick.js"); // Trailing text ignored
    }

    #[test]
    fn test_extract_hash_backtick() {
        let input = "## `hash/tick.css`";
        // println!("\n--- Test: test_extract_hash_backtick ---"); // DEBUG
        let caps = get_captures(input).expect("Regex failed to capture hash backtick"); // Added expect message
        let (action, path) = extract_action_path_from_captures(&caps).unwrap();
        assert_eq!(action, ACTION_FILE); // Use constant
        assert_eq!(path, "hash/tick.css");
    }

    #[test]
    fn test_extract_no_match() {
        // println!("\n--- Test: test_extract_no_match ---"); // DEBUG
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
        // Use function from super::*, use constant from crate::constants::...
        assert_eq!(get_action_type(ACTION_FILE), Some(ActionType::Create)); // Use ActionType from crate::core_types::...
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
}
