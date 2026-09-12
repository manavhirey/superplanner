//! Shared scalar validators, enums, and size limits per
//! `references/record-grammar.md`.

pub const SHA256_HEX: usize = 64;

pub const HANDOFF_MAX_BYTES: usize = 256 * 1024;
pub const BRIEF_MAX_BYTES: usize = 4 * 1024 * 1024;
pub const OPERATE_RECORD_MAX_BYTES: usize = 1024 * 1024;
pub const OPERATION_OUTPUT_LEAF_MAX_BYTES: usize = 64 * 1024 * 1024;
pub const REGISTERED_RECORD_MAX_BYTES: usize = 8 * 1024 * 1024;
pub const APPROVAL_ARTIFACT_MAX_BYTES: usize = 4 * 1024 * 1024;
pub const STATE_MAX_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeLimit {
    Handoff,
    Brief,
    OperateRecord,
    OperationOutputLeaf,
    RegisteredRecord,
    ApprovalArtifact,
    State,
}

impl SizeLimit {
    pub fn max_bytes(self) -> usize {
        match self {
            SizeLimit::Handoff => HANDOFF_MAX_BYTES,
            SizeLimit::Brief => BRIEF_MAX_BYTES,
            SizeLimit::OperateRecord => OPERATE_RECORD_MAX_BYTES,
            SizeLimit::OperationOutputLeaf => OPERATION_OUTPUT_LEAF_MAX_BYTES,
            SizeLimit::RegisteredRecord => REGISTERED_RECORD_MAX_BYTES,
            SizeLimit::ApprovalArtifact => APPROVAL_ARTIFACT_MAX_BYTES,
            SizeLimit::State => STATE_MAX_BYTES,
        }
    }

    pub fn check(self, len: usize) -> Result<(), String> {
        if len > self.max_bytes() {
            Err(format!(
                "payload of {} bytes exceeds the {:?} limit of {} bytes",
                len,
                self,
                self.max_bytes()
            ))
        } else {
            Ok(())
        }
    }
}

pub fn is_lowercase_hex(bytes: &[u8]) -> bool {
    bytes.len() == SHA256_HEX && bytes.iter().all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'))
}

fn is_lowercase_hex_of_len(bytes: &[u8], len: usize) -> bool {
    bytes.len() == len && bytes.iter().all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'))
}

pub fn validate_content_id_form(value: &str) -> Result<(), String> {
    let rest = value
        .strip_prefix("sha256:")
        .ok_or_else(|| "content-id form must start with sha256:".to_string())?;
    if !is_lowercase_hex(rest.as_bytes()) {
        return Err("content-id form must be sha256: followed by 64 lowercase hex".to_string());
    }
    Ok(())
}

pub fn validate_digest_form(value: &str) -> Result<(), String> {
    if !is_lowercase_hex(value.as_bytes()) {
        return Err("digest form must be exactly 64 lowercase hex".to_string());
    }
    Ok(())
}

/// A full non-`none` commit OID of the width bound to the repository object
/// format (`sha1` = 40 hex, `sha256` = 64 hex), per
/// `references/record-grammar.md` "Object-Identifier Width".
pub fn validate_git_sha_width(
    value: &str,
    format: crate::git_mediation::ObjectFormat,
    where_: &str,
) -> Result<(), String> {
    let width = format.oid_width();
    let bytes = value.as_bytes();
    if bytes.len() != width
        || !bytes.iter().all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'))
        || bytes.iter().all(|b| *b == b'0')
    {
        return Err(format!(
            "{where_} must be a full {width}-hex commit OID for {format:?}, got {value:?}"
        ));
    }
    Ok(())
}

