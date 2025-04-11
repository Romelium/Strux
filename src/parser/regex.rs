//! Defines and compiles regular expressions used by the parser.

use crate::constants::VALID_ACTIONS_REGEX_STR; // Use the pre-built string
use once_cell::sync::Lazy;
use regex::Regex;

// Use Lazy from once_cell for thread-safe static initialization of Regex objects.

// Regex to find file headers anchored to the start of a line.
// Simplified ## alternatives and removed verbose mode for robustness.
pub static HEADER_REGEX: Lazy<Regex> = Lazy::new(|| {
    let actions = &*VALID_ACTIONS_REGEX_STR; // Dereference Lazy<String>
                                             // println!("[REGEX INIT] Action words pattern part: '{}'", actions); // DEBUG

    // NOTE: Removed verbose mode (?x). Whitespace is now significant.
    // Order: Bold Action, Hash Backtick, Hash Action, Backtick Only, Numbered Backtick, Bold Backtick
    // ALL PARTS COMBINED INTO A SINGLE RAW STRING LITERAL for format!
    // Use [^\n] instead of . in capture groups to prevent matching across lines.
    // Added optional trailing text `(?:\s[^\n]*)?` to backtick_only and bold_backtick patterns.
    let pattern = format!(
        // Start of the single raw string literal containing the entire pattern
        r"(?m)^(?:\*\*\s*(?P<action_word_bold>{actions}):\s+(?P<content_bold>[^\n]+?)\s*\*\*(?:\s[^\n]*)?|##\s+`(?P<path_hash_backtick>[^`\n]+?)`\s*(?:\s[^\n]*)?|##\s+(?P<action_word_hash>{actions}):\s*(?P<content_hash>[^\n]*?)\s*(?:\s[^\n]*)?|`(?P<path_backtick_only>[^`\n]+?)`(?:\s[^\n]*)?|(?P<num>\d+)\.\s+`(?P<path_numbered_backtick>[^`\n]+?)`(?:\s[^\n]*)?|\*\*\s*`(?P<path_bold_backtick>[^`\n]+?)`\s*\*\*(?:\s[^\n]*)?)$",
        // Arguments for format! start after the format string literal
        actions = actions
    );
    // println!("[REGEX INIT] Full HEADER_REGEX pattern:\n{}", pattern); // DEBUG
    Regex::new(&pattern).expect("Failed to compile HEADER_REGEX")
});

// Regex to find the START of a fenced code block.
pub static OPENING_FENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Use raw string literal
    // Handle optional carriage return for CRLF compatibility
    Regex::new(r"(?m)^\s*(?P<fence>```|````)(?P<lang>[^\n\r]*)(\r?\n)")
        .expect("Failed to compile OPENING_FENCE_REGEX")
});

// Note: Closing fence regex is generated dynamically in pass1.rs based on the opening fence.

