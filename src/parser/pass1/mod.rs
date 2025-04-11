//! Logic for Pass 1 of markdown parsing: Associating code blocks with headers.

use crate::core_types::Action;
use crate::errors::ParseError;
use crate::parser::regex::OPENING_FENCE_REGEX;
use std::collections::HashSet;

// Import necessary items from submodules and parent modules
use crate::core_types::ActionType;
use crate::parser::header_utils; // For header extraction/validation
use crate::parser::helpers::ensure_trailing_newline;
use crate::parser::path_utils::validate_path_format;
use crate::parser::regex::HEADER_REGEX; // To check lines inside md blocks

// Declare submodules for Pass 1
mod external_header;
mod internal_header;
mod utils;

/// Executes Pass 1: Find code blocks and associate Create/Delete(special) actions.
#[allow(clippy::too_many_arguments)] // Necessary complexity for state passing
pub(crate) fn run_pass1(
    content_to_parse: &str,
    parse_offset: usize,
    actions_with_pos: &mut Vec<(usize, Action)>,
    processed_header_starts: &mut HashSet<usize>,
    processed_code_block_ranges: &mut HashSet<(usize, usize)>,
) -> Result<(), ParseError> {
    // Use captures_iter to get named groups
    for caps in OPENING_FENCE_REGEX.captures_iter(content_to_parse) {
        // Get the full match for positional info
        let full_match = caps.get(0).unwrap();
        let fence_start_pos = full_match.start(); // Relative to content_to_parse

        if utils::is_already_processed(fence_start_pos, processed_code_block_ranges) {
            // This block was already processed as the content part of a wrapped header
            println!(
                "    Skipping fence at {} as it's within an already processed range.",
                fence_start_pos + parse_offset
            );
            continue;
        }

        let fence_end_pos = full_match.end();
        // Get the captured fence characters using the name
        let fence_chars = caps.name("fence").unwrap().as_str();
        let lang = caps.name("lang").map(|m| m.as_str().trim()).unwrap_or(""); // Get lang tag

        let closing_match_opt =
            utils::find_closing_fence(content_to_parse, fence_chars, fence_end_pos);

        if closing_match_opt.is_none() {
            let original_pos = fence_start_pos + parse_offset;
            eprintln!(
                "Warning: Opening fence '{}' at original pos {} has no closing fence. Skipping.",
                fence_chars, original_pos
            );
            continue; // Skip this block entirely
        }
        let closing_match = closing_match_opt.unwrap();

        let block_content_start = fence_end_pos;
        let block_content_end = closing_match.start();
        let block_outer_end = closing_match.end();
        let original_block_start = fence_start_pos + parse_offset;

        println!(
            "  - Found code block: '{}' (lang: '{}') from original pos {} to {}",
            fence_chars,
            if lang.is_empty() { "none" } else { lang },
            original_block_start,
            block_outer_end + parse_offset
        );

        let mut current_action: Option<Action> = None;
        let mut header_line_start_pos_rel: Option<usize> = None;
        let mut action_source = "unknown"; // For debugging/logging

        // --- Check for External Header FIRST ---
        // This applies to *any* block, including ```markdown
        if let Some((action, header_pos)) = external_header::handle_external_header(
            content_to_parse,
            fence_start_pos,
            block_content_start,
            block_content_end,
            parse_offset,
        )? {
            current_action = Some(action);
            header_line_start_pos_rel = Some(header_pos);
            action_source = "external";
            // Mark header as processed (done below if action is Some)
        }

        // --- Check for Wrapped Header (only if lang is markdown/md and no external header found) ---
        if current_action.is_none() && (lang == "markdown" || lang == "md") {
            let md_block_content = &content_to_parse[block_content_start..block_content_end];
            let trimmed_lines: Vec<&str> = md_block_content
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .collect();

            if trimmed_lines.len() == 1 {
                let potential_header_line = trimmed_lines[0];
                println!(
                    "    Checking single line inside markdown block: '{}'",
                    potential_header_line
                );
                if let Some(header_caps) = HEADER_REGEX.captures(potential_header_line) {
                    if let Some((action_word, path)) =
                        header_utils::extract_action_path_from_captures(&header_caps)
                    {
                        if validate_path_format(&path).is_err() {
                            eprintln!(
                                "Warning: Invalid path format in wrapped header '{}'. Skipping.",
                                potential_header_line
                            );
                            // Mark md block as processed even if skipped due to error
                            processed_code_block_ranges.insert((fence_start_pos, block_outer_end));
                            continue; // Skip to next fence in outer loop
                        }

                        if let Some(action_type) = header_utils::get_action_type(&action_word) {
                            match action_type {
                                ActionType::Create => {
                                    // Look for the *next* code block immediately after this one
                                    if let Some(next_fence_caps) = utils::find_next_opening_fence(
                                        content_to_parse,
                                        block_outer_end, // Start search after *this* md block
                                        processed_code_block_ranges,
                                    ) {
                                        let next_fence_match = next_fence_caps.get(0).unwrap();
                                        // Check if it's immediately adjacent (allow whitespace lines between)
                                        let gap = &content_to_parse
                                            [block_outer_end..next_fence_match.start()];
                                        if gap.trim().is_empty() {
                                            println!("    Found wrapped header '{}' associated with the following code block.", potential_header_line);

                                            // Now process the *next* block using the header info we just extracted
                                            let next_fence_start = next_fence_match.start();
                                            let next_fence_end = next_fence_match.end();
                                            let next_fence_chars =
                                                next_fence_caps.name("fence").unwrap().as_str();

                                            if let Some(next_closing_match) =
                                                utils::find_closing_fence(
                                                    content_to_parse,
                                                    next_fence_chars,
                                                    next_fence_end,
                                                )
                                            {
                                                let next_content_start = next_fence_end;
                                                let next_content_end = next_closing_match.start();
                                                let next_outer_end = next_closing_match.end();

                                                let mut block_data = content_to_parse
                                                    [next_content_start..next_content_end]
                                                    .to_string();
                                                ensure_trailing_newline(&mut block_data);

                                                // Create the action using the wrapped header info
                                                let action = Action {
                                                    action_type: ActionType::Create,
                                                    path: path.clone(), // Clone path here
                                                    content: Some(block_data),
                                                    // Use original_pos of the markdown header block
                                                    original_pos: original_block_start,
                                                };
                                                current_action = Some(action);
                                                // Use the start pos of the markdown header block
                                                header_line_start_pos_rel = Some(fence_start_pos);
                                                action_source = "wrapped_create";

                                                // Mark *both* blocks as processed
                                                processed_code_block_ranges
                                                    .insert((fence_start_pos, block_outer_end));
                                                processed_code_block_ranges
                                                    .insert((next_fence_start, next_outer_end));

                                                println!(
                                                    "     -> Added CREATE action for '{}' from wrapped header.",
                                                    path // Use original path for println
                                                );
                                            } else {
                                                eprintln!("Warning: Found wrapped Create header '{}' but the following code block is unclosed. Skipping.", potential_header_line);
                                                processed_code_block_ranges
                                                    .insert((fence_start_pos, block_outer_end));
                                                // Mark md block
                                            }
                                        } else {
                                            eprintln!("Warning: Found wrapped Create header '{}' but it's not immediately followed by a code block (gap='{}'). Skipping.", potential_header_line, gap.escape_debug());
                                            processed_code_block_ranges
                                                .insert((fence_start_pos, block_outer_end));
                                            // Mark md block
                                        }
                                    } else {
                                        eprintln!("Warning: Found wrapped Create header '{}' but no subsequent code block found. Skipping.", potential_header_line);
                                        processed_code_block_ranges
                                            .insert((fence_start_pos, block_outer_end));
                                        // Mark md block
                                    }
                                }
                                ActionType::Delete => {
                                    println!(
                                        "    Found wrapped standalone DELETE action for: '{}'",
                                        path
                                    );
                                    let action = Action {
                                        action_type: ActionType::Delete,
                                        path, // Path is moved here
                                        content: None,
                                        original_pos: original_block_start,
                                    };
                                    current_action = Some(action);
                                    header_line_start_pos_rel = Some(fence_start_pos);
                                    action_source = "wrapped_delete";
                                    // Mark md block as processed
                                    processed_code_block_ranges
                                        .insert((fence_start_pos, block_outer_end));
                                }
                            }
                        } else {
                            // Action word was invalid, treat as normal markdown block
                            println!("    Single line in markdown block ('{}') did not contain a valid action word.", potential_header_line);
                        }
                    } else {
                        // Path extraction failed (e.g., empty path), treat as normal markdown block
                        println!(
                            "    Single line in markdown block ('{}') did not yield a valid path.",
                            potential_header_line
                        );
                    }
                } else {
                    // Line didn't match header regex, treat as normal markdown block
                    println!(
                        "    Single line in markdown block ('{}') did not match HEADER_REGEX.",
                        potential_header_line
                    );
                }
            } else {
                // Markdown block didn't contain exactly one non-empty line
                println!(
                    "    Markdown block does not contain exactly one non-empty line (found {}). Treating as regular content.",
                    trimmed_lines.len()
                );
            }

            // If this markdown block was handled (action added or skipped due to error/logic),
            // or if it was just a normal md block that wasn't part of a wrapped pair,
            // mark its range and continue to the next fence.
            if current_action.is_none()
                && !processed_code_block_ranges.contains(&(fence_start_pos, block_outer_end))
            {
                processed_code_block_ranges.insert((fence_start_pos, block_outer_end));
            }
            // If an action *was* created (either wrapped_create or wrapped_delete),
            // the necessary blocks are already marked. We proceed to add the action below.
        }

        // --- Check for Internal Headers (if no external or wrapped action found AND lang is not markdown) ---
        if current_action.is_none() && !(lang == "markdown" || lang == "md") {
            // Pass processed_header_starts to internal handler
            if let Some((action, header_pos)) = internal_header::handle_internal_header(
                content_to_parse,
                block_content_start,
                block_content_end,
                parse_offset,
                processed_header_starts, // Pass the mutable set here
            )? {
                current_action = Some(action);
                header_line_start_pos_rel = Some(header_pos);
                action_source = "internal";
                // Header processing (marking) is done inside handle_internal_header now
            }
            // Note: handle_internal_header now updates processed_header_starts directly
            // even if it returns Ok(None) for an ignored internal delete.
        }

        // --- Add action if found (from any source: external, wrapped, internal) ---
        if let (Some(mut action), Some(header_start_rel)) =
            (current_action, header_line_start_pos_rel)
        {
            let original_pos = header_start_rel + parse_offset;
            // Ensure original_pos wasn't already set by wrapped header logic
            if action.original_pos == 0 {
                action.original_pos = original_pos;
            }
            println!(
                "    -> Adding action from source '{}' with original_pos {}",
                action_source, action.original_pos
            );
            actions_with_pos.push((action.original_pos, action)); // Use final original_pos for sorting
            processed_header_starts.insert(original_pos); // Mark header associated with action
        } else if action_source == "unknown" {
            // Only log skip if no action was attempted
            // Check if the block was skipped because of an *ignored* internal header
            // (which would have been marked in processed_header_starts by handle_internal_header)
            let first_line_start_original = block_content_start + parse_offset;
            if !processed_header_starts.contains(&first_line_start_original) {
                println!(
                    "    Code block at original pos {} has no associated action header (checked external, wrapped, internal). Skipping.",
                    original_block_start
                );
            }
        }

        // Always record the block range if we successfully found opening and closing fences,
        // unless it was already added by the wrapped header logic pairing it with *this* block.
        if !processed_code_block_ranges.contains(&(fence_start_pos, block_outer_end)) {
            processed_code_block_ranges.insert((fence_start_pos, block_outer_end));
        }
    } // End of loop over fences
    Ok(())
}
