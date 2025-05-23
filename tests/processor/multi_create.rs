//! Tests for processing multiple create actions in one run.

use assert_fs::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_create_multiple_files() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: file1.txt\n```\nContent 1\n```\n\n`src/main.rs`\n```rust\nfn main() {}\n```\n\n**File: config/settings.yaml**\n```yaml\nkey: value\n```\n\n## File: docs/README.md\n```markdown\n# Project Docs\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir.child("file1.txt").assert("Content 1\n");
    temp_dir.child("src/main.rs").assert("fn main() {}\n");
    temp_dir
        .child("config/settings.yaml")
        .assert("key: value\n");
    temp_dir.child("docs/README.md").assert("# Project Docs\n");

    assert_summary(&summary, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

// Other tests moved to multi_delete.rs and mixed_actions.rs
