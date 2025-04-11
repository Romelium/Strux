//! Integration tests for the `parse_markdown` function.

use markdown_processor::core_types::{Action, ActionType};
use markdown_processor::parse_markdown;

// --- Test Helpers ---

fn assert_action(
    action: Option<&Action>,
    expected_type: ActionType,
    expected_path: &str,
    expected_content: Option<&str>,
) {
    let action = action.expect("Expected an action, but found None");
    assert_eq!(action.action_type, expected_type, "Action type mismatch");
    assert_eq!(action.path, expected_path, "Action path mismatch");
    assert_eq!(
        action.content.as_deref(),
        expected_content,
        "Action content mismatch"
    );
}

// --- Basic Create Tests ---

#[test]
fn test_parse_bold_file_header() {
    let md = "\nSome text before.\n\n**File: src/hello.txt**\n```\nHello, World!\n```\n\nSome text after.\n";
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

#[test]
fn test_parse_internal_comment_file_header_excluded() {
    let md = "\n```rust\n// File: src/lib.rs\nfn main() {\n    println!(\"Internal\");\n}\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "src/lib.rs",
        Some("fn main() {\n    println!(\"Internal\");\n}\n"), // Header line excluded
    );
}

#[test]
fn test_parse_internal_comment_path_header_included() {
    // Note: The parser looks for `//path`, so `# // path` won't match the *included* format.
    // Test the actual intended format:
    let md_correct = "\n```python\n//myapp/main.py\nimport sys\n\nprint(sys.argv)\n```\n";
    let actions = parse_markdown(md_correct).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "myapp/main.py",
        Some("//myapp/main.py\nimport sys\n\nprint(sys.argv)\n"), // Header line included
    );
}

#[test]
fn test_parse_internal_comment_backticks_path_excluded() {
    let md = "\n```\n// File: `path with spaces/file.txt`\nContent here.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "path with spaces/file.txt",
        Some("Content here.\n"), // Header line excluded
    );
}

// --- Basic Delete Tests ---

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
    // This is the special case where the path is *in* the block
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

// --- Ordering and Multiple Actions ---

