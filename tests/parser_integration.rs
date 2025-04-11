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
    let md = r#"
Some text before.

**File: src/hello.txt**
```
Hello, World!
```

Some text after.
"#;
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
    let md = r#"
## File: config/settings.yaml
```yaml
setting: value
another: 123
```
"#;
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
    let md = r#"
`my/script.sh`
```bash
#!/bin/bash
echo "Running..."
```
"#;
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
    let md = r#"
1. `path/to/data.json`
```json
{ "key": "value" }
```
"#;
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
    let md = r#"
**`relative/path.md`**
```markdown
# Content
```
"#;
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
    let md = r#"
## `another/file.ext`
```
Some raw content.
```
"#;
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
    let md = r#"
```rust
// File: src/lib.rs
fn main() {
    println!("Internal");
}
```
"#;
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
    let md_correct = r#"
```python
//myapp/main.py
import sys

print(sys.argv)
```
"#;
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
    let md = r#"
```
// File: `path with spaces/file.txt`
Content here.
```
"#;
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
    let md = r#"
**Deleted File: old_config.cfg**
"#;
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(actions.first(), ActionType::Delete, "old_config.cfg", None);
}

#[test]
fn test_parse_hash_deleted_file_header() {
    let md = r#"
## Deleted File: temp/file_to_remove.tmp
"#;
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
    let md = r#"
## Deleted File:
```
path/inside/block.log
```
"#;
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
    let md = r#"
First some setup.

## File: setup.sh
```bash
echo "Setting up..."
```

Then delete an old file.

**Deleted File: old.log**

Finally, create the main file.

`src/main.rs`
```rust
fn main() {}
```
"#;
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
    let md = r#"
Just some regular markdown text.
Maybe a code block without a header:
```
let x = 5;
```
"#;
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_unclosed_fence() {
    let md = r#"
**File: incomplete.txt**
```
This block never closes.
"#;
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
    let md = r#"
**File: orphan.txt**

Some other text.
"#;
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
    let md = r#"
```
No header here.
```
"#;
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_invalid_action_word() {
    let md = r#"
**Created: file.txt**
```
content
```
"#;
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty(), "Invalid action word should be ignored");
}

#[test]
fn test_parse_header_missing_path() {
    let md = r#"
## File:
```
content
```
"#;
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty(), "Header missing path should be ignored");
}

#[test]
fn test_parse_internal_delete_header_ignored() {
    // Delete headers *inside* blocks are ignored by Pass 1 and not picked up by Pass 2
    let md = r#"
```
**Deleted File: inside.txt**
This content is associated with no action.
```
"#;
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_special_delete_header_empty_block() {
    let md = r#"
## Deleted File:
```
```
"#;
    // Should parse, but log a warning and produce no action
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(actions.is_empty());
}

#[test]
fn test_parse_special_delete_header_multi_line_block() {
    let md = r#"
## Deleted File:
```
path/to/delete.txt
some other ignored line
```
"#;
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
    let md = r#"
```markdown
**File: ignored.txt**
```
This should not be parsed.
```
```
"#;
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
    let md_create = r#"
**File: bad//path.txt**
```
content
```
"#;
    let actions_create = parse_markdown(md_create).expect("Parsing failed");
    assert!(
        actions_create.is_empty(),
        "Create action with invalid path format should be skipped"
    );

    let md_delete = r#"
**Deleted File: another//bad/path**
"#;
    let actions_delete = parse_markdown(md_delete).expect("Parsing failed");
    assert!(
        actions_delete.is_empty(),
        "Delete action with invalid path format should be skipped"
    );
}

// --- Nested Blocks and Line Ending Tests ---

#[test]
fn test_parse_nested_code_blocks_simple() {
    // Renamed from test_parse_nested_code_blocks for clarity
    let md = r#"
Some introductory text.

## File: src/nested_example.md
```markdown
This is the outer block.

It contains inner blocks:

```bash
echo "Inner block 1"
ls -l
```

Some text between inner blocks.

```python
# Inner block 2
import sys
print(sys.version)
```

Outer block continues...
```

More text after the main block.
"#;
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);

    let expected_content = r#"This is the outer block.

It contains inner blocks:

```bash
echo "Inner block 1"
ls -l
```

Some text between inner blocks.

```python
# Inner block 2
import sys
print(sys.version)
```

Outer block continues...
"#; // Parser adds trailing newline

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
    let md = r#"
#

## Development

### Pre-commit Hooks

This project uses `pre-commit` to automatically run code quality checks and apply fixes (formatting, linting, tests) before each commit. This helps maintain code consistency and catch issues early.

**Setup:**

1. **Install pre-commit:**
    If you don't have it, install it using pip:

    ```bash
    pip install pre-commit
    # Or using brew on macOS:
    # brew install pre-commit
    ```

## File: Readme
```markdown
#

## Development

### Pre-commit Hooks

This project uses `pre-commit` to automatically run code quality checks and apply fixes (formatting, linting, tests) before each commit. This helps maintain code consistency and catch issues early.

**Setup:**

1. **Install pre-commit:**
    If you don't have it, install it using pip:

    ```bash
    pip install pre-commit
    # Or using brew on macOS:
    # brew install pre-commit
    ```

2. **Install the git hooks:**
    Run this command in the root directory of the repository:

    ```bash
    pre-commit install
    ```

**Usage:**

Once installed, `pre-commit` will run automatically when you run `git commit`.

* **Automatic Fixes:** Hooks for `cargo fmt`, `cargo fix`, and `cargo clippy --fix` will attempt to automatically fix formatting issues, compiler suggestions, and simple lints.
* **Commit Flow:**
    1. You run `git commit`.
    2. `pre-commit` runs the hooks.
    3. If any fixing hook modifies files (e.g., applies formatting), the commit will be **aborted**.
    4. You will see messages indicating which files were changed. **Review the changes** and use `git add <modified files>` to stage them.
    5. Run `git commit` **again**.
    6. This time, the fixing hooks should find nothing to change. The checking hooks (`clippy`'s check part, `cargo test`) will then run.
    7. If all checks pass, the commit succeeds.
* **Manual Fixes:** If `cargo clippy` or `cargo test` fail after the automatic fixing stage, you will need to manually fix the reported errors, `git add` your changes, and commit again.

**Manual Run:**

You can run all checks and fixes manually on all files at any time:

```bash
pre-commit run --all-files
```

**Skipping Checks (Use with Caution):**

If you need to bypass the pre-commit checks for a specific commit (e.g., work-in-progress), you can use the `--no-verify` flag:

```bash
git commit --no-verify -m "Your commit message"
```

```
"#; // End of the markdown input string

    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1, "Expected exactly one action");

    // IMPORTANT: Define the expected content precisely. It's everything INSIDE the outer ```markdown block.
    // The parser should add a trailing newline if the content doesn't end with one before the closing fence.
    let expected_content = r#"#

## Development

### Pre-commit Hooks

This project uses `pre-commit` to automatically run code quality checks and apply fixes (formatting, linting, tests) before each commit. This helps maintain code consistency and catch issues early.

**Setup:**

1. **Install pre-commit:**
    If you don't have it, install it using pip:

    ```bash
    pip install pre-commit
    # Or using brew on macOS:
    # brew install pre-commit
    ```

2. **Install the git hooks:**
    Run this command in the root directory of the repository:

    ```bash
    pre-commit install
    ```

**Usage:**

Once installed, `pre-commit` will run automatically when you run `git commit`.

* **Automatic Fixes:** Hooks for `cargo fmt`, `cargo fix`, and `cargo clippy --fix` will attempt to automatically fix formatting issues, compiler suggestions, and simple lints.
* **Commit Flow:**
    1. You run `git commit`.
    2. `pre-commit` runs the hooks.
    3. If any fixing hook modifies files (e.g., applies formatting), the commit will be **aborted**.
    4. You will see messages indicating which files were changed. **Review the changes** and use `git add <modified files>` to stage them.
    5. Run `git commit` **again**.
    6. This time, the fixing hooks should find nothing to change. The checking hooks (`clippy`'s check part, `cargo test`) will then run.
    7. If all checks pass, the commit succeeds.
* **Manual Fixes:** If `cargo clippy` or `cargo test` fail after the automatic fixing stage, you will need to manually fix the reported errors, `git add` your changes, and commit again.

**Manual Run:**

You can run all checks and fixes manually on all files at any time:

```bash
pre-commit run --all-files
```

**Skipping Checks (Use with Caution):**

If you need to bypass the pre-commit checks for a specific commit (e.g., work-in-progress), you can use the `--no-verify` flag:

```bash
git commit --no-verify -m "Your commit message"
```
"#; // Note: Parser adds trailing \n

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
    let md = r#"
## File: empty.txt
```
```
"#;
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    // Empty block -> empty content
    assert_action(actions.first(), ActionType::Create, "empty.txt", Some(""));
}

#[test]
fn test_parse_content_no_trailing_newline() {
    // The parser adds a trailing newline if missing
    let md = r#"
**File: data.csv**
```
col1,col2
val1,val2
```"#; // Note: No newline after ```
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Create,
        "data.csv",
        Some("col1,col2\nval1,val2\n"), // Newline added
    );
}
