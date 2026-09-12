//! Git-mediation record grammars per `references/integration-protocol.md`:
//! `owned.z` ownership paths, the raw diff manifest, and the canonical
//! `files-ref-namespace-v1` namespace manifest. All parsing is byte-exact
//! and fails closed.

use crate::core::{validate_digest_form, SizeLimit};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use unicode_casefold::UnicodeCaseFold;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectFormat {
    Sha1,
    Sha256,
}

impl ObjectFormat {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "sha1" => Ok(ObjectFormat::Sha1),
            "sha256" => Ok(ObjectFormat::Sha256),
            other => Err(format!(
                "object_format must be sha1 or sha256, not {other:?}"
            )),
        }
    }

    pub fn oid_width(self) -> usize {
        match self {
            ObjectFormat::Sha1 => 40,
            ObjectFormat::Sha256 => 64,
        }
    }
}

fn is_lowercase_hex_of_width(value: &str, width: usize) -> bool {
    value.len() == width
        && value
            .bytes()
            .all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'))
}

fn is_all_zero_hex_of_width(value: &str, width: usize) -> bool {
    value.len() == width && value.bytes().all(|b| b == b'0')
}

// ---------------------------------------------------------------------------
// owned.z
// ---------------------------------------------------------------------------

/// Validate an `owned.z` byte stream: NUL-delimited repository-relative leaf
/// paths, every record non-empty and NUL-terminated, exactly one terminal
/// NUL at EOF. Rejects absolute paths, globs, pathspec magic, trailing
/// slashes, empty records, `.`/`..` components, duplicates,
/// ancestor/file collisions, and case-folded or Unicode-normalization
/// collisions.
pub fn parse_owned_z(bytes: &[u8]) -> Result<Vec<Vec<u8>>, String> {
    SizeLimit::OperationOutputLeaf.check(bytes.len())?;
    if bytes.is_empty() {
        return Err("owned.z must not be empty".to_string());
    }
    if bytes.last() != Some(&0) {
        return Err("owned.z must end with a terminal NUL".to_string());
    }
    let records: Vec<&[u8]> = bytes[..bytes.len() - 1].split(|b| *b == 0).collect();
    // bytes = r1 NUL r2 NUL ... rN NUL => after removing the final NUL and
    // splitting, empty trailing items correspond to interior "NUL NUL" runs.
    if records.iter().any(|r| r.is_empty()) {
        return Err("owned.z must not contain empty records".to_string());
    }
    validate_owned_records(&records)
}

fn validate_owned_records(records: &[&[u8]]) -> Result<Vec<Vec<u8>>, String> {
    let mut exact = BTreeSet::new();
    let mut normalized = BTreeMap::new();
    let mut validated = Vec::with_capacity(records.len());
    for record in records {
        validate_owned_path(record)?;
        if !exact.insert(record.to_vec()) {
            return Err(format!(
                "owned.z duplicate path: {:?}",
                String::from_utf8_lossy(record)
            ));
        }
        let record_key = normalized_case_fold(record);
        if let Some(prior) = normalized.insert(record_key, record.to_vec()) {
            return Err(format!(
                "owned.z case-folded or normalized collision: {:?} vs {:?}",
                String::from_utf8_lossy(&prior),
                String::from_utf8_lossy(record)
            ));
        }
        validated.push(record.to_vec());
    }
    for (path, original) in &normalized {
        for slash in path
            .iter()
            .enumerate()
            .filter_map(|(index, byte)| (*byte == b'/').then_some(index))
        {
            if let Some(parent) = normalized.get(&path[..slash]) {
                return Err(format!(
                    "owned.z ancestor/file collision: {:?} vs {:?}",
                    String::from_utf8_lossy(parent),
                    String::from_utf8_lossy(original)
                ));
            }
        }
    }
    Ok(validated)
}

