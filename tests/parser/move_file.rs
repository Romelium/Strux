//! Tests for parsing 'Moved File' headers.

use super::common::*; // Use helper from common.rs
use strux::core_types::ActionType;
use strux::parse_markdown;

#[test]
fn test_parse_hash_moved_file_header() {
    let md = "\n## Moved File: old/file.txt to new/location/file.txt\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Move,
        "old/file.txt",
        Some("new/location/file.txt"),
        None,
    );
}

#[test]
fn test_parse_bold_moved_file_header() {
    let md = "\n**Moved File: source.md to destination.md**\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Move,
        "source.md",
        Some("destination.md"),
        None,
    );
}

#[test]
fn test_parse_moved_file_with_backticks_in_paths() {
    let md = "\n## Moved File: `old path/file name.txt` to `new path/another name.txt`\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Move,
        "old path/file name.txt",
        Some("new path/another name.txt"),
        None,
    );
}

#[test]
fn test_parse_moved_file_mixed_backticks() {
    let md = "\n**Moved File: `old path/file.txt` to new_plain_path/file.txt**\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Move,
        "old path/file.txt",
        Some("new_plain_path/file.txt"),
        None,
    );
}

#[test]
fn test_parse_moved_file_with_trailing_comment() {
    let md = "\n## Moved File: old.log to archive/old.log # Log archival\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Move,
        "old.log",
        Some("archive/old.log"), // Comment should be stripped
        None,
    );
}

#[test]
fn test_parse_moved_file_with_trailing_parenthesis_comment() {
    let md = "\n## Moved File: old.data to new.data (migrated)\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Move,
        "old.data",
        Some("new.data"), // Comment should be stripped
        None,
    );
}

#[test]
fn test_parse_moved_file_malformed_no_to_keyword() {
    let md = "\n## Moved File: old/path.txt new/path.txt\n"; // Missing " to "
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Malformed Moved File header should be ignored"
    );
}

#[test]
fn test_parse_moved_file_empty_source_path() {
    let md = "\n## Moved File: to new/path.txt\n"; // Empty source
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Moved File with empty source path should be ignored"
    );
}

#[test]
fn test_parse_moved_file_empty_destination_path() {
    let md = "\n## Moved File: old/path.txt to \n"; // Empty destination
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Moved File with empty destination path should be ignored"
    );
}

#[test]
fn test_parse_moved_file_invalid_source_path_format() {
    let md = "\n## Moved File: old//path.txt to new/path.txt\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Moved File with invalid source path should be skipped"
    );
}

#[test]
fn test_parse_moved_file_invalid_destination_path_format() {
    let md = "\n## Moved File: old/path.txt to new//path.txt\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert!(
        actions.is_empty(),
        "Moved File with invalid destination path should be skipped"
    );
}

#[test]
fn test_parse_wrapped_moved_file_header() {
    let md = "\n```markdown\n## Moved File: staging/report.pdf to final/report_v2.pdf\n```\n";
    let actions = parse_markdown(md).expect("Parsing failed");
    assert_eq!(actions.len(), 1);
    assert_action(
        actions.first(),
        ActionType::Move,
        "staging/report.pdf",
        Some("final/report_v2.pdf"),
        None,
    );
}

#[test]
fn test_parse_moved_file_path_containing_to_keyword_needs_backticks() {
    // This should fail to parse correctly as "file with to" will be the source, and "in name.txt to new_name.txt" the dest.
    // The user *must* use backticks for "file with to in name.txt"
    let md_bad = "\n## Moved File: file with to in name.txt to new_name.txt\n";
    let actions_bad = parse_markdown(md_bad).expect("Parsing failed for bad 'to'");
    // The header_utils.rs logic will split by the first " to ".
    assert_eq!(actions_bad.len(), 1);
    assert_eq!(actions_bad[0].path, "file with"); // This is what the current parser does
    assert_eq!(
        actions_bad[0].dest_path.as_deref(),
        Some("in name.txt to new_name.txt") // This is what the current parser does
    );

    let md_good = "\n## Moved File: `file with to in name.txt` to new_name.txt\n";
    let actions_good = parse_markdown(md_good).expect("Parsing failed for good 'to'");
    assert_eq!(actions_good.len(), 1);
    assert_action(
        actions_good.first(),
        ActionType::Move,
        "file with to in name.txt",
        Some("new_name.txt"),
        None,
    );

    let md_good_both_ticked =
        "\n## Moved File: `file with to in name.txt` to `another to file.log`\n";
    let actions_good_both =
        parse_markdown(md_good_both_ticked).expect("Parsing failed for good 'to' both sides");
    assert_eq!(actions_good_both.len(), 1);
    assert_action(
        actions_good_both.first(),
        ActionType::Move,
        "file with to in name.txt",
        Some("another to file.log"),
        None,
    );

    let md_dest_ticked_with_to = "\n## Moved File: source.txt to `dest with to in name.txt`\n";
    let actions_dest_ticked =
        parse_markdown(md_dest_ticked_with_to).expect("Parsing failed for dest ticked with 'to'");
    assert_eq!(actions_dest_ticked.len(), 1);
    assert_action(
        actions_dest_ticked.first(),
        ActionType::Move,
        "source.txt",
        Some("dest with to in name.txt"),
        None,
    );
}
