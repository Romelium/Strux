//! Tests for action ordering and mixing header types.

use super::common::*; // Use helper from common.rs
use markdown_processor::core_types::ActionType;
use markdown_processor::parse_markdown;

#[test]
fn test_parse_multiple_actions_ordered() {
    let md = "\n## File: setup.sh\n```bash\necho \"Setting up...\"\n```\n\n**Deleted File: old.log**\n\n`src/main.rs`\n```rust\nfn main() {}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 3);

    // Actions should be in document order
    assert_action(
        actions.first(),
        ActionType::Create,
        "setup.sh",
        Some("echo \"Setting up...\"\n"),
    );
    assert_action(actions.get(1), ActionType::Delete, "old.log", None);
    assert_action(
        actions.get(2),
        ActionType::Create,
        "src/main.rs",
        Some("fn main() {}\n"),
    );
}

#[test]
fn test_parse_mixed_wrapped_and_unwrapped() {
    let md = "\n## File: unwrapped1.txt\n```\nUnwrapped 1\n```\n\n```markdown\n## File: wrapped1.txt\n```\n```\nWrapped 1\n```\n\n```markdown\n## Deleted File: wrapped_del.log\n```\n\n**Deleted File: unwrapped_del.log**\n\n`unwrapped2.txt`\n```\nUnwrapped 2\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 5, "Expected 5 actions");

    // Check order and details
    assert_action(
        actions.first(),
        ActionType::Create,
        "unwrapped1.txt",
        Some("Unwrapped 1\n"),
    );
    assert_action(
        actions.get(1),
        ActionType::Create,
        "wrapped1.txt",
        Some("Wrapped 1\n"),
    );
    assert_action(actions.get(2), ActionType::Delete, "wrapped_del.log", None);
    assert_action(
        actions.get(3),
        ActionType::Delete,
        "unwrapped_del.log",
        None,
    );
    assert_action(
        actions.get(4),
        ActionType::Create,
        "unwrapped2.txt",
        Some("Unwrapped 2\n"),
    );
}
