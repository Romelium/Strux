//! Tests for parser handling of invalid path formats.

use strux::parse_markdown;

#[test]
fn test_parse_invalid_path_format_skipped() {
    let md_create = "\n**File: bad//path.txt**\n```\ncontent\n```\n";
    let actions_create = parse_markdown(md_create).expect("Parsing failed");
    assert!(
        actions_create.is_empty(),
        "Create action with invalid path format should be skipped"
    );

    let md_delete = "\n**Deleted File: another//bad/path**\n";
    let actions_delete = parse_markdown(md_delete).expect("Parsing failed");
    assert!(
        actions_delete.is_empty(),
        "Delete action with invalid path format should be skipped"
    );

    let md_create_trailing = "\n**File: bad/path/**\n```\ncontent\n```\n";
    let actions_create_trailing = parse_markdown(md_create_trailing).expect("Parsing failed");
    assert!(
        actions_create_trailing.is_empty(),
        "Create action with trailing slash path format should be skipped"
    );

    let md_delete_trailing = "\n**Deleted File: another/bad/path/**\n";
    let actions_delete_trailing = parse_markdown(md_delete_trailing).expect("Parsing failed");
    assert!(
        actions_delete_trailing.is_empty(),
        "Delete action with trailing slash path format should be skipped"
    );
}
