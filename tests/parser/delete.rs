//! Tests for parsing 'Delete' headers (standalone, special block case, wrapped).

use super::common::*; // Use helper from common.rs
use markdown_processor::core_types::ActionType;
use markdown_processor::parse_markdown;

#[test]
fn test_parse_bold_deleted_file_header() {
    let md = "\n**Deleted File: old_config.cfg**\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(actions.first(), ActionType::Delete, "old_config.cfg", None);
}

#[test]
fn test_parse_hash_deleted_file_header() {
    let md = "\n## Deleted File: temp/file_to_remove.tmp\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Delete,
        "temp/file_to_remove.tmp",
        None,
    );
}

#[test]
fn test_parse_hash_deleted_file_header_with_block() {
    // Special case: path is *in* the block
    let md = "\n## Deleted File:\n```\npath/inside/block.log\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Delete,
        "path/inside/block.log", // Path comes from block content
        None,
    );
}

#[test]
fn test_parse_special_delete_header_empty_block() {
    let md = "\n## Deleted File:\n```\n```\n";
    // Should parse, but log a warning and produce no action
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_special_delete_header_multi_line_block() {
    let md = "\n## Deleted File:\n```\npath/to/delete.txt\nsome other ignored line\n```\n";
    // Should parse, log a warning, but use the first line
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Delete,
        "path/to/delete.txt",
        None,
    );
}

#[test]
fn test_parse_wrapped_hash_deleted_file_header() {
    let md = "\nSome text.\n\n```markdown\n## Deleted File: old/data.json\n```\n\nMore text.\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(actions.first(), ActionType::Delete, "old/data.json", None);
}

#[test]
fn test_parse_wrapped_bold_deleted_file_header() {
    let md = "\n```md\n**Deleted File: temp.log**\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(actions.first(), ActionType::Delete, "temp.log", None);
}
