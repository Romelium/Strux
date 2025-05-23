//! CLI Integration Tests Entry Point

// Declare the common module for CLI tests (tests/cli/common.rs)
#[path = "cli/common.rs"]
mod common;

// Declare the specific test modules (tests/cli/*.rs)
#[path = "cli/append_prepend.rs"] // ADDED
mod append_prepend;
#[path = "cli/basic.rs"]
mod basic;
#[path = "cli/create.rs"]
mod create;
#[path = "cli/delete.rs"]
mod delete;
#[path = "cli/empty_input.rs"]
mod empty_input;
#[path = "cli/errors.rs"]
mod errors;
#[path = "cli/move_file.rs"]
mod move_file;
#[path = "cli/overwrite_skip.rs"]
mod overwrite_skip;

// Declare the top-level common module (tests/test_common.rs)
// This makes helpers like setup_temp_dir available via crate::test_common::*
#[path = "test_common.rs"]
mod test_common;
