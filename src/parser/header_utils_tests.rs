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
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/to/file.txt");
}

#[test]
fn test_extract_bold_file_with_backticks() {
    let input = "**File: `path/in/ticks.txt`**";
    let caps = get_captures(input).expect("Regex failed to capture bold file with backticks");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/in/ticks.txt"); // Backticks are stripped by extractor
}

#[test]
fn test_extract_hash_deleted_file() {
    let input = "## Deleted File: old_file.log  "; // Trailing space handled by regex/extractor
    let caps = get_captures(input).expect("Regex failed to capture hash deleted file");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_DELETED_FILE);
    assert_eq!(details.path, "old_file.log"); // Trailing space trimmed by extractor
}

#[test]
fn test_extract_hash_deleted_file_with_backticks() {
    let input = "## Deleted File: `another/tick.log`";
    let caps =
        get_captures(input).expect("Regex failed to capture hash deleted file with backticks");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_DELETED_FILE);
    assert_eq!(details.path, "another/tick.log"); // Backticks stripped by extractor
}

#[test]
fn test_extract_backtick_only() {
    let input = "`simple/path.rs`";
    let caps = get_captures(input).expect("Regex failed to capture backtick only");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "simple/path.rs");
}

#[test]
fn test_extract_bold_file_with_trailing_text_outside() {
    let input = "**File: path/to/file.txt** (description)";
    // Note: The simplified bold regex `.+` captures `path/to/file.txt** (description)`
    // The extractor logic then needs to handle this.
    let caps = get_captures(input).expect("Regex failed to capture bold file with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/to/file.txt"); // Trailing text ignored by extractor
}

#[test]
fn test_extract_hash_file_with_trailing_text() {
    let input = "## File: path/to/file.txt # comment";
    // Note: The simplified hash regex `.*` captures `path/to/file.txt # comment`
    // The extractor logic then needs to handle this.
    let caps = get_captures(input).expect("Regex failed to capture hash file with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/to/file.txt"); // Trailing text ignored by extractor
}

#[test]
fn test_extract_hash_file_with_backticks_and_trailing_text() {
    let input = "## File: `path/in/ticks.txt` (description)";
    // Note: The simplified hash regex `.*` captures `` `path/in/ticks.txt` (description)``
    // The extractor logic then needs to handle this (find backticks first).
    let caps = get_captures(input)
        .expect("Regex failed to capture hash file with ticks and trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/in/ticks.txt"); // Backticks stripped, trailing text ignored
}

