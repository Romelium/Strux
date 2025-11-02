//! Defines and compiles regular expressions used by the parser.

use crate::constants::VALID_ACTIONS_REGEX_STR; // Use the pre-built string
use once_cell::sync::Lazy;
use regex::Regex;

// Use Lazy from once_cell for thread-safe static initialization of Regex objects.

// Regex to find file headers anchored to the start of a line.
// Revised to simplify capture groups for non-backticked paths and rely more on Rust extraction.
pub static HEADER_REGEX: Lazy<Regex> = Lazy::new(|| {
    let actions = &*VALID_ACTIONS_REGEX_STR; // Dereference Lazy<String>

    let pattern = format!(
        // **Action: content**: Capture content greedily, allow optional trailing text after **.
        // ## Action: content: Capture content greedily, no optional trailing text needed here (extractor handles).
        // Backtick versions remain specific but allow optional trailing text after marker.
        "(?m)^(?:\\*\\*\\s*(?P<action_word_bold>{actions}):\\s*(?P<content_bold>.+?)\\s*\\*\\*(?:[^\\n]*)?$|#{{2,}}\\s+`(?P<path_hash_backtick>[^`\\n]+?)`(?:[^\\n]*)?$|#{{2,}}\\s+(?:.*?)?(?P<action_word_hash>{actions}):\\s*(?P<content_hash>.*)$|`(?P<path_backtick_only>[^`\\n]+?)`(?:[^\\n]*)?$|(?P<num>\\d+)\\.\\s+`(?P<path_numbered_backtick>[^`\\n]+?)`(?:[^\\n]*)?$|\\*\\*\\s*`(?P<path_bold_backtick>[^`\\n]+?)`\\s*\\*\\*(?:[^\\n]*)?$)",
        actions = actions // Argument for format!
    );
    // Explanation of changes:
    // - **Bold (`**...**`)**:
    //   - `action_word_bold` captures the action.
    //   - `content_bold` captures `.+?` (non-greedy) after `Action: \s*`. <--- Reverted to non-greedy
    //   - Requires `\s*\*\*` after content.
    //   - Added `(?:[^\n]*)?$` back to allow optional trailing text AFTER the closing **. <--- FIX
    // - **Hash (`##... Action: ...`)**:
    //   - `#{2,}` allows two or more hash symbols.
    //   - `(?:.*?)?` non-greedily captures any preceding text on the line.
    //   - `action_word_hash` captures the action.
    //   - `content_hash` captures `.*` (greedy, zero or more chars) after `Action: \s*`.
    // - **Backtick paths (`## `path``, `` `path` ``, `1. `path``, `**`path`**`)**:
    //   - These alternatives remain largely unchanged, capturing the path inside backticks specifically.
    //   - They still allow optional trailing text `(?:[^\n]*)?$` after the closing backtick/bold marker.
    // println!("[REGEX INIT] Revised HEADER_REGEX pattern:\n{}", pattern); // DEBUG (optional)
    Regex::new(&pattern).expect("Failed to compile HEADER_REGEX")
});

// Regex to find the START of a fenced code block.
pub static OPENING_FENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Handle optional carriage return for CRLF compatibility
    Regex::new("(?m)^\\s*(?P<fence>`{3,})(?P<lang>[^\\n\\r]*)(\\r?\\n)")
        .expect("Failed to compile OPENING_FENCE_REGEX")
});

// Note: Closing fence regex is generated dynamically in pass1.rs based on the opening fence.