#[test]
fn test_parse_multiple_actions_ordered() {
    let md = "\nFirst some setup.\n\n## File: setup.sh\n```bash\necho \"Setting up...\"\n```\n\nThen delete an old file.\n\n**Deleted File: old.log**\n\nFinally, create the main file.\n\n`src/main.rs`\n```rust\nfn main() {}\n```\n";
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

// --- Edge Cases and Invalid Formats ---

#[test]
fn test_parse_empty_input() {
    let md = "";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_no_actions() {
    let md = "\nJust some regular markdown text.\nMaybe a code block without a header:\n```\nlet x = 5;\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_unclosed_fence() {
    let md = "\n**File: incomplete.txt**\n```\nThis block never closes.\n";
    // Expect parsing to succeed but skip the action due to missing close fence
    // A warning should be logged (can't easily test stderr here)
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Action should be skipped for unclosed fence"
    );
}

#[test]
fn test_parse_header_without_block() {
    let md = "\n**File: orphan.txt**\n\nSome other text.\n";
    // Expect parsing to succeed but skip the action
    // A warning should be logged
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
    // Delete headers *inside* blocks are ignored by Pass 1 and not picked up by Pass 2
    let md =
        "\n```\n**Deleted File: inside.txt**\nThis content is associated with no action.\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
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
fn test_parse_ignore_markdown_wrapper() {
    let md = "\n```markdown\n**File: ignored.txt**\n```\nThis should not be parsed.\n```\n```\n";
    // With the improved preprocess_markdown, this should now be correctly ignored.
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

#[test]
fn test_parse_invalid_path_format_skipped() {
    // Pass 1 and Pass 2 check path format and skip if invalid
    let md_create = "\n**File: bad//path.txt**\n```\ncontent\n```\n";
    let actions_create = parse_markdown(md_create).expect("Parsing failed");
    assert!(
        actions_create.is_empty(),
        "Create action with invalid path format should be skipped"
    );

    let md_delete = "\n**Deleted File: another//bad/path**\n";
    let actions_delete = parse_markdown(md_delete).expect("Parsing failed");
    assert!(
        actions_delete.is_empty(),
        "Delete action with invalid path format should be skipped"
    );
}

// --- Heuristic / False Positive Tests ---

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
    // Note: Raw strings in Rust (r#""#) are useful here, but the request is to avoid them.
    // We need to escape the internal triple quotes.
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

// --- Nested Blocks and Line Ending Tests ---

#[test]
fn test_parse_nested_code_blocks_simple() {
    // Renamed from test_parse_nested_code_blocks for clarity
    let md = "\nSome introductory text.\n\n## File: src/nested_example.md\n```markdown\nThis is the outer block.\n\nIt contains inner blocks:\n\n```bash\necho \"Inner block 1\"\nls -l\n```\n\nSome text between inner blocks.\n\n```python\n# Inner block 2\nimport sys\nprint(sys.version)\n```\n\nOuter block continues...\n```\n\nMore text after the main block.\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);

    // Use a regular string literal for expected content as well
    let expected_content = "This is the outer block.\n\nIt contains inner blocks:\n\n```bash\necho \"Inner block 1\"\nls -l\n```\n\nSome text between inner blocks.\n\n```python\n# Inner block 2\nimport sys\nprint(sys.version)\n```\n\nOuter block continues...\n"; // Parser adds trailing newline

    assert_action(
        actions.first(),
        ActionType::Create,
        "src/nested_example.md",
        Some(expected_content),
    );
}

#[test]
fn test_parse_nested_blocks_from_readme_example() {
    // Test case based on the user's problematic input
    let md = "\n#\n\n## Development\n\n### Pre-commit Hooks\n\nThis project uses `pre-commit` to automatically run code quality checks and apply fixes (formatting, linting, tests) before each commit. This helps maintain code consistency and catch issues early.\n\n**Setup:**\n\n1. **Install pre-commit:**\n    If you don't have it, install it using pip:\n\n    ```bash\n    pip install pre-commit\n    # Or using brew on macOS:\n    # brew install pre-commit\n    ```\n\n## File: Readme\n```markdown\n#\n\n## Development\n\n### Pre-commit Hooks\n\nThis project uses `pre-commit` to automatically run code quality checks and apply fixes (formatting, linting, tests) before each commit. This helps maintain code consistency and catch issues early.\n\n**Setup:**\n\n1. **Install pre-commit:**\n    If you don't have it, install it using pip:\n\n    ```bash\n    pip install pre-commit\n    # Or using brew on macOS:\n    # brew install pre-commit\n    ```\n\n2. **Install the git hooks:**\n    Run this command in the root directory of the repository:\n\n    ```bash\n    pre-commit install\n    ```\n\n**Usage:**\n\nOnce installed, `pre-commit` will run automatically when you run `git commit`.\n\n* **Automatic Fixes:** Hooks for `cargo fmt`, `cargo fix`, and `cargo clippy --fix` will attempt to automatically fix formatting issues, compiler suggestions, and simple lints.\n* **Commit Flow:**\n    1. You run `git commit`.\n    2. `pre-commit` runs the hooks.\n    3. If any fixing hook modifies files (e.g., applies formatting), the commit will be **aborted**.\n    4. You will see messages indicating which files were changed. **Review the changes** and use `git add <modified files>` to stage them.\n    5. Run `git commit` **again**.\n    6. This time, the fixing hooks should find nothing to change. The checking hooks (`clippy`'s check part, `cargo test`) will then run.\n    7. If all checks pass, the commit succeeds.\n* **Manual Fixes:** If `cargo clippy` or `cargo test` fail after the automatic fixing stage, you will need to manually fix the reported errors, `git add` your changes, and commit again.\n\n**Manual Run:**\n\nYou can run all checks and fixes manually on all files at any time:\n\n```bash\npre-commit run --all-files\n```\n\n**Skipping Checks (Use with Caution):**\n\nIf you need to bypass the pre-commit checks for a specific commit (e.g., work-in-progress), you can use the `--no-verify` flag:\n\n```bash\ngit commit --no-verify -m \"Your commit message\"\n```\n\n```\n"; // End of the markdown input string

    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1, "Expected exactly one action");

    // IMPORTANT: Define the expected content precisely. It's everything INSIDE the outer ```markdown block.
    // The parser should add a trailing newline if the content doesn't end with one before the closing fence.
    let expected_content = "#\n\n## Development\n\n### Pre-commit Hooks\n\nThis project uses `pre-commit` to automatically run code quality checks and apply fixes (formatting, linting, tests) before each commit. This helps maintain code consistency and catch issues early.\n\n**Setup:**\n\n1. **Install pre-commit:**\n    If you don't have it, install it using pip:\n\n    ```bash\n    pip install pre-commit\n    # Or using brew on macOS:\n    # brew install pre-commit\n    ```\n\n2. **Install the git hooks:**\n    Run this command in the root directory of the repository:\n\n    ```bash\n    pre-commit install\n    ```\n\n**Usage:**\n\nOnce installed, `pre-commit` will run automatically when you run `git commit`.\n\n* **Automatic Fixes:** Hooks for `cargo fmt`, `cargo fix`, and `cargo clippy --fix` will attempt to automatically fix formatting issues, compiler suggestions, and simple lints.\n* **Commit Flow:**\n    1. You run `git commit`.\n    2. `pre-commit` runs the hooks.\n    3. If any fixing hook modifies files (e.g., applies formatting), the commit will be **aborted**.\n    4. You will see messages indicating which files were changed. **Review the changes** and use `git add <modified files>` to stage them.\n    5. Run `git commit` **again**.\n    6. This time, the fixing hooks should find nothing to change. The checking hooks (`clippy`'s check part, `cargo test`) will then run.\n    7. If all checks pass, the commit succeeds.\n* **Manual Fixes:** If `cargo clippy` or `cargo test` fail after the automatic fixing stage, you will need to manually fix the reported errors, `git add` your changes, and commit again.\n\n**Manual Run:**\n\nYou can run all checks and fixes manually on all files at any time:\n\n```bash\npre-commit run --all-files\n```\n\n**Skipping Checks (Use with Caution):**\n\nIf you need to bypass the pre-commit checks for a specific commit (e.g., work-in-progress), you can use the `--no-verify` flag:\n\n```bash\ngit commit --no-verify -m \"Your commit message\"\n```\n"; // Note: Parser adds trailing \n

    assert_action(
        actions.first(),
        ActionType::Create,
        "Readme", // Path extracted from "## File: Readme"
        Some(expected_content),
    );
}

// --- Content Variations ---

#[test]
fn test_parse_empty_content() {
    let md = "\n## File: empty.txt\n```\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    // Empty block -> empty content
    assert_action(actions.first(), ActionType::Create, "empty.txt", Some(""));
}

#[test]
fn test_parse_content_no_trailing_newline() {
    // The parser adds a trailing newline if missing
    let md = "\n**File: data.csv**\n```\ncol1,col2\nval1,val2\n```"; // Note: No newline after ```
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "data.csv",
        Some("col1,col2\nval1,val2\n"), // Newline added
    );
}
