//! Tests for parser heuristics (avoiding false positives).

// Use helper from common.rs
// use strux::core_types::ActionType; // Not needed here
use strux::parse_markdown;

#[test]
fn test_parse_internal_header_looks_like_comment() {
    let md = "\n```rust\n// ## File: commented_out.rs\nlet x = 1;\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Internal header matching comment heuristic should be ignored"
    );
}

#[test]
fn test_parse_internal_header_looks_like_string() {
    let md = "\n```javascript\nconst errorMsg = \"**File: config.json** not found\";\nconsole.log(errorMsg);\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Internal header matching string heuristic should be ignored"
    );
}

#[test]
fn test_parse_internal_header_looks_like_string_backticks() {
    let md = "\n```python\nquery = f\"\"\"**File: query.sql**\nSELECT * FROM users;\n\"\"\"\nprint(query)\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Internal header matching backtick string heuristic should be ignored"
    );
}

#[test]
fn test_parse_standalone_header_inside_code_block_ignored_by_pass2() {
    // This header is *not* on the first line, so Pass 1 ignores it.
    // Pass 2 should also ignore it because it's inside a processed code block range.
    let md = "\n```\nSome code here.\n**Deleted File: should_be_ignored.txt**\nMore code.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Standalone header inside code block should be ignored by Pass 2"
    );
}
