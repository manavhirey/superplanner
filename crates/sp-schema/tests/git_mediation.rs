use sp_schema::git_mediation::{
    parse_checksum_line, parse_owned_z, parse_raw_manifest, NamespaceManifest, NamespaceRecord,
    ObjectFormat, RefValue, FILES_REF_NAMESPACE_TAG,
};

fn sha1() -> ObjectFormat {
    ObjectFormat::Sha1
}

const OID: &str = "0123456789abcdef0123456789abcdef01234567";
const ZERO: &str = "0000000000000000000000000000000000000000";
const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn owned() -> Vec<Vec<u8>> {
    vec![
        b"src/new.rs".to_vec(),
        b"src/gone.rs".to_vec(),
        b"src/mod.rs".to_vec(),
        b"link".to_vec(),
    ]
}

// --------------------------------------------------------------- owned.z

fn owned_z(paths: &[&str]) -> Vec<u8> {
    let mut out = Vec::new();
    for p in paths {
        out.extend_from_slice(p.as_bytes());
        out.push(0);
    }
    out
}

#[test]
fn owned_z_parses() {
    let paths = parse_owned_z(&owned_z(&["src/a.rs", "docs/b.md"])).unwrap();
    assert_eq!(paths, vec![b"src/a.rs".to_vec(), b"docs/b.md".to_vec()]);
}

#[test]
fn owned_z_requires_terminal_nul() {
    let mut bytes = owned_z(&["a"]);
    bytes.pop();
    assert!(parse_owned_z(&bytes).is_err());
}

#[test]
fn owned_z_rejects_absolute_and_magic() {
    assert!(parse_owned_z(&owned_z(&["/etc/passwd"])).is_err());
    assert!(parse_owned_z(&owned_z(&["src/*.rs"])).is_err());
    assert!(parse_owned_z(&owned_z(&[":(top)file"])).is_err());
    assert!(parse_owned_z(&owned_z(&["dir/"])).is_err());
    assert!(parse_owned_z(&owned_z(&["a/../b"])).is_err());
    assert!(parse_owned_z(&owned_z(&["a/./b"])).is_err());
}

#[test]
fn owned_z_preserves_valid_raw_path_bytes() {
    let bytes = b"path with space\0line\nfeed\0nonutf8-\xff\0";
    let paths = parse_owned_z(bytes).unwrap();
    assert_eq!(paths[0], b"path with space");
    assert_eq!(paths[1], b"line\nfeed");
    assert_eq!(paths[2], b"nonutf8-\xff");
}

#[test]
fn owned_z_rejects_duplicates_and_collisions() {
    assert!(parse_owned_z(&owned_z(&["a", "a"])).is_err());
    assert!(parse_owned_z(&owned_z(&["a/b", "a"])).is_err());
    assert!(parse_owned_z(&owned_z(&["A", "a"])).is_err());
    assert!(parse_owned_z(&owned_z(&["A", "a/b"])).is_err());
    assert!(parse_owned_z(&owned_z(&["cafe\u{301}", "caf\u{e9}"])).is_err());
}

// ---------------------------------------------------------- raw manifest

fn record(header: &str, path: &str) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(header.as_bytes());
    out.push(0);
    out.extend_from_slice(path.as_bytes());
    out.push(0);
    out
}

#[test]
fn raw_manifest_accepts_add_delete_modify_typechange() {
    let mut bytes = record(&format!(":000000 100644 {ZERO} {OID} A"), "src/new.rs");
    bytes.extend(record(
        &format!(":100644 000000 {OID} {ZERO} D"),
        "src/gone.rs",
    ));
    let other = "89abcdef0123456789abcdef0123456789abcdef";
    bytes.extend(record(
        &format!(":100644 100755 {OID} {other} M"),
        "src/mod.rs",
    ));
    bytes.extend(record(&format!(":100644 120000 {OID} {other} T"), "link"));
    let records = parse_raw_manifest(&bytes, sha1(), &owned()).unwrap();
    assert_eq!(records.len(), 4);
    assert_eq!(records[0].status, 'A');
    assert_eq!(records[3].old_mode, "100644");
    assert_eq!(records[3].new_mode, "120000");
}