#[test]
fn test_extract_backtick_only_with_trailing_text_outside() {
    let input = "`simple/path.rs` (some comment)";
    // Note: The backtick regex itself handles the trailing text optionally.
    let caps =
        get_captures(input).expect("Regex failed to capture backtick only with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "simple/path.rs"); // Trailing text ignored
}

// This test was already correct, but adding a comment for clarity
#[test]
fn test_extract_backtick_only_with_trailing_text() {
    let input = "`simple/path.rs` (some comment)";
    let caps =
        get_captures(input).expect("Regex failed to capture backtick only with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "simple/path.rs"); // Trailing text ignored
} // This test case is effectively duplicated by the one above now.

#[test]
fn test_extract_numbered_backtick() {
    let input = "12. `numbered/path.py`";
    let caps = get_captures(input).expect("Regex failed to capture numbered backtick");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "numbered/path.py");
}

#[test]
fn test_extract_numbered_backtick_with_trailing_text() {
    let input = "12. `numbered/path.py` # comment";
    let caps =
        get_captures(input).expect("Regex failed to capture numbered backtick with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "numbered/path.py"); // Trailing text ignored
}

#[test]
fn test_extract_bold_backtick() {
    let input = "**`bold/tick.js`**";
    let caps = get_captures(input).expect("Regex failed to capture bold backtick");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "bold/tick.js");
}

// This test was already correct, but adding a comment for clarity
#[test]
fn test_extract_bold_backtick_with_trailing_text() {
    let input = "**`bold/tick.js`** and more";
    let caps =
        get_captures(input).expect("Regex failed to capture bold backtick with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "bold/tick.js"); // Trailing text ignored
} // This test case is effectively duplicated by the one above now.

#[test]
fn test_extract_hash_backtick() {
    let input = "## `hash/tick.css`";
    let caps = get_captures(input).expect("Regex failed to capture hash backtick");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "hash/tick.css");
}

#[test]
fn test_extract_hash_backtick_with_trailing_text() {
    let input = "## `hash/tick.css` (style file)";
    let caps =
        get_captures(input).expect("Regex failed to capture hash backtick with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "hash/tick.css"); // Trailing text ignored
}

#[test]
fn test_extract_no_match() {
    assert!(get_captures("Just text").is_none());
    assert!(get_captures("**NotAnAction: path**").is_none());

    // Test case: "## File: "
    // Regex for hash action: ##\s+(?P<action_word_hash>{actions}):\s*(?P<content_hash>.*)$
    // This regex *should* match "## File: ", with content_hash = "".
    // The extractor should then return None because the path derived from empty content is empty.
    match get_captures("## File: ") {
        Some(caps) => {
            assert!(
                extract_header_action_details(&caps).is_none(),
                "Extractor should return None for '## File: ' (empty path)"
            );
        }
        None => {
            // This panic indicates that the HEADER_REGEX is not matching "## File: "
            // when analysis suggests it should (if content_hash uses '.*').
            panic!("HEADER_REGEX failed to match '## File: ', which was unexpected. Check if content_hash part is '.*'.");
        }
    }

    // Test case: "**File:**"
    // Regex for bold action: \*\*\s*(?P<action_word_bold>{actions}):\s*(?P<content_bold>.+?)\s*\*\*...
    // The `.+?` for content_bold requires at least one character.
    // So, "**File:**" (empty content) should NOT be matched by this part of HEADER_REGEX.
    assert!(get_captures("**File:**").is_none(),
            "HEADER_REGEX should not match '**File:**' because content_bold (.+?) requires non-empty content");

    // Test case: "## File: ``"
    // Regex should match (content_hash = " `` "). Extractor should return None (path from backticks is empty).
    match get_captures("## File: ``") {
        Some(caps) => {
            assert!(
                extract_header_action_details(&caps).is_none(),
                "Extractor should return None for '## File: ``' (empty path from backticks)"
            );
        }
        None => {
            panic!("HEADER_REGEX failed to match '## File: ``', which was unexpected.");
        }
    }

    // Test case: "**File: ``**"
    // Regex for bold: \*\*\s*(?P<action_word_bold>{actions}):\s*(?P<content_bold>.+?)\s*\*\*...
    // Here, content_bold would be "``". This is non-empty, so regex should match.
    // Extractor should then return None because path_between_ticks from "``" is empty.
    match get_captures("**File: ``**") {
        Some(caps) => {
            assert!(
                extract_header_action_details(&caps).is_none(),
                "Extractor should return None for '**File: ``**' (empty path from backticks)"
            );
        }
        None => {
            panic!("HEADER_REGEX failed to match '**File: ``**', which was unexpected. content_bold (.+?) should match ' `` '.");
        }
    }

    // Test case: "## File: ```" (three backticks)
    // Regex should match (content_hash = " ``` ").
    // parse_single_path_from_content(" ``` ") -> path_between_ticks is "`".
    // is_path_valid_for_action("`") is false. So extractor returns None.
    match get_captures("## File: ```") {
        Some(caps) => {
            assert!(
                extract_header_action_details(&caps).is_none(),
                "Extractor should return None for '## File: ```' (path '`' is invalid)"
            );
        }
        None => {
            panic!("HEADER_REGEX failed to match '## File: ```', which was unexpected.");
        }
    }

    // Test case: "**File: ```**" (three backticks)
    // Regex should match (content_bold = " ``` ").
    // Extractor logic is same as above: path is "`", which is invalid. So returns None.
    match get_captures("**File: ```**") {
        Some(caps) => {
            assert!(
                extract_header_action_details(&caps).is_none(),
                "Extractor should return None for '**File: ```**' (path '`' is invalid)"
            );
        }
        None => {
            panic!("HEADER_REGEX failed to match '**File: ```**', which was unexpected.");
        }
    }
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
