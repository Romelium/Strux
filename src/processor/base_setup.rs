//! Contains logic for setting up the base directory for processing.

use crate::errors::AppError;
use std::fs;
use std::path::Path;

/// Ensures the base directory exists and is a directory. Accepts the user-provided path.
pub(crate) fn setup_base_directory(base_dir_to_setup: &Path) -> Result<(), AppError> {
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
