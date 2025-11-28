//! Tests for parsing wrapped 'Create' headers (header in ```markdown block).

use super::common::*; // Use helper from common.rs
use strux::core_types::ActionType;
use strux::parse_markdown;

#[test]
fn test_parse_wrapped_hash_file_header() {
    let md = "\n```markdown\n## File: wrapped/config.toml\n```\n\n```toml\n[settings]\nkey = \"value\"\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "wrapped/config.toml",
        None, // No dest_path for Create
        Some("[settings]\nkey = \"value\"\n"),
    );
}

#[test]
fn test_parse_wrapped_bold_file_header() {
    let md = "\n```md\n**File: src/main.js**\n```\n\n```javascript\nconsole.log('Hello');\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/main.js",
        None, // No dest_path for Create
        Some("console.log('Hello');\n"),
    );
}

#[test]
fn test_parse_wrapped_numbered_list_header() {
    // New test for flexible numbered list inside wrapper
    let md = "\n```markdown\n1. wrapped/list_item.txt\n```\n\n```\ncontent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "wrapped/list_item.txt",
        None,
        Some("content\n"),
    );
}

#[test]
fn test_parse_wrapped_create_keyword_header() {
    // New test for flexible Create keyword inside wrapper
    let md = "\n```markdown\n## Create wrapped/keyword.txt\n```\n\n```\ncontent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "wrapped/keyword.txt",
        None,
        Some("content\n"),
    );
}

#[test]
fn test_parse_wrapped_file_header_not_followed_by_block() {
    let md =
        "\n```markdown\n## File: orphan_wrapped.txt\n```\n\nThis is just text, not a code block.\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Wrapped File header not followed by code block should be ignored"
    );
}

#[test]
fn test_parse_wrapped_file_header_followed_by_non_adjacent_block() {
    let md = "\n```markdown\n## File: spaced_out.txt\n```\n\nSome separating text.\n\n```\nContent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Wrapped File header not immediately followed by code block should be ignored"
    );
}

#[test]
fn test_parse_wrapped_header_invalid_path() {
    let md = "\n```markdown\n## File: invalid//path.cfg\n```\n\n```\n[ignored]\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Wrapped header with invalid path should be ignored"
    );
}

#[test]
fn test_parse_markdown_block_multiple_lines() {
    let md =
        "\n```markdown\n## File: multi.txt\nThis is a second line.\n```\n\n```\nContent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Markdown block with multiple lines should not be treated as a header block"
    );
}

#[test]
fn test_parse_markdown_block_not_a_header() {
    let md = "\n```markdown\nJust some markdown text inside.\n```\n\n```\nContent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Markdown block with non-header line should be ignored"
    );
}

#[test]
fn test_parse_wrapped_hash_file_header_with_trailing_text() {
    let md = "\n```markdown\n## File: wrapped/config.toml # Main config\n```\n\n```toml\n[settings]\nkey = \"value\"\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "wrapped/config.toml", // Trailing text ignored
        None,                  // No dest_path for Create
        Some("[settings]\nkey = \"value\"\n"),
    );
}
