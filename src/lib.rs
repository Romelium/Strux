//! # Markdown Processor Lib
//!
//! Core library for parsing markdown and processing file actions.

// Declare modules following the refactored structure
pub mod constants;
pub mod core_types;
pub mod errors;
pub mod parser;
// Removed: pub mod patch;
pub mod processor;

// Re-export essential types/functions for easier use by the binary crate (main.rs)
// or potentially other consumers of this library.
pub use constants::*;
pub use core_types::{
    print_summary, Action, ActionType, CreateStatus, DeleteStatus,
    /* Removed PatchStatus */ Summary,
};
pub use errors::{AppError, ParseError, ProcessError /* Removed PatchError */};
pub use parser::parse_markdown;
pub use processor::process_actions;

// Note: Specific functions within submodules (like process_create) are typically
// kept internal to the library (pub(crate) or private) unless intended for direct use.
// `parse_markdown` and `process_actions` are the main public entry points here.
