//! Tests for nested code blocks and content variations.

use super::common::*; // Use helper from common.rs
use markdown_processor::core_types::ActionType;
use markdown_processor::parse_markdown;

#[test]
fn test_parse_nested_code_blocks_simple() {
    let md = "\n## File: src/nested_example.md\n```markdown\nOuter block.\n```bash\nInner\n```\nOuter continues.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    let expected_content = "Outer block.\n```bash\nInner\n```\nOuter continues.\n";
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/nested_example.md",
        Some(expected_content),
    );
}

#[test]
fn test_parse_nested_blocks_from_readme_example() {
    // Simplified version of the complex README example focusing on nesting
    let md = "\n## File: Readme\n```markdown\n# Section\n```bash\nCode\n```\nMore text.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    let expected_content = "# Section\n```bash\nCode\n```\nMore text.\n";
    assert_action(
        actions.first(),
        ActionType::Create,
        "Readme",
        Some(expected_content),
    );
}

#[test]
fn test_parse_empty_content() {
    let md = "\n## File: empty.txt\n```\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    // CORRECTED: Empty block should result in empty content.
    // ensure_trailing_newline only adds a newline if the content is non-empty.
    assert_action(actions.first(), ActionType::Create, "empty.txt", Some(""));
}

#[test]
fn test_parse_content_no_trailing_newline() {
    // CORRECTED: Place closing fence on its own line.
    let md = "\n**File: data.csv**\n```\ncol1,col2\nval1,val2\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1, "Expected one action");
    // Content extracted is "col1,col2\nval1,val2".
    // ensure_trailing_newline adds the final '\n'.
    assert_action(
        actions.first(),
        ActionType::Create,
        "data.csv",
        Some("col1,col2\nval1,val2\n"),
    );
}
