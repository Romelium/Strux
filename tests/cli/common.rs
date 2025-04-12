//! Common helpers for CLI tests.

use assert_cmd::prelude::*;
use std::process::Command;

// Helper to get the command for the binary crate
pub fn get_cmd() -> Command {
    Command::cargo_bin("strux").expect("Failed to find binary") // <-- UPDATED BINARY NAME
}
