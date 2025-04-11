//! Path validation utilities for the parser.

use crate::errors::ProcessError; // Keep ProcessError for the return type consistency

/// Validates if a path string contains potentially problematic components like empty segments.
/// Note: This is a basic check run during parsing. More robust canonicalization and safety
/// checks happen during processing.
pub(crate) fn validate_path_format(path_str: &str) -> Result<(), ProcessError> {
    // Allow empty input string here, processor will handle it if needed.
    // The main goal is to catch `a//b` or `/` at the end causing empty components.
    if path_str.is_empty() {
        // Let processor handle empty paths if they somehow get through parsing rules
        return Ok(());
    }

    // Check for consecutive separators
    if path_str.contains("//") || path_str.contains(r"\\") {
        return Err(ProcessError::InvalidPathFormat {
            path: path_str.to_string(),
        });
    }

    // Check for trailing separators (unless it's just the root separator)
    if (path_str.ends_with('/') && path_str.len() > 1)
        || (path_str.ends_with('\\') && path_str.len() > 1)
    {
        Err(ProcessError::InvalidPathFormat {
            path: path_str.to_string(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path_format_valid() {
        assert!(validate_path_format("a/b/c.txt").is_ok());
        assert!(validate_path_format("a").is_ok());
        assert!(validate_path_format("a/b/").is_err()); // Trailing slash creates empty component
        assert!(validate_path_format("/a/b").is_ok()); // Leading slash is root dir, not empty component
        assert!(validate_path_format(".").is_ok());
        assert!(validate_path_format("..").is_ok());
        assert!(validate_path_format("a/../b").is_ok());
        assert!(validate_path_format("a b/c d").is_ok()); // Spaces are allowed
        assert!(validate_path_format("").is_ok()); // Empty string is allowed by this check
    }

    #[test]
    fn test_validate_path_format_invalid() {
        match validate_path_format("a//b") {
            Err(ProcessError::InvalidPathFormat { path }) => assert_eq!(path, "a//b"),
            _ => panic!("Expected InvalidPathFormat error"),
        }
        match validate_path_format("a/b//c.txt") {
            Err(ProcessError::InvalidPathFormat { path }) => assert_eq!(path, "a/b//c.txt"),
            _ => panic!("Expected InvalidPathFormat error"),
        }
        match validate_path_format("//a") {
            // Starts with //
            Err(ProcessError::InvalidPathFormat { path }) => assert_eq!(path, "//a"),
            _ => panic!("Expected InvalidPathFormat error"),
        }
        match validate_path_format("a/") {
            // Ends with /
            Err(ProcessError::InvalidPathFormat { path }) => assert_eq!(path, "a/"),
            _ => panic!("Expected InvalidPathFormat error"),
        }
        match validate_path_format("a/b/") {
            // Ends with /
            Err(ProcessError::InvalidPathFormat { path }) => assert_eq!(path, "a/b/"),
            _ => panic!("Expected InvalidPathFormat error"),
        }
    }
}
