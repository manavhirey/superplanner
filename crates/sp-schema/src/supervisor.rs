//! Supervisor `spawn`/`resume` request and result schemas, the `operate`
//! envelope, and the canonical model-route table per
//! `references/handoff-contract.md` and `references/model-routing.md`.
//!
//! Per-operation payload schemas for `operate` land with their owning
//! issues (#9-#13); this module owns the envelope and the lifecycle
//! records, validated as canonical JSON with unknown-field rejection.

use serde::{Deserialize, Serialize};

use crate::core::{validate_agent_id, validate_session_id};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrivateRoots {
    pub config: String,
    pub data: String,
    pub cache: String,
    pub state: String,
    pub temp: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillEntry {
    pub name: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderManifest {
    pub provider: String,
    pub package: String,
    pub version: String,
    pub sha256: String,
    pub broker_endpoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IsolationEvidence {
    pub mode: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpawnRequest {
    pub agent_id: String,
    pub model: String,
    pub variant: String,
    pub worktree: String,
    pub private_home: String,
    pub private_roots: PrivateRoots,
    pub opencode_path: String,
    pub opencode_sha256: String,
    pub brief: String,
    pub brief_sha256: String,
    pub source_agent_id: String,
    pub source_agent_sha256: String,
    pub runtime_wrapper: String,
    pub runtime_wrapper_sha256: String,
    pub permission_sha256: String,
    pub skills: Vec<SkillEntry>,
    pub provider: ProviderManifest,
    pub credential_manifest: Vec<String>,
    pub sandbox_profile: String,
    pub isolation: IsolationEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResumeRequest {
    pub spawn: SpawnRequest,
    pub opencode_session_id: String,
    pub resume_evidence_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessStatus {
    Exited,
    Timeout,
    Signaled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PrincipalVerdict {
    Pass,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpawnResponse {
    pub opencode_session_id: String,
    pub observed_agent_id: String,
    pub handoff_bytes: String,
    pub process_status: ProcessStatus,
    pub principal_verdict: PrincipalVerdict,
    pub principal_evidence: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperateOperation {
    SafeGit,
    SandboxCommand,
    Package,
    Integrate,
    Verify,
    CommitPrepare,
    Commit,
    PushPrepare,
    PushPresent,
    PushRetire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperateStatus {
    Ok,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperateOutput {
    pub path: String,
    pub identity: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperateResult {
    pub operation_id: String,
    pub request_sha256: String,
    pub status: OperateStatus,
    pub outputs: Vec<OperateOutput>,
    pub evidence: String,
    pub state_transition: String,
}

pub const ROUTE_TABLE: [(&str, [&str; 3]); 3] = [
    ("openai/gpt-5.6-sol", ["xhigh", "high", "medium"]),
    ("zai/glm-5.3", ["max", "", ""]),
    ("openrouter/moonshotai/kimi-k3", ["max", "", ""]),
];

pub fn validate_route(model: &str, variant: &str) -> Result<(), String> {
    for (candidate, variants) in ROUTE_TABLE {
        if candidate == model {
            if variants.contains(&variant) {
                return Ok(());
            }
            return Err(format!(
                "variant {variant:?} is not valid for {model:?}; allowed: {variants:?}"
            ));
        }
    }
    Err(format!("model {model:?} is not a canonical route"))
}

fn validate_absolute(value: &str, where_: &str) -> Result<(), String> {
    if !value.starts_with('/') {
        return Err(format!("{where_} must be an absolute path: {value:?}"));
    }
    Ok(())
}

impl SpawnRequest {
    pub fn validate(&self) -> Result<(), String> {
        validate_agent_id(&self.agent_id)?;
        validate_route(&self.model, &self.variant)?;
        validate_absolute(&self.worktree, "worktree")?;
        validate_absolute(&self.private_home, "private_home")?;
        let roots = [
            (&self.private_roots.config, "config"),
            (&self.private_roots.data, "data"),
            (&self.private_roots.cache, "cache"),
            (&self.private_roots.state, "state"),
            (&self.private_roots.temp, "temp"),
        ];
        for (path, name) in roots {
            validate_absolute(path, &format!("private_roots.{name}"))?;
        }
        validate_absolute(&self.opencode_path, "opencode_path")?;
        crate::core::validate_digest_form(&self.opencode_sha256)?;
        crate::core::validate_digest_form(&self.brief_sha256)?;
        validate_agent_id(&self.source_agent_id)?;
        crate::core::validate_digest_form(&self.source_agent_sha256)?;
        validate_agent_id(&self.runtime_wrapper)?;
        crate::core::validate_digest_form(&self.runtime_wrapper_sha256)?;
        crate::core::validate_digest_form(&self.permission_sha256)?;
        if self.skills.is_empty() {
            return Err("skills manifest may be empty only when the agent allowlist is empty; it must still be present".to_string());
        }
        for skill in &self.skills {
            validate_agent_id(&format!("{}-x", skill.name.replace('_', "-")))
                .map_err(|_| format!("skill name {skill:?} is not a canonical skill id"))?;
            validate_absolute(&skill.path, "skill.path")?;
            crate::core::validate_digest_form(&skill.sha256)?;
        }
        crate::core::validate_digest_form(&self.provider.sha256)?;
        if self.provider.broker_endpoint.is_empty() {
            return Err("broker_endpoint must not be empty".to_string());
        }
        for path in &self.credential_manifest {
            validate_absolute(path, "credential_manifest entry")?;
        }
        if !matches!(
            self.isolation.mode.as_str(),
            "distinct-principal" | "kernel-enforced-same-principal"
        ) {
            return Err(format!(
                "isolation mode must be distinct-principal or kernel-enforced-same-principal, not {:?}",
                self.isolation.mode
            ));
        }
        if self.isolation.evidence.is_empty() {
            return Err("isolation evidence must not be empty".to_string());
        }
        if self.sandbox_profile.is_empty() {
            return Err("sandbox_profile must not be empty".to_string());
        }
        Ok(())
    }
}

impl ResumeRequest {
    pub fn validate(&self) -> Result<(), String> {
        self.spawn.validate()?;
        validate_session_id(&self.opencode_session_id)?;
        crate::core::validate_digest_form(&self.resume_evidence_sha256)?;
        Ok(())
    }
}

impl SpawnResponse {
    pub fn validate(&self) -> Result<(), String> {
        validate_session_id(&self.opencode_session_id)?;
        validate_agent_id(&self.observed_agent_id)?;
        if self.principal_verdict != PrincipalVerdict::Pass {
            return Err(
                "a returned response with a failing verdict must block startup".to_string(),
            );
        }
        if self.principal_evidence.is_empty() {
            return Err("principal evidence must not be empty".to_string());
        }
        Ok(())
    }
}

impl OperateResult {
    pub fn validate(&self) -> Result<(), String> {
        crate::core::validate_sp_id("operation", &self.operation_id)?;
        crate::core::validate_digest_form(&self.request_sha256)?;
        for output in &self.outputs {
            validate_absolute(&output.path, "output.path")?;
            crate::core::validate_digest_form(&output.sha256)?;
            if output.identity.is_empty() {
                return Err("output.identity must not be empty".to_string());
            }
        }
        if self.evidence.is_empty() || self.state_transition.is_empty() {
            return Err("operate results must carry evidence and a state transition".to_string());
        }
        Ok(())
    }
}

pub fn parse_spawn_request(json: &str) -> Result<SpawnRequest, String> {
    let request: SpawnRequest =
        serde_json::from_str(json).map_err(|e| format!("spawn request rejected: {e}"))?;
    request.validate()?;
    Ok(request)
}

pub fn parse_resume_request(json: &str) -> Result<ResumeRequest, String> {
    let request: ResumeRequest =
        serde_json::from_str(json).map_err(|e| format!("resume request rejected: {e}"))?;
    request.validate()?;
    Ok(request)
}

pub fn parse_spawn_response(json: &str) -> Result<SpawnResponse, String> {
    let response: SpawnResponse =
        serde_json::from_str(json).map_err(|e| format!("spawn response rejected: {e}"))?;
    response.validate()?;
    Ok(response)
}
