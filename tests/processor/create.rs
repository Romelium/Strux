//! Tests for basic file creation logic in the processor.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir};
// Use the helper from this module's common
use super::common::*;

#[test]
fn test_process_create_file() {
    let temp_dir = setup_temp_dir();
    let md = "\n## File: new_file.txt\n```\nThis is a new file.\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir
        .child("new_file.txt")
        .assert(predicate::path::is_file());
    temp_dir
        .child("new_file.txt")
        .assert("This is a new file.\n");
    assert_summary(&summary, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

#[test]
fn test_process_create_with_parent_dir() {
    let temp_dir = setup_temp_dir();
    let md = "\n`src/app/main.rs`\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    temp_dir.child("src").assert(predicate::path::is_dir());
    temp_dir.child("src/app").assert(predicate::path::is_dir());
    temp_dir
        .child("src/app/main.rs")
        .assert(predicate::path::is_file());
    temp_dir
        .child("src/app/main.rs")
        .assert("fn main() {\n    println!(\"Hello\");\n}\n");
    assert_summary(&summary, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}
