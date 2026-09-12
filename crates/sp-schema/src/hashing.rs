//! Marker-pair validation and the two canonical hashing algorithms per
//! `references/record-grammar.md` ("Hash-Field Forms"):
//!
//! - `approval_content_id` hashes the exact bytes **outside** the single
//!   approval marker pair (`references/artifact-contracts.md`).
//! - `bounded_brief_content_id` hashes the exact bytes **inside** the
//!   bounded-brief marker pair (`references/resumable-state.md`).
//!
//! No normalization of line endings, whitespace, encoding, or the final
//! newline is performed anywhere in this module.

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerKind {
    ApprovalMarkdown,
    ApprovalGherkin,
    BoundedBrief,
}

impl MarkerKind {
    fn markers(self) -> (&'static [u8], &'static [u8]) {
        match self {
            MarkerKind::ApprovalMarkdown => (
                b"<!-- superplanner-approval:start -->",
                b"<!-- superplanner-approval:end -->",
            ),
            MarkerKind::ApprovalGherkin => (
                b"# superplanner-approval:start",
                b"# superplanner-approval:end",
            ),
            MarkerKind::BoundedBrief => (
                b"<!-- superplanner-bounded-brief:start -->",
                b"<!-- superplanner-bounded-brief:end -->",
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkerPair {
    pub start_line: (usize, usize),
    pub end_line: (usize, usize),
}

fn line_ranges(bytes: &[u8]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = 0;
    for (i, b) in bytes.iter().enumerate() {
        if *b == b'\n' {
            ranges.push((start, i + 1));
            start = i + 1;
        }
    }
    if start < bytes.len() {
        ranges.push((start, bytes.len()));
    }
    ranges
}

fn bytes_contains(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && haystack.windows(needle.len()).any(|w| w == needle)
}

fn find_marker_pair(bytes: &[u8], kind: MarkerKind) -> Result<MarkerPair, String> {
    let (start_marker, end_marker) = kind.markers();
    let mut start_hits: Vec<usize> = Vec::new();
    let mut end_hits: Vec<usize> = Vec::new();

    for (line_no, (lo, hi)) in line_ranges(bytes).iter().enumerate() {
        let line = &bytes[*lo..*hi];
        let body = trim_line_ending(line);
        if body == start_marker {
            start_hits.push(line_no);
        } else if body == end_marker {
            end_hits.push(line_no);
        } else if bytes_contains(body, start_marker) || bytes_contains(body, end_marker) {
            return Err(format!(
                "marker with leading or trailing bytes on line {}: {:?}",
                line_no + 1,
                String::from_utf8_lossy(body)
            ));
        }
    }

    match (start_hits.len(), end_hits.len()) {
        (1, 1) => {}
        (0, _) | (_, 0) => {
            return Err(format!("{kind:?}: missing start or end marker line"));
        }
        _ => {
            return Err(format!("{kind:?}: duplicate marker lines"));
        }
    }

    let (start_line_no, end_line_no) = (start_hits[0], end_hits[0]);
    if start_line_no >= end_line_no {
        return Err(format!("{kind:?}: misordered or nested marker lines"));
    }
    let ranges = line_ranges(bytes);
    Ok(MarkerPair {
        start_line: ranges[start_line_no],
        end_line: ranges[end_line_no],
    })
}

fn trim_line_ending(line: &[u8]) -> &[u8] {
    if line.ends_with(b"\r\n") {
        &line[..line.len() - 2]
    } else if line.ends_with(b"\n") {
        &line[..line.len() - 1]
    } else {
        line
    }
}

fn sha256_prefixed(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(SHA256_LEN);
    out.push_str("sha256:");
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

const SHA256_LEN: usize = 71;

pub fn approval_content_id(bytes: &[u8], kind: MarkerKind) -> Result<String, String> {
    if !matches!(
        kind,
        MarkerKind::ApprovalMarkdown | MarkerKind::ApprovalGherkin
    ) {
        return Err("approval_content_id requires an approval marker kind".to_string());
    }
    let pair = find_marker_pair(bytes, kind)?;
    let mut remainder = Vec::with_capacity(bytes.len());
    remainder.extend_from_slice(&bytes[..pair.start_line.0]);
    remainder.extend_from_slice(&bytes[pair.end_line.1..]);
    Ok(sha256_prefixed(&remainder))
}

pub fn bounded_brief_content_id(bytes: &[u8]) -> Result<String, String> {
    let pair = find_marker_pair(bytes, MarkerKind::BoundedBrief)?;
    Ok(sha256_prefixed(&bytes[pair.start_line.1..pair.end_line.0]))
}

pub fn find_pair(bytes: &[u8], kind: MarkerKind) -> Result<MarkerPair, String> {
    find_marker_pair(bytes, kind)
}