fn validate_owned_path(path: &[u8]) -> Result<(), String> {
    let display = String::from_utf8_lossy(path);
    if path.is_empty() {
        return Err("owned.z path must not be empty".to_string());
    }
    if path.starts_with(b"/") {
        return Err(format!("owned.z path must be relative: {display:?}"));
    }
    if path.ends_with(b"/") {
        return Err(format!(
            "owned.z path must be a leaf, not a directory: {display:?}"
        ));
    }
    if path.iter().any(|b| matches!(b, b'*' | b'?' | b'[')) {
        return Err(format!("owned.z path carries glob bytes: {display:?}"));
    }
    if path.starts_with(b":") {
        return Err(format!("owned.z path carries pathspec magic: {display:?}"));
    }
    for component in path.split(|b| *b == b'/') {
        if component.is_empty() {
            return Err(format!("owned.z path has an empty component: {display:?}"));
        }
        if component == b"." || component == b".." {
            return Err(format!("owned.z path has a . or .. component: {display:?}"));
        }
    }
    Ok(())
}

fn normalized_case_fold(value: &[u8]) -> Vec<u8> {
    match std::str::from_utf8(value) {
        Ok(value) => value.case_fold().nfc().collect::<String>().into_bytes(),
        Err(_) => value.iter().map(u8::to_ascii_lowercase).collect(),
    }
}

// ---------------------------------------------------------------------------
// Raw diff manifest (manifest.raw.z)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawDiffRecord {
    pub old_mode: String,
    pub new_mode: String,
    pub old_oid: String,
    pub new_oid: String,
    pub status: char,
    pub path: Vec<u8>,
}

const ALLOWED_MODES: [&str; 4] = ["000000", "100644", "100755", "120000"];

