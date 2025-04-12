//! Defines core data structures like Action, Summary, and status enums.

// Removed unused ProcessError import
// Removed unused Path import

// --- Core Types ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    Create,
    Delete,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub action_type: ActionType,
    pub path: String, // Store as String initially, convert to PathBuf in processor
    pub content: Option<String>, // For Create
    pub original_pos: usize, // Byte offset in original markdown content
}

#[derive(Debug, Default)]
pub struct Summary {
    pub created: u32,
    pub overwritten: u32,
    pub deleted: u32,
    pub skipped_exists: u32,
    pub skipped_not_found: u32,
    pub skipped_isdir_delete: u32,
    pub skipped_other_type: u32,
    pub failed_io: u32,
    pub failed_isdir_create: u32,
    pub failed_parent_isdir: u32,
    pub failed_unsafe: u32,
    pub error_other: u32,
}

// --- Status Enums ---
// Used by processor helpers to signal outcomes for summary counting.
#[derive(Debug, PartialEq, Eq)]
pub enum CreateStatus {
    Created,
    Overwritten,
    SkippedExists,
}
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteStatus {
    Deleted,
    SkippedNotFound,
    SkippedIsDir,
    SkippedOtherType,
}
