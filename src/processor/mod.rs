//! Orchestrates the processing of parsed file actions (create, delete).

use crate::core_types::{Action, Summary};
use crate::errors::AppError;
// Removed unused fs import
use std::path::Path;

// Declare processor submodules
mod action_handler;
mod append; // ADDED
mod base_setup;
mod create;
mod delete;
mod move_file;
mod prepend; // ADDED
mod safety;
mod summary_updater;

// Declare the unit test module for safety
#[cfg(test)]
mod safety_tests;
#[cfg(test)] // Also declare the existing summary_updater_tests module here
mod summary_updater_tests;

/// Processes a list of actions against the filesystem relative to a base directory.
pub fn process_actions(
    base_dir: &Path,
    actions: Vec<Action>,
    overwrite: bool,
) -> Result<Summary, AppError> {
    let mut summary = Summary::default();

    // --- Ensure base directory exists FIRST ---
    // Use the user-provided path for setup.
    println!(
        "Ensuring target base directory exists: {}",
        base_dir.display()
    );
    base_setup::setup_base_directory(base_dir)?; // Use new module

    // --- Resolve base directory path AFTER ensuring it exists ---
    // This is needed for safety checks.
    let resolved_base = match base_dir.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            // If canonicalize fails even after setup_base_directory, it's a more serious issue.
            eprintln!(
                "Error: Could not resolve base directory path '{}' after ensuring existence: {}",
                base_dir.display(),
                e
            );
            return Err(AppError::Io(e)); // Treat as fatal setup error
        }
    };
    println!(
        "Processing actions relative to resolved base: {}",
        resolved_base.display()
    );

    println!(
        "\nProcessing {} actions in '{}'...",
        actions.len(),
        resolved_base.display()
    ); // Use resolved for consistency

    for (item_index, item) in actions.iter().enumerate() {
        // Delegate processing of a single action
        action_handler::process_single_action(
            item,
            item_index,
            actions.len(),
            &resolved_base, // Pass the canonicalized path for safety checks
            overwrite,
            &mut summary,
        );
    }

    Ok(summary)
}

// --- Moved to base_setup.rs ---
// setup_base_directory

// --- Moved to action_handler.rs ---
// process_single_action

// --- Moved to summary_updater.rs ---
// update_summary_create
// update_summary_delete
// update_summary_error
