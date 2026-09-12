//! Canonical JSON per `references/record-grammar.md`: UTF-8, no
//! insignificant whitespace, object keys sorted by UTF-8 byte order,
//! integers only where a number appears, no `null`, and no duplicate keys.
//!
//! `serde_json`'s default map is a `BTreeMap`, so serializing a `Value` or
//! any `Serialize` type yields key-sorted compact output. Duplicate keys
//! are rejected structurally by typed deserialization
//! (`deny_unknown_fields`), and `null` is rejected here.

use serde::Serialize;
use serde_json::Value;

use crate::hashing::sha256_prefixed;

pub fn canonical_json(value: &Value) -> Result<String, String> {
    validate_tree(value)?;
    serde_json::to_string(value).map_err(|e| format!("serialization failed: {e}"))
}

pub fn canonical_digest(value: &Value) -> Result<String, String> {
    canonical_json(value).map(|text| sha256_prefixed(text.as_bytes()))
}

pub fn canonical_digest_of<T: Serialize>(value: &T) -> Result<String, String> {
    let value = serde_json::to_value(value).map_err(|e| format!("serialization failed: {e}"))?;
    canonical_digest(&value)
}

fn validate_tree(value: &Value) -> Result<(), String> {
    match value {
        Value::Null => Err("null is not a valid canonical JSON value".to_string()),
        Value::Bool(_) => Ok(()),
        Value::Number(n) => {
            if n.is_u64() || n.is_i64() {
                Ok(())
            } else {
                Err("canonical JSON numbers must be integers".to_string())
            }
        }
        Value::String(_) => Ok(()),
        Value::Array(items) => {
            items.iter().try_for_each(validate_tree)?;
            Ok(())
        }
        Value::Object(map) => {
            for (key, item) in map {
                if key.is_empty() {
                    return Err("canonical JSON object keys must be non-empty".to_string());
                }
                validate_tree(item)?;
            }
            Ok(())
        }
    }
}
