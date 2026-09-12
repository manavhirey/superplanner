use sp_schema::commit::{
    AuthorizationTuple, CommitPrepare, COMMIT_AUTHORIZATION_TAG, COMMIT_PREPARE_TAG,
};
use sp_schema::git_mediation::ObjectFormat;
use sp_schema::push_custody::{
    bind_request_to_authorization, validate_push_refspec, validate_push_url, PushEligibility,
    PushPresentationRequest, ResolvedInput, PUSH_PRESENTATION_REQUEST_TAG,
};
use sp_schema::push_encoding::{decode, encode, Field};
use sp_schema::sha256_digest;
use sp_schema::{FilesystemIdentity, RegistryIdentity};

const OID: &str = "0123456789abcdef0123456789abcdef01234567";
const TREE: &str = "89abcdef0123456789abcdef0123456789abcdef";
const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn registry_identity(path: &str, inode: u64) -> String {
    RegistryIdentity {
        path: path.to_string(),
        device: 1,
        inode,
        link_count: 1,
        sha256: DIGEST.to_string(),
    }
    .to_canonical_scalar()
    .unwrap()
}

fn filesystem_identity(path: &str, inode: u64) -> String {
    FilesystemIdentity {
        path: path.to_string(),
        device: 1,
        inode,
    }
    .to_canonical_scalar()
    .unwrap()
}

fn prepare() -> CommitPrepare {
    let message = "Implement schema tranche three\n".to_string();
    let mut record = CommitPrepare {
        schema: COMMIT_PREPARE_TAG.to_string(),
        object_format: ObjectFormat::Sha1,
        patch_sha256: DIGEST.to_string(),
        base_commit: OID.to_string(),
        base_tree: TREE.to_string(),
        candidate_tree: OID.to_string(),
        message_path: "/artifacts/message.txt".to_string(),
        message_sha256: sha256_digest(message.as_bytes()),
        message,
        target_ref: "refs/heads/main".to_string(),
        target_ref_at_base: OID.to_string(),
        author_name: "Manav Hirey".to_string(),
        author_email: "manavhirey@example.com".to_string(),
        committer_name: "Manav Hirey".to_string(),
        committer_email: "manavhirey@example.com".to_string(),
        author_date: "1789099003 -0400".to_string(),
        committer_date: "1789099003 -0400".to_string(),
        expected_commit_oid: TREE.to_string(),
        reflog_reason: "authorized integration of T001".to_string(),
    };
    record.expected_commit_oid = record.computed_commit_oid();
    record
}

fn authorization() -> AuthorizationTuple {
    let prepared = prepare();
    AuthorizationTuple {
        schema: COMMIT_AUTHORIZATION_TAG.to_string(),
        object_format: prepared.object_format,
        patch_sha256: prepared.patch_sha256,
        candidate_tree: prepared.candidate_tree,
        message_sha256: prepared.message_sha256,
        base_commit: prepared.base_commit,
        target_ref: prepared.target_ref,
        target_state: prepared.target_ref_at_base,
        author_date: prepared.author_date,
        committer_date: prepared.committer_date,
        expected_commit_oid: prepared.expected_commit_oid,
        author_name: prepared.author_name,
        author_email: prepared.author_email,
        committer_name: prepared.committer_name,
        committer_email: prepared.committer_email,
        reflog_reason: prepared.reflog_reason,
    }
}

#[test]
fn commit_prepare_validates() {
    let prepared = prepare();
    prepared.validate().unwrap();
    assert_eq!(
        prepared.expected_commit_oid,
        "b37b5c86a5266a1796a58ad44feeb29d58a04af8"
    );
}

