//! # Strux Lib
//!
//! Core library for parsing markdown and processing file actions.

// Declare modules following the refactored structure
pub mod constants;
pub mod core_types;
pub mod errors;
pub mod parser;
pub mod processor;

// Re-export essential types/functions for easier use by the binary crate (main.rs)
// or potentially other consumers of this library.
pub use constants::*;
pub use core_types::{Action, ActionType, CreateStatus, DeleteStatus, Summary}; // Removed print_summary
pub use errors::{AppError, ParseError, ProcessError};
pub use parser::parse_markdown;
pub use processor::process_actions;

// Note: Specific functions within submodules (like process_create) are typically
// kept internal to the library (pub(crate) or private) unless intended for direct use.
// `parse_markdown` and `process_actions` are the main public entry points here.
