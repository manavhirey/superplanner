//! Canonical embedded filesystem and immutable-record identity tuples.

use crate::core::{validate_digest_form, SizeLimit};
use crate::json::{canonical_json_of, parse_canonical_json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilesystemIdentity {
    pub path: String,
    pub device: u64,
    pub inode: u64,
}

impl FilesystemIdentity {
    pub fn validate(&self) -> Result<(), String> {
        validate_identity_path(&self.path)?;
        if self.inode == 0 {
            return Err("filesystem identity inode must be nonzero".to_string());
        }
        Ok(())
    }

    pub fn to_canonical_scalar(&self) -> Result<String, String> {
        self.validate()?;
        let value = canonical_json_of(self)?;
        SizeLimit::RegisteredRecord.check(value.len())?;
        Ok(value)
    }

    pub fn from_canonical_scalar(value: &str) -> Result<Self, String> {
        let identity: Self = parse_canonical_json(value.as_bytes(), SizeLimit::RegisteredRecord)?;
        identity.validate()?;
        Ok(identity)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryIdentity {
    pub path: String,
    pub device: u64,
    pub inode: u64,
    pub link_count: u64,
    pub sha256: String,
}

impl RegistryIdentity {
    pub fn validate(&self) -> Result<(), String> {
        validate_identity_path(&self.path)?;
        if self.inode == 0 {
            return Err("registry identity inode must be nonzero".to_string());
        }
        if self.link_count != 1 {
            return Err("registered immutable record link_count must be 1".to_string());
        }
        validate_digest_form(&self.sha256)
    }

    pub fn to_canonical_scalar(&self) -> Result<String, String> {
        self.validate()?;
        let value = canonical_json_of(self)?;
        SizeLimit::RegisteredRecord.check(value.len())?;
        Ok(value)
    }

    pub fn from_canonical_scalar(value: &str) -> Result<Self, String> {
        let identity: Self = parse_canonical_json(value.as_bytes(), SizeLimit::RegisteredRecord)?;
        identity.validate()?;
        Ok(identity)
    }
}

fn validate_identity_path(path: &str) -> Result<(), String> {
    if !path.starts_with('/') {
        return Err("identity path must be canonical and absolute".to_string());
    }
    if path
        .bytes()
        .any(|byte| byte.is_ascii_control() || byte == 0x7f)
    {
        return Err("identity path carries control bytes".to_string());
    }
    if path != "/"
        && (path.ends_with('/')
            || path[1..]
                .split('/')
                .any(|component| component.is_empty() || component == "." || component == ".."))
    {
        return Err("identity path must not contain empty, . or .. components".to_string());
    }
    Ok(())
}
