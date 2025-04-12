//! Defines and compiles regular expressions used by the parser.

use crate::constants::VALID_ACTIONS_REGEX_STR; // Use the pre-built string
use once_cell::sync::Lazy;
use regex::Regex;

// Use Lazy from once_cell for thread-safe static initialization of Regex objects.

// Regex to find file headers anchored to the start of a line.
// Revised to prevent matching across lines and simplify trailing whitespace handling.
pub static HEADER_REGEX: Lazy<Regex> = Lazy::new(|| {
    let actions = &*VALID_ACTIONS_REGEX_STR; // Dereference Lazy<String>

    // Use a single multi-line raw string literal r#"..."# as the format string
    // Put back \s*$ anchor specifically for content_bold and content_hash alternatives.
    // Use non-greedy *? for content capture before the \s*$ anchor.
    // No final \s*$ at the very end of the whole pattern string.
    let pattern = format!(
        r#"(?m)^(?:\*\*\s*(?P<action_word_bold>{actions}):\s+(?P<content_bold>[^\n]+?)\s*\*\*|##\s+`(?P<path_hash_backtick>[^`\n]+?)`|##\s+(?P<action_word_hash>{actions}):\s*(?P<content_hash>[^\n]*?)\s*$|`(?P<path_backtick_only>[^`\n]+?)`|(?P<num>\d+)\.\s+`(?P<path_numbered_backtick>[^`\n]+?)`|\*\*\s*`(?P<path_bold_backtick>[^`\n]+?)`\s*\*\*)"#,
        // Note: Added \s*$ to content_hash alternative only. content_bold already had \s*\*\* which acts similarly.
        //       Kept content_hash as *? (non-greedy)
        actions = actions // Argument for format!
    );
    // println!("[REGEX INIT] Revised HEADER_REGEX pattern:\n{}", pattern); // DEBUG (optional)
    Regex::new(&pattern).expect("Failed to compile HEADER_REGEX")
});

// Regex to find the START of a fenced code block.
pub static OPENING_FENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Use raw string literal
    // Handle optional carriage return for CRLF compatibility
    Regex::new(r#"(?m)^\s*(?P<fence>```|````)(?P<lang>[^\n\r]*)(\r?\n)"#)
        .expect("Failed to compile OPENING_FENCE_REGEX")
});

// Note: Closing fence regex is generated dynamically in pass1.rs based on the opening fence.

// --- REMOVED DEBUG FUNCTION ---
// debug_hash_regexes() removed to reduce line count.
