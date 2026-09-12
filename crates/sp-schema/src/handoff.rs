//! Canonical handoff parsing per `references/handoff-contract.md`,
//! `references/templates/handoff.md`, and the finding grammar in
//! `references/record-grammar.md`.
//!
//! The parser accepts exactly the documented field order and shapes; any
//! deviation (missing, extra, duplicated, misordered, or empty field;
//! empty list; worker carrying the reviewer extension; reviewer claiming
//! changed files) fails closed.

use crate::core::{
    validate_agent_id, validate_iso8601_z, validate_sp_id, validate_task_id, HandoffStatus,
    ReviewResult, Severity, SizeLimit,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Worker,
    Reviewer,
}

impl Role {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "worker" => Ok(Role::Worker),
            "reviewer" => Ok(Role::Reviewer),
            other => Err(format!("role must be worker or reviewer, not {other:?}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verification {
    pub command: String,
    pub result: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub finding_id: String,
    pub severity: Severity,
    pub file: String,
    pub line: String,
    pub criterion: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedRisk {
    pub finding_id: String,
    pub reviewed_sha: String,
    pub scope: String,
    pub user_evidence: String,
    pub consequence: String,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewExtension {
    pub review_result: ReviewResult,
    pub reviewed_sha: String,
    pub findings: Vec<Finding>,
    pub accepted_risks: Vec<AcceptedRisk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdversarialExtension {
    pub standard_approval_record_id: String,
    pub standard_approval_record_identity: String,
    pub standard_approval_record_sha256: String,
    pub standard_approval_sha: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationResultFields {
    pub base_sha: String,
    pub start_tree: String,
    pub packaged_tree: String,
    pub packaging_index: String,
    pub packaging_objects: String,
    pub setup_objects: String,
    pub setup_alternates: String,
    pub source_setup_context: String,
    pub baseline_objects: String,
    pub baseline_construction_manifest: String,
    pub baseline_no_hardlink_proof: String,
    pub alternate_absence_manifest: String,
    pub namespace_snapshot_manifests: String,
    pub candidate_objects: String,
    pub owned_paths_file: String,
    pub owned_paths_sha256: String,
    pub patch_file: String,
    pub patch_sha256: String,
    pub manifest_file: String,
    pub manifest_sha256: String,
    pub object_format: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrationResult {
    None,
    Full(Box<IntegrationResultFields>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffRecord {
    pub task_id: String,
    pub agent_id: String,
    pub role: Role,
    pub status: HandoffStatus,
    pub worktree: String,
    pub git_sha: String,
    pub completion_time: String,
    pub scope: Vec<String>,
    pub artifacts: Vec<String>,
    pub changed_files: Vec<String>,
    pub verification: Vec<Verification>,
    pub blockers: Vec<String>,
    pub assumptions: Vec<String>,
    pub next_action: String,
    pub review: Option<ReviewExtension>,
    pub adversarial: Option<AdversarialExtension>,
    pub integration_result: Option<IntegrationResult>,
}

const INTEGRATION_ROLES: [&str; 3] = [
    "superplanner.builder",
    "superplanner.debugger",
    "superplanner.documenter",
];

const INTEGRATION_FIELDS: [&str; 21] = [
    "base_sha",
    "start_tree",
    "packaged_tree",
    "packaging_index",
    "packaging_objects",
    "setup_objects",
    "setup_alternates",
    "source_setup_context",
    "baseline_objects",
    "baseline_construction_manifest",
    "baseline_no_hardlink_proof",
    "alternate_absence_manifest",
    "namespace_snapshot_manifests",
    "candidate_objects",
    "owned_paths_file",
    "owned_paths_sha256",
    "patch_file",
    "patch_sha256",
    "manifest_file",
    "manifest_sha256",
    "object_format",
];

struct Cursor {
    lines: Vec<String>,
    pos: usize,
}

fn strip_final_newline(lines: &mut Vec<String>) {
    if lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
}

impl Cursor {
    fn err<T>(&self, message: String) -> Result<T, String> {
        Err(format!("line {}: {message}", self.pos + 1))
    }

    fn peek(&self) -> Option<&str> {
        self.lines.get(self.pos).map(String::as_str)
    }

    fn scalar(&mut self, key: &str) -> Result<String, String> {
        let line = match self.peek() {
            Some(l) => l.to_string(),
            None => return Err(format!("unexpected end of handoff, expected {key:?}")),
        };
        let prefix = format!("{key}: ");
        let value = match line.strip_prefix(&prefix) {
            Some(v) => v,
            None => {
                self.pos += 1;
                return self.err(format!("expected {key:?}: <value>, found {line:?}"));
            }
        };
        self.pos += 1;
        if value.is_empty() || value.trim() != value {
            return self.err(format!("{key:?} value is empty or padded: {value:?}"));
        }
        if value.bytes().any(|b| b.is_ascii_control()) {
            return self.err(format!("{key:?} value carries control bytes"));
        }
        Ok(value.to_string())
    }

    fn list_items(&mut self, key: &str) -> Result<Vec<String>, String> {
        let line = match self.peek() {
            Some(l) => l.to_string(),
            None => return Err(format!("unexpected end of handoff, expected {key:?}")),
        };
        if line != format!("{key}:") {
            self.pos += 1;
            return self.err(format!("expected {key:?} list header, found {line:?}"));
        }
        self.pos += 1;
        let mut items = Vec::new();
        while let Some(next) = self.peek() {
            if let Some(item) = next.strip_prefix("  - ") {
                if item.is_empty() || item.trim() != item {
                    return self.err(format!("{key:?} item is empty or padded: {item:?}"));
                }
                if item.bytes().any(|b| b.is_ascii_control()) {
                    return self.err(format!("{key:?} item carries control bytes"));
                }
                items.push(item.to_string());
                self.pos += 1;
            } else {
                break;
            }
        }
        if items.is_empty() {
            return self.err(format!(
                "{key:?} must list at least one item, or the sole item none"
            ));
        }
        if items.len() == 1 && items[0] == "none" {
            return Ok(Vec::new());
        }
        Ok(items)
    }

    fn record_list<const N: usize>(
        &mut self,
        key: &str,
        fields: [&str; N],
    ) -> Result<Vec<[String; N]>, String> {
        let line = match self.peek() {
            Some(l) => l.to_string(),
            None => return Err(format!("unexpected end of handoff, expected {key:?}")),
        };
        if line != format!("{key}:") {
            self.pos += 1;
            return self.err(format!("expected {key:?} list header, found {line:?}"));
        }
        self.pos += 1;
        self.record_items(key, fields)
    }

    fn maybe_none_record_list<const N: usize>(
        &mut self,
        key: &str,
        fields: [&str; N],
    ) -> Result<Vec<[String; N]>, String> {
        let line = match self.peek() {
            Some(l) => l.to_string(),
            None => return Err(format!("unexpected end of handoff, expected {key:?}")),
        };
        if line != format!("{key}:") {
            self.pos += 1;
            return self.err(format!("expected {key:?} list header, found {line:?}"));
        }
        self.pos += 1;
        if self.peek() == Some("  - none") {
            self.pos += 1;
            if self.peek().is_some_and(|l| l.starts_with("  - ")) {
                return self.err(format!("{key:?} none must be the sole list item"));
            }
            return Ok(Vec::new());
        }
        self.record_items(key, fields)
    }

    fn record_items<const N: usize>(
        &mut self,
        key: &str,
        fields: [&str; N],
    ) -> Result<Vec<[String; N]>, String> {
        let mut records = Vec::new();
        while self.peek().is_some_and(|l| l.starts_with("  - ")) {
            let mut values: [String; N] = (0..N)
                .map(|_| String::new())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let first = self.peek().unwrap().strip_prefix("  - ").unwrap();
            let first_prefix = format!("{}: ", fields[0]);
            let first_value = first.strip_prefix(&first_prefix).ok_or_else(|| {
                format!(
                    "line {}: {} item must start with {:?}",
                    self.pos + 1,
                    key,
                    first_prefix
                )
            })?;
            if first_value.is_empty() {
                return self.err(format!("{key:?} field {:?} is empty", fields[0]));
            }
            values[0] = first_value.to_string();
            self.pos += 1;
            for (index, field) in fields.iter().enumerate().skip(1) {
                let continuation = match self.peek() {
                    Some(l) => l.to_string(),
                    None => return Err(format!("unexpected end inside {key:?} record")),
                };
                let prefix = format!("    {field}: ");
                let value = continuation.strip_prefix(&prefix).ok_or_else(|| {
                    format!(
                        "line {}: expected {:?} continuation inside {key:?}, found {continuation:?}",
                        self.pos + 1, field
                    )
                })?;
                if value.is_empty() {
                    return self.err(format!("{key:?} field {field:?} is empty"));
                }
                if value.bytes().any(|b| b.is_ascii_control()) {
                    return self.err(format!("{key:?} field {field:?} carries control bytes"));
                }
                values[index] = value.to_string();
                self.pos += 1;
            }
            if values.iter().any(|v| v.trim() != *v) {
                return self.err(format!("{key:?} record carries padded values"));
            }
            records.push(values);
        }
        if records.is_empty() {
            return self.err(format!(
                "{key:?} must contain records or the sole item none"
            ));
        }
        Ok(records)
    }
}

fn validate_git_sha(value: &str, where_: &str) -> Result<(), String> {
    if value == "none" {
        return Ok(());
    }
    let bytes = value.as_bytes();
    let width_ok = bytes.len() == 40 || bytes.len() == 64;
    let hex_ok = bytes.iter().all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'));
    if !width_ok || !hex_ok {
        return Err(format!(
            "{where_} must be a full 40- or 64-hex OID or none, got {value:?}"
        ));
    }
    Ok(())
}

fn validate_absolute(value: &str, where_: &str) -> Result<(), String> {
    if !value.starts_with('/') {
        return Err(format!("{where_} must be an absolute path: {value:?}"));
    }
    Ok(())
}

pub fn parse_handoff(bytes: &[u8]) -> Result<HandoffRecord, String> {
    SizeLimit::Handoff.check(bytes.len())?;
    let text = std::str::from_utf8(bytes).map_err(|_| "handoff must be valid UTF-8".to_string())?;
    let mut lines: Vec<String> = text
        .split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l).to_string())
        .collect();
    strip_final_newline(&mut lines);
    let mut cursor = Cursor { lines, pos: 0 };

    let task_id = cursor.scalar("task_id")?;
    let agent_id = cursor.scalar("agent_id")?;
    let role = Role::parse(&cursor.scalar("role")?)?;
    let status = HandoffStatus::parse(&cursor.scalar("status")?)?;
    let worktree = cursor.scalar("worktree")?;
    let git_sha = cursor.scalar("git_sha")?;
    let completion_time = cursor.scalar("completion_time")?;
    let scope = cursor.list_items("scope")?;
    let artifacts = cursor.list_items("artifacts")?;
    let changed_files = cursor.list_items("changed_files")?;
    let verification_records =
        cursor.record_list("verification", ["command", "result", "evidence"])?;
    let blockers = cursor.list_items("blockers")?;
    let assumptions = cursor.list_items("assumptions")?;
    let next_action = cursor.scalar("next_action")?;

    let mut review = None;
    let mut adversarial = None;
    let mut integration_result = None;

    if cursor
        .peek()
        .is_some_and(|l| l.starts_with("integration_result:"))
    {
        if role != Role::Worker {
            return Err("integration_result is only valid on worker handoffs".to_string());
        }
        if !INTEGRATION_ROLES.contains(&agent_id.as_str()) {
            return Err(format!(
                "agent {agent_id:?} may not carry integration_result; only builder, debugger, or documenter"
            ));
        }
        let line = cursor.peek().unwrap().to_string();
        if line == "integration_result: none" {
            cursor.pos += 1;
            integration_result = Some(IntegrationResult::None);
        } else if line == "integration_result:" {
            cursor.pos += 1;
            let mut fields: Vec<String> = Vec::with_capacity(INTEGRATION_FIELDS.len());
            for field in INTEGRATION_FIELDS {
                let continuation = match cursor.peek() {
                    Some(l) => l.to_string(),
                    None => return Err("unexpected end inside integration_result".to_string()),
                };
                let prefix = format!("  {field}: ");
                let value = continuation.strip_prefix(&prefix).ok_or_else(|| {
                    format!(
                        "line {}: expected integration_result field {:?}, found {continuation:?}",
                        cursor.pos + 1,
                        field
                    )
                })?;
                if value.is_empty() {
                    return Err(format!("integration_result field {field:?} is empty"));
                }
                fields.push(value.to_string());
                cursor.pos += 1;
            }
            integration_result = Some(IntegrationResult::Full(Box::new(IntegrationResultFields {
                base_sha: fields[0].clone(),
                start_tree: fields[1].clone(),
                packaged_tree: fields[2].clone(),
                packaging_index: fields[3].clone(),
                packaging_objects: fields[4].clone(),
                setup_objects: fields[5].clone(),
                setup_alternates: fields[6].clone(),
                source_setup_context: fields[7].clone(),
                baseline_objects: fields[8].clone(),
                baseline_construction_manifest: fields[9].clone(),
                baseline_no_hardlink_proof: fields[10].clone(),
                alternate_absence_manifest: fields[11].clone(),
                namespace_snapshot_manifests: fields[12].clone(),
                candidate_objects: fields[13].clone(),
                owned_paths_file: fields[14].clone(),
                owned_paths_sha256: fields[15].clone(),
                patch_file: fields[16].clone(),
                patch_sha256: fields[17].clone(),
                manifest_file: fields[18].clone(),
                manifest_sha256: fields[19].clone(),
                object_format: fields[20].clone(),
            })));
        } else {
            cursor.pos += 1;
            return Err(format!(
                "line {}: integration_result must be a record or the literal none",
                cursor.pos
            ));
        }
    }

    if cursor
        .peek()
        .is_some_and(|l| l.starts_with("review_result:"))
    {
        if role != Role::Reviewer {
            return Err("worker handoffs must not carry the reviewer extension".to_string());
        }
        let review_result = ReviewResult::parse(&cursor.scalar("review_result")?)?;
        let reviewed_sha = cursor.scalar("reviewed_sha")?;
        let finding_records = cursor.maybe_none_record_list(
            "findings",
            [
                "finding_id",
                "severity",
                "file",
                "line",
                "criterion",
                "evidence",
            ],
        )?;
        let mut findings = Vec::new();
        for [finding_id, severity, file, line, criterion, evidence] in finding_records {
            validate_sp_id("finding", &finding_id).map_err(|e| format!("findings: {e}"))?;
            let severity = Severity::parse(&severity).map_err(|e| format!("findings: {e}"))?;
            if line != "none" && !line.bytes().all(|b| b.is_ascii_digit()) {
                return Err(format!(
                    "findings: line must be a positive integer or none, got {line:?}"
                ));
            }
            if file.starts_with('/') {
                return Err("findings: file must be worktree-relative".to_string());
            }
            findings.push(Finding {
                finding_id,
                severity,
                file,
                line,
                criterion,
                evidence,
            });
        }
        let risk_records = cursor.maybe_none_record_list(
            "accepted_risks",
            [
                "finding_id",
                "reviewed_sha",
                "scope",
                "user_evidence",
                "consequence",
                "rationale",
            ],
        )?;
        let mut accepted_risks = Vec::new();
        for [finding_id, reviewed_sha, scope, user_evidence, consequence, rationale] in risk_records
        {
            validate_sp_id("finding", &finding_id).map_err(|e| format!("accepted_risks: {e}"))?;
            validate_git_sha(&reviewed_sha, "accepted_risks.reviewed_sha")?;
            accepted_risks.push(AcceptedRisk {
                finding_id,
                reviewed_sha,
                scope,
                user_evidence,
                consequence,
                rationale,
            });
        }
        review = Some(ReviewExtension {
            review_result,
            reviewed_sha,
            findings,
            accepted_risks,
        });

        if cursor
            .peek()
            .is_some_and(|l| l.starts_with("standard_approval_record_id:"))
        {
            let standard_approval_record_id = cursor.scalar("standard_approval_record_id")?;
            let standard_approval_record_identity =
                cursor.scalar("standard_approval_record_identity")?;
            let standard_approval_record_sha256 =
                cursor.scalar("standard_approval_record_sha256")?;
            let standard_approval_sha = cursor.scalar("standard_approval_sha")?;
            adversarial = Some(AdversarialExtension {
                standard_approval_record_id,
                standard_approval_record_identity,
                standard_approval_record_sha256,
                standard_approval_sha,
            });
        }
    }

    if let Some(line) = cursor.peek() {
        return Err(format!(
            "line {}: unexpected trailing handoff content: {line:?}",
            cursor.pos + 1
        ));
    }

    let record = HandoffRecord {
        task_id,
        agent_id,
        role,
        status,
        worktree,
        git_sha,
        completion_time,
        scope,
        artifacts,
        changed_files,
        verification: verification_records
            .into_iter()
            .map(|[command, result, evidence]| Verification {
                command,
                result,
                evidence,
            })
            .collect(),
        blockers,
        assumptions,
        next_action,
        review,
        adversarial,
        integration_result,
    };
    validate_record(&record)?;
    Ok(record)
}

fn validate_record(record: &HandoffRecord) -> Result<(), String> {
    validate_task_id(&record.task_id)?;
    validate_agent_id(&record.agent_id)?;
    validate_git_sha(&record.git_sha, "git_sha")?;
    validate_iso8601_z(&record.completion_time)?;
    validate_absolute(&record.worktree, "worktree")?;
    for verification in &record.verification {
        if !matches!(
            verification.result.as_str(),
            "passed" | "failed" | "not-run"
        ) {
            return Err(format!(
                "verification result must be passed, failed, or not-run, not {:?}",
                verification.result
            ));
        }
    }
    match record.status {
        HandoffStatus::Complete => {
            if !record.blockers.is_empty() {
                return Err("a complete handoff must carry blockers: [none]".to_string());
            }
        }
        HandoffStatus::Blocked => {
            if record.blockers.is_empty() {
                return Err("a blocked handoff must name at least one blocker".to_string());
            }
        }
    }
    if let Some(review) = &record.review {
        if !record.changed_files.is_empty() {
            return Err("a reviewer handoff must carry changed_files: [none]".to_string());
        }
        validate_git_sha(&review.reviewed_sha, "reviewed_sha")?;
        match (record.status, review.review_result) {
            (HandoffStatus::Complete, ReviewResult::NotCompleted) => {
                return Err("a completed review must not report not-completed".to_string());
            }
            (HandoffStatus::Blocked, ReviewResult::Approved)
            | (HandoffStatus::Blocked, ReviewResult::Findings) => {
                return Err("a blocked review must report not-completed".to_string());
            }
            _ => {}
        }
        if record.status == HandoffStatus::Complete {
            if review.reviewed_sha == "none" {
                return Err("a completed review must name the reviewed SHA".to_string());
            }
            if review.reviewed_sha != record.git_sha {
                return Err("for a completed review, reviewed_sha must equal git_sha".to_string());
            }
            let finding_ids: Vec<&str> = review
                .findings
                .iter()
                .map(|f| f.finding_id.as_str())
                .collect();
            for risk in &review.accepted_risks {
                if risk.reviewed_sha != review.reviewed_sha {
                    return Err(format!(
                        "accepted risk {:?} must bind the same reviewed_sha as the handoff",
                        risk.finding_id
                    ));
                }
                if !finding_ids.contains(&risk.finding_id.as_str()) {
                    return Err(format!(
                        "accepted risk {:?} must reference a visible finding",
                        risk.finding_id
                    ));
                }
            }
            if review.review_result == ReviewResult::Approved {
                let accepted: Vec<&str> = review
                    .accepted_risks
                    .iter()
                    .map(|r| r.finding_id.as_str())
                    .collect();
                for finding in &review.findings {
                    if !accepted.contains(&finding.finding_id.as_str()) {
                        return Err(format!(
                            "approval with findings requires every finding accepted or resolved; {:?} is neither",
                            finding.finding_id
                        ));
                    }
                }
            }
        }
        if let Some(adversarial) = &record.adversarial {
            if record.status == HandoffStatus::Complete {
                if adversarial.standard_approval_record_id == "none"
                    || adversarial.standard_approval_record_identity == "none"
                    || adversarial.standard_approval_record_sha256 == "none"
                    || adversarial.standard_approval_sha == "none"
                {
                    return Err(
                        "a completed adversarial review must bind the standard approval record"
                            .to_string(),
                    );
                }
                validate_sp_id("record", &adversarial.standard_approval_record_id)?;
                crate::core::validate_digest_form(&adversarial.standard_approval_record_sha256)?;
                if adversarial.standard_approval_sha != review.reviewed_sha
                    || adversarial.standard_approval_sha != record.git_sha
                {
                    return Err(
                        "standard_approval_sha, reviewed_sha, and git_sha must be the same commit"
                            .to_string(),
                    );
                }
            }
        }
    } else if record.adversarial.is_some() {
        return Err("the adversarial extension requires the reviewer extension".to_string());
    }
    if let Some(IntegrationResult::Full(fields)) = &record.integration_result {
        if !matches!(fields.object_format.as_str(), "sha1" | "sha256") {
            return Err(format!(
                "object_format must be sha1 or sha256, not {:?}",
                fields.object_format
            ));
        }
        let oid_width = if fields.object_format == "sha1" {
            40
        } else {
            64
        };
        for (value, name) in [
            (&fields.base_sha, "base_sha"),
            (&fields.start_tree, "start_tree"),
            (&fields.packaged_tree, "packaged_tree"),
        ] {
            let bytes = value.as_bytes();
            if bytes.len() != oid_width
                || !bytes.iter().all(|b| matches!(b, b'a'..=b'f' | b'0'..=b'9'))
            {
                return Err(format!(
                    "integration_result {name} must be a full {oid_width}-hex OID of the declared format"
                ));
            }
        }
        for (value, name) in [
            (&fields.owned_paths_sha256, "owned_paths_sha256"),
            (&fields.patch_sha256, "patch_sha256"),
            (&fields.manifest_sha256, "manifest_sha256"),
        ] {
            crate::core::validate_digest_form(value)
                .map_err(|e| format!("integration_result {name}: {e}"))?;
        }
        for (value, name) in [
            (&fields.packaging_index, "packaging_index"),
            (&fields.packaging_objects, "packaging_objects"),
            (&fields.setup_objects, "setup_objects"),
            (&fields.baseline_objects, "baseline_objects"),
            (&fields.candidate_objects, "candidate_objects"),
            (&fields.owned_paths_file, "owned_paths_file"),
            (&fields.patch_file, "patch_file"),
            (&fields.manifest_file, "manifest_file"),
        ] {
            validate_absolute(value, &format!("integration_result {name}"))?;
        }
    }
    Ok(())
}
