//! Parser Integration Tests Entry Point

// Declare the common module for parser tests (tests/parser/common.rs)
#[path = "parser/common.rs"]
mod common;

// Declare the specific test modules (tests/parser/*.rs)
#[path = "parser/append_prepend.rs"] // ADDED
mod append_prepend;
#[path = "parser/create_distant.rs"]
mod create_distant;
#[path = "parser/create_external.rs"]
mod create_external;
#[path = "parser/create_internal.rs"]
mod create_internal;
#[path = "parser/create_wrapped.rs"]
mod create_wrapped;
#[path = "parser/delete.rs"]
mod delete;
#[path = "parser/edge_cases.rs"]
mod edge_cases;
#[path = "parser/heuristics.rs"]
mod heuristics;
#[path = "parser/invalid_paths.rs"]
mod invalid_paths;
#[path = "parser/move_file.rs"]
mod move_file;
#[path = "parser/nested_content.rs"]
mod nested_content;
#[path = "parser/ordering.rs"]
mod ordering;

// Declare the top-level common module (tests/test_common.rs)
// This isn't strictly needed by parser tests currently, but good practice
// if shared helpers are added later.
#[path = "test_common.rs"]
#[allow(unused_imports)] // Allow unused if no top-level helpers are used yet
mod test_common;
