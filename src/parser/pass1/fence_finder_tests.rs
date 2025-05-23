//! Unit tests for the fence_finder module.

#[cfg(test)]
mod tests {
    use crate::parser::pass1::fence_finder::find_closing_fence;

    #[test]
    fn find_simple_closing_fence() {
        let content = "```rust\nfn main() {}\n```";
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1; // After "```rust\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(closing_match.is_some());
        assert_eq!(closing_match.unwrap().as_str(), "```");
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap()
        );
    }

    #[test]
    fn find_closing_fence_no_newline_after_open() {
        // This case should not occur with current OPENING_FENCE_REGEX, but tests robustness
        let content = "```fn main() {}\n```";
        let fence_chars = "```";
        let search_start_pos = 3; // After "```"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(closing_match.is_some());
        assert_eq!(closing_match.unwrap().as_str(), "```");
    }

    #[test]
    fn find_closing_fence_unclosed() {
        let content = "```rust\nfn main() {";
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        assert!(find_closing_fence(content, fence_chars, search_start_pos).is_none());
    }

    #[test]
    fn find_closing_fence_nested_same_type() {
        let content = "```markdown\nOuter\n```rust\nInner\n```\nOuter again\n```";
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1; // After "```markdown\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(closing_match.is_some());
        assert_eq!(closing_match.unwrap().as_str(), "```");
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap()
        ); // Should be the last one
    }

    #[test]
    fn find_closing_fence_multiple_nesting_levels() {
        let content = "```L0\n```L1\n```L2\nText\n```\n```\n```";
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1; // After "```L0\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(closing_match.is_some());
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap()
        );
    }

    #[test]
    fn find_closing_fence_different_fence_types_ignored() {
        let content = "```markdown\nOuter\n````rust\nInner four-tick\n````\nOuter again\n```";
        let fence_chars = "```"; // Looking for three-ticks
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Failed on: find_closing_fence_different_fence_types_ignored. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap()
        );
    }

    #[test]
    fn minimal_different_fence_ignored() {
        let content = "```\n````\n```"; // Open ```, then ```` (should be ignored), then close ```
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1; // After "```\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Minimal case: closing_match should be Some. Content:\n{}",
            content
        );
        assert_eq!(closing_match.unwrap().as_str(), "```");
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap(),
            "Minimal case: Mismatched closing fence position"
        );
    }

    #[test]
    fn find_closing_fence_four_ticks_nested() {
        let content = "````yaml\nOuter\n```bash\nInner three-tick\n```\nOuter again\n````";
        let fence_chars = "````"; // Looking for four-ticks
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(closing_match.is_some());
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("````").unwrap()
        );
    }

    #[test]
    fn find_closing_fence_empty_block() {
        let content = "```\n```";
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(closing_match.is_some());
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap()
        );
    }

    #[test]
    fn find_closing_fence_content_after_final_fence() {
        let content = "```\nBlock\n```\nSome trailing text.";
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(closing_match.is_some());
        assert_eq!(
            closing_match.unwrap().start(),
            content.find("```\nSome trailing text.").unwrap()
        );
    }

    #[test]
    fn find_closing_fence_sequential_blocks() {
        // This tests if find_closing_fence correctly identifies the closer for the *first* block.
        let content = "```first\nContent1\n```\n```second\nContent2\n```";
        let fence_chars = "```";
        let search_start_pos = content.find("first\n").unwrap() + "first\n".len();
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Should find closing fence for the first block"
        );
        // It should match the first "```" that closes the block starting with "```first"
        assert_eq!(
            closing_match.unwrap().start(),
            content.find("```\n```second").unwrap()
        );
    }

    #[test]
    fn find_closing_fence_deeply_nested_and_sequential() {
        let content = "\n```L0_Start\nContent L0 Before L1\n```L1_Start\nContent L1 Before L2\n```L2_Start\nContent L2\n```L2_End\nContent L1 After L2\n```L1_End\nContent L0 After L1\n```L0_End\n";
        // Normalize newlines for consistent testing if needed, but regexes handle \r?\n
        let normalized_content = content
            .replace("L0_Start", "markdown")
            .replace("L1_Start", "rust")
            .replace("L2_Start", "json")
            .replace("L2_End", "")
            .replace("L1_End", "")
            .replace("L0_End", "");

        let fence_chars = "```";
        // Find the position after the first line "```markdown\n"
        let first_newline = normalized_content.find('\n').unwrap();
        let search_start_pos =
            normalized_content[first_newline + 1..].find('\n').unwrap() + first_newline + 1 + 1;

        let closing_match = find_closing_fence(&normalized_content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Should find the final closing fence"
        );
        assert_eq!(
            closing_match.unwrap().start(),
            normalized_content.rfind("```").unwrap(),
            "Closing fence mismatch"
        );
    }

    #[test]
    fn readme_like_structure_find_closing_fence() {
        // A simplified version of the problematic structure from meta_readme test
        // Outer ```md block containing various other ``` blocks.
        let readme_content_sim = "\n## Some Title\n\n```bash\necho \"Hello\"\n```\n\nMore markdown text.\n\n```rust\nfn main() {\n    // A comment with ``` backticks\n}\n```\n\nEnd of markdown.\n";
        let full_input = format!("```md\n{}\n```", readme_content_sim);
        let fence_chars = "```";
        let search_start_pos = "```md\n".len();

        let closing_match = find_closing_fence(&full_input, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "find_closing_fence failed for README-like structure. Content:\n{}",
            full_input
        );
        assert_eq!(
            closing_match.unwrap().start(),
            full_input.rfind("\n```").unwrap() + 1,
            "Mismatched closing fence position"
        );
    }

    #[test]
    fn closing_fence_preceded_by_blank_line() {
        let content = "```\ntext\n\n```"; // Closing fence is preceded by a blank line
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1; // After "```\n"

        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Should find closing fence. Content:\n{}",
            content
        );

        // Calculate expected position: start of the last "```"
        // content.rfind("\n```") finds the newline before the last ```, +1 for the backtick.
        let expected_pos = content.rfind("\n```").unwrap() + 1;
        assert_eq!(
            closing_match.unwrap().start(),
            expected_pos,
            "Mismatched start position for closing fence preceded by blank line"
        );
    }

    #[test]
    fn closing_fence_at_eof_no_trailing_newline() {
        let content = "```\ntext\n```"; // No newline after the final fence
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Fence at EOF not found. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap()
        );
    }

    #[test]
    fn closing_fence_with_leading_whitespace_on_line() {
        let content = "```\ntext\n  ```"; // Closing fence has leading spaces
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Fence with leading whitespace not found. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("  ```").unwrap()
        );
        assert_eq!(closing_match.unwrap().as_str(), "  ```"); // The match includes whitespace due to \s*
    }

    // --- New Test Cases ---

    #[test]
    fn crlf_line_endings() {
        let content = "```rust\r\nfn main() {}\r\n```\r\n";
        let fence_chars = "```";
        let search_start_pos = content.find("\r\n").unwrap() + 2; // After "```rust\r\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "CRLF: closing_match should be Some. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().as_str(),
            "```",
            "CRLF: Matched string unexpected."
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```\r\n").unwrap(),
            "CRLF: Mismatched closing fence position."
        );
    }

    #[test]
    fn fence_chars_in_text_not_as_fence() {
        let content = "```\nThis line contains ``` as text.\nAnd another ```` as text.\n```";
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Fence chars in text: closing_match should be Some. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap(),
            "Fence chars in text: Mismatched closing fence position."
        );
    }

    #[test]
    fn complex_nesting_with_different_types_outer_four() {
        let content = "````yaml\nkey:\n  ```json\n  {\n    \"inner_key\": \"value with ``` backticks\"\n  }\n  ```\nvalue: outer\n````";
        let fence_chars = "````"; // Looking for four-ticks
        let search_start_pos = content.find('\n').unwrap() + 1; // After "````yaml\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Complex nesting (outer four): closing_match should be Some. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("````").unwrap(),
            "Complex nesting (outer four): Mismatched closing fence position."
        );
    }

    #[test]
    fn complex_nesting_with_different_types_outer_three() {
        let content = "```yaml\nkey:\n  ````json\n  {\n    \"inner_key\": \"value with ```` backticks\"\n  }\n  ````\nvalue: outer\n```";
        let fence_chars = "```"; // Looking for three-ticks
        let search_start_pos = content.find('\n').unwrap() + 1; // After "```yaml\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Complex nesting (outer three): closing_match should be Some. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap(),
            "Complex nesting (outer three): Mismatched closing fence position."
        );
    }

    #[test]
    fn unclosed_inner_block_makes_outer_unclosed() {
        let content = "```outer\n  ```inner_unclosed\n  some text in unclosed inner block\n"; // Outer should be unclosed
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1; // After "```outer\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_none(),
            "Unclosed inner: outer closing_match should be None. Content:\n{}",
            content
        );
    }

    #[test]
    fn closing_fence_not_on_own_line_ignored() {
        let content = "```\ntext ``` still on same line\nanother line\n```"; // The middle ``` should be ignored as closer
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Closing not on own line: closing_match should be Some. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap(),
            "Closing not on own line: Mismatched closing fence position."
        );
    }

    #[test]
    fn unclosed_due_to_mismatched_fence_type_at_end() {
        let content = "```\n````"; // Opened with ```, only ```` follows
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1; // After "```\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_none(),
            "Unclosed mismatched end: closing_match should be None. Content:\n{}",
            content
        );
    }

    #[test]
    fn unclosed_due_to_mismatched_fence_type_at_end_four_tick_open() {
        let content = "````\n```"; // Opened with ````, only ``` follows
        let fence_chars = "````";
        let search_start_pos = content.find('\n').unwrap() + 1; // After "````\n"
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_none(),
            "Unclosed mismatched end (four-tick): closing_match should be None. Content:\n{}",
            content
        );
    }

    #[test]
    fn opening_fence_with_lang_tag_and_spaces() {
        let content = "```rust   \nfn main() {}\n```";
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Lang tag spaces: closing_match should be Some. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("```").unwrap()
        );
    }

    #[test]
    fn fences_immediately_adjacent_first_block() {
        let content = "```\n```\n```\n```"; // Two blocks: ```\n``` and ```\n```
        let fence_chars = "```";
        // Test closing for the first block
        let search_start_pos1 = content.find('\n').unwrap() + 1; // After first "```\n"
        let closing_match1 = find_closing_fence(content, fence_chars, search_start_pos1);
        assert!(
            closing_match1.is_some(),
            "Adjacent fences (block 1): closing_match1 should be Some. Content:\n{}",
            content
        );
        assert_eq!(
            closing_match1.unwrap().start(),
            content.find("```\n```").unwrap() + 4,
            "Adjacent fences (block 1): Mismatched position."
        );
    }

    #[test]
    fn closing_fence_with_tabs_on_line() {
        let content = "```\ntext\n\t```\t"; // Closing fence line has tabs around it
        let fence_chars = "```";
        let search_start_pos = content.find('\n').unwrap() + 1;
        let closing_match = find_closing_fence(content, fence_chars, search_start_pos);
        assert!(
            closing_match.is_some(),
            "Closing with tabs: closing_match should be Some. Content:\n{}",
            content
        );
        assert_eq!(closing_match.unwrap().as_str(), "\t```\t");
        assert_eq!(
            closing_match.unwrap().start(),
            content.rfind("\t```\t").unwrap()
        );
    }
}
