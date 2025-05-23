//! Tests for parsing 'Append File' and 'Prepend File' headers.

use super::common::*; // Use helper from common.rs
use strux::core_types::ActionType;
use strux::parse_markdown;

#[test]
fn test_parse_hash_append_file_header() {
    let md = "\n## Append File: data/log.txt\n```\nNew log entry\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed for ## Append File");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Append,
        "data/log.txt",
        None,
        Some("New log entry\n"),
    );
}

#[test]
fn test_parse_bold_append_file_header() {
    let md = "\n**Append File: notes.md**\n```markdown\n- Another point\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed for **Append File");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Append,
        "notes.md",
        None,
        Some("- Another point\n"),
    );
}

#[test]
fn test_parse_hash_prepend_file_header() {
    let md = "\n## Prepend File: script.sh\n```bash\n#!/bin/bash\n# Prepended header\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed for ## Prepend File");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Prepend,
        "script.sh",
        None,
        Some("#!/bin/bash\n# Prepended header\n"),
    );
}

#[test]
fn test_parse_bold_prepend_file_header() {
    let md = "\n**Prepend File: styles.css**\n```css\n/* Prepend common styles */\nbody { margin: 0; }\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed for **Prepend File");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Prepend,
        "styles.css",
        None,
        Some("/* Prepend common styles */\nbody { margin: 0; }\n"),
    );
}

#[test]
fn test_parse_wrapped_append_file_header() {
    let md = "\n```markdown\n## Append File: report.txt\n```\n\n```\nConclusion section.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed for wrapped Append File");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Append,
        "report.txt",
        None,
        Some("Conclusion section.\n"),
    );
}

#[test]
fn test_parse_wrapped_prepend_file_header() {
    let md =
        "\n```md\n**Prepend File: chapter.tex**\n```\n\n```latex\n\\documentclass{article}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed for wrapped Prepend File");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Prepend,
        "chapter.tex",
        None,
        Some("\\documentclass{article}\n"),
    );
}

#[test]
fn test_parse_append_file_header_no_block() {
    let md = "\n## Append File: orphan.log\nSome text but no code block.";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Append File header without a block should be ignored by Pass 1 and warned by Pass 2"
    );
}

#[test]
fn test_parse_prepend_file_header_no_block() {
    let md = "\n**Prepend File: orphan.ini**\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Prepend File header without a block should be ignored by Pass 1 and warned by Pass 2"
    );
}

// Internal comment headers for Append/Prepend are not yet supported by the current logic
// (INTERNAL_COMMENT_ACTION_PREFIX is only for "File:").
// If they were to be supported, tests like these would be added:
/*
#[test]
fn test_parse_internal_comment_append_file_header() {
    let md = "\n```\n// Append File: list.txt\n- item 3\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Append,
        "list.txt",
        None,
        Some("- item 3\n"),
    );
}
*/
