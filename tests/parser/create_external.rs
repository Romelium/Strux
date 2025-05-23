//! Tests for parsing external 'Create' headers (## File:, **File:, `path`, etc.).

use super::common::*; // Use helper from common.rs
use strux::core_types::ActionType;
use strux::parse_markdown;

#[test]
fn test_parse_bold_file_header() {
    let md = "\n**File: src/hello.txt**\n```\nHello, World!\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/hello.txt",
        None, // No dest_path for Create
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
        None, // No dest_path for Create
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
        None, // No dest_path for Create
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
        None, // No dest_path for Create
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
        None, // No dest_path for Create
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
        None, // No dest_path for Create
        Some("Some raw content.\n"),
    );
}

#[test]
fn test_parse_hash_file_header_with_trailing_comment() {
    let md = "\n## File: config.cfg # Main config file\n```\nkey=value\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "config.cfg", // Trailing comment ignored
        None,         // No dest_path for Create
        Some("key=value\n"),
    );
}

#[test]
fn test_parse_bold_file_header_with_trailing_text_outside() {
    let md = "\n**File: data.json** (important data)\n```json\n{}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "data.json", // Trailing text ignored
        None,        // No dest_path for Create
        Some("{}\n"),
    );
}

#[test]
fn test_parse_backtick_path_header_with_trailing_text() {
    let md = "\n`script.pl` # Perl script\n```perl\n#!/usr/bin/perl\nprint \"Hi\";\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "script.pl", // Trailing text ignored
        None,        // No dest_path for Create
        Some("#!/usr/bin/perl\nprint \"Hi\";\n"),
    );
}
