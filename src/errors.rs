//! Defines custom error types for the application.

use std::io;
use std::path::PathBuf;
use thiserror::Error;

// --- Top-Level Error ---
// This aggregates errors from different stages (parsing, processing, I/O).
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Markdown parsing error: {0}")]
    Parse(#[from] ParseError),
    #[error("File processing error: {0}")]
    Process(#[from] ProcessError),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Argument error: {0}")]
    Argument(String),
}

// --- Parsing Errors ---
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Regex compilation/matching error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Found opening fence '{fence}' at byte {pos} but no closing fence")]
    NoClosingFence { fence: String, pos: usize },
    #[error("Invalid header format found near byte {pos}: {details}")]
    InvalidHeaderFormat { pos: usize, details: String },
    #[error("Internal logic error during parsing: {0}")]
    Internal(String),
    #[error("I/O error during parsing (unexpected): {0}")]
    Io(#[from] io::Error), // Should be rare for parsing string content
}

// --- Processing Errors ---
// Errors occurring during file system operations or validation within that stage.
// Removed PartialEq, Eq because io::Error doesn't support them.
#[derive(Error, Debug)]
pub enum ProcessError {
    // Note: Cannot derive PartialEq/Eq on the enum containing this variant
    //       because io::Error does not implement them.
    #[error("I/O error: {source}")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("Path resolution/canonicalization error for '{path}': {details}")]
    PathResolution { path: PathBuf, details: String },
    #[error("Unsafe path detected! Resolved path '{resolved_path}' is outside base '{base_path}'")]
    PathNotSafe {
        resolved_path: PathBuf,
        base_path: PathBuf,
    },
    #[error("Invalid path format detected for '{path}': Contains empty components (e.g., '//')")]
    InvalidPathFormat { path: String },
    #[error("Cannot perform operation. Target path '{path}' exists and is a directory.")]
    TargetIsDirectory { path: PathBuf },
    // #[error("Cannot perform operation. Target path '{path}' is not a file.")]
    // TargetIsNotFile { path: PathBuf },
    #[error(
        "Cannot create file '{path}'. Parent path '{parent_path}' exists but is not a directory."
    )]
    ParentIsNotDirectory { path: PathBuf, parent_path: PathBuf },
    #[error("Unknown action type encountered")]
    UnknownAction, // Should not happen if parsing is correct
    #[error("Unexpected internal error: {0}")]
    Internal(String),
}
