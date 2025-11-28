//! Defines constants used throughout the application.

use once_cell::sync::Lazy; // Used for regex string builder, though regexes themselves are in parser::regex
use regex; // Needed for regex::escape

// --- Action Keywords ---
pub const ACTION_FILE: &str = "File";
pub const ACTION_DELETED_FILE: &str = "Deleted File";
pub const ACTION_MOVED_FILE: &str = "Moved File";
pub const ACTION_APPEND_FILE: &str = "Append File";
pub const ACTION_PREPEND_FILE: &str = "Prepend File";
pub const ACTION_CREATE: &str = "Create";
pub const ACTION_UPDATE: &str = "Update";

// --- Parsing ---
pub const INTERNAL_COMMENT_ACTION_PREFIX: &str = "// File:";
// Consider if we need // Append File: or // Prepend File: prefixes later. For now, stick to File.

// Helper to build the VALID_ACTIONS_REGEX string component once.
// This is used by the regex definition in `parser::regex`.
pub static VALID_ACTIONS_REGEX_STR: Lazy<String> = Lazy::new(|| {
    [
        ACTION_FILE,
        ACTION_DELETED_FILE,
        ACTION_MOVED_FILE,
        ACTION_APPEND_FILE,
        ACTION_PREPEND_FILE,
        ACTION_CREATE, // Added
        ACTION_UPDATE, // Added
    ]
    .iter()
    .map(|a| regex::escape(a))
    .collect::<Vec<_>>()
    .join("|")
});

// Note: Regex objects themselves are defined in `src/parser/regex.rs`
// to keep them close to the parsing logic and avoid circular dependencies
// if constants were needed *by* the regex module setup.
