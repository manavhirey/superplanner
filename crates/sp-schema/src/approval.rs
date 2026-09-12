//! Approval-block parsing and field invariants per
//! `references/artifact-contracts.md` and the exact template field order.

use crate::core::{validate_content_id_form, validate_iso8601_z};
use crate::hashing::MarkerKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalFlavor {
    Markdown,
    Gherkin,
}

impl ApprovalFlavor {
    fn marker_kind(self) -> MarkerKind {
        match self {
            ApprovalFlavor::Markdown => MarkerKind::ApprovalMarkdown,
            ApprovalFlavor::Gherkin => MarkerKind::ApprovalGherkin,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalBlock {
    pub status: String,
    pub approver: String,
    pub approved_at: String,
    pub approval_evidence: String,
    pub content_id: String,
}

const FIELDS: [&str; 5] = [
    "status",
    "approver",
    "approved_at",
    "approval_evidence",
    "content_id",
];

fn parse_field_line(line: &str, flavor: ApprovalFlavor) -> Result<(&str, &str), String> {
    match flavor {
        ApprovalFlavor::Markdown => {
            let rest = line
                .strip_prefix("- ")
                .ok_or_else(|| format!("approval field must start with \"- \": {line:?}"))?;
            let (key, value) = rest
                .split_once(": ")
                .ok_or_else(|| format!("approval field must be \"- key: `value`\": {line:?}"))?;
            let value = value
                .strip_prefix('`')
                .and_then(|v| v.strip_suffix('`'))
                .ok_or_else(|| {
                    format!("markdown approval value must be backtick-wrapped: {line:?}")
                })?;
            Ok((key, value))
        }
        ApprovalFlavor::Gherkin => {
            let rest = line.strip_prefix("# ").ok_or_else(|| {
                format!("gherkin approval field must start with \"# \": {line:?}")
            })?;
            let (key, value) = rest.split_once(": ").ok_or_else(|| {
                format!("gherkin approval field must be \"# key: value\": {line:?}")
            })?;
            Ok((key, value))
        }
    }
}

pub fn parse_approval_block(bytes: &[u8], flavor: ApprovalFlavor) -> Result<ApprovalBlock, String> {
    let pair = crate::hashing::find_pair(bytes, flavor.marker_kind())?;
    let enclosed = &bytes[pair.start_line.1..pair.end_line.0];

    let text = std::str::from_utf8(enclosed)
        .map_err(|_| "approval block must be valid UTF-8".to_string())?;
    let lines: Vec<&str> = text
        .split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
        .collect();

    let mut field_lines = lines.clone();
    if field_lines.last() == Some(&"") {
        field_lines.pop();
    }
    if field_lines.len() != FIELDS.len() {
        return Err(format!(
            "approval block must contain exactly {} field lines, found {}",
            FIELDS.len(),
            field_lines.len()
        ));
    }

    let mut values: Vec<(String, String)> = Vec::with_capacity(FIELDS.len());
    for (index, line) in field_lines.iter().enumerate() {
        let (key, value) = parse_field_line(line, flavor)?;
        if key != FIELDS[index] {
            return Err(format!(
                "approval field {} must be {:?}, found {key:?}",
                index + 1,
                FIELDS[index]
            ));
        }
        if value.is_empty() || value.bytes().any(|b| b.is_ascii_control() || b == b'`') {
            return Err(format!(
                "approval value for {key:?} is empty or carries invalid bytes"
            ));
        }
        values.push((key.to_string(), value.to_string()));
    }

    let block = ApprovalBlock {
        status: values[0].1.clone(),
        approver: values[1].1.clone(),
        approved_at: values[2].1.clone(),
        approval_evidence: values[3].1.clone(),
        content_id: values[4].1.clone(),
    };
    block.validate()?;
    Ok(block)
}

impl ApprovalBlock {
    pub fn validate(&self) -> Result<(), String> {
        if self.status != "pending" && self.status != "approved" {
            return Err(format!(
                "approval status must be pending or approved, not {:?}",
                self.status
            ));
        }
        validate_content_id_form(&self.content_id)?;
        match self.status.as_str() {
            "pending" => {
                if self.approver != "none"
                    || self.approved_at != "none"
                    || self.approval_evidence != "none"
                {
                    return Err(
                        "pending approval must carry none identity, time, and evidence".to_string(),
                    );
                }
            }
            "approved" => {
                if self.approver == "none" {
                    return Err("approved status requires a non-none approver".to_string());
                }
                if self.approved_at == "none" {
                    return Err("approved status requires an approval time".to_string());
                }
                validate_iso8601_z(&self.approved_at)?;
                if self.approval_evidence == "none" {
                    return Err(
                        "approved status requires explicit user-message evidence".to_string()
                    );
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}
