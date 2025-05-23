//! Tests for action ordering and mixing header types.

use super::common::*; // Use helper from common.rs
use strux::core_types::ActionType;
use strux::parse_markdown;

#[test]
fn test_parse_multiple_actions_ordered() {
    let md = "\n## File: setup.sh\n```bash\necho \"Setting up...\"\n```\n\n**Deleted File: old.log**\n\n`src/main.rs`\n```rust\nfn main() {}\n```\n\n## Moved File: temp/old_name.txt to final/new_name.txt\n\n## Append File: data.log\n```\nAppended line\n```\n\n**Prepend File: config.txt**\n```\nPrepended line\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 6);

    // Actions should be in document order
    assert_action(
        actions.first(), // 0
        ActionType::Create,
        "setup.sh",
        None,
        Some("echo \"Setting up...\"\n"),
    );
    assert_action(actions.get(1), ActionType::Delete, "old.log", None, None);
    assert_action(
        actions.get(2),
        ActionType::Create,
        "src/main.rs",
        None,
        Some("fn main() {}\n"),
    );
    assert_action(
        actions.get(3),
        ActionType::Move,
        "temp/old_name.txt",
        Some("final/new_name.txt"),
        None,
    );
    assert_action(
        actions.get(4),
        ActionType::Append,
        "data.log",
        None,
        Some("Appended line\n"),
    );
    assert_action(
        actions.get(5),
        ActionType::Prepend,
        "config.txt",
        None,
        Some("Prepended line\n"),
    );
}

#[test]
fn test_parse_mixed_wrapped_and_unwrapped() {
    let md = "\n## File: unwrapped1.txt\n```\nUnwrapped 1\n```\n\n```markdown\n## File: wrapped1.txt\n```\n```\nWrapped 1\n```\n\n```markdown\n## Deleted File: wrapped_del.log\n```\n\n**Deleted File: unwrapped_del.log**\n\n`unwrapped2.txt`\n```\nUnwrapped 2\n```\n\n```markdown\n## Moved File: staging/data.zip to archive/data.zip\n```\n\n```markdown\n## Append File: notes.txt\n```\n```\nAppended note\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 7, "Expected 7 actions");

    // Check order and details
    assert_action(
        actions.first(), // 0
        ActionType::Create,
        "unwrapped1.txt",
        None,
        Some("Unwrapped 1\n"),
    );
    assert_action(
        actions.get(1),
        ActionType::Create,
        "wrapped1.txt",
        None,
        Some("Wrapped 1\n"),
    );
    assert_action(
        actions.get(2),
        ActionType::Delete,
        "wrapped_del.log",
        None,
        None,
    );
    assert_action(
        actions.get(3),
        ActionType::Delete,
        "unwrapped_del.log",
        None,
        None,
    );
    assert_action(
        actions.get(4),
        ActionType::Create,
        "unwrapped2.txt",
        None,
        Some("Unwrapped 2\n"),
    );
    assert_action(
        actions.get(5),
        ActionType::Move,
        "staging/data.zip",
        Some("archive/data.zip"),
        None,
    );
    assert_action(
        actions.get(6),
        ActionType::Append,
        "notes.txt",
        None,
        Some("Appended note\n"),
    );
}
