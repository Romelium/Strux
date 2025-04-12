//! Utilities for parsing internal comment headers (e.g., // File: path).

use crate::constants::INTERNAL_COMMENT_ACTION_PREFIX;

/// Extracts path from internal comment formats like `// File: path` or `// path`.
/// Returns Option<(path_string, is_header_included)>.
pub(crate) fn extract_path_from_internal_comment<'a>(
    line: &'a str, // Use original line for accurate check
    stripped_line: &'a str,
) -> Option<(String, bool)> {
    // bool is true if header line is included

    // Format: // File: path or // File: `path`
    if stripped_line.starts_with(INTERNAL_COMMENT_ACTION_PREFIX) {
        let content = stripped_line
            .strip_prefix(INTERNAL_COMMENT_ACTION_PREFIX)
            .unwrap()
            .trim();
        let path = if content.len() > 1 && content.starts_with('`') && content.ends_with('`') {
            content[1..content.len() - 1].trim().to_string()
        } else {
            content.to_string()
        };
        if path.is_empty() {
            None // Empty path after prefix is invalid
        } else {
            Some((path, false)) // Excluded from output
        }
    }
    // Format: //path or // path (but not // File:)
    else if let Some(path_part) = stripped_line.strip_prefix("//") {
        let potential_path = path_part.trim();

        // Basic validation: not empty
        if potential_path.is_empty() {
            return None;
        }

        // Heuristic: If it looks like another header format commented out, ignore it.
        if potential_path.starts_with("##") || potential_path.starts_with("**") {
            return None;
        }

        // Check if original line (ignoring leading whitespace) starts with "// "
        let original_starts_with_comment_space = line.trim_start().starts_with("// ");

        if !original_starts_with_comment_space {
            // Format is //path (no space after //) -> Treat as path, include header
            Some((potential_path.to_string(), true)) // INCLUDE = TRUE
        } else {
            // Format is // path (space after //) -> Ambiguous (path or comment?)
            // Heuristic: Treat as path only if it contains typical path chars. Exclude header line.
            let looks_like_path = potential_path.contains('/')
                || potential_path.contains('\\')
                || potential_path.contains('.');

            if looks_like_path {
                Some((potential_path.to_string(), false)) // INCLUDE = FALSE
            } else {
                None // Treat as a regular comment
            }
        }
    } else {
        None // Not a comment format we handle
    }
}

// REMOVED: Test module declaration moved to src/parser/mod.rs
// #[cfg(test)]
// mod internal_comment_tests;
