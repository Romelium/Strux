//! Handles processing a single action item.

use crate::core_types::{Action, ActionType, Summary};
use crate::errors::ProcessError;
use crate::processor::{create, delete, move_file, safety, summary_updater}; // Added move_file
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
    let relative_path_str = &item.path; // This is source_path for Move

    // --- Path Validation and Setup ---
    // For Move, we need to validate and resolve both source and destination paths.
    // For Create/Delete, only item.path is relevant here.

    let (log_path_display, validated_primary_path, validated_secondary_path_opt) = match action_type
    {
        ActionType::Move => {
            let dest_path_str = item
                .dest_path
                .as_ref()
                .expect("Move action missing destination path");
            println!(
                "\n[{}/{}] Action: {:?}, From: '{}', To: '{}'",
                item_index + 1,
                total_actions,
                action_type,
                relative_path_str,
                dest_path_str
            );
            // Validate source path
            if let Err(e) = validate_and_prepare_path(relative_path_str, resolved_base) {
                summary_updater::update_summary_error(summary, e);
                return;
            }
            // Validate destination path
            if let Err(e) = validate_and_prepare_path(dest_path_str, resolved_base) {
                summary_updater::update_summary_error(summary, e);
                return;
            }
            (
                format!("{} -> {}", relative_path_str, dest_path_str),
                resolved_base.join(relative_path_str),
                Some(resolved_base.join(dest_path_str)),
            )
        }
        _ => {
            // Create or Delete
            println!(
                "\n[{}/{}] Action: {:?}, Path: '{}'",
                item_index + 1,
                total_actions,
                action_type,
                relative_path_str
            );
            if let Err(e) = validate_and_prepare_path(relative_path_str, resolved_base) {
                summary_updater::update_summary_error(summary, e);
                return;
            }
            (
                relative_path_str.to_string(),
                resolved_base.join(relative_path_str),
                None,
            )
        }
    };

    // --- Safety Check ---
    // Check primary path (source for Move, target for Create/Delete)
    if let Err(e) = safety::ensure_path_safe(resolved_base, &validated_primary_path) {
        eprintln!("Error processing action for '{}': {}", log_path_display, e);
        summary_updater::update_summary_error(summary, e);
        return;
    }
    // Check secondary path if it exists (destination for Move)
    if let Some(ref secondary_path) = validated_secondary_path_opt {
        if let Err(e) = safety::ensure_path_safe(resolved_base, secondary_path) {
            eprintln!("Error processing action for '{}': {}", log_path_display, e);
            summary_updater::update_summary_error(summary, e);
            return;
        }
    }

    // --- Dispatch to Action Handler ---
    let result: Result<(), ProcessError> = match action_type {
        ActionType::Create => create::process_create(
            item,
            &validated_primary_path, // This is the target path for create
            relative_path_str,       // Original relative path for logging
            resolved_base,
            overwrite,
        )
        .map(|status| summary_updater::update_summary_create(summary, status)),
        ActionType::Delete => {
            delete::process_delete(&validated_primary_path, relative_path_str) // Target path for delete
                .map(|status| summary_updater::update_summary_delete(summary, status))
        }
        ActionType::Move => {
            let dest_path_str = item
                .dest_path
                .as_ref()
                .expect("Move action missing dest_path string for logging");
            move_file::process_move(
                &validated_primary_path, // Source path for move
                validated_secondary_path_opt
                    .as_ref()
                    .expect("Move action missing resolved dest_path"), // Dest path for move
                relative_path_str,       // Original source relative path for logging
                dest_path_str,           // Original dest relative path for logging
                resolved_base,
                overwrite,
            )
            .map(|status| summary_updater::update_summary_move(summary, status))
        }
    };

    // --- Handle Errors from Action Handlers ---
    if let Err(e) = result {
        eprintln!("Error processing action for '{}': {}", log_path_display, e);
        summary_updater::update_summary_error(summary, e);
    }
}

/// Validates path format (string-based checks) and ensures no empty components after PathBuf conversion.
fn validate_and_prepare_path(
    relative_path_str: &str,
    _resolved_base: &Path, // Currently unused, but kept for signature consistency
) -> Result<(), ProcessError> {
    // String-based format checks
    if relative_path_str.contains("//") || relative_path_str.contains(r"\\") {
        eprintln!(
            "Error: Invalid path format (consecutive separators) for '{}'. Skipping.",
            relative_path_str
        );
        return Err(ProcessError::InvalidPathFormat {
            path: relative_path_str.to_string(),
        });
    }
    if (relative_path_str.ends_with('/') || relative_path_str.ends_with('\\'))
        && relative_path_str.len() > 1
    {
        eprintln!(
            "Error: Invalid path format (trailing separator) for '{}'. Skipping.",
            relative_path_str
        );
        return Err(ProcessError::InvalidPathFormat {
            path: relative_path_str.to_string(),
        });
    }
    if relative_path_str.trim().is_empty() {
        eprintln!("Error: Invalid path format (empty path string). Skipping.");
        return Err(ProcessError::InvalidPathFormat {
            path: relative_path_str.to_string(),
        });
    }

    // PathBuf component check
    let relative_path = PathBuf::from(relative_path_str);
    if relative_path
        .components()
        .any(|comp| comp.as_os_str().is_empty())
    {
        eprintln!("Error: Invalid path format (empty components detected after PathBuf conversion) for '{}'. Skipping.", relative_path_str);
        return Err(ProcessError::InvalidPathFormat {
            path: relative_path_str.to_string(),
        });
    }
    Ok(())
}
