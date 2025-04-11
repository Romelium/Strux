//! Main markdown parsing logic orchestrator.

use crate::core_types::Action;
use crate::errors::ParseError;
use std::collections::HashSet; // Removed unused HashMap

// Declare submodules within the parser module
mod header_utils; // Added
mod helpers;
mod internal_comment; // Added
mod pass1;
mod pass2;
mod path_utils; // Added
mod regex; // Contains regex definitions

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
    let mut processed_code_block_ranges: HashSet<(usize, usize)> = HashSet::new();

    let (content_to_parse, parse_offset) = helpers::preprocess_markdown(markdown_content);

    if content_to_parse.is_empty() && parse_offset > 0 {
        // Only contained the ignored ```markdown block
        return Ok(Vec::new());
    }

    // --- Pass 1: Find Code Blocks and associate actions ---
    println!("Step 1: Locating code blocks and associating actions...");
    pass1::run_pass1(
        // Now calls the function in the pass1 module
        content_to_parse,
        parse_offset,
        &mut actions_with_pos,
        &mut processed_header_starts,
        &mut processed_code_block_ranges,
    )?;

    // --- Pass 2: Find standalone Delete headers and orphaned Create ---
    println!("\nStep 2: Locating standalone Delete headers and orphaned Create...");
    pass2::run_pass2(
        // Now calls the function in the pass2 module
        content_to_parse,
        parse_offset,
        &mut actions_with_pos,
        &processed_header_starts,     // Pass as immutable ref
        &processed_code_block_ranges, // Pass as immutable ref
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
    helpers::check_action_conflicts(&final_actions);

    println!("\nParsing complete. Found {} actions.", final_actions.len());
    Ok(final_actions)
}