#[test]
fn sha256_commit_oid_matches_golden() {
    let mut prepared = prepare();
    prepared.object_format = ObjectFormat::Sha256;
    prepared.base_commit = DIGEST.to_string();
    prepared.target_ref_at_base = DIGEST.to_string();
    prepared.base_tree =
        "89abcdef0123456789abcdef0123456789abcdef0123456789abcdef01234567".to_string();
    prepared.candidate_tree = DIGEST.to_string();
    prepared.expected_commit_oid = prepared.computed_commit_oid();
    assert_eq!(
        prepared.expected_commit_oid,
        "403780ae5a0385d647759f4f069d6b688c0864dda47b89c62627188e588287b8"
    );
    prepared.validate().unwrap();
}

#[test]
fn commit_prepare_rejects_identical_trees() {
    let mut p = prepare();
    p.candidate_tree = p.base_tree.clone();
    assert!(p.validate().is_err());
}

#[test]
fn commit_prepare_rejects_relative_message_and_newline_reason() {
    let mut p = prepare();
    p.message_path = "message.txt".to_string();
    assert!(p.validate().is_err());
    let mut q = prepare();
    q.reflog_reason = "two\nlines".to_string();
    assert!(q.validate().is_err());
    let mut control = prepare();
    control.author_name = "bad\rname".to_string();
    assert!(control.validate().is_err());
    let mut normalized_name = prepare();
    normalized_name.author_name = " Manav Hirey".to_string();
    assert!(normalized_name.validate().is_err());
    let mut normalized_date = prepare();
    normalized_date.author_date = "01789099003 -0400".to_string();
    assert!(normalized_date.validate().is_err());
    let mut normalized_reason = prepare();
    normalized_reason.reflog_reason = "authorized  integration".to_string();
    assert!(normalized_reason.validate().is_err());
}

#[test]
fn commit_prepare_rejects_message_or_target_drift() {
    let mut message = prepare();
    message.message = "Implement schema tranche four\n".to_string();
    let error = message.validate().unwrap_err();
    assert!(error.contains("message_sha256"), "{error}");

    let mut target = prepare();
    target.target_ref_at_base = TREE.to_string();
    assert!(target.validate().is_err());
}

#[test]
fn exact_authorization_passes() {
    prepare().authorize(&authorization()).unwrap();
}

#[test]
fn authorization_must_name_every_value() {
    let mut a = authorization();
    a.patch_sha256 = "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210".to_string();
    let err = prepare().authorize(&a).unwrap_err();
    assert!(err.contains("patch hash"), "{err}");
    let mut b = authorization();
    b.committer_date = "1750001000 +0000".to_string();
    assert!(prepare().authorize(&b).is_err());
    let mut c = authorization();
    c.expected_commit_oid = OID.to_string();
    assert!(prepare().authorize(&c).is_err());
    let mut d = authorization();
    d.target_ref = "refs/heads/other".to_string();
    assert!(prepare().authorize(&d).is_err());
    let mut identity = authorization();
    identity.author_name = "Different Author".to_string();
    assert!(prepare().authorize(&identity).is_err());
}

#[test]
fn authorization_tuple_rejects_bad_forms() {
    let mut a = authorization();
    a.author_date = "1789099003 -0000".to_string();
    assert!(a.validate(ObjectFormat::Sha1).is_err());
    let mut b = authorization();
    b.target_state = "0123".to_string();
    assert!(b.validate(ObjectFormat::Sha1).is_err());
    let mut normalized_reason = authorization();
    normalized_reason.reflog_reason = "authorized  integration".to_string();
    assert!(normalized_reason.validate(ObjectFormat::Sha1).is_err());
}

#[test]
fn authorization_does_not_accept_matching_invalid_values() {
    let mut p = prepare();
    let mut a = authorization();
    p.author_date = "not-a-date".to_string();
    a.author_date = "not-a-date".to_string();
    assert!(p.authorize(&a).is_err());
}

