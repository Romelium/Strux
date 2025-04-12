//! Defines and compiles regular expressions used by the parser.

use crate::constants::VALID_ACTIONS_REGEX_STR; // Use the pre-built string
use once_cell::sync::Lazy;
use regex::Regex;

// Use Lazy from once_cell for thread-safe static initialization of Regex objects.

// Regex to find file headers anchored to the start of a line.
// Revised to prevent matching across lines and simplify trailing whitespace handling.
pub static HEADER_REGEX: Lazy<Regex> = Lazy::new(|| {
    let actions = &*VALID_ACTIONS_REGEX_STR; // Dereference Lazy<String>

    // Use a single multi-line raw string literal r#"..."# as the format string
    // Put back \s*$ anchor specifically for content_bold and content_hash alternatives.
    // Use non-greedy *? for content capture before the \s*$ anchor.
    // No final \s*$ at the very end of the whole pattern string.
    let pattern = format!(
        r#"(?m)^(?:\*\*\s*(?P<action_word_bold>{actions}):\s+(?P<content_bold>[^\n]+?)\s*\*\*|##\s+`(?P<path_hash_backtick>[^`\n]+?)`|##\s+(?P<action_word_hash>{actions}):\s*(?P<content_hash>[^\n]*?)\s*$|`(?P<path_backtick_only>[^`\n]+?)`|(?P<num>\d+)\.\s+`(?P<path_numbered_backtick>[^`\n]+?)`|\*\*\s*`(?P<path_bold_backtick>[^`\n]+?)`\s*\*\*)"#,
        // Note: Added \s*$ to content_hash alternative only. content_bold already had \s*\*\* which acts similarly.
        //       Kept content_hash as *? (non-greedy)
        actions = actions // Argument for format!
    );
    // println!("[REGEX INIT] Revised HEADER_REGEX pattern:\n{}", pattern); // DEBUG (optional)
    Regex::new(&pattern).expect("Failed to compile HEADER_REGEX")
});

// Regex to find the START of a fenced code block.
pub static OPENING_FENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Use raw string literal
    // Handle optional carriage return for CRLF compatibility
    Regex::new(r#"(?m)^\s*(?P<fence>```|````)(?P<lang>[^\n\r]*)(\r?\n)"#)
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

    // Test Option 2 pattern (with trailing \s*$)
    let pattern2_nonl_str = format!(
        r#"(?m)^##\s+(?P<action_word_hash>{actions}):\s*(?P<content_hash>[^\n]*?)\s*$"#, // Added \s*$ back
        actions = actions
    );
    let regex2_nonl = Regex::new(&pattern2_nonl_str).unwrap();
    let input2a = "## Deleted File: old_file.log  ";
    let input2b = "## Deleted File: `another/tick.log`";
    let input2c = "## File: "; // Should match, content empty

    println!(
        "\nTesting Pattern 2 (with trailing anchor): '{}'", // Updated description
        pattern2_nonl_str
    );
    println!("Input 2a: '{}'", input2a);
    match regex2_nonl.captures(input2a) {
        Some(caps) => println!(
            "  MATCHED Option 2 on 2a: action='{}', content='{}'",
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str() // Should capture content before trailing spaces
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
    println!("\nTesting Combined HEADER_REGEX (Re-anchored content_hash):"); // Updated description
    println!("Input 6 (hash backtick): '{}'", input6);
    match HEADER_REGEX.captures(input6) {
        Some(caps) => println!(
            "  Combined MATCHED Input 6: path_hash_backtick='{:?}'",
            caps.name("path_hash_backtick").map(|m| m.as_str())
        ),
        None => println!("  Combined FAILED TO MATCH Input 6"),
    }
    println!("Input 2a (hash deleted): '{}'", input2a); // Test the failing case
    match HEADER_REGEX.captures(input2a) {
        Some(caps) => println!(
            "  Combined MATCHED Input 2a: action='{}', content='{}'", // EXPECTED MATCH
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str()
        ),
        None => println!("  Combined FAILED TO MATCH Input 2a"),
    }
    println!("Input 2b (hash deleted backtick): '{}'", input2b); // Test the failing case
    match HEADER_REGEX.captures(input2b) {
        Some(caps) => println!(
            "  Combined MATCHED Input 2b: action='{}', content='{}'", // EXPECTED MATCH
            caps.name("action_word_hash").unwrap().as_str(),
            caps.name("content_hash").unwrap().as_str()
        ),
        None => println!("  Combined FAILED TO MATCH Input 2b"),
    }
    println!("Input 2c (hash file empty): '{}'", input2c);
    match HEADER_REGEX.captures(input2c) {
        Some(caps) => println!(
            "  Combined MATCHED Input 2c: action='{}', content='{}'",
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

    // Test the previously failing cases (should still pass)
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

    // Test the multi-line case that failed the test
    let multi_line_input = "\n## Deleted File: file1.txt\n**Deleted File: dir/file2.txt**\n";
    println!("\nTesting Multi-Line Input:\n{}", multi_line_input);
    for (i, caps) in HEADER_REGEX.captures_iter(multi_line_input).enumerate() {
        println!("  Match {}: '{}'", i + 1, caps.get(0).unwrap().as_str());
        if let Some(m) = caps.name("action_word_hash") {
            println!("    action_word_hash: {}", m.as_str());
        }
        if let Some(m) = caps.name("content_hash") {
            println!("    content_hash: {}", m.as_str());
        }
        if let Some(m) = caps.name("action_word_bold") {
            println!("    action_word_bold: {}", m.as_str());
        }
        if let Some(m) = caps.name("content_bold") {
            println!("    content_bold: {}", m.as_str());
        }
        if let Some(m) = caps.name("path_hash_backtick") {
            println!("    path_hash_backtick: {}", m.as_str());
        }
        if let Some(m) = caps.name("path_backtick_only") {
            println!("    path_backtick_only: {}", m.as_str());
        }
        if let Some(m) = caps.name("path_numbered_backtick") {
            println!("    path_numbered_backtick: {}", m.as_str());
        }
        if let Some(m) = caps.name("path_bold_backtick") {
            println!("    path_bold_backtick: {}", m.as_str());
        }
    }

    println!("--- End Regex Debug ---");
}
// --- END TEMPORARY DEBUG FUNCTION ---
