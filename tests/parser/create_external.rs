//! Tests for parsing external 'Create' headers (## File:, **File:, `path`, etc.).

use super::common::*; // Use helper from common.rs
use markdown_processor::core_types::ActionType;
use markdown_processor::parse_markdown;

#[test]
fn test_parse_bold_file_header() {
    let md = "\n**File: src/hello.txt**\n```\nHello, World!\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/hello.txt",
        Some("Hello, World!\n"),
    );
}

#[test]
fn test_parse_hash_file_header() {
    let md = "\n## File: config/settings.yaml\n```yaml\nsetting: value\nanother: 123\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "config/settings.yaml",
        Some("setting: value\nanother: 123\n"),
    );
}

#[test]
fn test_parse_backtick_path_header() {
    let md = "\n`my/script.sh`\n```bash\n#!/bin/bash\necho \"Running...\"\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "my/script.sh",
        Some("#!/bin/bash\necho \"Running...\"\n"),
    );
}

#[test]
fn test_parse_numbered_backtick_path_header() {
    let md = "\n1. `path/to/data.json`\n```json\n{ \"key\": \"value\" }\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "path/to/data.json",
        Some("{ \"key\": \"value\" }\n"),
    );
}

#[test]
fn test_parse_bold_backtick_path_header() {
    let md = "\n**`relative/path.md`**\n```markdown\n# Content\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "relative/path.md",
        Some("# Content\n"),
    );
}

#[test]
fn test_parse_hash_backtick_path_header() {
    let md = "\n## `another/file.ext`\n```\nSome raw content.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "another/file.ext",
        Some("Some raw content.\n"),
    );
}
