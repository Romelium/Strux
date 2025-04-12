//! Handles processing a single action item.

use crate::core_types::{Action, ActionType, Summary};
use crate::errors::ProcessError;
use crate::processor::{create, delete, safety, summary_updater};
use std::path::{Path, PathBuf};

/// Processes a single action item, handling path validation, safety, and dispatching.
/// Updates the summary based on the outcome.
#[allow(clippy::too_many_arguments)]
pub(crate) fn process_single_action(
    item: &Action,
    item_index: usize,
    total_actions: usize,
    resolved_base: &Path,
    overwrite: bool,
    summary: &mut Summary,
) {
    let action_type = &item.action_type;
    let relative_path_str = &item.path;
    println!(
        "\n[{}/{}] Action: {:?}, Path: '{}'",
        item_index + 1,
        total_actions,
        action_type,
        relative_path_str
    );

    // --- Validate Path Format (String-based check FIRST) ---
    // (Validation logic remains the same)
    if relative_path_str.contains("//") || relative_path_str.contains(r"\\") {
        eprintln!(
            "Error: Invalid path format (consecutive separators) for '{}'. Skipping.",
            relative_path_str
        );
        // Directly update summary for format errors before path conversion
        summary_updater::update_summary_error(
            summary,
            ProcessError::InvalidPathFormat {
                path: relative_path_str.to_string(),
            },
        );
        return;
    }
    if (relative_path_str.ends_with('/') || relative_path_str.ends_with('\\'))
        && relative_path_str.len() > 1
    {
        eprintln!(
            "Error: Invalid path format (trailing separator) for '{}'. Skipping.",
            relative_path_str
        );
        summary_updater::update_summary_error(
            summary,
            ProcessError::InvalidPathFormat {
                path: relative_path_str.to_string(),
            },
        );
        return;
    }
    if relative_path_str.trim().is_empty() {
        eprintln!("Error: Invalid path format (empty path string). Skipping.");
        summary_updater::update_summary_error(
            summary,
            ProcessError::InvalidPathFormat {
                path: relative_path_str.to_string(),
            },
        );
        return;
    }

    // --- Convert to PathBuf and check components (redundant but safe) ---
    let relative_path = PathBuf::from(relative_path_str);
    if relative_path
        .components()
        .any(|comp| comp.as_os_str().is_empty())
    {
        eprintln!("Error: Invalid path format (empty components detected after PathBuf conversion) for '{}'. Skipping.", relative_path_str);
        summary_updater::update_summary_error(
            summary,
            ProcessError::InvalidPathFormat {
                path: relative_path_str.to_string(),
            },
        );
        return;
    }
    let potential_full_path = resolved_base.join(&relative_path);

    // --- Safety Check ---
    if let Err(e) = safety::ensure_path_safe(resolved_base, &potential_full_path) {
        eprintln!("Error processing action for '{}': {}", relative_path_str, e);
        // *** Call the summary updater function here ***
        summary_updater::update_summary_error(summary, e);
        return; // Exit before dispatching
    }

    // --- Dispatch to Action Handler ---
    let result: Result<(), ProcessError> = match action_type {
        ActionType::Create => create::process_create(
            item,
            &potential_full_path,
            relative_path_str,
            resolved_base,
            overwrite,
        )
        .map(|status| summary_updater::update_summary_create(summary, status)),
        ActionType::Delete => delete::process_delete(&potential_full_path, relative_path_str)
            .map(|status| summary_updater::update_summary_delete(summary, status)),
    };

    // --- Handle Errors from Action Handlers ---
    if let Err(e) = result {
        // Debug log can be removed now if desired
        // eprintln!("[DEBUG] Action handler returned error variant: {:?}", e);
        eprintln!("Error processing action for '{}': {}", relative_path_str, e);
        // *** Call the summary updater function here too ***
        summary_updater::update_summary_error(summary, e);
    }
}