/// Parse and validate a NUL-delimited raw-diff manifest
/// (`:<omode> <nmode> <ooid> <noid> <status> NUL <path> NUL` per record)
/// against the declared object format and the authoritative `owned.z` set.
pub fn parse_raw_manifest(
    bytes: &[u8],
    format: ObjectFormat,
    owned_paths: &[Vec<u8>],
) -> Result<Vec<RawDiffRecord>, String> {
    SizeLimit::OperationOutputLeaf.check(bytes.len())?;
    let owned_records: Vec<&[u8]> = owned_paths.iter().map(Vec::as_slice).collect();
    validate_owned_records(&owned_records)?;
    if bytes.is_empty() {
        return Err("manifest.raw.z must not be empty".to_string());
    }
    if bytes.last() != Some(&0) {
        return Err("manifest.raw.z must end with a terminal NUL".to_string());
    }
    let tokens: Vec<&[u8]> = bytes[..bytes.len() - 1].split(|b| *b == 0).collect();
    // Records are header+path pairs; an odd token count is a trailing
    // partial record.
    if tokens.is_empty() {
        return Err("manifest.raw.z must not contain empty records".to_string());
    }
    if tokens.iter().any(|t| t.is_empty()) {
        return Err("manifest.raw.z must not contain empty tokens".to_string());
    }
    if !tokens.len().is_multiple_of(2) {
        return Err("manifest.raw.z ends with a trailing partial record".to_string());
    }
    let width = format.oid_width();
    let mut records = Vec::new();
    let owned_set: BTreeSet<&[u8]> = owned_paths.iter().map(Vec::as_slice).collect();
    let mut seen_paths = BTreeSet::new();
    let mut index = 0;
    while index < tokens.len() {
        let header = std::str::from_utf8(tokens[index])
            .map_err(|_| "manifest header must be valid UTF-8".to_string())?;
        let path = tokens[index + 1].to_vec();
        index += 2;

        let rest = header
            .strip_prefix(':')
            .ok_or_else(|| format!("manifest header must begin with ':': {header:?}"))?;
        let parts: Vec<&str> = rest.split(' ').collect();
        if parts.len() != 5 {
            return Err(format!(
                "manifest header must be exactly five space-separated fields: {header:?}"
            ));
        }
        let (old_mode, new_mode, old_oid, new_oid, status) =
            (parts[0], parts[1], parts[2], parts[3], parts[4]);

        for mode in [old_mode, new_mode] {
            if mode_is_gitlink(mode) {
                return Err("manifest rejects 160000 gitlinks".to_string());
            }
            if !ALLOWED_MODES.contains(&mode) {
                return Err(format!(
                    "manifest mode must be one of {ALLOWED_MODES:?}, not {mode:?}"
                ));
            }
        }
        let status_chars: Vec<char> = status.chars().collect();
        if status_chars.len() != 1 || !matches!(status_chars[0], 'A' | 'D' | 'M' | 'T') {
            return Err(format!(
                "manifest status must be one of A, D, M, T, not {status:?}"
            ));
        }
        let status = status_chars[0];
        for (oid, name) in [(old_oid, "old"), (new_oid, "new")] {
            if !is_lowercase_hex_of_width(oid, width) {
                return Err(format!(
                    "manifest {name} OID must be {width} lowercase hex for {format:?}"
                ));
            }
        }
        let all_zero = |oid: &str| is_all_zero_hex_of_width(oid, width);
        let nonzero = |oid: &str| !all_zero(oid);
        if old_mode == "000000" && !all_zero(old_oid) {
            return Err("zero mode requires the all-zero OID".to_string());
        }
        if old_mode != "000000" && !nonzero(old_oid) {
            return Err("nonzero mode requires a nonzero OID".to_string());
        }
        if new_mode == "000000" && !all_zero(new_oid) {
            return Err("zero mode requires the all-zero OID".to_string());
        }
        if new_mode != "000000" && !nonzero(new_oid) {
            return Err("nonzero mode requires a nonzero OID".to_string());
        }
        match status {
            'A' => {
                if !all_zero(old_oid) || !nonzero(new_oid) {
                    return Err("A requires a zero old side and nonzero new side".to_string());
                }
            }
            'D' => {
                if !nonzero(old_oid) || !all_zero(new_oid) {
                    return Err("D requires a nonzero old side and zero new side".to_string());
                }
            }
            'M' | 'T' => {
                if !nonzero(old_oid) || !nonzero(new_oid) {
                    return Err("M/T require two nonzero sides".to_string());
                }
                let same_type = type_of(old_mode) == type_of(new_mode);
                if status == 'M' {
                    if !same_type {
                        return Err("M requires the same type on both sides".to_string());
                    }
                    if old_mode == new_mode && old_oid == new_oid {
                        return Err("M requires a changed mode/OID tuple".to_string());
                    }
                } else if same_type {
                    return Err("T requires a regular-file/symlink type change".to_string());
                }
            }
            _ => unreachable!(),
        }
        if !owned_set.contains(path.as_slice()) {
            return Err(format!(
                "manifest path is not owned: {:?}",
                String::from_utf8_lossy(&path)
            ));
        }
        if !seen_paths.insert(path.clone()) {
            return Err(format!(
                "manifest duplicate path: {:?}",
                String::from_utf8_lossy(&path)
            ));
        }
        records.push(RawDiffRecord {
            old_mode: old_mode.to_string(),
            new_mode: new_mode.to_string(),
            old_oid: old_oid.to_string(),
            new_oid: new_oid.to_string(),
            status,
            path,
        });
    }
    Ok(records)
}

fn mode_is_gitlink(mode: &str) -> bool {
    mode == "160000"
}

fn type_of(mode: &str) -> &'static str {
    if mode == "120000" {
        "symlink"
    } else {
        "regular"
    }
}

// ---------------------------------------------------------------------------
// files-ref-namespace-v1
// ---------------------------------------------------------------------------