#[test]
fn commit_records_require_canonical_json() {
    let record = prepare();
    let canonical = record.to_canonical_json().unwrap();
    assert_eq!(
        CommitPrepare::from_canonical_json(canonical.as_bytes()).unwrap(),
        record
    );
    assert_eq!(
        sha256_digest(canonical.as_bytes()),
        "d9bfdc2a0f259119f8da1f7853e9de50da80df349bad3d8a2e85c00e1186c64c"
    );
    let pretty = serde_json::to_string_pretty(&record).unwrap();
    assert!(CommitPrepare::from_canonical_json(pretty.as_bytes()).is_err());

    let duplicate = canonical.replacen('{', &format!("{{\"schema\":\"{COMMIT_PREPARE_TAG}\","), 1);
    assert!(CommitPrepare::from_canonical_json(duplicate.as_bytes()).is_err());
    let oversized = vec![b' '; sp_schema::core::REGISTERED_RECORD_MAX_BYTES + 1];
    assert!(CommitPrepare::from_canonical_json(&oversized).is_err());

    let authorization = authorization();
    let canonical = authorization.to_canonical_json(ObjectFormat::Sha1).unwrap();
    assert_eq!(
        canonical,
        r#"{"author_date":"1789099003 -0400","author_email":"manavhirey@example.com","author_name":"Manav Hirey","base_commit":"0123456789abcdef0123456789abcdef01234567","candidate_tree":"0123456789abcdef0123456789abcdef01234567","committer_date":"1789099003 -0400","committer_email":"manavhirey@example.com","committer_name":"Manav Hirey","expected_commit_oid":"b37b5c86a5266a1796a58ad44feeb29d58a04af8","message_sha256":"db01ba114fc4d810bd64a1b663a80cda3b1e906a808d0d0e6feb866d8ddd32b0","object_format":"sha1","patch_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","reflog_reason":"authorized integration of T001","schema":"commit-authorization-v1","target_ref":"refs/heads/main","target_state":"0123456789abcdef0123456789abcdef01234567"}"#
    );
    assert_eq!(
        sha256_digest(canonical.as_bytes()),
        "054838a818b2c989eb01a3bf2bf1dd232e5b7cb782c40ab8ab1703e4ae1dcebc"
    );
    assert_eq!(
        AuthorizationTuple::from_canonical_json(canonical.as_bytes(), ObjectFormat::Sha1).unwrap(),
        authorization
    );
}

// ------------------------------------------------------------- URL grammar

#[test]
fn push_url_accepts_canonical_https() {
    validate_push_url("https://github.com/manavhirey/superplanner").unwrap();
    validate_push_url("https://example.com:8443/path").unwrap();
}

#[test]
fn push_url_rejects_adversarial_forms() {
    assert!(validate_push_url("git@github.com:manavhirey/superplanner").is_err());
    assert!(validate_push_url("ssh://git@github.com/x").is_err());
    assert!(validate_push_url("https://user:pass@example.com/").is_err());
    assert!(validate_push_url("https://example.com/path?x=1").is_err());
    assert!(validate_push_url("https://example.com/path#frag").is_err());
    assert!(validate_push_url("https://[2001:db8::1]/").is_err());
    assert!(validate_push_url("https://example.com:0/repository").is_err());
    assert!(validate_push_url("https://example.com:65536/repository").is_err());
    assert!(validate_push_url("http://example.com/").is_err());
    assert!(validate_push_url("https://exa mple.com/").is_err());
    assert!(validate_push_url("https://-bad.example.com/").is_err());
    assert!(validate_push_url("https://example.com/").is_err());
}

#[test]
fn push_url_accepts_single_label_dns_host() {
    validate_push_url("https://localhost/repository").unwrap();
}

#[test]
fn refspec_rejects_force_forms() {
    validate_push_refspec("refs/heads/main:refs/heads/main").unwrap();
    assert!(validate_push_refspec("+refs/heads/main:refs/heads/main").is_err());
    assert!(validate_push_refspec("refs/heads/main").is_err());
    assert!(validate_push_refspec("main:refs/heads/main").is_err());
}

// ------------------------------------------------------------ TLV codec

