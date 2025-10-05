//! Main markdown parsing logic orchestrator.

use crate::core_types::Action;
use crate::errors::ParseError;
use std::collections::HashSet; // Removed unused HashMap

// Declare submodules within the parser module
mod action_checker; // ADDED
mod header_utils;
mod helpers;
mod internal_comment;
mod pass1;
mod pass2; // Find unassociated content headers and link forward
mod pass3; // Find standalone Delete/Move headers
mod path_utils;
mod regex; // Contains regex definitions

// Declare the test modules for submodules
#[cfg(test)]
mod header_utils_tests; // ADDED
#[cfg(test)]
mod internal_comment_tests;

// Re-export the main parsing function
// Removed pub use of check_action_conflicts as it's crate-internal
pub use self::regex::{HEADER_REGEX, OPENING_FENCE_REGEX}; // Export regexes if needed by passes

/// Parses markdown content to extract file actions.
pub fn parse_markdown(markdown_content: &str) -> Result<Vec<Action>, ParseError> {
    // --- TEMPORARY DEBUG ---
    // Call the debug function to test isolated regex patterns
    // Make sure to run tests with --nocapture to see this output
    // Only run this once, perhaps check an env var or just run it during test builds
    // For simplicity here, we'll just call it.
    // Note: This will print during *every* parse_markdown call in tests.
    // crate::parser::regex::debug_hash_regexes(); // Uncomment to run debug prints
    // --- END TEMPORARY DEBUG ---

    let mut actions_with_pos: Vec<(usize, Action)> = Vec::new();
    let mut processed_header_starts: HashSet<usize> = HashSet::new();
    // Store (start, end) byte indices relative to content_to_parse
    let mut all_code_block_ranges: HashSet<(usize, usize)> = HashSet::new();
    let mut processed_code_block_ranges: HashSet<(usize, usize)> = HashSet::new();

    let (content_to_parse, parse_offset) = helpers::preprocess_markdown(markdown_content);

    if content_to_parse.is_empty() && parse_offset > 0 {
        // Only contained the ignored ```markdown block
        return Ok(Vec::new());
    }

    // --- Pass 1: Find Code Blocks and associate actions ---
    println!(
        "Step 1: Locating code blocks and associating with adjacent/internal/wrapped headers..."
    );
    pass1::run_pass1(
        // Now calls the function in the pass1 module
        content_to_parse,
        parse_offset,
        &mut actions_with_pos,
        &mut processed_header_starts,
        &mut all_code_block_ranges,
        &mut processed_code_block_ranges,
    )?;

    // --- Pass 2: Find unassociated content headers and link to next block ---
    println!(
        "\nStep 2: Locating unassociated content headers and linking to subsequent code blocks..."
    );
    pass2::run_pass2(
        content_to_parse,
        parse_offset,
        &mut actions_with_pos,
        &mut processed_header_starts,
        &mut processed_code_block_ranges,
    )?;

    // --- Pass 3: Find standalone Delete/Move headers ---
    println!("\nStep 3: Locating standalone Delete/Move headers...");
    pass3::run_pass3(
        content_to_parse,
        parse_offset,
        &mut actions_with_pos,
        &processed_header_starts, // Pass as immutable ref
        &all_code_block_ranges,   // Pass as immutable ref
    )?;

    // --- Sort actions by original position ---
    println!("\nSorting actions by document order...");
    actions_with_pos.sort_by_key(|&(pos, _)| pos);

    // Extract sorted action dictionaries
    let final_actions: Vec<Action> = actions_with_pos
        .into_iter()
        .map(|(_, action)| action)
        .collect();

    // --- Final check for conflicting actions on the same path ---
    action_checker::check_action_conflicts(&final_actions); // Use new module

    println!("\nParsing complete. Found {} actions.", final_actions.len());
    Ok(final_actions)
}
