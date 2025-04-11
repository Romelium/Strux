//! CLI tests related to file deletion.

use crate::common::get_cmd;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
// Import setup_temp_dir from the top-level tests/common
use crate::common::setup_temp_dir;

#[test]
fn test_cli_basic_delete() {
    let temp_dir = setup_temp_dir();
    let md_path = temp_dir.child("input.md");
    md_path
        .write_str("\n**Deleted File: old_stuff.log**\n")
        .unwrap();
    let output_dir = temp_dir.child("generated");
    output_dir.create_dir_all().unwrap(); // Pre-create for delete test
    output_dir
        .child("old_stuff.log")
        .write_str("delete me")
        .unwrap();

    let mut cmd = get_cmd();
    cmd.arg(md_path.path()).arg("-o").arg(output_dir.path());

    cmd.assert().success().stdout(predicate::str::contains(
        "Files deleted:                      1",
    ));

    output_dir
        .child("old_stuff.log")
        .assert(predicate::path::missing());
}
