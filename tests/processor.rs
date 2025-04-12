//! Processor Integration Tests Entry Point

// Declare the common module for processor tests (tests/processor/common.rs)
#[path = "processor/common.rs"]
mod common;

// Declare the specific test modules (tests/processor/*.rs)
#[path = "processor/base_dir.rs"]
mod base_dir;
#[path = "processor/create.rs"]
mod create;
#[path = "processor/delete.rs"]
mod delete;
#[path = "processor/errors.rs"]
mod errors;
#[path = "processor/interactions.rs"]
mod interactions;
#[path = "processor/mixed_actions.rs"]
mod mixed_actions;
#[path = "processor/multi_create.rs"]
mod multi_create;
#[path = "processor/multi_delete.rs"]
mod multi_delete;
#[path = "processor/overwrite_skip.rs"]
mod overwrite_skip;

// Declare the top-level common module (tests/test_common.rs)
// This makes helpers like setup_temp_dir available via crate::test_common::*
#[path = "test_common.rs"]
mod test_common;
