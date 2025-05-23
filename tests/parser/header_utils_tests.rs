//! Unit tests for header_utils.rs functionality.

// Bring items from the specific module being tested into scope
use strux::parser::header_utils::{extract_header_action_details, get_action_type}; // Corrected path

// Bring items from other modules needed for tests into scope
use strux::constants::{
    ACTION_APPEND_FILE, ACTION_DELETED_FILE, ACTION_FILE, ACTION_MOVED_FILE, ACTION_PREPEND_FILE,
}; // Corrected path
use strux::core_types::ActionType; // Corrected path
use strux::parser::regex::HEADER_REGEX; // Corrected path

fn get_captures(text: &str) -> Option<regex::Captures<'_>> {
    let caps = HEADER_REGEX.captures(text);
    caps
}

#[test]
fn test_extract_bold_file() {
    let input = "**File: path/to/file.txt**";
    let caps = get_captures(input).expect("Regex failed to capture bold file");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/to/file.txt");
    assert!(details.dest_path.is_none());
}

#[test]
fn test_extract_hash_append_file() {
    let input = "## Append File: log.txt";
    let caps = get_captures(input).expect("Regex failed to capture hash append file");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_APPEND_FILE);
    assert_eq!(details.path, "log.txt");
    assert!(details.dest_path.is_none());
}

#[test]
fn test_extract_bold_prepend_file() {
    let input = "**Prepend File: header.md**";
    let caps = get_captures(input).expect("Regex failed to capture bold prepend file");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_PREPEND_FILE);
    assert_eq!(details.path, "header.md");
    assert!(details.dest_path.is_none());
}

#[test]
fn test_extract_hash_moved_file() {
    let input = "## Moved File: old.txt to new.txt";
    let caps = get_captures(input).expect("Regex failed to capture hash moved file");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_MOVED_FILE);
    assert_eq!(details.path, "old.txt");
    assert_eq!(details.dest_path, Some("new.txt".to_string()));
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
    assert_eq!(details.action_word, ACTION_FILE); // Backtick only implies File
    assert_eq!(details.path, "simple/path.rs");
}

#[test]
fn test_extract_bold_file_with_trailing_text_outside() {
    let input = "**File: path/to/file.txt** (description)";
    let caps = get_captures(input).expect("Regex failed to capture bold file with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/to/file.txt"); // Trailing text ignored by extractor
}

#[test]
fn test_extract_hash_file_with_trailing_text() {
    let input = "## File: path/to/file.txt # comment";
    let caps = get_captures(input).expect("Regex failed to capture hash file with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/to/file.txt"); // Trailing text ignored by extractor
}

#[test]
fn test_extract_hash_file_with_backticks_and_trailing_text() {
    let input = "## File: `path/in/ticks.txt` (description)";
    let caps = get_captures(input)
        .expect("Regex failed to capture hash file with ticks and trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "path/in/ticks.txt"); // Backticks stripped, trailing text ignored
}

#[test]
fn test_extract_backtick_only_with_trailing_text_outside() {
    let input = "`simple/path.rs` (some comment)";
    let caps =
        get_captures(input).expect("Regex failed to capture backtick only with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "simple/path.rs"); // Trailing text ignored
}

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

#[test]
fn test_extract_bold_backtick_with_trailing_text() {
    let input = "**`bold/tick.js`** and more";
    let caps =
        get_captures(input).expect("Regex failed to capture bold backtick with trailing text");
    let details = extract_header_action_details(&caps).unwrap();
    assert_eq!(details.action_word, ACTION_FILE);
    assert_eq!(details.path, "bold/tick.js"); // Trailing text ignored
}

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

    match get_captures("## File: ") {
        Some(caps) => {
            assert!(
                extract_header_action_details(&caps).is_none(),
                "Extractor should return None for '## File: ' (empty path)"
            );
        }
        None => {
            panic!("HEADER_REGEX failed to match '## File: ', which was unexpected. Check if content_hash part is '.*'.");
        }
    }
    assert!(get_captures("**File:**").is_none(),
            "HEADER_REGEX should not match '**File:**' because content_bold (.+?) requires non-empty content");

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
    assert_eq!(get_action_type(ACTION_MOVED_FILE), Some(ActionType::Move));
    assert_eq!(
        get_action_type(ACTION_APPEND_FILE),
        Some(ActionType::Append)
    );
    assert_eq!(
        get_action_type(ACTION_PREPEND_FILE),
        Some(ActionType::Prepend)
    );
}

#[test]
fn test_get_action_type_invalid() {
    assert_eq!(get_action_type("Create File"), None);
    assert_eq!(get_action_type(""), None);
    assert_eq!(get_action_type(" Patch File "), None);
}
