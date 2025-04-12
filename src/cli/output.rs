//! Handles printing output like the summary.
use markdown_processor::Summary; // Use library's Summary
use std::path::Path;

/// Prints the final processing summary to the console.
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
        summary.failed_isdir_create
    );
    println!(
        "  Failed (create, parent is file):    {}",
        summary.failed_parent_isdir
    );
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
