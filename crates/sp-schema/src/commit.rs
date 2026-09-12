//! Foundational commit-gate records per `references/quality-gates.md`
//! sections 3-6: canonical commit-object preparation and the exact user
//! authorization tuple. Transaction manifests land with commit runtime work.

use crate::core::{validate_digest_form, validate_git_date};
use crate::git_mediation::{validate_full_ref_name, ObjectFormat};
use crate::hashing::sha256_digest;
use crate::json::{canonical_json_of, parse_canonical_json};
use crate::SizeLimit;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use sha2::Sha256;

pub const COMMIT_PREPARE_TAG: &str = "commit-prepare-v1";
pub const COMMIT_AUTHORIZATION_TAG: &str = "commit-authorization-v1";

fn validate_oid(value: &str, format: ObjectFormat, where_: &str) -> Result<(), String> {
    let width = format.oid_width();
    let bytes = value.as_bytes();
    if bytes.len() != width
        || !bytes.iter().all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'))
        || bytes.iter().all(|b| *b == b'0')
    {
        return Err(format!(
            "{where_} must be a full nonzero {width}-hex OID for {format:?}, got {value:?}"
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommitPrepare {
    pub schema: String,
    pub object_format: ObjectFormat,
    pub patch_sha256: String,
    pub base_commit: String,
    pub base_tree: String,
    pub candidate_tree: String,
    pub message_path: String,
    pub message: String,
    pub message_sha256: String,
    pub target_ref: String,
    pub target_ref_at_base: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub author_date: String,
    pub committer_date: String,
    pub expected_commit_oid: String,
    pub reflog_reason: String,
}

impl CommitPrepare {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != COMMIT_PREPARE_TAG {
            return Err(format!("unknown commit-prepare schema: {:?}", self.schema));
        }
        let format = self.object_format;
        validate_digest_form(&self.patch_sha256)?;
        validate_oid(&self.base_commit, format, "base_commit")?;
        validate_oid(&self.base_tree, format, "base_tree")?;
        validate_oid(&self.candidate_tree, format, "candidate_tree")?;
        if self.base_tree == self.candidate_tree {
            return Err("candidate tree must differ from the base tree".to_string());
        }
        if !self.message_path.starts_with('/') {
            return Err("message_path must be absolute and external".to_string());
        }
        if has_control_bytes(&self.message_path) {
            return Err("message_path carries control bytes".to_string());
        }
        validate_digest_form(&self.message_sha256)?;
        if !self.message.ends_with('\n') || self.message.ends_with("\n\n") {
            return Err("message must have exactly one terminal newline".to_string());
        }
        if sha256_digest(self.message.as_bytes()) != self.message_sha256 {
            return Err("message_sha256 does not match the exact UTF-8 message bytes".to_string());
        }
        validate_full_ref_name(&self.target_ref)?;
        validate_oid(&self.target_ref_at_base, format, "target_ref_at_base")?;
        if self.target_ref_at_base != self.base_commit {
            return Err("target_ref_at_base must equal base_commit".to_string());
        }
        validate_git_date(&self.author_date)?;
        validate_git_date(&self.committer_date)?;
        validate_oid(&self.expected_commit_oid, format, "expected_commit_oid")?;
        if self.computed_commit_oid() != self.expected_commit_oid {
            return Err(
                "expected_commit_oid does not match the canonical commit object".to_string(),
            );
        }
        validate_reflog_reason(&self.reflog_reason)?;
        validate_identity_name(&self.author_name, "author_name")?;
        validate_identity_email(&self.author_email, "author_email")?;
        validate_identity_name(&self.committer_name, "committer_name")?;
        validate_identity_email(&self.committer_email, "committer_email")?;
        Ok(())
    }

    pub fn to_canonical_json(&self) -> Result<String, String> {
        self.validate()?;
        let text = canonical_json_of(self)?;
        SizeLimit::RegisteredRecord.check(text.len())?;
        Ok(text)
    }

    pub fn from_canonical_json(bytes: &[u8]) -> Result<Self, String> {
        let record: Self = parse_canonical_json(bytes, SizeLimit::RegisteredRecord)?;
        record.validate()?;
        Ok(record)
    }

    pub fn computed_commit_oid(&self) -> String {
        let mut content = format!(
            "tree {}\nparent {}\nauthor {} <{}> {}\ncommitter {} <{}> {}\n\n",
            self.candidate_tree,
            self.base_commit,
            self.author_name,
            self.author_email,
            self.author_date,
            self.committer_name,
            self.committer_email,
            self.committer_date
        )
        .into_bytes();
        content.extend_from_slice(self.message.as_bytes());

        let mut object = format!("commit {}\0", content.len()).into_bytes();
        object.extend_from_slice(&content);
        match self.object_format {
            ObjectFormat::Sha1 => hex_digest(Sha1::digest(&object).as_slice()),
            ObjectFormat::Sha256 => hex_digest(Sha256::digest(&object).as_slice()),
        }
    }

    /// The authorization that itself names every prepared value; any missing
    /// or differing field is rejected. A detached `HEAD` never reaches this
    /// point because `target_ref` must be a full direct ref.
    pub fn authorize(&self, authorization: &AuthorizationTuple) -> Result<(), String> {
        self.validate()?;
        authorization.validate(self.object_format)?;
        let mismatches = [
            (
                authorization.object_format == self.object_format,
                "object format",
            ),
            (
                authorization.patch_sha256 == self.patch_sha256,
                "patch hash",
            ),
            (
                authorization.candidate_tree == self.candidate_tree,
                "candidate tree",
            ),
            (
                authorization.message_sha256 == self.message_sha256,
                "message hash",
            ),
            (
                authorization.base_commit == self.base_commit,
                "full base commit",
            ),
            (
                authorization.target_state == self.target_ref_at_base,
                "target state",
            ),
            (
                authorization.target_ref == self.target_ref,
                "full target ref",
            ),
            (authorization.author_date == self.author_date, "author date"),
            (
                authorization.committer_date == self.committer_date,
                "committer date",
            ),
            (
                authorization.expected_commit_oid == self.expected_commit_oid,
                "expected commit OID",
            ),
            (authorization.author_name == self.author_name, "author name"),
            (
                authorization.author_email == self.author_email,
                "author email",
            ),
            (
                authorization.committer_name == self.committer_name,
                "committer name",
            ),
            (
                authorization.committer_email == self.committer_email,
                "committer email",
            ),
            (
                authorization.reflog_reason == self.reflog_reason,
                "reflog reason",
            ),
        ];
        let failures: Vec<&str> = mismatches
            .iter()
            .filter(|(ok, _)| !ok)
            .map(|(_, name)| *name)
            .collect();
        if !failures.is_empty() {
            return Err(format!(
                "authorization must itself explicitly name the exact {}: {}",
                failures.join(", "),
                self.summarize()
            ));
        }
        Ok(())
    }

    fn summarize(&self) -> String {
        format!(
            "patch_sha256={}, candidate_tree={}, message_sha256={}, base_commit={}, \
             target_ref={}, target_state={}, author_date={}, committer_date={}, expected_oid={}",
            self.patch_sha256,
            self.candidate_tree,
            self.message_sha256,
            self.base_commit,
            self.target_ref,
            self.target_ref_at_base,
            self.author_date,
            self.committer_date,
            self.expected_commit_oid
        )
    }
}

/// The exact tuple the user authorization must itself name: patch hash,
/// candidate tree, message hash, full base commit, target state, both exact
/// numeric-timezone dates, and the expected full commit OID
/// (`references/quality-gates.md` commit gate).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationTuple {
    pub schema: String,
    pub object_format: ObjectFormat,
    pub patch_sha256: String,
    pub candidate_tree: String,
    pub message_sha256: String,
    pub base_commit: String,
    pub target_ref: String,
    pub target_state: String,
    pub author_date: String,
    pub committer_date: String,
    pub expected_commit_oid: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub reflog_reason: String,
}

impl AuthorizationTuple {
    pub fn validate(&self, format: ObjectFormat) -> Result<(), String> {
        if self.schema != COMMIT_AUTHORIZATION_TAG {
            return Err(format!(
                "unknown commit-authorization schema: {:?}",
                self.schema
            ));
        }
        if self.object_format != format {
            return Err("authorization object format does not match its phase context".to_string());
        }
        validate_digest_form(&self.patch_sha256)?;
        validate_oid(&self.candidate_tree, format, "candidate_tree")?;
        validate_digest_form(&self.message_sha256)?;
        validate_oid(&self.base_commit, format, "base_commit")?;
        validate_full_ref_name(&self.target_ref)?;
        validate_oid(&self.target_state, format, "target_state")?;
        if self.target_state != self.base_commit {
            return Err("target_state must equal base_commit".to_string());
        }
        validate_git_date(&self.author_date)?;
        validate_git_date(&self.committer_date)?;
        validate_oid(&self.expected_commit_oid, format, "expected_commit_oid")?;
        validate_identity_name(&self.author_name, "author_name")?;
        validate_identity_email(&self.author_email, "author_email")?;
        validate_identity_name(&self.committer_name, "committer_name")?;
        validate_identity_email(&self.committer_email, "committer_email")?;
        validate_reflog_reason(&self.reflog_reason)?;
        Ok(())
    }

    pub fn to_canonical_json(&self, format: ObjectFormat) -> Result<String, String> {
        self.validate(format)?;
        let text = canonical_json_of(self)?;
        SizeLimit::RegisteredRecord.check(text.len())?;
        Ok(text)
    }

    pub fn from_canonical_json(bytes: &[u8], format: ObjectFormat) -> Result<Self, String> {
        let record: Self = parse_canonical_json(bytes, SizeLimit::RegisteredRecord)?;
        record.validate(format)?;
        Ok(record)
    }
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn validate_identity_name(value: &str, name: &str) -> Result<(), String> {
    if value.is_empty()
        || value
            .bytes()
            .any(|b| b.is_ascii_control() || b == 0x7f || b == b'<' || b == b'>')
        || has_git_identity_crud_at_edge(value)
    {
        return Err(format!("{name} is not a valid Git identity name"));
    }
    Ok(())
}

fn has_control_bytes(value: &str) -> bool {
    value.bytes().any(|b| b.is_ascii_control() || b == 0x7f)
}

fn validate_reflog_reason(value: &str) -> Result<(), String> {
    if value.is_empty()
        || has_control_bytes(value)
        || value.starts_with(' ')
        || value.ends_with(' ')
        || value.contains("  ")
    {
        return Err("reflog_reason must use canonical single-space formatting".to_string());
    }
    Ok(())
}

fn has_git_identity_crud_at_edge(value: &str) -> bool {
    let is_crud = |byte: u8| {
        byte <= b' '
            || matches!(
                byte,
                b',' | b':' | b';' | b'<' | b'>' | b'"' | b'\\' | b'\''
            )
    };
    value.as_bytes().first().is_none_or(|byte| is_crud(*byte))
        || value.as_bytes().last().is_none_or(|byte| is_crud(*byte))
}

fn validate_identity_email(value: &str, name: &str) -> Result<(), String> {
    if value
        .bytes()
        .any(|b| b.is_ascii_control() || b == 0x7f || b == b'<' || b == b'>')
        || has_git_identity_crud_at_edge(value)
    {
        return Err(format!("{name} carries invalid Git identity bytes"));
    }
    let Some((local, domain)) = value.split_once('@') else {
        return Err(format!("{name} must carry one @"));
    };
    if local.is_empty() || domain.is_empty() || domain.contains('@') {
        return Err(format!(
            "{name} must carry non-empty local and domain parts"
        ));
    }
    Ok(())
}