#[test]
fn raw_manifest_rejects_gitlink() {
    let bytes = record(&format!(":160000 160000 {OID} {OID} M"), "sub");
    assert!(parse_raw_manifest(&bytes, sha1(), &owned())
        .unwrap_err()
        .contains("160000 gitlinks"));
}

#[test]
fn raw_manifest_rejects_unowned_and_duplicate_paths() {
    let bytes = record(&format!(":000000 100644 {ZERO} {OID} A"), "src/unowned.rs");
    assert!(parse_raw_manifest(&bytes, sha1(), &owned()).is_err());

    let mut dup = record(&format!(":000000 100644 {ZERO} {OID} A"), "src/new.rs");
    dup.extend(record(
        &format!(":100644 100755 {OID} {OID} M"),
        "src/new.rs",
    ));
    assert!(parse_raw_manifest(&dup, sha1(), &owned()).is_err());
}

#[test]
fn raw_manifest_enforces_status_sides() {
    // A with nonzero old side
    assert!(parse_raw_manifest(
        &record(&format!(":100644 100644 {OID} {OID} A"), "src/new.rs"),
        sha1(),
        &owned()
    )
    .is_err());
    // D with nonzero new side
    assert!(parse_raw_manifest(
        &record(&format!(":100644 100644 {OID} {OID} D"), "src/gone.rs"),
        sha1(),
        &owned()
    )
    .is_err());
    // M with identical sides
    assert!(parse_raw_manifest(
        &record(&format!(":100644 100644 {OID} {OID} M"), "src/mod.rs"),
        sha1(),
        &owned()
    )
    .is_err());
    // T without a type change
    assert!(parse_raw_manifest(
        &record(&format!(":100644 100755 {OID} {OID} T"), "src/mod.rs"),
        sha1(),
        &owned()
    )
    .is_err());
    // zero mode with nonzero OID
    assert!(parse_raw_manifest(
        &record(&format!(":000000 100644 {OID} {OID} A"), "src/new.rs"),
        sha1(),
        &owned()
    )
    .is_err());
}

#[test]
fn raw_manifest_rejects_trailing_partial_and_bad_width() {
    let mut bytes = record(&format!(":000000 100644 {ZERO} {OID} A"), "src/new.rs");
    bytes.extend(b":000000 100644");
    bytes.push(0);
    assert!(parse_raw_manifest(&bytes, sha1(), &owned()).is_err());

    let short = "0123";
    assert!(parse_raw_manifest(
        &record(&format!(":000000 100644 {ZERO} {short} A"), "src/new.rs"),
        sha1(),
        &owned()
    )
    .is_err());
}

#[test]
fn raw_manifest_rejects_unknown_status_and_header_shape() {
    assert!(parse_raw_manifest(
        &record(&format!(":100644 100644 {OID} {OID} R"), "src/mod.rs"),
        sha1(),
        &owned()
    )
    .is_err());
    assert!(parse_raw_manifest(
        &record(&format!("100644 100644 {OID} {OID} M"), "src/mod.rs"),
        sha1(),
        &owned()
    )
    .is_err());
    assert!(parse_raw_manifest(
        &record(&format!(":100644 100644 {OID} {OID}"), "src/mod.rs"),
        sha1(),
        &owned()
    )
    .is_err());
}

#[test]
fn raw_manifest_supports_sha256_and_non_utf8_paths() {
    let oid = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let zero = "0".repeat(64);
    let path = b"nonutf8-\xff".to_vec();
    let mut bytes = format!(":000000 100644 {zero} {oid} A").into_bytes();
    bytes.push(0);
    bytes.extend_from_slice(&path);
    bytes.push(0);
    let records =
        parse_raw_manifest(&bytes, ObjectFormat::Sha256, std::slice::from_ref(&path)).unwrap();
    assert_eq!(records[0].path, path);
}

