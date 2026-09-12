//! Verification and documentation-synchronization records per
//! `references/quality-gates.md` sections 1-2. Both records bind to the
//! exact candidate bytes they describe and are current only while that
//! candidate remains unchanged.

use crate::core::{validate_digest_form, validate_git_sha_width, SizeLimit};
use crate::git_mediation::{parse_owned_z, ObjectFormat};
use crate::hashing::sha256_digest;
use crate::identity::RegistryIdentity;
use crate::json::{canonical_json_of, parse_canonical_json};
use serde::{Deserialize, Serialize};

pub const VERIFICATION_TAG: &str = "verification-v1";
pub const DOCUMENTATION_TAG: &str = "documentation-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationRecord {
    pub schema: String,
    pub object_format: ObjectFormat,
    pub bound_head: String,
    pub candidate_tree: String,
    pub candidate_patch_sha256: String,
    pub candidate_patch_identity: RegistryIdentity,
    pub checks: Vec<VerificationCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationCheck {
    pub command: String,
    pub result: String,
    pub evidence: String,
}

impl VerificationRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != VERIFICATION_TAG {
            return Err(format!("unknown verification schema: {:?}", self.schema));
        }
        validate_git_sha_width(&self.bound_head, self.object_format, "bound_head")?;
        validate_git_sha_width(&self.candidate_tree, self.object_format, "candidate_tree")?;
        validate_digest_form(&self.candidate_patch_sha256)?;
        self.candidate_patch_identity.validate()?;
        if self.candidate_patch_identity.sha256 != self.candidate_patch_sha256 {
            return Err(
                "candidate patch identity does not bind candidate_patch_sha256".to_string(),
            );
        }
        if self.checks.is_empty() {
            return Err("verification must record at least one check".to_string());
        }
        for check in &self.checks {
            check.validate()?;
        }
        Ok(())
    }

    pub fn require_passed(&self) -> Result<(), String> {
        self.validate()?;
        if self.checks.iter().any(|check| check.result != "passed") {
            return Err("verification gate requires every recorded check to pass".to_string());
        }
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
}

impl VerificationCheck {
    fn validate(&self) -> Result<(), String> {
        if !matches!(self.result.as_str(), "passed" | "failed" | "not-run") {
            return Err(format!(
                "verification result must be passed, failed, or not-run, not {:?}",
                self.result
            ));
        }
        if self.command.is_empty() || self.evidence.is_empty() {
            return Err("verification checks must carry a command and evidence".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentationStatus {
    Current,
    Stale,
    Blocked,
}

impl DocumentationStatus {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "current" => Ok(DocumentationStatus::Current),
            "stale" => Ok(DocumentationStatus::Stale),
            "blocked" => Ok(DocumentationStatus::Blocked),
            other => Err(format!(
                "documentation status must be current, stale, or blocked, not {other:?}"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentationRecord {
    pub schema: String,
    pub object_format: ObjectFormat,
    pub bound_head: String,
    pub candidate_tree: String,
    pub candidate_patch_sha256: String,
    pub candidate_patch_identity: RegistryIdentity,
    pub updated_paths: Vec<String>,
    /// Exact explanation when no documentation path changed, or `none` when
    /// `updated_paths` is non-empty.
    pub no_change_reason: String,
    pub blocked_reason: String,
    pub status: DocumentationStatus,
    pub checks: Vec<VerificationCheck>,
    /// The verification record bound to the same candidate bytes, required
    /// after any documentation-induced refresh.
    pub bound_verification_sha256: String,
}

impl DocumentationRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != DOCUMENTATION_TAG {
            return Err(format!("unknown documentation schema: {:?}", self.schema));
        }
        validate_git_sha_width(&self.bound_head, self.object_format, "bound_head")?;
        validate_git_sha_width(&self.candidate_tree, self.object_format, "candidate_tree")?;
        validate_digest_form(&self.candidate_patch_sha256)?;
        self.candidate_patch_identity.validate()?;
        if self.candidate_patch_identity.sha256 != self.candidate_patch_sha256 {
            return Err(
                "candidate patch identity does not bind candidate_patch_sha256".to_string(),
            );
        }
        validate_digest_form(&self.bound_verification_sha256)?;
        if !self.updated_paths.is_empty() {
            let mut encoded_paths = Vec::new();
            for path in &self.updated_paths {
                if path.as_bytes().contains(&0) {
                    return Err("documentation path contains NUL".to_string());
                }
                encoded_paths.extend_from_slice(path.as_bytes());
                encoded_paths.push(0);
            }
            parse_owned_z(&encoded_paths)
                .map_err(|error| format!("invalid documentation path set: {error}"))?;
        }
        if self
            .updated_paths
            .windows(2)
            .any(|pair| pair[0].as_bytes() >= pair[1].as_bytes())
        {
            return Err("documentation paths must be uniquely ordered by raw bytes".to_string());
        }
        for check in &self.checks {
            check.validate()?;
        }
        match self.status {
            DocumentationStatus::Current => {
                if self.blocked_reason != "none" {
                    return Err("current documentation must not carry a blocked reason".to_string());
                }
                if self.checks.is_empty() {
                    return Err("current documentation must record its checks".to_string());
                }
                if self.checks.iter().any(|check| check.result != "passed") {
                    return Err("current documentation requires every check to pass".to_string());
                }
                if self.updated_paths.is_empty() {
                    if self.no_change_reason.is_empty() || self.no_change_reason == "none" {
                        return Err("current documentation with no updated paths must state why"
                            .to_string());
                    }
                } else if self.no_change_reason != "none" {
                    return Err(
                        "current documentation with updated paths must use no_change_reason=none"
                            .to_string(),
                    );
                }
            }
            DocumentationStatus::Stale => {
                if self.blocked_reason != "none" {
                    return Err("stale documentation must not carry a blocked reason".to_string());
                }
                if self.updated_paths.is_empty() {
                    return Err("stale documentation must name the paths left stale".to_string());
                }
                if self.no_change_reason != "none" {
                    return Err("stale documentation must use no_change_reason=none".to_string());
                }
            }
            DocumentationStatus::Blocked => {
                if self.blocked_reason == "none" || self.blocked_reason.is_empty() {
                    return Err("blocked documentation must state why".to_string());
                }
                if self.no_change_reason != "none" {
                    return Err("blocked documentation must use no_change_reason=none".to_string());
                }
            }
        }
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
}

pub fn bind_verification_to_documentation(
    verification: &VerificationRecord,
    documentation: &DocumentationRecord,
) -> Result<(), String> {
    verification.require_passed()?;
    documentation.validate()?;
    if documentation.status != DocumentationStatus::Current {
        return Err("documentation must be current".to_string());
    }
    if verification.object_format != documentation.object_format
        || verification.bound_head != documentation.bound_head
        || verification.candidate_tree != documentation.candidate_tree
        || verification.candidate_patch_sha256 != documentation.candidate_patch_sha256
        || verification.candidate_patch_identity != documentation.candidate_patch_identity
    {
        return Err(
            "verification and documentation must bind the same exact candidate".to_string(),
        );
    }
    let verification_bytes = verification.to_canonical_json()?;
    if documentation.bound_verification_sha256 != sha256_digest(verification_bytes.as_bytes()) {
        return Err("documentation does not bind the canonical verification record".to_string());
    }
    Ok(())
}
