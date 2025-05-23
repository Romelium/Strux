//! Defines core data structures like Action, Summary, and status enums.

// Removed unused ProcessError import
// Removed unused Path import

// --- Core Types ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    Create,
    Delete,
    Move, // New action type
}

#[derive(Debug, Clone)]
pub struct Action {
    pub action_type: ActionType,
    pub path: String, // Source path for Move, target path for Create/Delete
    pub dest_path: Option<String>, // Destination path for Move
    pub content: Option<String>, // Content for Create
    pub original_pos: usize, // Byte offset in original markdown content
}

#[derive(Debug, Default)]
pub struct Summary {
    pub created: u32,
    pub overwritten: u32,
    pub deleted: u32,
    pub moved: u32,             // New summary field
    pub moved_overwritten: u32, // New summary field
    pub skipped_exists: u32,
    pub skipped_not_found: u32,
    pub skipped_isdir_delete: u32,
    pub skipped_other_type: u32,
    pub skipped_move_src_not_found: u32, // New summary field
    pub skipped_move_src_is_dir: u32,    // New summary field
    pub skipped_move_dst_exists: u32,    // New summary field
    pub skipped_move_dst_isdir: u32,     // New summary field
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

#[derive(Debug, PartialEq, Eq)]
pub enum MoveStatus {
    Moved,
    MovedOverwritten, // When --force is used and destination file is overwritten
    SkippedSourceNotFound,
    SkippedSourceIsDir,
    SkippedDestinationExists, // When destination file exists and --force is not used
    SkippedDestinationIsDir,  // When destination path is an existing directory
}
