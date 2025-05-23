//! Tests for parsing internal 'Create' headers (// File:, //path, ## File:).

use super::common::*; // Use helper from common.rs
use strux::core_types::ActionType;
use strux::parse_markdown;

#[test]
fn test_parse_internal_comment_file_header_excluded() {
    let md = "\n```rust\n// File: src/lib.rs\nfn main() {\n    println!(\"Internal\");\n}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/lib.rs",
        None,                                                  // No dest_path for Create
        Some("fn main() {\n    println!(\"Internal\");\n}\n"), // Header line excluded
    );
}

#[test]
fn test_parse_internal_comment_path_header_included() {
    let md_correct = "\n```python\n//myapp/main.py\nimport sys\n\nprint(sys.argv)\n```\n";
    let actions = parse_markdown(md_correct).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "myapp/main.py",
        None,                                                     // No dest_path for Create
        Some("//myapp/main.py\nimport sys\n\nprint(sys.argv)\n"), // Header line included
    );
}

#[test]
fn test_parse_internal_comment_backticks_path_excluded() {
    let md = "\n```\n// File: `path with spaces/file.txt`\nContent here.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "path with spaces/file.txt",
        None,                    // No dest_path for Create
        Some("Content here.\n"), // Header line excluded
    );
}

// --- NEW TESTS for ## File: internal header ---

#[test]
fn test_parse_internal_hash_file_header_excluded() {
    let md = "\n```yaml\n## File: config/settings.yaml\nkey: value\nlist:\n  - item1\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "config/settings.yaml",
        None,                                   // No dest_path for Create
        Some("key: value\nlist:\n  - item1\n"), // Header line excluded
    );
}

#[test]
fn test_parse_internal_hash_file_header_backticks_excluded() {
    let md = "\n```\n## File: `docs/My Document.md`\n# Title\nSome text.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "docs/My Document.md",
        None,                          // No dest_path for Create
        Some("# Title\nSome text.\n"), // Header line excluded
    );
}

#[test]
fn test_parse_internal_hash_file_header_not_first_line_ignored() {
    let md = "\n```\nSome initial content.\n## File: should_be_ignored.txt\nMore content.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Internal ## File: header not on the first line should be ignored"
    );
}

#[test]
fn test_parse_internal_hash_file_header_invalid_path_ignored() {
    let md = "\n```\n## File: invalid//path.log\nLog data\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Internal ## File: header with invalid path should be ignored"
    );
}

#[test]
fn test_parse_internal_hash_file_header_empty_path_ignored() {
    let md = "\n```\n## File: \nContent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Internal ## File: header with empty path should be ignored"
    );
}
