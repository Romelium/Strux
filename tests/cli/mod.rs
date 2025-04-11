//! CLI Integration Test Modules

// Declare the common module first if it contains helpers used by others
mod common;

// Declare the test modules
mod basic;
mod create;
mod delete;
mod errors;
mod overwrite_skip;
// Note: default_output tests were merged into basic.rs
