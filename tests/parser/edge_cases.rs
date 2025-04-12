//! Tests for parser edge cases and invalid formats.

// Use helper from common.rs
use markdown_processor::parse_markdown;

#[test]
fn test_parse_empty_input() {
    let md = "";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_no_actions() {
    let md = "\nRegular markdown.\n```\nlet x = 5;\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_unclosed_fence() {
    let md = "\n**File: incomplete.txt**\n```\nThis block never closes.\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Action should be skipped for unclosed fence"
    );
}

#[test]
fn test_parse_header_without_block() {
    let md = "\n**File: orphan.txt**\n\nSome other text.\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Action should be skipped for header without block"
    );
}

#[test]
fn test_parse_block_without_header() {
    let md = "\n```\nNo header here.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_invalid_action_word() {
    let md = "\n**Created: file.txt**\n```\ncontent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty(), "Invalid action word should be ignored");
}

#[test]
fn test_parse_header_missing_path() {
    let md = "\n## File:\n```\ncontent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty(), "Header missing path should be ignored");
}

#[test]
fn test_parse_internal_delete_header_ignored() {
    let md =
        "\n```\n**Deleted File: inside.txt**\nThis content is associated with no action.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_ignore_markdown_wrapper() {
    let md = "\n```markdown\n**File: ignored.txt**\n```\nThis should not be parsed.\n```\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Content wrapped in ```markdown should be ignored"
    );
}

#[test]
fn test_parse_only_markdown_wrapper() {
    let md = "```markdown\n```";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

// test_parse_invalid_path_format_skipped MOVED to invalid_paths.rs
