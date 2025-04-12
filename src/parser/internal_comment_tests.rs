//! Unit tests for internal_comment.rs functionality.

// Bring items from the specific module being tested into scope
use super::internal_comment::*; // CHANGED from super::*

#[test]
fn test_extract_internal_file_prefix() {
    let line = "// File: src/main.rs";
    let stripped = line.trim();
    let result = extract_path_from_internal_comment(line, stripped);
    assert_eq!(result, Some(("src/main.rs".to_string(), false))); // false = excluded
}

#[test]
fn test_extract_internal_file_prefix_with_ticks() {
    let line = "  // File: `path/with spaces/file.txt` ";
    let stripped = line.trim();
    let result = extract_path_from_internal_comment(line, stripped);
    assert_eq!(
        result,
        Some(("path/with spaces/file.txt".to_string(), false))
    ); // false = excluded
}

#[test]
fn test_extract_internal_file_prefix_empty_path() {
    let line = "// File: ";
    let stripped = line.trim();
    let result = extract_path_from_internal_comment(line, stripped);
    assert_eq!(result, None);
}

#[test]
fn test_extract_internal_path_only_no_space() {
    let line = "//src/app.js"; // No space after //
    let stripped = line.trim();
    let result = extract_path_from_internal_comment(line, stripped);
    // original_starts_with_comment_space = false -> returns Some(..., true)
    assert_eq!(result, Some(("src/app.js".to_string(), true))); // true = included
}

#[test]
fn test_extract_internal_path_only_with_space() {
    // This format is ambiguous, rely on path-like characters
    let line_with_space_path = "// src/app.js"; // Space after //
    let stripped_with_space_path = line_with_space_path.trim();
    let result_with_space_path =
        extract_path_from_internal_comment(line_with_space_path, stripped_with_space_path);
    // original_starts_with_comment_space = true. looks_like_path = true. -> returns Some(..., false)
    assert_eq!(
        result_with_space_path,
        Some(("src/app.js".to_string(), false)),
        "Test case: // src/app.js"
    ); // false = excluded

    let line_simple = "// simple.txt";
    let stripped_simple = line_simple.trim();
    let result_simple = extract_path_from_internal_comment(line_simple, stripped_simple);
    // original_starts_with_comment_space = true. looks_like_path = true. -> returns Some(..., false)
    assert_eq!(
        result_simple,
        Some(("simple.txt".to_string(), false)),
        "Test case: // simple.txt"
    ); // false = excluded

    let line_no_ext_with_space = "// justapath"; // Space after //
    let stripped_no_ext_with_space = line_no_ext_with_space.trim();
    let result_no_ext_with_space =
        extract_path_from_internal_comment(line_no_ext_with_space, stripped_no_ext_with_space);
    // original_starts_with_comment_space = true. looks_like_path = false. -> returns None
    assert_eq!(result_no_ext_with_space, None, "Test case: // justapath"); // Treat as comment

    // Let's add the NO SPACE version of justapath explicitly
    let line_no_ext_no_space = "//justapath"; // NO Space after //
    let stripped_no_ext_no_space = line_no_ext_no_space.trim();
    let result_no_ext_no_space =
        extract_path_from_internal_comment(line_no_ext_no_space, stripped_no_ext_no_space);
    // original_starts_with_comment_space = false -> returns Some(..., true)
    assert_eq!(
        result_no_ext_no_space,
        Some(("justapath".to_string(), true)),
        "Test case: //justapath"
    ); // Include header

    let line_comment = "// This is just a comment"; // Space after //
    let stripped_comment = line_comment.trim();
    let result_comment = extract_path_from_internal_comment(line_comment, stripped_comment);
    // original_starts_with_comment_space = true. looks_like_path = false. -> returns None
    assert_eq!(result_comment, None, "Test case: // This is just a comment");
    // Treat as comment
}

#[test]
fn test_extract_internal_path_only_empty() {
    let line = "//";
    let stripped = line.trim();
    let result = extract_path_from_internal_comment(line, stripped);
    // potential_path is empty -> returns None
    assert_eq!(result, None);

    let line_space = "// ";
    let stripped_space = line_space.trim();
    let result_space = extract_path_from_internal_comment(line_space, stripped_space);
    // potential_path is empty -> returns None
    assert_eq!(result_space, None);
}

#[test]
fn test_extract_not_a_comment() {
    let line = "File: not/a/comment.txt";
    let stripped = line.trim();
    let result = extract_path_from_internal_comment(line, stripped);
    assert_eq!(result, None);
}

#[test]
fn test_extract_internal_path_looks_like_file_prefix() {
    // Should not confuse "// File: path" with "// File:path" when parsing "// path" format
    let line_ambiguous = "//File:path.txt"; // No space after //
    let stripped_ambiguous = line_ambiguous.trim();
    let result_ambiguous = extract_path_from_internal_comment(line_ambiguous, stripped_ambiguous);
    // Doesn't start "// File:". Starts "//". potential_path = "File:path.txt".
    // original_starts_with_comment_space = false -> returns Some(("File:path.txt", true))
    assert_eq!(result_ambiguous, Some(("File:path.txt".to_string(), true)));

    // Check the actual `// File:` rule again to be sure.
    let line_correct_file = "// File: path.txt";
    let stripped_correct_file = line_correct_file.trim();
    let result_correct_file =
        extract_path_from_internal_comment(line_correct_file, stripped_correct_file);
    // Starts "// File:" -> returns Some(("path.txt", false))
    assert_eq!(result_correct_file, Some(("path.txt".to_string(), false))); // Matches File: rule
}