#[test]
fn tlv_round_trips_scalars_and_collections() {
    let bytes = encode(
        "push-test-v1",
        &[
            Field::scalar_str("alpha"),
            Field::Collection(vec![
                vec![Field::scalar_str("r1"), Field::scalar_str("d1")],
                vec![Field::scalar_str("r2"), Field::scalar_str("d2")],
            ]),
            Field::Collection(vec![]),
        ],
    )
    .unwrap();
    let decoded = decode(&bytes).unwrap();
    assert_eq!(decoded.tag, "push-test-v1");
    assert_eq!(decoded.fields[0].as_scalar_str().unwrap(), "alpha");
    let items = decoded.fields[1].as_collection().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0][1].as_scalar_str().unwrap(), "d1");
    assert_eq!(decoded.fields[2].as_collection().unwrap().len(), 0);
    assert_eq!(encode(&decoded.tag, &decoded.fields).unwrap(), bytes);
}

#[test]
fn tlv_rejects_truncated_and_trailing() {
    let bytes = encode("push-test-v1", &[Field::scalar_str("x")]).unwrap();
    assert!(decode(&bytes[..bytes.len() - 1]).is_err());
    let mut trailing = bytes.clone();
    trailing.push(0xff);
    assert!(decode(&trailing).is_err());
    assert!(decode(&[]).is_err());
}

#[test]
fn tlv_matches_canonical_golden_bytes() {
    let bytes = encode("x-v1", &[Field::scalar_str("a")]).unwrap();
    let mut expected = vec![0x00];
    expected.extend_from_slice(&4_u64.to_be_bytes());
    expected.extend_from_slice(b"x-v1");
    expected.extend_from_slice(&1_u64.to_be_bytes());
    expected.push(0x00);
    expected.extend_from_slice(&1_u64.to_be_bytes());
    expected.push(b'a');
    assert_eq!(bytes, expected);
}

#[test]
fn tlv_rejects_noncanonical_tags_nested_collections_and_huge_counts() {
    for tag in ["x", "x-v0", "x-v01", "X-v1", "x_thing-v1"] {
        assert!(encode(tag, &[]).is_err(), "accepted {tag:?}");
    }
    let nested = Field::Collection(vec![vec![Field::Collection(vec![])]]);
    assert!(encode("x-v1", &[nested]).is_err());

    let mut huge = encode("x-v1", &[]).unwrap();
    let count_offset = 1 + 8 + "x-v1".len();
    huge[count_offset..count_offset + 8].copy_from_slice(&u64::MAX.to_be_bytes());
    assert!(decode(&huge).is_err());

    let oversized = vec![0; sp_schema::core::REGISTERED_RECORD_MAX_BYTES + 1];
    assert!(decode(&oversized).is_err());

    let mut huge_items = encode("x-v1", &[Field::Collection(vec![])]).unwrap();
    let field_offset = 1 + 8 + "x-v1".len() + 8;
    huge_items[field_offset + 1..field_offset + 9].copy_from_slice(&u64::MAX.to_be_bytes());
    assert!(decode(&huge_items).is_err());

    let mut huge_scalars = encode("x-v1", &[Field::Collection(vec![vec![]])]).unwrap();
    let scalar_count_offset = field_offset + 1 + 8;
    huge_scalars[scalar_count_offset..scalar_count_offset + 8]
        .copy_from_slice(&u64::MAX.to_be_bytes());
    assert!(decode(&huge_scalars).is_err());
}

// --------------------------------------------- push custody records

