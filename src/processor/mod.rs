//! Orchestrates the processing of parsed file actions (create, delete).

use crate::core_types::{Action, Summary};
use crate::errors::AppError;
use std::fs;
use std::path::Path;

// Declare processor submodules
mod action_handler;
mod create;
mod delete;
mod safety;
mod summary_updater;

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
    setup_base_directory(base_dir)?; // Pass the original path

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

/// Ensures the base directory exists and is a directory. Accepts the user-provided path.
fn setup_base_directory(base_dir_to_setup: &Path) -> Result<(), AppError> {
    if !base_dir_to_setup.exists() {
        fs::create_dir_all(base_dir_to_setup).map_err(|e| {
            eprintln!(
                "Error: Could not create base directory '{}': {}",
                base_dir_to_setup.display(),
                e
            );
            AppError::Io(e)
        })?;
        println!("Created base directory: {}", base_dir_to_setup.display());
    } else if !base_dir_to_setup.is_dir() {
        eprintln!(
            "Error: Specified base path '{}' exists but is not a directory.",
            base_dir_to_setup.display()
        );
        return Err(AppError::Argument(
            "Base path is not a directory".to_string(),
        ));
    }
    Ok(())
}

// --- Moved to action_handler.rs ---
// process_single_action

// --- Moved to summary_updater.rs ---
// update_summary_create
// update_summary_delete
// update_summary_error
