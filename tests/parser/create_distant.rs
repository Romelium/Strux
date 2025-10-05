//! Tests for parsing 'Create' headers that are not immediately adjacent to their code blocks.

use super::common::*;
use strux::core_types::ActionType;
use strux::parse_markdown;

#[test]
fn test_parse_header_separated_by_newlines() {
    let md = "\n### File: newlines.txt\n\n\n```\nSeparated by newlines.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "newlines.txt",
        None,
        Some("Separated by newlines.\n"),
    );
}

#[test]
fn test_parse_header_separated_by_paragraph() {
    let md = "\n#### Some extra text before the File: action/path.js\n\nThis is a paragraph of text that separates the header from the code block that follows it.\n\nIt can contain multiple lines.\n\n```javascript\nconsole.log('Distant block');\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "action/path.js",
        None,
        Some("console.log('Distant block');\n"),
    );
}

#[test]
fn test_parse_distant_header_with_backticks() {
    let md = "\n### File: `path/with spaces.txt`\n\nSome text in between.\n\n```\nContent.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "path/with spaces.txt",
        None,
        Some("Content.\n"),
    );
}

#[test]
fn test_parse_distant_header_claims_next_available_block() {
    let md = "\n`immediate.txt`\n```\nImmediate content.\n```\n\n### File: distant.txt\n\nSome text.\n\n```\nDistant content.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 2);

    // Check that they are ordered correctly by their position in the document
    assert_action(
        actions.first(),
        ActionType::Create,
        "immediate.txt",
        None,
        Some("Immediate content.\n"),
    );
    assert_action(
        actions.get(1),
        ActionType::Create,
        "distant.txt",
        None,
        Some("Distant content.\n"),
    );
}

#[test]
fn test_parse_distant_header_skips_processed_block() {
    let md = "\n### File: distant.txt\n\n`immediate.txt`\n```\nImmediate content.\n```\n\n```\nDistant content.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 2);

    // The `immediate.txt` header is closer to the first block, so Pass 1 associates it.
    // The `distant.txt` header is found in Pass 2, and it should then find the *next available* block, which is the second one.
    // Final actions are sorted by header position.
    assert_action(
        actions.first(),
        ActionType::Create,
        "distant.txt",
        None,
        Some("Distant content.\n"),
    );
    assert_action(
        actions.get(1),
        ActionType::Create,
        "immediate.txt",
        None,
        Some("Immediate content.\n"),
    );
}

#[test]
fn test_parse_orphaned_header_if_no_more_blocks() {
    let md = "\n`file1.txt`\n```\nContent 1.\n```\n\n### File: orphan.txt\n\nNo more code blocks after this header.\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1); // Only file1.txt should be created
    assert_action(
        actions.first(),
        ActionType::Create,
        "file1.txt",
        None,
        Some("Content 1.\n"),
    );
}

#[test]
fn test_parse_complex_distant_header() {
    let md = "\n### 1. New File: `lib/example.dart`\n\nblah blah\n\nblahblahblah\n\n```dart\ncontent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed for complex distant header");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "lib/example.dart",
        None,
        Some("content\n"),
    );
}
