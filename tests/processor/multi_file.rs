//! Tests for processing multiple actions (create, delete, overwrite) in one run.

use assert_fs::prelude::*;
use predicates::prelude::*;
// Use helpers from the top-level test_common module
use crate::test_common::{run_processor, setup_temp_dir, setup_temp_dir_with_files};
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

    assert_summary(&summary, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0); // 4 created
}

#[test]
fn test_process_create_and_delete_mixed() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("to_delete_1.log", "delete me"),
        ("data/to_delete_2.tmp", "more deletable data"),
    ]);

    let md = "\n## File: new_script.py\n```python\nprint(\"Hello\")\n```\n\n**Deleted File: to_delete_1.log**\n\n`config.toml`\n```toml\nenabled = true\n```\n\n## Deleted File: data/to_delete_2.tmp\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    // Check created files
    temp_dir.child("new_script.py").assert("print(\"Hello\")\n");
    temp_dir.child("config.toml").assert("enabled = true\n");

    // Check deleted files
    temp_dir
        .child("to_delete_1.log")
        .assert(predicate::path::missing());
    temp_dir
        .child("data/to_delete_2.tmp")
        .assert(predicate::path::missing());

    assert_summary(&summary, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0); // 2 created, 2 deleted
}

#[test]
fn test_process_create_and_overwrite_mixed_force() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("config.ini", "[old]\nvalue=1"),
        ("data/params.json", "{\"old\": true}"),
    ]);

    let md = "\n## File: main.go\n```go\npackage main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Go!\") }\n```\n\n**File: config.ini**\n```ini\n[new]\nvalue = 2\n```\n\n`data/params.json`\n```json\n{\n  \"new\": true,\n  \"overwritten\": true\n}\n```\n\n## File: another_new.txt\n```\nJust another file.\n```\n";

    // Run with overwrite = true
    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed");

    // Check new files
    temp_dir
        .child("main.go")
        .assert("package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Go!\") }\n");
    temp_dir
        .child("another_new.txt")
        .assert("Just another file.\n");

    // Check overwritten files
    temp_dir.child("config.ini").assert("[new]\nvalue = 2\n");
    temp_dir
        .child("data/params.json")
        .assert("{\n  \"new\": true,\n  \"overwritten\": true\n}\n");

    assert_summary(&summary, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0); // 2 created, 2 overwritten
}

#[test]
fn test_process_create_overwrite_delete_complex() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("app.cfg", "old_config"),      // To be overwritten
        ("logs/yesterday.log", "log1"), // To be deleted
    ]);

    let md = "\n## File: src/mod.rs\n```rust\n// Module file\n```\n\n**File: app.cfg**\n```\nnew_config\n```\n\n## Deleted File: logs/yesterday.log\n\n`tests/run.sh`\n```bash\n#!/bin/bash\necho \"Testing...\"\n```\n\n**Deleted File: non_existent.tmp**\n";

    // Run with overwrite = true
    let (summary, _) = run_processor(md, &temp_dir, true).expect("Processing failed");

    // Check created files
    temp_dir.child("src/mod.rs").assert("// Module file\n");
    temp_dir
        .child("tests/run.sh")
        .assert("#!/bin/bash\necho \"Testing...\"\n");

    // Check overwritten file
    temp_dir.child("app.cfg").assert("new_config\n");

    // Check deleted file
    temp_dir
        .child("logs/yesterday.log")
        .assert(predicate::path::missing());

    // Check non-existent file (still missing)
    temp_dir
        .child("non_existent.tmp")
        .assert(predicate::path::missing());

    assert_summary(&summary, 2, 1, 1, 0, 1, 0, 0, 0, 0, 0); // 2 created, 1 overwritten, 1 deleted, 1 skipped_not_found
}

#[test]
fn test_process_delete_multiple_files() {
    let temp_dir = setup_temp_dir_with_files(&[
        ("file1.txt", "1"),
        ("dir/file2.txt", "2"),
        ("dir/subdir/file3.txt", "3"),
        ("another.log", "4"),
        ("keep_me.txt", "5"), // This one is NOT deleted
    ]);

    let md = "\n## Deleted File: file1.txt\n**Deleted File: dir/file2.txt**\n## Deleted File: dir/subdir/file3.txt\n**Deleted File: another.log**\n\n## File: new_file.txt\n```\nThis should still be created.\n```\n";

    let (summary, _) = run_processor(md, &temp_dir, false).expect("Processing failed");

    // Check deleted files
    temp_dir
        .child("file1.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("dir/file2.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("dir/subdir/file3.txt")
        .assert(predicate::path::missing());
    temp_dir
        .child("another.log")
        .assert(predicate::path::missing());

    // Check file that was NOT deleted
    temp_dir.child("keep_me.txt").assert("5");

    // Check file that was created
    temp_dir
        .child("new_file.txt")
        .assert("This should still be created.\n");

    assert_summary(&summary, 1, 0, 4, 0, 0, 0, 0, 0, 0, 0); // 1 created, 4 deleted
}