fn request() -> PushPresentationRequest {
    PushPresentationRequest {
        user_decision_record_id: "sp-record-0123456789abcdef0123456789abcdef".to_string(),
        user_decision_record_sha256: DIGEST.to_string(),
        commit_result_id: "sp-record-0123456789abcdef0123456789abcde1".to_string(),
        commit_result_sha256: DIGEST.to_string(),
        commit_full_oid: prepare().expected_commit_oid,
        standard_review_id: "sp-record-0123456789abcdef0123456789abcde2".to_string(),
        standard_review_identity: registry_identity("/registry/standard-review", 1),
        standard_review_sha256: DIGEST.to_string(),
        adversarial_review_id: "sp-record-0123456789abcdef0123456789abcde3".to_string(),
        adversarial_review_identity: registry_identity("/registry/adversarial-review", 2),
        adversarial_review_sha256: DIGEST.to_string(),
        destination_snapshot_id: "sp-record-0123456789abcdef0123456789abcde4".to_string(),
        destination_snapshot_sha256: DIGEST.to_string(),
        remote_name: "origin".to_string(),
        literal_url: "https://github.com/manavhirey/superplanner".to_string(),
        source_ref: "refs/heads/main".to_string(),
        destination_ref: "refs/heads/main".to_string(),
        refspec: "refs/heads/main:refs/heads/main".to_string(),
        no_force_policy: "no-force".to_string(),
    }
}

#[test]
fn presentation_request_round_trips() {
    let bytes = request().encode().unwrap();
    assert_eq!(
        sha256_digest(&bytes),
        "9768c8824cf1f5b12d64e2f5154845ea5976f42831dd07ce8e641393c37fef4a"
    );
    let decoded = PushPresentationRequest::decode(&bytes).unwrap();
    assert_eq!(decoded, request());
}

#[test]
fn presentation_request_rejects_noncanonical_frames() {
    let bytes = request().encode().unwrap();

    let mut wrong_tag = bytes.clone();
    wrong_tag[9] = b'x';
    assert!(PushPresentationRequest::decode(&wrong_tag).is_err());

    assert!(PushPresentationRequest::decode(&bytes[..bytes.len() - 1]).is_err());
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(PushPresentationRequest::decode(&trailing).is_err());

    let mut wrong_field_kind = bytes;
    let first_field = 1 + 8 + PUSH_PRESENTATION_REQUEST_TAG.len() + 8;
    wrong_field_kind[first_field] = 0x01;
    assert!(PushPresentationRequest::decode(&wrong_field_kind).is_err());
}

#[test]
fn presentation_request_rejects_ssh_url_and_force() {
    let mut r = request();
    r.literal_url = "git@github.com:manavhirey/superplanner".to_string();
    assert!(r.validate().is_err());
    let mut f = request();
    f.refspec = "+refs/heads/main:refs/heads/main".to_string();
    assert!(f.validate().is_err());
    let mut p = request();
    p.no_force_policy = "force".to_string();
    assert!(p.validate().is_err());
    let mut mismatch = request();
    mismatch.refspec = "refs/heads/other:refs/heads/main".to_string();
    assert!(mismatch.validate().is_err());
    let mut noncanonical_identity = request();
    noncanonical_identity.standard_review_identity =
        serde_json::to_string_pretty(&RegistryIdentity {
            path: "/registry/standard-review".to_string(),
            device: 1,
            inode: 1,
            link_count: 1,
            sha256: DIGEST.to_string(),
        })
        .unwrap();
    assert!(noncanonical_identity.validate().is_err());
}

#[test]
fn request_binds_authorization_oid() {
    bind_request_to_authorization(&request(), &authorization(), ObjectFormat::Sha1).unwrap();
    let mut a = authorization();
    a.expected_commit_oid = "fedcba9876543210fedcba9876543210fedcba98".to_string();
    assert!(bind_request_to_authorization(&request(), &a, ObjectFormat::Sha1).is_err());
    let mut ref_mismatch = request();
    ref_mismatch.source_ref = "refs/heads/other".to_string();
    ref_mismatch.refspec = "refs/heads/other:refs/heads/main".to_string();
    assert!(
        bind_request_to_authorization(&ref_mismatch, &authorization(), ObjectFormat::Sha1).is_err()
    );
    let mut invalid_request = request();
    invalid_request.no_force_policy = "force".to_string();
    assert!(
        bind_request_to_authorization(&invalid_request, &authorization(), ObjectFormat::Sha1)
            .is_err()
    );
}

