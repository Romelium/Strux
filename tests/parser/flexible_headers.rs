//! Tests for the new flexible header formats (Create, Update, Numbered lists).

use super::common::*;
use strux::core_types::ActionType;
use strux::parse_markdown;

// --- Numbered List Tests ---

#[test]
fn test_parse_numbered_list_hashes_plain_path() {
    // Pattern: [hashtags] [integer]. [path]
    let md = "\n### 1. src/main.rs\n```rust\nfn main() {}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/main.rs",
        None,
        Some("fn main() {}\n"),
    );
}

#[test]
fn test_parse_numbered_list_no_hashes_plain_path() {
    // Pattern: [integer]. [path]
    let md = "\n1. src/main.rs\n```rust\nfn main() {}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/main.rs",
        None,
        Some("fn main() {}\n"),
    );
}

#[test]
fn test_parse_numbered_list_large_number() {
    // Pattern: [large integer]. [path]
    let md = "\n99. src/final.rs\n```rust\n// final\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/final.rs",
        None,
        Some("// final\n"),
    );
}

#[test]
fn test_parse_numbered_list_with_text_and_backticks() {
    // Pattern: [integer]. [text] [path in ticks]
    // The parser extracts the content inside backticks if present.
    let md = "\n1. Create the file `src/main.rs`\n```rust\nfn main() {}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/main.rs",
        None,
        Some("fn main() {}\n"),
    );
}

#[test]
fn test_parse_numbered_list_hashes_text_and_backticks() {
    // Pattern: [hashtags] [integer]. [text] [path in ticks]
    let md = "\n### 10. Setup config: `config.toml`\n```toml\n[key]\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "config.toml",
        None,
        Some("[key]\n"),
    );
}

#[test]
fn test_parse_numbered_list_with_create_keyword() {
    // Pattern: [integer]. Create [path]
    // This matches the numbered list regex, and "Create src/file.txt" is the content.
    // The path extractor sees "Create src/file.txt".
    // "Create src/file.txt" has 1 space. It passes the space heuristic.
    // However, the path will be extracted as "Create src/file.txt" unless we use backticks.
    // This test confirms that without backticks, the whole string is the path.
    let md = "\n1. Create src/file.txt\n```\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    // Note: This behavior is expected given the current "numbered list" regex which captures everything.
    // Users should use backticks if they add descriptive text in numbered lists.
    assert_eq!(actions[0].path, "Create src/file.txt");
}

// --- Create/Update Keyword Tests ---

#[test]
fn test_parse_create_keyword_simple() {
    // Pattern: [hashtags] Create [path]
    let md = "\n## Create src/lib.rs\n```rust\n// lib\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/lib.rs",
        None,
        Some("// lib\n"),
    );
}

#[test]
fn test_parse_update_keyword_simple() {
    // Pattern: [hashtags] Update [path]
    // Update maps to Create action type
    let md = "\n## Update src/config.toml\n```toml\nkey=value\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/config.toml",
        None,
        Some("key=value\n"),
    );
}

#[test]
fn test_parse_create_keyword_with_prefix_text() {
    // Pattern: [hashtags] [text]Create [path]
    let md = "\n## Please Create src/utils.rs\n```rust\n// utils\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/utils.rs",
        None,
        Some("// utils\n"),
    );
}

#[test]
fn test_parse_update_keyword_with_prefix_text_and_colon() {
    // Pattern: [hashtags] [text]Update: [path]
    let md = "\n### Step 2: Update: src/utils.rs\n```rust\n// updated\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/utils.rs",
        None,
        Some("// updated\n"),
    );
}

#[test]
fn test_parse_create_keyword_with_intermediate_text_and_backticks() {
    // Pattern: [hashtags] Create [text] [path in ticks]
    // The regex captures "Create" as action, and "file `src/main.rs`" as content.
    // The path extractor finds the backticks.
    let md = "\n## Create file `src/main.rs`\n```rust\nfn main() {}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/main.rs",
        None,
        Some("fn main() {}\n"),
    );
}

#[test]
fn test_parse_update_keyword_with_intermediate_text_no_backticks_fail() {
    // Pattern: [hashtags] Update [text] [path] (no ticks)
    // Content becomes "file src/main.rs".
    // This is treated as the path. If it has few spaces, it might be accepted as a filename with spaces.
    // If it has many spaces, it is rejected.
    let md = "\n## Update the configuration file src/config.toml\n```\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    // "the configuration file src/config.toml" has 4 spaces.
    // The heuristic allows <= 5 spaces. So this is technically accepted as a filename!
    // This test documents that behavior.
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].path, "the configuration file src/config.toml");
}

#[test]
fn test_parse_update_keyword_with_intermediate_text_many_spaces_fail() {
    // Pattern: [hashtags] Update [long text] [path]
    let md = "\n## Update the very long configuration file description src/config.toml\n```\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    // > 5 spaces, should be rejected by heuristic
    assert!(actions.is_empty());
}

#[test]
fn test_parse_create_one_hash() {
    // Pattern: # Create [path]
    let md = "\n# Create root.txt\n```\nroot\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "root.txt",
        None,
        Some("root\n"),
    );
}

#[test]
fn test_parse_create_many_hashes() {
    // Pattern: #### Create [path]
    let md = "\n#### Create deep.txt\n```\ndeep\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "deep.txt",
        None,
        Some("deep\n"),
    );
}

#[test]
fn test_parse_create_no_colon_after_keyword() {
    // Pattern: ## Create [path] (Space separator, no colon)
    let md = "\n## Create src/simple.rs\n```\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/simple.rs",
        None,
        Some(""), // Empty block results in empty content (no newline added)
    );
}

#[test]
fn test_parse_path_with_spaces_in_flexible_header_valid() {
    // "path with spaces.txt" has 3 spaces. Heuristic limit is 5.
    // Should be accepted.
    let md = "\n## Create path with spaces.txt\n```\ncontent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].path, "path with spaces.txt");
}

#[test]
fn test_parse_path_with_spaces_in_flexible_header_backticks() {
    // Backticks explicitly define the path, so spaces inside don't matter for the heuristic
    // (though the heuristic check runs on the extracted path, 3 spaces is fine).
    let md = "\n## Create `path with spaces.txt`\n```\ncontent\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].path, "path with spaces.txt");
}

// --- Numbered List with Action Keyword Tests (New) ---

#[test]
fn test_parse_numbered_list_with_action_keyword_and_hashes() {
    // Pattern: [hashes] [int]. [text] [Action]: [path]
    // Matches the Hash Action regex because of the leading hashes.
    let md = "\n### 1. Setup Step Create: src/setup.rs\n```rust\nfn setup() {}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/setup.rs",
        None,
        Some("fn setup() {}\n"),
    );
}

#[test]
fn test_parse_numbered_list_with_update_keyword_and_hashes() {
    // Pattern: [hashes] [int]. [text] Update [path] (no colon)
    let md = "\n## 2. Config Update src/config.toml\n```toml\n[cfg]\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create, // Update maps to Create
        "src/config.toml",
        None,
        Some("[cfg]\n"),
    );
}

#[test]
fn test_parse_numbered_list_with_action_keyword_backticks() {
    // Pattern: [hashes] [int]. [text] Create: `path`
    let md = "\n### 3. Script Create: `scripts/run.sh`\n```bash\n#!/bin/bash\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "scripts/run.sh",
        None,
        Some("#!/bin/bash\n"),
    );
}