pub fn validate_iso8601_z(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    let invalid = || format!("time must match YYYY-MM-DDTHH:MM:SSZ: {value:?}");
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return Err(invalid());
    }
    let digits =
        |range: std::ops::Range<usize>| -> bool { bytes[range].iter().all(|b| b.is_ascii_digit()) };
    if !(digits(0..4)
        && digits(5..7)
        && digits(8..10)
        && digits(11..13)
        && digits(14..16)
        && digits(17..19))
    {
        return Err(invalid());
    }
    let month = value[5..7].parse::<u8>().unwrap();
    let day = value[8..10].parse::<u8>().unwrap();
    let hour = value[11..13].parse::<u8>().unwrap();
    let minute = value[14..16].parse::<u8>().unwrap();
    let second = value[17..19].parse::<u8>().unwrap();
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return Err(invalid());
    }
    Ok(())
}

pub fn validate_git_date(value: &str) -> Result<(), String> {
    let invalid = || format!("git date must match <unix-seconds> <+|-HHMM>: {value:?}");
    let Some((seconds, zone)) = value.split_once(' ') else {
        return Err(invalid());
    };
    if seconds.is_empty() || !seconds.bytes().all(|b| b.is_ascii_digit()) {
        return Err(invalid());
    }
    if seconds.len() > 1 && seconds.starts_with('0') {
        return Err(invalid());
    }
    if zone.len() != 5 {
        return Err(invalid());
    }
    let zone_bytes = zone.as_bytes();
    let sign = zone_bytes[0];
    if sign != b'+' && sign != b'-' {
        return Err(invalid());
    }
    if !zone_bytes[1..].iter().all(|b| b.is_ascii_digit()) {
        return Err(invalid());
    }
    if &zone[1..3] > "23" || &zone[3..5] > "59" {
        return Err(invalid());
    }
    if sign == b'-' && &zone[1..] == "0000" {
        return Err(invalid());
    }
    Ok(())
}

pub fn validate_agent_id(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() > 64 {
        return Err(format!("agent_id too long or empty: {value:?}"));
    }
    if !bytes[0].is_ascii_lowercase() && !bytes[0].is_ascii_digit() {
        return Err(format!("agent_id must start with [a-z0-9]: {value:?}"));
    }
    let allowed = |b: &u8| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-' || *b == b'.';
    if !bytes.iter().all(allowed) {
        return Err(format!("agent_id must match [a-z0-9-.]: {value:?}"));
    }
    if *bytes.last().unwrap() == b'.' || value.contains("..") {
        return Err(format!(
            "agent_id must not end with a dot or contain consecutive dots: {value:?}"
        ));
    }
    Ok(())
}

pub fn validate_sp_id(kind: &str, value: &str) -> Result<(), String> {
    let allowed = ["operation", "record", "dispatch", "review", "finding"];
    if !allowed.contains(&kind) {
        return Err(format!("unknown sp id kind: {kind:?}"));
    }
    let prefix = format!("sp-{kind}-");
    let rest = value
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("id must start with {prefix:?}: {value:?}"))?;
    if !is_lowercase_hex_of_len(rest.as_bytes(), 32) {
        return Err(format!("id must end with 32 lowercase hex: {value:?}"));
    }
    Ok(())
}

pub fn validate_lease_id(value: &str) -> Result<(), String> {
    let rest = value
        .strip_prefix("sp-lease-")
        .ok_or_else(|| format!("lease ID must start with sp-lease-: {value:?}"))?;
    if !is_lowercase_hex_of_len(rest.as_bytes(), 32) {
        return Err(format!(
            "lease ID must end with 32 lowercase hex: {value:?}"
        ));
    }
    Ok(())
}

pub fn validate_session_id(value: &str) -> Result<(), String> {
    let rest = value
        .strip_prefix("ses_")
        .ok_or_else(|| format!("session id must start with ses_: {value:?}"))?;
    if rest.is_empty()
        || !rest
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(format!(
            "session id must match ^ses_[A-Za-z0-9_-]+$: {value:?}"
        ));
    }
    Ok(())
}

