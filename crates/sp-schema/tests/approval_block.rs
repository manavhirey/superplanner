use sp_schema::approval::{parse_approval_block, ApprovalFlavor};

fn pending_markdown() -> Vec<u8> {
    b"Head\n<!-- superplanner-approval:start -->\n- status: `pending`\n- approver: `none`\n- approved_at: `none`\n- approval_evidence: `none`\n- content_id: `sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef`\n<!-- superplanner-approval:end -->\nTail\n".to_vec()
}

fn approved_markdown() -> Vec<u8> {
    b"<!-- superplanner-approval:start -->\n- status: `approved`\n- approver: `manavhirey`\n- approved_at: `2026-09-11T20:54:39Z`\n- approval_evidence: `user message 123 approving displayed artifact and content id`\n- content_id: `sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef`\n<!-- superplanner-approval:end -->\n".to_vec()
}

fn gherkin_pending() -> Vec<u8> {
    b"# superplanner-approval:start\n# status: pending\n# approver: none\n# approved_at: none\n# approval_evidence: none\n# content_id: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\n# superplanner-approval:end\n".to_vec()
}

#[test]
fn pending_markdown_block_parses() {
    let block = parse_approval_block(&pending_markdown(), ApprovalFlavor::Markdown).unwrap();
    assert_eq!(block.status, "pending");
    assert_eq!(block.approver, "none");
    assert_eq!(
        block.content_id,
        "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    );
}

#[test]
fn pending_gherkin_block_parses() {
    let block = parse_approval_block(&gherkin_pending(), ApprovalFlavor::Gherkin).unwrap();
    assert_eq!(block.status, "pending");
}

#[test]
fn approved_markdown_block_parses() {
    let block = parse_approval_block(&approved_markdown(), ApprovalFlavor::Markdown).unwrap();
    assert_eq!(block.status, "approved");
    assert_eq!(block.approved_at, "2026-09-11T20:54:39Z");
}

#[test]
fn approved_requires_non_none_approver() {
    let mut bytes = approved_markdown();
    let text = String::from_utf8(bytes.clone()).unwrap();
    let edited = text.replace("- approver: `manavhirey`", "- approver: `none`");
    bytes = edited.into_bytes();
    assert!(parse_approval_block(&bytes, ApprovalFlavor::Markdown).is_err());
}

#[test]
fn approved_requires_valid_time() {
    let text = String::from_utf8(approved_markdown()).unwrap();
    let edited = text.replace("2026-09-11T20:54:39Z", "2026-09-11 20:54:39");
    assert!(parse_approval_block(edited.as_bytes(), ApprovalFlavor::Markdown).is_err());
}

#[test]
fn pending_rejects_identity_fields() {
    let text = String::from_utf8(pending_markdown()).unwrap();
    let edited = text.replace("- approver: `none`", "- approver: `someone`");
    assert!(parse_approval_block(edited.as_bytes(), ApprovalFlavor::Markdown).is_err());
}

#[test]
fn bad_content_id_form_blocks() {
    let text = String::from_utf8(pending_markdown()).unwrap();
    let edited = text.replace(
        "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "0123456789abcdef",
    );
    assert!(parse_approval_block(edited.as_bytes(), ApprovalFlavor::Markdown).is_err());
}

#[test]
fn extra_field_line_blocks() {
    let text = String::from_utf8(pending_markdown()).unwrap();
    let edited = text.replace(
        "<!-- superplanner-approval:end -->",
        "- extra: `field`\n<!-- superplanner-approval:end -->",
    );
    assert!(parse_approval_block(edited.as_bytes(), ApprovalFlavor::Markdown).is_err());
}

#[test]
fn wrong_field_order_blocks() {
    let text = String::from_utf8(approved_markdown()).unwrap();
    let edited = text.replace(
        "- status: `approved`\n- approver: `manavhirey`",
        "- approver: `manavhirey`\n- status: `approved`",
    );
    assert!(parse_approval_block(edited.as_bytes(), ApprovalFlavor::Markdown).is_err());
}

#[test]
fn markdown_values_must_be_backticked() {
    let text = String::from_utf8(pending_markdown()).unwrap();
    let edited = text.replace("- status: `pending`", "- status: pending");
    assert!(parse_approval_block(edited.as_bytes(), ApprovalFlavor::Markdown).is_err());
}

#[test]
fn missing_field_blocks() {
    let text = String::from_utf8(pending_markdown()).unwrap();
    let edited = text.replace("- approval_evidence: `none`\n", "");
    assert!(parse_approval_block(edited.as_bytes(), ApprovalFlavor::Markdown).is_err());
}

#[test]
fn invalid_utf8_blocks() {
    let mut bytes = pending_markdown();
    bytes[50] = 0xff;
    assert!(parse_approval_block(&bytes, ApprovalFlavor::Markdown).is_err());
}
