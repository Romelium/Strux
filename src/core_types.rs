//! Defines core data structures like Action, Summary, and status enums.

// Removed unused ProcessError import
use std::path::Path; // Removed unused PathBuf

// --- Core Types ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
    Create,
    Delete,
    // Removed: Patch,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub action_type: ActionType,
    pub path: String, // Store as String initially, convert to PathBuf in processor
    pub content: Option<String>, // For Create
    // Removed: pub patch_content: Option<String>, // For Patch
    pub original_pos: usize, // Byte offset in original markdown content
}

#[derive(Debug, Default)]
pub struct Summary {
    pub created: u32,
    pub overwritten: u32,
    pub deleted: u32,
    // Removed: pub patched: u32,
    pub skipped_exists: u32,
    pub skipped_not_found: u32,
    pub skipped_isdir_delete: u32,
    pub skipped_other_type: u32,
    // Removed: pub failed_patch_target_missing: u32,
    // Removed: pub failed_patch_target_not_file: u32,
    // Removed: pub failed_patch_format: u32,
    // Removed: pub failed_patch_context: u32,
    // Removed: pub failed_patch_ambiguous: u32,
    pub failed_io: u32,
    pub failed_isdir_create_patch: u32, // Renamed to failed_isdir_create
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
// Removed: #[derive(Debug, PartialEq, Eq)] pub enum PatchStatus { Patched }

// --- Summary Printing ---
pub fn print_summary(summary: &Summary, resolved_base: &Path) {
    println!("{}", "-".repeat(40));
    println!("Processing Summary:");
    println!(
        "  Base Directory:                     {}",
        resolved_base.display()
    );
    println!("  Files created:                      {}", summary.created);
    println!(
        "  Files overwritten (--force):        {}",
        summary.overwritten
    );
    // Removed: println!("  Files patched successfully:         {}", summary.patched);
    println!("  Files deleted:                      {}", summary.deleted);
    println!("{}", "-".repeat(14) + " Skipped " + &"-".repeat(19));
    println!(
        "  Skipped (create, exists):           {}",
        summary.skipped_exists
    );
    println!(
        "  Skipped (delete, not found):        {}",
        summary.skipped_not_found
    );
    println!(
        "  Skipped (delete, is dir):           {}",
        summary.skipped_isdir_delete
    );
    println!(
        "  Skipped (delete, other type):       {}",
        summary.skipped_other_type
    );
    println!("{}", "-".repeat(12) + " Failed/Errors " + &"-".repeat(13));
    println!(
        "  Failed (unsafe/invalid path):       {}",
        summary.failed_unsafe
    );
    println!(
        "  Failed (create, target is dir):     {}",
        summary.failed_isdir_create_patch
    ); // Renamed field name
    println!(
        "  Failed (create, parent is file):    {}",
        summary.failed_parent_isdir
    );
    // Removed: println!("  Failed (patch, target missing):     {}", summary.failed_patch_target_missing);
    // Removed: println!("  Failed (patch, target not file):    {}", summary.failed_patch_target_not_file);
    // Removed: println!("  Failed (patch, bad format):         {}", summary.failed_patch_format);
    // Removed: println!("  Failed (patch, context not found):  {}", summary.failed_patch_context);
    // Removed: println!("  Failed (patch, ambiguous context):  {}", summary.failed_patch_ambiguous);
    println!(
        "  Failed (I/O or Path error):         {}",
        summary.failed_io
    );
    println!(
        "  Failed (other unexpected errors):   {}",
        summary.error_other
    );
    println!("{}", "-".repeat(40));
}