pub fn validate_task_id(value: &str) -> Result<(), String> {
    let three_digits = |s: &str| -> bool { s.len() == 3 && s.bytes().all(|b| b.is_ascii_digit()) };
    if let Some((feature, task)) = value.split_once('/') {
        let task_ok = match task.split_once('-') {
            Some((num, _)) => task.starts_with('T') && three_digits(&num[1..]),
            None => task.len() == 4 && task.starts_with('T') && three_digits(&task[1..]),
        };
        let feature_ok =
            feature.len() == 4 && feature.starts_with('F') && three_digits(&feature[1..]);
        if feature_ok && task_ok {
            return Ok(());
        }
    } else if let Some((task, slug)) = value.split_once('-') {
        if task.starts_with('T')
            && three_digits(&task[1..])
            && !slug.is_empty()
            && slug
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Ok(());
        }
    } else if value.len() == 4 && value.starts_with('T') && three_digits(&value[1..]) {
        return Ok(());
    }
    Err(format!(
        "task_id must be F<nnn>/T<nnn> or T<nnn>-<slug>: {value:?}"
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffStatus {
    Complete,
    Blocked,
}

impl HandoffStatus {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "complete" => Ok(HandoffStatus::Complete),
            "blocked" => Ok(HandoffStatus::Blocked),
            other => Err(format!("handoff status must be complete or blocked, not {other:?} (timeout is normalized to blocked)")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewResult {
    Approved,
    Findings,
    NotCompleted,
}

impl ReviewResult {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "approved" => Ok(ReviewResult::Approved),
            "findings" => Ok(ReviewResult::Findings),
            "not-completed" => Ok(ReviewResult::NotCompleted),
            other => Err(format!(
                "review_result must be approved, findings, or not-completed, not {other:?}"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Blocker,
    Major,
    Minor,
    Info,
}

impl Severity {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "blocker" => Ok(Severity::Blocker),
            "major" => Ok(Severity::Major),
            "minor" => Ok(Severity::Minor),
            "info" => Ok(Severity::Info),
            other => Err(format!(
                "severity must be blocker, major, minor, or info, not {other:?}"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvocationState {
    NotStarted,
    Running,
    Ended,
    TimedOut,
    CancellationConfirmed,
}

impl InvocationState {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "not-started" => Ok(InvocationState::NotStarted),
            "running" => Ok(InvocationState::Running),
            "ended" => Ok(InvocationState::Ended),
            "timed-out" => Ok(InvocationState::TimedOut),
            "cancellation-confirmed" => Ok(InvocationState::CancellationConfirmed),
            other => Err(format!("unknown invocation state {other:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringEventType {
    Dispatch,
    Checkpoint,
    Handoff,
    Timeout,
    Resume,
    LiveCheck,
    CancelRequested,
    CancellationConfirmed,
    InvocationEnded,
    Replacement,
    Escalated,
}

impl MonitoringEventType {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "dispatch" => Ok(MonitoringEventType::Dispatch),
            "checkpoint" => Ok(MonitoringEventType::Checkpoint),
            "handoff" => Ok(MonitoringEventType::Handoff),
            "timeout" => Ok(MonitoringEventType::Timeout),
            "resume" => Ok(MonitoringEventType::Resume),
            "live-check" => Ok(MonitoringEventType::LiveCheck),
            "cancel-requested" => Ok(MonitoringEventType::CancelRequested),
            "cancellation-confirmed" => Ok(MonitoringEventType::CancellationConfirmed),
            "invocation-ended" => Ok(MonitoringEventType::InvocationEnded),
            "replacement" => Ok(MonitoringEventType::Replacement),
            "escalated" => Ok(MonitoringEventType::Escalated),
            other => Err(format!("unknown monitoring event type {other:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseState {
    Prepared,
    Active,
    Retiring,
    CleanupFailed,
    Retired,
}

impl LeaseState {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "prepared" => Ok(LeaseState::Prepared),
            "active" => Ok(LeaseState::Active),
            "retiring" => Ok(LeaseState::Retiring),
            "cleanup-failed" => Ok(LeaseState::CleanupFailed),
            "retired" => Ok(LeaseState::Retired),
            other => Err(format!("unknown lease state {other:?}")),
        }
    }
}
