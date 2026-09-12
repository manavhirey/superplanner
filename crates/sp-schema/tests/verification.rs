use sp_schema::git_mediation::ObjectFormat;
use sp_schema::verification::{
    bind_verification_to_documentation, DocumentationRecord, DocumentationStatus,
    VerificationCheck, VerificationRecord, DOCUMENTATION_TAG, VERIFICATION_TAG,
};
use sp_schema::{sha256_digest, RegistryIdentity};

const OID: &str = "0123456789abcdef0123456789abcdef01234567";
const TREE: &str = "89abcdef0123456789abcdef0123456789abcdef";
const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn check() -> VerificationCheck {
    VerificationCheck {
        command: "cargo test --workspace".to_string(),
        result: "passed".to_string(),
        evidence: "all tests passed".to_string(),
    }
}

fn verification() -> VerificationRecord {
    VerificationRecord {
        schema: VERIFICATION_TAG.to_string(),
        object_format: ObjectFormat::Sha1,
        bound_head: OID.to_string(),
        candidate_tree: TREE.to_string(),
        candidate_patch_sha256: DIGEST.to_string(),
        candidate_patch_identity: RegistryIdentity {
            path: "/registry/candidate.patch".to_string(),
            device: 1,
            inode: 1,
            link_count: 1,
            sha256: DIGEST.to_string(),
        },
        checks: vec![check()],
    }
}

fn documentation() -> DocumentationRecord {
    let verification = verification();
    let verification_sha256 = sha256_digest(verification.to_canonical_json().unwrap().as_bytes());
    DocumentationRecord {
        schema: DOCUMENTATION_TAG.to_string(),
        object_format: ObjectFormat::Sha1,
        bound_head: OID.to_string(),
        candidate_tree: TREE.to_string(),
        candidate_patch_sha256: DIGEST.to_string(),
        candidate_patch_identity: verification.candidate_patch_identity,
        updated_paths: vec!["docs/runtime.md".to_string()],
        no_change_reason: "none".to_string(),
        blocked_reason: "none".to_string(),
        status: DocumentationStatus::Current,
        checks: vec![check()],
        bound_verification_sha256: verification_sha256,
    }
}

#[test]
fn verification_accepts_current_evidence() {
    verification().validate().unwrap();
}

#[test]
fn verification_rejects_bad_binding_and_check_forms() {
    let mut zero = verification();
    zero.bound_head = "0000000000000000000000000000000000000000".to_string();
    assert!(zero.validate().is_err());

    let mut bad_result = verification();
    bad_result.checks[0].result = "success".to_string();
    assert!(bad_result.validate().is_err());

    let mut empty = verification();
    empty.checks.clear();
    assert!(empty.validate().is_err());

    let mut bad_tree = verification();
    bad_tree.candidate_tree = "0123".to_string();
    assert!(bad_tree.validate().is_err());

    let mut failed = verification();
    failed.checks[0].result = "failed".to_string();
    failed.validate().unwrap();
    assert!(failed.require_passed().is_err());
}

#[test]
fn documentation_accepts_updated_paths() {
    let documentation = documentation();
    documentation.validate().unwrap();
    bind_verification_to_documentation(&verification(), &documentation).unwrap();
}

#[test]
fn documentation_accepts_none_with_reason() {
    let mut record = documentation();
    record.updated_paths.clear();
    record.no_change_reason = "behavior has no user-facing documentation impact".to_string();
    record.validate().unwrap();
}

#[test]
fn documentation_requires_exact_none_with_reason_shape() {
    let mut missing_reason = documentation();
    missing_reason.updated_paths.clear();
    assert!(missing_reason.validate().is_err());

    let mut conflicting_reason = documentation();
    conflicting_reason.no_change_reason = "not needed".to_string();
    assert!(conflicting_reason.validate().is_err());

    let mut blocked_without_reason = documentation();
    blocked_without_reason.status = DocumentationStatus::Blocked;
    assert!(blocked_without_reason.validate().is_err());
}

#[test]
fn documentation_rejects_duplicate_or_nonrelative_paths() {
    let mut duplicate = documentation();
    duplicate.updated_paths.push("docs/runtime.md".to_string());
    assert!(duplicate.validate().is_err());

    let mut absolute = documentation();
    absolute.updated_paths = vec!["/docs/runtime.md".to_string()];
    assert!(absolute.validate().is_err());

    let mut unordered = documentation();
    unordered.updated_paths = vec!["docs/z.md".to_string(), "docs/a.md".to_string()];
    assert!(unordered.validate().is_err());

    let mut nul = documentation();
    nul.updated_paths = vec!["docs/a\0docs/b".to_string()];
    assert!(nul.validate().is_err());
}

#[test]
fn documentation_stale_and_blocked_states_validate() {
    let mut stale = documentation();
    stale.status = DocumentationStatus::Stale;
    stale.checks.clear();
    stale.validate().unwrap();

    let mut blocked = documentation();
    blocked.status = DocumentationStatus::Blocked;
    blocked.no_change_reason = "none".to_string();
    blocked.blocked_reason = "sandbox unavailable".to_string();
    blocked.checks.clear();
    blocked.validate().unwrap();
}

#[test]
fn verification_records_require_canonical_json() {
    let record = verification();
    let canonical = record.to_canonical_json().unwrap();
    assert_eq!(
        sha256_digest(canonical.as_bytes()),
        "512947f8ebc6a3c54d8238bd4369c30e567955fc58df400a912ce7a761068519"
    );
    assert_eq!(
        VerificationRecord::from_canonical_json(canonical.as_bytes()).unwrap(),
        record
    );
    let pretty = serde_json::to_string_pretty(&record).unwrap();
    assert!(VerificationRecord::from_canonical_json(pretty.as_bytes()).is_err());

    let documentation = documentation();
    let canonical = documentation.to_canonical_json().unwrap();
    assert_eq!(
        sha256_digest(canonical.as_bytes()),
        "3ada65532953f4b7bd2a2889dc3e373a3cbbd9b9f1fa8f5b7a22d67677daa0cf"
    );
    assert_eq!(
        DocumentationRecord::from_canonical_json(canonical.as_bytes()).unwrap(),
        documentation
    );
}

#[test]
fn documentation_rejects_stale_verification_binding() {
    let mut stale_candidate = documentation();
    stale_candidate.candidate_tree = OID.to_string();
    assert!(bind_verification_to_documentation(&verification(), &stale_candidate).is_err());

    let mut bad_hash = documentation();
    bad_hash.bound_verification_sha256 = DIGEST.to_string();
    assert!(bind_verification_to_documentation(&verification(), &bad_hash).is_err());
}