#[test]
fn checksum_line_requires_digest_form_and_path() {
    let line = format!("{DIGEST}  /tmp/output");
    assert_eq!(
        parse_checksum_line(&line).unwrap(),
        (DIGEST.to_string(), "/tmp/output".to_string())
    );
    assert!(parse_checksum_line(&format!("sha256:{DIGEST}  /tmp/output")).is_err());
    assert!(parse_checksum_line(&format!("{DIGEST}  ")).is_err());
}

#[test]
fn ref_names_reject_forbidden_bracket_and_internal_lock_suffix() {
    assert!(sp_schema::validate_full_ref_name("refs/heads/a[b").is_err());
    assert!(sp_schema::validate_full_ref_name("refs/heads/a.lock/b").is_err());
}

// ----------------------------------------------- files-ref-namespace-v1

fn sample_namespace() -> NamespaceManifest {
    NamespaceManifest {
        format: ObjectFormat::Sha1,
        head: NamespaceRecord {
            name: b"HEAD".to_vec(),
            value: RefValue::Symbolic(b"refs/heads/main".to_vec()),
        },
        refs: vec![
            NamespaceRecord {
                name: b"refs/heads/main".to_vec(),
                value: RefValue::Direct("0123456789abcdef0123456789abcdef01234567".to_string()),
            },
            NamespaceRecord {
                name: b"refs/tags/v1".to_vec(),
                value: RefValue::Direct("89abcdef0123456789abcdef0123456789abcdef".to_string()),
            },
        ],
    }
}

fn encode_namespace_unchecked(manifest: &NamespaceManifest) -> Vec<u8> {
    let format = match manifest.format {
        ObjectFormat::Sha1 => "sha1",
        ObjectFormat::Sha256 => "sha256",
    };
    let mut out = Vec::new();
    for value in [
        FILES_REF_NAMESPACE_TAG,
        format,
        &(manifest.refs.len() + 1).to_string(),
    ] {
        out.extend_from_slice(value.as_bytes());
        out.push(0);
    }
    for record in std::iter::once(&manifest.head).chain(&manifest.refs) {
        out.extend_from_slice(&record.name);
        out.push(0);
        match &record.value {
            RefValue::Direct(value) => {
                out.extend_from_slice(b"direct\0");
                out.extend_from_slice(value.as_bytes());
            }
            RefValue::Symbolic(value) => {
                out.extend_from_slice(b"symbolic\0");
                out.extend_from_slice(value);
            }
        }
        out.push(0);
    }
    out
}

#[test]
fn namespace_round_trips_byte_exactly() {
    let manifest = sample_namespace();
    let encoded = manifest.encode().unwrap();
    let decoded = NamespaceManifest::decode(&encoded).unwrap();
    assert_eq!(decoded, manifest);
    assert_eq!(encoded.last(), Some(&0));
    assert_ne!(encoded.get(encoded.len() - 2), Some(&0));
    assert_eq!(
        sp_schema::sha256_digest(&encoded),
        "6ca7ca18372dc0fb41755812d67ddc4cc3ef66520fb4fab12351b731aeb54606"
    );
    // Round-trip stability: encoding the decode yields identical bytes.
    assert_eq!(decoded.encode().unwrap(), encoded);
}

#[test]
fn namespace_sha256_matches_golden_bytes() {
    let mut manifest = sample_namespace();
    manifest.format = ObjectFormat::Sha256;
    manifest.refs[0].value = RefValue::Direct(DIGEST.to_string());
    manifest.refs[1].value = RefValue::Direct(
        "89abcdef89abcdef89abcdef89abcdef89abcdef89abcdef89abcdef89abcdef".to_string(),
    );
    let encoded = manifest.encode().unwrap();
    assert_eq!(
        sp_schema::sha256_digest(&encoded),
        "174545a2413e5bf1f7620b6ed647ea87ab97e189d2b3e7d96a2d1dfdd362e075"
    );
    assert_eq!(NamespaceManifest::decode(&encoded).unwrap(), manifest);
}