// --- TEMPORARY DEBUG FUNCTION ---
// Keep this function for verification if needed, but the main regex is the focus.
#[allow(dead_code)]
pub fn debug_hash_regexes() {
    println!("--- Running Regex Debug ---");
    let actions = &*VALID_ACTIONS_REGEX_STR;
    println!("Action words pattern part: '{}'", actions);

    // Test Option 6 pattern (Simplified)
    let pattern6_str = r#"(?m)^##\s+`(?P<path_hash_backtick>[^`\n]+?)`\s*$"#.to_string();
    let regex6 = Regex::new(&pattern6_str).unwrap();
    let input6 = "## `hash/tick.css`";
    println!("\nTesting Pattern 6 (Simplified): '{}'", pattern6_str);
    println!("Input 6: '{}'", input6);
    match regex6.captures(input6) {
        Some(caps) => println!(
            "  MATCHED Option 6: path='{:?}'",
            caps.name("path_hash_backtick").map(|m| m.as_str())
        ),
        None => println!("  FAILED TO MATCH Option 6"),
    }

    // Test Option 2 pattern (No newline match)
    let pattern2_nonl_str = format!(
        r#"(?m)^##\s+(?P<action_word_hash>{actions}):\s*(?P<content_hash>[^\n]*?)\s*$"#,
        actions = actions
    );
    let regex2_nonl = Regex::new(&pattern2_nonl_str).unwrap();
    let input2a = "## Deleted File: old_file.log  ";
    let input2b = "## Deleted File: `another/tick.log`";
    let input2c = "## File: "; // Should match, content empty

    println!(
        "\nTesting Pattern 2 (No newline match): '{}'",
        pattern2_nonl_str
    );
    println!("Input 2a: '{}'", input2a);
    match regex2_nonl.captures(input2a) {
        Some(caps) => println!(
            "  MATCHED Option 2 on 2a: action='{}', content='{}'",
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str()
        ),
        None => println!("  FAILED TO MATCH Option 2 on 2a"),
    }
    println!("Input 2b: '{}'", input2b);
    match regex2_nonl.captures(input2b) {
        Some(caps) => println!(
            "  MATCHED Option 2 on 2b: action='{}', content='{}'",
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str()
        ),
        None => println!("  FAILED TO MATCH Option 2 on 2b"),
    }
    println!("Input 2c: '{}'", input2c);
    match regex2_nonl.captures(input2c) {
        Some(caps) => println!(
            "  MATCHED Option 2 on 2c: action='{}', content='{}'", // EXPECTED MATCH, content empty
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str()
        ),
        None => println!("  FAILED TO MATCH Option 2 on 2c"),
    }

    // Test Combined HEADER_REGEX on failing inputs
    println!("\nTesting Combined HEADER_REGEX (No newline match):");
    println!("Input 6 (hash backtick): '{}'", input6);
    match HEADER_REGEX.captures(input6) {
        Some(caps) => println!(
            "  Combined MATCHED Input 6: path_hash_backtick='{:?}'",
            caps.name("path_hash_backtick").map(|m| m.as_str())
        ),
        None => println!("  Combined FAILED TO MATCH Input 6"),
    }
    println!("Input 2a (hash deleted): '{}'", input2a);
    match HEADER_REGEX.captures(input2a) {
        Some(caps) => println!(
            "  Combined MATCHED Input 2a: action='{}', content='{}'",
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str()
        ),
        None => println!("  Combined FAILED TO MATCH Input 2a"),
    }
    println!("Input 2b (hash deleted backtick): '{}'", input2b);
    match HEADER_REGEX.captures(input2b) {
        Some(caps) => println!(
            "  Combined MATCHED Input 2b: action='{}', content='{}'",
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str()
        ),
        None => println!("  Combined FAILED TO MATCH Input 2b"),
    }
    println!("Input 2c (hash file empty): '{}'", input2c); // Test the problematic case
    match HEADER_REGEX.captures(input2c) {
        Some(caps) => println!(
            "  Combined MATCHED Input 2c: action='{}', content='{}'", // EXPECTED MATCH, content empty
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str()
        ),
        None => println!("  Combined FAILED TO MATCH Input 2c"),
    }

    // Test bold backtick path
    let input_bold_tick = "**`bold/tick.js`**";
    println!("Input bold backtick: '{}'", input_bold_tick);
    match HEADER_REGEX.captures(input_bold_tick) {
        Some(caps) => println!(
            "  Combined MATCHED bold backtick: path_bold_backtick='{:?}'",
            caps.name("path_bold_backtick").map(|m| m.as_str())
        ),
        None => println!("  Combined FAILED TO MATCH bold backtick"),
    }

    // Test the previously failing cases
    let input_backtick_trailing = "`simple/path.rs` (some comment)";
    println!("Input backtick trailing: '{}'", input_backtick_trailing);
    match HEADER_REGEX.captures(input_backtick_trailing) {
        Some(caps) => println!(
            "  Combined MATCHED backtick trailing: path_backtick_only='{:?}'",
            caps.name("path_backtick_only").map(|m| m.as_str())
        ),
        None => println!("  Combined FAILED TO MATCH backtick trailing"),
    }

    let input_bold_backtick_trailing = "**`bold/tick.js`** and more";
    println!(
        "Input bold backtick trailing: '{}'",
        input_bold_backtick_trailing
    );
    match HEADER_REGEX.captures(input_bold_backtick_trailing) {
        Some(caps) => println!(
            "  Combined MATCHED bold backtick trailing: path_bold_backtick='{:?}'",
            caps.name("path_bold_backtick").map(|m| m.as_str())
        ),
        None => println!("  Combined FAILED TO MATCH bold backtick trailing"),
    }

    println!("--- End Regex Debug ---");
}
// --- END TEMPORARY DEBUG FUNCTION ---