pub const FILES_REF_NAMESPACE_TAG: &str = "files-ref-namespace-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefValue {
    Direct(String),
    Symbolic(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceRecord {
    pub name: Vec<u8>,
    pub value: RefValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceManifest {
    pub format: ObjectFormat,
    pub head: NamespaceRecord,
    pub refs: Vec<NamespaceRecord>,
}

impl NamespaceManifest {
    /// Serialize to the canonical byte form:
    /// `<tag> NUL <format> NUL <count> NUL <records...> NUL` where each
    /// record is `<name> NUL <direct|symbolic> NUL <value> NUL`, `HEAD`
    /// comes first, and refs follow ordered by raw name bytes. The final
    /// value's delimiter is the terminal NUL and is followed by exact EOF.
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        let format = match self.format {
            ObjectFormat::Sha1 => "sha1",
            ObjectFormat::Sha256 => "sha256",
        };
        let count = self.refs.len() + 1;
        let mut out = Vec::new();
        out.extend_from_slice(FILES_REF_NAMESPACE_TAG.as_bytes());
        out.push(0);
        out.extend_from_slice(format.as_bytes());
        out.push(0);
        out.extend_from_slice(count.to_string().as_bytes());
        out.push(0);
        let mut records = vec![&self.head];
        records.extend(self.refs.iter());
        for record in records {
            out.extend_from_slice(&record.name);
            out.push(0);
            out.extend_from_slice(match &record.value {
                RefValue::Direct(_) => b"direct",
                RefValue::Symbolic(_) => b"symbolic",
            });
            out.push(0);
            let value_bytes: &[u8] = match &record.value {
                RefValue::Direct(oid) => oid.as_bytes(),
                RefValue::Symbolic(target) => target,
            };
            out.extend_from_slice(value_bytes);
            out.push(0);
        }
        SizeLimit::RegisteredRecord.check(out.len())?;
        Self::decode(&out)?;
        Ok(out)
    }

    /// Parse and validate the canonical byte form. Rejects unknown tags,
    /// noncanonical counts, unordered refs, invalid names, duplicate names,
    /// path-prefix conflicts, invalid targets or OIDs, a missing terminal
    /// NUL, or trailing data.
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        SizeLimit::RegisteredRecord.check(bytes.len())?;
        let mut cursor = 0;
        let take = |cursor: &mut usize| -> Result<Vec<u8>, String> {
            let start = *cursor;
            let end = bytes[start..]
                .iter()
                .position(|b| *b == 0)
                .map(|offset| start + offset)
                .ok_or_else(|| "files-ref-namespace-v1 ended early".to_string())?;
            *cursor = end + 1;
            Ok(bytes[start..end].to_vec())
        };

        let tag = take(&mut cursor)?;
        if tag != FILES_REF_NAMESPACE_TAG.as_bytes() {
            return Err(format!(
                "unknown namespace tag: {:?}",
                String::from_utf8_lossy(&tag)
            ));
        }
        let format_bytes = take(&mut cursor)?;
        let format_text = std::str::from_utf8(&format_bytes)
            .map_err(|_| "object format must be ASCII".to_string())?;
        let format = ObjectFormat::parse(format_text)?;
        let count_bytes = take(&mut cursor)?;
        let count_text = std::str::from_utf8(&count_bytes)
            .map_err(|_| "record count must be ASCII".to_string())?;
        if count_text.is_empty()
            || !count_text.bytes().all(|b| b.is_ascii_digit())
            || count_text.starts_with('0')
        {
            return Err(format!("noncanonical record count: {count_text:?}"));
        }
        let count: usize = count_text
            .parse()
            .map_err(|_| format!("unparsable record count: {count_text:?}"))?;
        if count == 0 {
            return Err("record count must include at least HEAD".to_string());
        }
        if count > bytes.len().saturating_sub(cursor) / 6 {
            return Err("record count exceeds remaining namespace bytes".to_string());
        }

        let width = format.oid_width();
        let mut records = Vec::with_capacity(count);
        for _ in 0..count {
            let name = take(&mut cursor)?;
            let kind = take(&mut cursor)?;
            let value = take(&mut cursor)?;
            if name.is_empty() {
                return Err("ref name must not be empty".to_string());
            }
            let record = match kind.as_slice() {
                b"direct" => {
                    let value = String::from_utf8(value)
                        .map_err(|_| "direct OID must be ASCII".to_string())?;
                    if !is_lowercase_hex_of_width(&value, width)
                        || is_all_zero_hex_of_width(&value, width)
                    {
                        return Err(format!(
                            "direct value must be a {width}-hex nonzero OID: {value:?}"
                        ));
                    }
                    NamespaceRecord {
                        name,
                        value: RefValue::Direct(value),
                    }
                }
                b"symbolic" => {
                    validate_full_ref_name_bytes(&value)?;
                    NamespaceRecord {
                        name,
                        value: RefValue::Symbolic(value),
                    }
                }
                other => {
                    return Err(format!(
                        "record kind must be direct or symbolic: {:?}",
                        String::from_utf8_lossy(other)
                    ))
                }
            };
            records.push(record);
        }
        // The final value's delimiter is the one terminal NUL.
        if cursor != bytes.len() {
            return Err("trailing data after the terminal NUL".to_string());
        }
        let head = records
            .first()
            .ok_or_else(|| "manifest must start with HEAD".to_string())?;
        if head.name != b"HEAD" {
            return Err("the first record must be HEAD".to_string());
        }
        let refs: Vec<NamespaceRecord> = records[1..].to_vec();
        let mut names = BTreeSet::new();
        for record in &refs {
            if record.name == b"HEAD" {
                return Err("duplicate HEAD".to_string());
            }
            validate_full_ref_name_bytes(&record.name)?;
            if !names.insert(record.name.clone()) {
                return Err(format!(
                    "duplicate ref name: {:?}",
                    String::from_utf8_lossy(&record.name)
                ));
            }
        }
        for index in 1..refs.len() {
            let prior = &refs[index - 1].name;
            let current = &refs[index].name;
            if prior >= current {
                return Err(format!(
                    "refs must be ordered by raw name bytes: {:?} >= {:?}",
                    String::from_utf8_lossy(prior),
                    String::from_utf8_lossy(current)
                ));
            }
        }
        for name in &names {
            for slash in name
                .iter()
                .enumerate()
                .filter_map(|(index, byte)| (*byte == b'/').then_some(index))
            {
                if let Some(parent) = names.get(&name[..slash]) {
                    return Err(format!(
                        "path-prefix file/directory conflict: {:?} vs {:?}",
                        String::from_utf8_lossy(parent),
                        String::from_utf8_lossy(name)
                    ));
                }
            }
        }
        Ok(NamespaceManifest {
            format,
            head: head.clone(),
            refs,
        })
    }
}

