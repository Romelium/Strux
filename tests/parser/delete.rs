//! Tests for parsing 'Delete' headers (standalone, special block case, wrapped).

use super::common::*; // Use helper from common.rs
use strux::core_types::ActionType;
use strux::parse_markdown;

#[test]
fn test_parse_bold_deleted_file_header() {
    let md = "\n**Deleted File: old_config.cfg**\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Delete,
        "old_config.cfg",
        None, // No dest_path for Delete
        None,
    );
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
        None, // No dest_path for Delete
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
        None,                    // No dest_path for Delete
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
        None, // No dest_path for Delete
        None,
    );
}

#[test]
fn test_parse_wrapped_hash_deleted_file_header() {
    let md = "\nSome text.\n\n```markdown\n## Deleted File: old/data.json\n```\n\nMore text.\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Delete,
        "old/data.json",
        None, // No dest_path for Delete
        None,
    );
}

#[test]
fn test_parse_wrapped_bold_deleted_file_header() {
    let md = "\n```md\n**Deleted File: temp.log**\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Delete,
        "temp.log",
        None, // No dest_path for Delete
        None,
    );
}

#[test]
fn test_parse_hash_deleted_file_header_with_trailing_comment() {
    let md = "\n## Deleted File: old_cache.dat # Remove this\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Delete,
        "old_cache.dat", // Trailing comment ignored
        None,            // No dest_path for Delete
        None,
    );
}

#[test]
fn test_parse_bold_deleted_file_header_with_trailing_text_outside() {
    let md = "\n**Deleted File: report.pdf** (old version)\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Delete,
        "report.pdf",
        None, // Trailing text ignored, no dest_path for Delete
        None,
    );
}
