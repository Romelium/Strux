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
        // 1. Bold Action: **Action: content**
        // 2. Hash Backtick: #+ `path`
        // 3. Hash Action: #+ [text]Action[:] content (Colon is now optional, hashes 1+)
        // 4. Backtick Only: `path`
        // 5. Numbered: [#+] int. content (Hashes optional, content generic)
        // 6. Bold Backtick: **`path`**
        "(?m)^(?:\\*\\*\\s*(?P<action_word_bold>{actions}):\\s*(?P<content_bold>.+?)\\s*\\*\\*(?:[^\\n]*)?$|#+\\s+`(?P<path_hash_backtick>[^`\\n]+?)`(?:[^\\n]*)?$|#+\\s+(?:.*?)?(?P<action_word_hash>{actions})(?::\\s*|\\s+)(?P<content_hash>.*)$|`(?P<path_backtick_only>[^`\\n]+?)`(?:[^\\n]*)?$|(?:#+\\s+)?(?P<num>\\d+)\\.\\s+(?P<content_numbered>.*)$|\\*\\*\\s*`(?P<path_bold_backtick>[^`\\n]+?)`\\s*\\*\\*(?:[^\\n]*)?)",
        actions = actions // Argument for format!
    );

    Regex::new(&pattern).expect("Failed to compile HEADER_REGEX")
});

// Regex to find the START of a fenced code block.
pub static OPENING_FENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Handle optional carriage return for CRLF compatibility
    Regex::new("(?m)^\\s*(?P<fence>`{3,})(?P<lang>[^\\n\\r]*)(\\r?\\n)")
        .expect("Failed to compile OPENING_FENCE_REGEX")
});

// Note: Closing fence regex is generated dynamically in pass1.rs based on the opening fence.