/// A full ref name under Git's `check-ref-format` rules (without `--allow-*`
/// relaxations), additionally required to live under `refs/` (or be `HEAD`
/// where explicitly allowed).
pub fn validate_full_ref_name(name: &str) -> Result<(), String> {
    validate_full_ref_name_bytes(name.as_bytes())
}

fn validate_full_ref_name_bytes(name: &[u8]) -> Result<(), String> {
    let display = String::from_utf8_lossy(name);
    if name.is_empty() {
        return Err("ref name must not be empty".to_string());
    }
    if !name.starts_with(b"refs/") {
        return Err(format!("full ref must start with refs/: {display:?}"));
    }
    if name.ends_with(b"/") || name.ends_with(b".lock") {
        return Err(format!("invalid ref name suffix: {display:?}"));
    }
    if name.ends_with(b".")
        || name.windows(2).any(|window| window == b"..")
        || name.windows(2).any(|window| window == b"//")
        || name.windows(2).any(|window| window == b"@{")
    {
        return Err(format!("invalid ref name: {display:?}"));
    }
    if name.iter().any(|b| {
        b.is_ascii_control()
            || *b == b' '
            || *b == b'~'
            || *b == b'^'
            || *b == b':'
            || *b == b'?'
            || *b == b'*'
            || *b == b'['
            || *b == b'\\'
            || *b == 0x7f
    }) {
        return Err(format!("ref name carries forbidden bytes: {display:?}"));
    }
    for component in name.split(|b| *b == b'/') {
        if component.is_empty() || component.starts_with(b".") || component.ends_with(b".lock") {
            return Err(format!("invalid ref component in {display:?}"));
        }
    }
    Ok(())
}

/// Validate a SHA256SUMS-style conventional checksum line output
/// (`<hex><two spaces><path>`), used to cross-check `sha256sum`/`shasum`
/// output captured as evidence.
pub fn parse_checksum_line(line: &str) -> Result<(String, String), String> {
    SizeLimit::OperationOutputLeaf.check(line.len())?;
    let (hex, path) = line
        .split_once("  ")
        .ok_or_else(|| format!("checksum line must be `<hex><two spaces><path>`: {line:?}"))?;
    validate_digest_form(hex)?;
    if path.is_empty() {
        return Err("checksum path must not be empty".to_string());
    }
    Ok((hex.to_string(), path.to_string()))
}
