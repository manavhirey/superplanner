use sp_schema::{approval_content_id, bounded_brief_content_id, MarkerKind};

const GOLDEN_A: &[u8] = b"Title\n<!-- superplanner-approval:start -->\n- status: `pending`\n<!-- superplanner-approval:end -->\nTail\n";
const GOLDEN_A_ID: &str = "sha256:8f96c4cff8d4ca67417eb0fe90e3eb877791bb6ef06e6c617f96a7abcc820ed9";

const GOLDEN_B: &[u8] = b"Pre\n<!-- superplanner-bounded-brief:start -->\nINSIDE\n<!-- superplanner-bounded-brief:end -->\nPost\n";
const GOLDEN_B_ID: &str = "sha256:4dafdd538c8bae26e89f703b1370bfa1138cf3ba332d1f7127101e4fcce8df28";

const GOLDEN_C: &[u8] =
    b"A\r\n<!-- superplanner-approval:start -->\r\nx\r\n<!-- superplanner-approval:end -->\r\nTail\r\n";
const GOLDEN_C_ID: &str = "sha256:43e051fc2988bd92137a0570c98565b188843624c1a4a7c7cd58e3bb83a098da";

const GOLDEN_D: &[u8] =
    b"<!-- superplanner-approval:start -->\nx\n<!-- superplanner-approval:end -->\nTail";

#[test]
fn approval_golden_matches_reference_digest() {
    assert_eq!(
        approval_content_id(GOLDEN_A, MarkerKind::ApprovalMarkdown).unwrap(),
        GOLDEN_A_ID
    );
}

#[test]
fn bounded_brief_golden_matches_reference_digest() {
    assert_eq!(bounded_brief_content_id(GOLDEN_B).unwrap(), GOLDEN_B_ID);
}

#[test]
fn crlf_remainder_is_preserved_byte_exactly() {
    assert_eq!(
        approval_content_id(GOLDEN_C, MarkerKind::ApprovalMarkdown).unwrap(),
        GOLDEN_C_ID
    );
}

#[test]
fn final_newline_is_not_synthesized() {
    let without = approval_content_id(GOLDEN_D, MarkerKind::ApprovalMarkdown).unwrap();
    assert_eq!(without, format!("sha256:{}", sha256_of(b"Tail")));
    assert_ne!(without, format!("sha256:{}", sha256_of(b"Tail\n")));
}

#[test]
fn inside_edits_do_not_change_approval_content_id() {
    let edited = b"Title\n<!-- superplanner-approval:start -->\n- status: `approved`\n- approver: `u`\n- approved_at: `2026-09-11T00:00:00Z`\n- approval_evidence: `e`\n- content_id: `sha256:0000000000000000000000000000000000000000000000000000000000000000`\n<!-- superplanner-approval:end -->\nTail\n";
    assert_eq!(
        approval_content_id(edited, MarkerKind::ApprovalMarkdown).unwrap(),
        GOLDEN_A_ID
    );
}

#[test]
fn outside_edits_change_approval_content_id() {
    let edited = b"Title!\n<!-- superplanner-approval:start -->\n- status: `pending`\n<!-- superplanner-approval:end -->\nTail\n";
    assert_ne!(
        approval_content_id(edited, MarkerKind::ApprovalMarkdown).unwrap(),
        GOLDEN_A_ID
    );
}

#[test]
fn missing_marker_blocks() {
    let bad = b"Title\n- status: `pending`\nTail\n";
    assert!(approval_content_id(bad, MarkerKind::ApprovalMarkdown).is_err());
}

#[test]
fn duplicate_marker_blocks() {
    let bad = b"<!-- superplanner-approval:start -->\n<!-- superplanner-approval:start -->\nx\n<!-- superplanner-approval:end -->\n";
    assert!(approval_content_id(bad, MarkerKind::ApprovalMarkdown).is_err());
}

#[test]
fn nested_marker_blocks() {
    let bad = b"<!-- superplanner-approval:start -->\n<!-- superplanner-approval:start -->\n<!-- superplanner-approval:end -->\n<!-- superplanner-approval:end -->\n";
    assert!(approval_content_id(bad, MarkerKind::ApprovalMarkdown).is_err());
}

#[test]
fn misordered_marker_blocks() {
    let bad = b"<!-- superplanner-approval:end -->\nx\n<!-- superplanner-approval:start -->\n";
    assert!(approval_content_id(bad, MarkerKind::ApprovalMarkdown).is_err());
}

#[test]
fn marker_with_trailing_bytes_blocks() {
    let bad =
        b"<!-- superplanner-approval:start --> extra\nx\n<!-- superplanner-approval:end -->\n";
    assert!(approval_content_id(bad, MarkerKind::ApprovalMarkdown).is_err());
}

#[test]
fn marker_with_leading_bytes_blocks() {
    let bad = b" <!-- superplanner-approval:start -->\nx\n<!-- superplanner-approval:end -->\n";
    assert!(approval_content_id(bad, MarkerKind::ApprovalMarkdown).is_err());
}

#[test]
fn gherkin_markers_validate() {
    let g = b"# t\n# superplanner-approval:start\n# status: pending\n# superplanner-approval:end\n# u\n";
    assert!(approval_content_id(g, MarkerKind::ApprovalGherkin).is_ok());
    let rem = b"# t\n# u\n";
    let digest = sha256_of(rem);
    assert_eq!(
        approval_content_id(g, MarkerKind::ApprovalGherkin).unwrap(),
        format!("sha256:{digest}")
    );
}

fn sha256_of(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}
