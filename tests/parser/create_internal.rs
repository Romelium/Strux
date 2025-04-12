//! Tests for parsing internal 'Create' headers (// File:, //path).

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
        Some("Content here.\n"), // Header line excluded
    );
}