fn eligibility() -> PushEligibility {
    let mut next_id = 1_u128;
    let mut input = |kind: &str| {
        let id = format!("sp-record-{next_id:032x}");
        let identity = registry_identity(&format!("/registry/{kind}"), next_id as u64);
        next_id += 1;
        ResolvedInput {
            record_type: kind.to_string(),
            record_id: id,
            record_identity: identity,
            record_sha256: DIGEST.to_string(),
        }
    };
    PushEligibility {
        resolved_inputs: vec![
            input("source-context"),
            input("object-route"),
            input("authorization"),
            input("commit-result"),
            input("standard-review"),
            input("adversarial-review"),
            input("presentation-request"),
            input("verification"),
            input("documentation"),
            input("destination-snapshot"),
        ],
        snapshot_id: "sp-record-0000000000000000000000000000000b".to_string(),
        snapshot_identity: registry_identity("/registry/snap", 11),
        snapshot_sha256: DIGEST.to_string(),
        approved_full_sha: OID.to_string(),
        source_ref: "refs/heads/main".to_string(),
        remote_name: "origin".to_string(),
        literal_url: "https://github.com/manavhirey/superplanner".to_string(),
        refspec: "refs/heads/main:refs/heads/main".to_string(),
        object_ref_format: ObjectFormat::Sha1,
        no_force_policy: "no-force".to_string(),
        layout_record_id: "sp-record-0000000000000000000000000000000c".to_string(),
        layout_record_identity: registry_identity("/registry/layout", 12),
        layout_record_sha256: DIGEST.to_string(),
        closure_record_id: "sp-record-0000000000000000000000000000000d".to_string(),
        closure_record_identity: registry_identity("/registry/closure", 13),
        closure_record_sha256: DIGEST.to_string(),
        command_record_id: "sp-record-0000000000000000000000000000000e".to_string(),
        command_record_identity: registry_identity("/registry/command", 14),
        command_record_sha256: DIGEST.to_string(),
        custody_root_identity: filesystem_identity("/custody/root", 99),
        lease_id: "sp-lease-00000000000000000000000000000001".to_string(),
    }
}

#[test]
fn eligibility_round_trips_and_requires_all_inputs() {
    let bytes = eligibility().encode().unwrap();
    assert_eq!(
        sha256_digest(&bytes),
        "6621436566fb6d070c395c0793ca772b9574dec5c621545367e102a14cd94d81"
    );
    let decoded = PushEligibility::decode(&bytes).unwrap();
    assert_eq!(decoded, eligibility());

    let mut missing = eligibility();
    missing.resolved_inputs.truncate(9);
    assert!(missing.validate().is_err());

    let mut unordered = eligibility();
    unordered.resolved_inputs.swap(0, 1);
    assert!(unordered.validate().is_err());

    let mut duplicate_id = eligibility();
    duplicate_id.resolved_inputs[1].record_id = duplicate_id.resolved_inputs[0].record_id.clone();
    assert!(duplicate_id.validate().is_err());
}

#[test]
fn eligibility_rejects_wrong_collection_shape() {
    let mut bytes = eligibility().encode().unwrap();
    let first_field = 1 + 8 + sp_schema::push_custody::PUSH_ELIGIBILITY_TAG.len() + 8;
    bytes[first_field] = 0x00;
    assert!(PushEligibility::decode(&bytes).is_err());
}

#[test]
fn eligibility_rejects_bad_sha_width_and_policy() {
    let mut e = eligibility();
    e.approved_full_sha = "0123".to_string();
    assert!(e.validate().is_err());
    let mut p = eligibility();
    p.no_force_policy = "allow-force".to_string();
    assert!(p.validate().is_err());
    let mut bytes = eligibility().encode().unwrap();
    let offset = bytes
        .windows(b"sha1/files".len())
        .position(|window| window == b"sha1/files")
        .unwrap();
    bytes[offset..offset + b"sha1/files".len()].copy_from_slice(b"bad!/files");
    assert!(PushEligibility::decode(&bytes).is_err());
    let mut lease = eligibility();
    lease.lease_id = "lease-0001".to_string();
    assert!(lease.validate().is_err());
}