#[test]
fn namespace_rejects_wrong_tag_and_bad_count() {
    let mut bytes = sample_namespace().encode().unwrap();
    // Corrupt the tag.
    bytes[0] = b'x';
    assert!(NamespaceManifest::decode(&bytes).is_err());

    let mut noncanonical = sample_namespace().encode().unwrap();
    let count_offset = FILES_REF_NAMESPACE_TAG.len() + 1 + "sha1".len() + 1;
    noncanonical.insert(count_offset, b'0');
    assert!(NamespaceManifest::decode(&noncanonical)
        .unwrap_err()
        .contains("noncanonical record count"));
}

#[test]
fn namespace_rejects_unordered_and_duplicate_names() {
    let mut manifest = sample_namespace();
    manifest.refs = vec![
        NamespaceRecord {
            name: b"refs/tags/v1".to_vec(),
            value: RefValue::Direct("0123456789abcdef0123456789abcdef01234567".to_string()),
        },
        NamespaceRecord {
            name: b"refs/heads/main".to_vec(),
            value: RefValue::Direct("89abcdef0123456789abcdef0123456789abcdef".to_string()),
        },
    ];
    let bytes = encode_namespace_unchecked(&manifest);
    assert!(NamespaceManifest::decode(&bytes)
        .unwrap_err()
        .contains("ordered by raw name bytes"));

    let mut dup = sample_namespace();
    dup.refs.push(NamespaceRecord {
        name: b"refs/heads/main".to_vec(),
        value: RefValue::Direct("89abcdef0123456789abcdef0123456789abcdef".to_string()),
    });
    assert!(NamespaceManifest::decode(&encode_namespace_unchecked(&dup))
        .unwrap_err()
        .contains("duplicate ref name"));
    assert!(dup.encode().is_err());
}

#[test]
fn namespace_rejects_prefix_conflicts_and_trailing_data() {
    let mut conflicting = sample_namespace();
    conflicting.refs.insert(
        1,
        NamespaceRecord {
            name: b"refs/heads/main/extra".to_vec(),
            value: RefValue::Direct("89abcdef0123456789abcdef0123456789abcdef".to_string()),
        },
    );
    let bytes = encode_namespace_unchecked(&conflicting);
    assert!(NamespaceManifest::decode(&bytes)
        .unwrap_err()
        .contains("path-prefix"));

    let mut trailing = sample_namespace().encode().unwrap();
    trailing.extend_from_slice(b"x");
    assert!(NamespaceManifest::decode(&trailing).is_err());

    let mut missing_terminal = sample_namespace().encode().unwrap();
    missing_terminal.pop();
    assert!(NamespaceManifest::decode(&missing_terminal).is_err());
}

#[test]
fn namespace_rejects_zero_direct_oid() {
    let mut manifest = sample_namespace();
    manifest.refs[0].value = RefValue::Direct(ZERO.to_string());
    assert!(NamespaceManifest::decode(&encode_namespace_unchecked(&manifest)).is_err());
    assert!(manifest.encode().is_err());
}

#[test]
fn namespace_rejects_oversized_input() {
    let bytes = vec![0; sp_schema::core::REGISTERED_RECORD_MAX_BYTES + 1];
    assert!(NamespaceManifest::decode(&bytes).is_err());
}

#[test]
fn namespace_preserves_dangling_and_chained_symrefs() {
    let mut chained = sample_namespace();
    chained.refs.insert(
        0,
        NamespaceRecord {
            name: b"refs/heads/loop".to_vec(),
            value: RefValue::Symbolic(b"refs/heads/missing".to_vec()),
        },
    );
    let bytes = chained.encode().unwrap();
    let decoded = NamespaceManifest::decode(&bytes).unwrap();
    assert_eq!(
        decoded.refs[0].value,
        RefValue::Symbolic(b"refs/heads/missing".to_vec())
    );
}

#[test]
fn namespace_preserves_non_utf8_ref_bytes() {
    let mut manifest = sample_namespace();
    manifest.refs.insert(
        1,
        NamespaceRecord {
            name: b"refs/heads/nonutf8-\xff".to_vec(),
            value: RefValue::Symbolic(b"refs/heads/target-\xfe".to_vec()),
        },
    );
    let bytes = manifest.encode().unwrap();
    assert_eq!(NamespaceManifest::decode(&bytes).unwrap(), manifest);
}
