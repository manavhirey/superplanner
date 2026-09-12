//! The canonical binary TLV encoding shared by every `push-*-v1` record per
//! `references/quality-gates.md`:
//!
//! - the ASCII schema tag as the first scalar field
//! - an eight-byte unsigned big-endian top-level field count
//! - a scalar field: `0x00`, 8-byte BE byte length, then exactly that many
//!   raw bytes
//! - a collection field: `0x01`, 8-byte BE item count, then items; each
//!   item starts with an 8-byte BE scalar-field count followed by that
//!   item's fixed-order scalar fields
//!
//! Nested collections, optional fields, and extra fields are invalid. Record-
//! specific validators enforce collection uniqueness and canonical ordering.
//! An empty collection has count zero. Decoding rejects any trailing byte.

use crate::core::SizeLimit;

pub const SCALAR: u8 = 0x00;
pub const COLLECTION: u8 = 0x01;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Scalar(Vec<u8>),
    Collection(Vec<Vec<Field>>),
}

impl Field {
    pub fn scalar_str(value: &str) -> Field {
        Field::Scalar(value.as_bytes().to_vec())
    }

    pub fn as_scalar_str(&self) -> Result<String, String> {
        match self {
            Field::Scalar(bytes) => String::from_utf8(bytes.clone())
                .map_err(|_| "scalar field is not valid UTF-8".to_string()),
            Field::Collection(_) => Err("expected a scalar field, found a collection".to_string()),
        }
    }

    pub fn as_collection(&self) -> Result<&Vec<Vec<Field>>, String> {
        match self {
            Field::Collection(items) => Ok(items),
            Field::Scalar(_) => Err("expected a collection field, found a scalar".to_string()),
        }
    }
}

fn put_u64_be(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn read_u64_be(bytes: &[u8], cursor: &mut usize) -> Result<u64, String> {
    let start = *cursor;
    let end = start + 8;
    if bytes.len() < end {
        return Err("truncated 8-byte big-endian length".to_string());
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[start..end]);
    *cursor = end;
    Ok(u64::from_be_bytes(buf))
}

pub fn encode(tag: &str, fields: &[Field]) -> Result<Vec<u8>, String> {
    validate_schema_tag(tag)?;
    let mut out = Vec::new();
    encode_field(&mut out, &Field::scalar_str(tag))?;
    put_u64_be(&mut out, fields.len() as u64);
    for field in fields {
        encode_field(&mut out, field)?;
    }
    SizeLimit::RegisteredRecord.check(out.len())?;
    Ok(out)
}

fn encode_field(out: &mut Vec<u8>, field: &Field) -> Result<(), String> {
    match field {
        Field::Scalar(bytes) => {
            out.push(SCALAR);
            put_u64_be(out, bytes.len() as u64);
            out.extend_from_slice(bytes);
        }
        Field::Collection(items) => {
            out.push(COLLECTION);
            put_u64_be(out, items.len() as u64);
            for item in items {
                put_u64_be(out, item.len() as u64);
                for field in item {
                    if !matches!(field, Field::Scalar(_)) {
                        return Err("nested collections are invalid".to_string());
                    }
                    encode_field(out, field)?;
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    pub tag: String,
    pub fields: Vec<Field>,
}

pub fn decode(bytes: &[u8]) -> Result<Decoded, String> {
    SizeLimit::RegisteredRecord.check(bytes.len())?;
    let mut cursor = 0;
    let tag = decode_scalar(bytes, &mut cursor)?;
    let tag = String::from_utf8(tag).map_err(|_| "schema tag must be valid UTF-8".to_string())?;
    validate_schema_tag(&tag)?;
    let count = read_u64_be(bytes, &mut cursor)?;
    let field_count = usize::try_from(count)
        .map_err(|_| "top-level field count does not fit this platform".to_string())?;
    if field_count > bytes.len().saturating_sub(cursor) / 9 {
        return Err("top-level field count exceeds remaining bytes".to_string());
    }
    let mut fields = Vec::with_capacity(field_count);
    for _ in 0..field_count {
        fields.push(decode_field(bytes, &mut cursor)?);
    }
    if cursor != bytes.len() {
        return Err("trailing data after the declared field count".to_string());
    }
    Ok(Decoded { tag, fields })
}

fn validate_schema_tag(tag: &str) -> Result<(), String> {
    let Some((name, version)) = tag.rsplit_once("-v") else {
        return Err(format!(
            "schema tag must end in -v<positive-integer>: {tag:?}"
        ));
    };
    if name.is_empty()
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        || version.is_empty()
        || version.starts_with('0')
        || !version.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(format!("schema tag must be canonical ASCII: {tag:?}"));
    }
    Ok(())
}

fn decode_field(bytes: &[u8], cursor: &mut usize) -> Result<Field, String> {
    let discriminator = *bytes
        .get(*cursor)
        .ok_or_else(|| "truncated field discriminator".to_string())?;
    *cursor += 1;
    match discriminator {
        SCALAR => Ok(Field::Scalar(decode_scalar_payload(bytes, cursor)?)),
        COLLECTION => {
            let item_count = read_u64_be(bytes, cursor)?;
            let item_count = usize::try_from(item_count)
                .map_err(|_| "collection item count does not fit this platform".to_string())?;
            if item_count > bytes.len().saturating_sub(*cursor) / 8 {
                return Err("collection item count exceeds remaining bytes".to_string());
            }
            let mut items = Vec::with_capacity(item_count);
            for _ in 0..item_count {
                let scalar_count = read_u64_be(bytes, cursor)?;
                let scalar_count = usize::try_from(scalar_count)
                    .map_err(|_| "item scalar count does not fit this platform".to_string())?;
                if scalar_count > bytes.len().saturating_sub(*cursor) / 9 {
                    return Err("item scalar count exceeds remaining bytes".to_string());
                }
                let mut item = Vec::with_capacity(scalar_count);
                for _ in 0..scalar_count {
                    item.push(Field::Scalar(decode_scalar(bytes, cursor)?));
                }
                items.push(item);
            }
            Ok(Field::Collection(items))
        }
        other => Err(format!("unknown field discriminator: {other:#04x}")),
    }
}

fn decode_scalar(bytes: &[u8], cursor: &mut usize) -> Result<Vec<u8>, String> {
    let discriminator = *bytes
        .get(*cursor)
        .ok_or_else(|| "truncated scalar discriminator".to_string())?;
    if discriminator != SCALAR {
        return Err("expected a scalar field".to_string());
    }
    *cursor += 1;
    decode_scalar_payload(bytes, cursor)
}

fn decode_scalar_payload(bytes: &[u8], cursor: &mut usize) -> Result<Vec<u8>, String> {
    let length = usize::try_from(read_u64_be(bytes, cursor)?)
        .map_err(|_| "scalar length does not fit this platform".to_string())?;
    let start = *cursor;
    let end = start
        .checked_add(length)
        .ok_or_else(|| "scalar length overflow".to_string())?;
    if bytes.len() < end {
        return Err("truncated scalar payload".to_string());
    }
    *cursor = end;
    Ok(bytes[start..end].to_vec())
}
