//! Supervisor `spawn`/`resume` request and result schemas, the `operate`
//! envelope, and the canonical model-route table per
//! `references/handoff-contract.md` and `references/model-routing.md`.
//!
//! Per-operation payload schemas for `operate` land with their owning
//! issues (#9-#13); this module owns the envelope and the lifecycle
//! records, validated as canonical JSON with unknown-field rejection.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::core::{validate_agent_id, validate_session_id};
use crate::model_routing::{provider_for_model, validate_route_shape, ValidatedModelPolicy};

pub const RESUME_ENVELOPE_RESERVED_BYTES: usize = 1024;
pub const SPAWN_REQUEST_MAX_BYTES: usize =
    crate::core::REGISTERED_RECORD_MAX_BYTES - RESUME_ENVELOPE_RESERVED_BYTES;
pub const MODEL_ROUTE_READINESS_TAG: &str = "model-route-readiness-v1";

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

pub use crate::model_routing::ProviderManifest;

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
    pub model_catalog_sha256: String,
    pub model_profile_sha256: String,
    pub model_installation_sha256: String,
    pub worktree: String,
    pub private_home: String,
    pub private_roots: PrivateRoots,
    pub opencode_path: String,
    pub opencode_sha256: String,
    pub brief: String,
    pub brief_sha256: String,
    pub source_agent_id: String,
    pub source_template_sha256: String,
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
    pub spawn_response_sha256: String,
    pub resume_evidence_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelRouteReadiness {
    pub schema: String,
    pub model_catalog_sha256: String,
    pub model_profile_sha256: String,
    pub model_installation_sha256: String,
    pub agent_id: String,
    pub model: String,
    pub variant: String,
    pub provider_manifest_sha256: String,
    pub checked_at: String,
    pub challenge_sha256: String,
    pub probe_sha256: String,
    pub status: ModelRouteReadinessStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelRouteReadinessStatus {
    Pass,
    Blocked,
}

#[derive(Debug)]
struct RegisteredModelRouteReadiness {
    evidence: ModelRouteReadiness,
}

#[derive(Debug, Default)]
struct RouteReadinessRegistryState {
    pending_challenges: BTreeMap<String, String>,
    used_challenges: BTreeSet<String>,
    records: BTreeMap<String, RegisteredModelRouteReadiness>,
}

#[derive(Debug, Default)]
pub struct RouteReadinessRegistry {
    state: Mutex<RouteReadinessRegistryState>,
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

fn validate_absolute(value: &str, where_: &str) -> Result<(), String> {
    if !value.starts_with('/') {
        return Err(format!("{where_} must be an absolute path: {value:?}"));
    }
    Ok(())
}

impl SpawnRequest {
    pub fn validate(&self, policy: &ValidatedModelPolicy) -> Result<(), String> {
        self.validate_common()?;
        if self.model_catalog_sha256 != policy.catalog_sha256() {
            return Err("spawn request does not bind the selected model catalog".to_string());
        }
        if self.model_profile_sha256 != policy.profile_sha256() {
            return Err("spawn request does not bind the selected model profile".to_string());
        }
        if self.model_installation_sha256 != policy.installation_sha256() {
            return Err("spawn request does not bind the installed model registry".to_string());
        }
        let selection = policy
            .profile()
            .selection_for(&self.agent_id)
            .ok_or_else(|| format!("model profile has no route for {:?}", self.agent_id))?;
        if self.model != selection.model || self.variant != selection.variant {
            return Err(format!(
                "spawn route {:?} variant {:?} does not match selected route {:?} variant {:?}",
                self.model, self.variant, selection.model, selection.variant
            ));
        }
        if self.source_agent_id != self.agent_id {
            return Err("source_agent_id must equal the selected agent_id".to_string());
        }
        let installed_agent = policy
            .installation()
            .installed_agent_for(&self.agent_id)
            .ok_or_else(|| format!("installed registry has no agent {:?}", self.agent_id))?;
        if self.source_template_sha256 != installed_agent.source_template_sha256
            || self.source_agent_sha256 != installed_agent.installed_agent_sha256
        {
            return Err("spawn request does not bind the installed agent files".to_string());
        }
        let provider_id = provider_for_model(&self.model)?;
        let expected_provider = policy
            .installation()
            .provider_for(provider_id)
            .ok_or_else(|| format!("installed registry has no provider {provider_id:?}"))?;
        if &self.provider != expected_provider {
            return Err("spawn provider manifest does not match installed provider".to_string());
        }
        Ok(())
    }

    fn validate_common(&self) -> Result<(), String> {
        let encoded = crate::json::canonical_json_of(self)?;
        if encoded.len() > SPAWN_REQUEST_MAX_BYTES {
            return Err(format!(
                "spawn request exceeds resumable limit of {SPAWN_REQUEST_MAX_BYTES} bytes"
            ));
        }
        validate_agent_id(&self.agent_id)?;
        validate_route_shape(&self.model, &self.variant)?;
        crate::core::validate_digest_form(&self.model_catalog_sha256)?;
        crate::core::validate_digest_form(&self.model_profile_sha256)?;
        crate::core::validate_digest_form(&self.model_installation_sha256)?;
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
        crate::core::validate_digest_form(&self.source_template_sha256)?;
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
        self.provider.validate()?;
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
    pub fn validate(
        &self,
        original_spawn: &SpawnRequest,
        original_response: &SpawnResponse,
        policy: &ValidatedModelPolicy,
    ) -> Result<(), String> {
        if &self.spawn != original_spawn {
            return Err("resume request must contain the exact original spawn request".to_string());
        }
        self.spawn.validate(policy)?;
        original_response.validate(original_spawn)?;
        validate_session_id(&self.opencode_session_id)?;
        if self.opencode_session_id != original_response.opencode_session_id {
            return Err(
                "resume session ID does not match the registered spawn response".to_string(),
            );
        }
        crate::core::validate_digest_form(&self.spawn_response_sha256)?;
        if self.spawn_response_sha256 != original_response.canonical_sha256(original_spawn)? {
            return Err("resume request does not bind the registered spawn response".to_string());
        }
        crate::core::validate_digest_form(&self.resume_evidence_sha256)?;
        Ok(())
    }
}

impl ModelRouteReadiness {
    pub fn validate(
        &self,
        spawn: &SpawnRequest,
        policy: &ValidatedModelPolicy,
    ) -> Result<(), String> {
        spawn.validate(policy)?;
        if self.schema != MODEL_ROUTE_READINESS_TAG {
            return Err(format!("unknown route readiness schema: {:?}", self.schema));
        }
        for digest in [
            &self.model_catalog_sha256,
            &self.model_profile_sha256,
            &self.model_installation_sha256,
            &self.provider_manifest_sha256,
            &self.challenge_sha256,
            &self.probe_sha256,
        ] {
            crate::core::validate_digest_form(digest)?;
        }
        validate_agent_id(&self.agent_id)?;
        crate::model_routing::validate_route_shape(&self.model, &self.variant)?;
        crate::core::validate_iso8601_z(&self.checked_at)?;
        if self.model_catalog_sha256 != spawn.model_catalog_sha256
            || self.model_profile_sha256 != spawn.model_profile_sha256
            || self.model_installation_sha256 != spawn.model_installation_sha256
            || self.agent_id != spawn.agent_id
            || self.model != spawn.model
            || self.variant != spawn.variant
        {
            return Err(
                "route readiness does not bind the original spawn policy and route".to_string(),
            );
        }
        let provider_json = crate::json::canonical_json_of(&spawn.provider)?;
        if self.provider_manifest_sha256 != crate::hashing::sha256_digest(provider_json.as_bytes())
        {
            return Err("route readiness does not bind the spawn provider manifest".to_string());
        }
        Ok(())
    }

    pub fn canonical_sha256(
        &self,
        spawn: &SpawnRequest,
        policy: &ValidatedModelPolicy,
    ) -> Result<String, String> {
        self.validate(spawn, policy)?;
        let encoded = crate::json::canonical_json_of(self)?;
        crate::core::SizeLimit::RegisteredRecord.check(encoded.len())?;
        Ok(crate::hashing::sha256_digest(encoded.as_bytes()))
    }
}

impl RouteReadinessRegistry {
    pub fn issue_challenge(&self, challenge_sha256: &str, checked_at: &str) -> Result<(), String> {
        crate::core::validate_digest_form(challenge_sha256)?;
        crate::core::validate_iso8601_z(checked_at)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| "route-readiness registry lock is poisoned".to_string())?;
        if state.pending_challenges.contains_key(challenge_sha256)
            || state.used_challenges.contains(challenge_sha256)
        {
            return Err("route-readiness challenge was already issued".to_string());
        }
        state
            .pending_challenges
            .insert(challenge_sha256.to_string(), checked_at.to_string());
        Ok(())
    }

    pub fn register_broker_result(
        &self,
        evidence: ModelRouteReadiness,
        spawn: &SpawnRequest,
        policy: &ValidatedModelPolicy,
    ) -> Result<String, String> {
        evidence.validate(spawn, policy)?;
        let sha256 = evidence.canonical_sha256(spawn, policy)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| "route-readiness registry lock is poisoned".to_string())?;
        let issued_at = state
            .pending_challenges
            .get(&evidence.challenge_sha256)
            .ok_or_else(|| "broker result does not answer a pending challenge".to_string())?;
        if issued_at != &evidence.checked_at {
            return Err("broker result timestamp does not match its issued challenge".to_string());
        }
        if state.records.contains_key(&sha256) {
            return Err("route-readiness result is already registered".to_string());
        }
        state.pending_challenges.remove(&evidence.challenge_sha256);
        state
            .used_challenges
            .insert(evidence.challenge_sha256.clone());
        if evidence.status != ModelRouteReadinessStatus::Pass {
            return Err("blocked broker readiness cannot authorize resume".to_string());
        }
        state
            .records
            .insert(sha256.clone(), RegisteredModelRouteReadiness { evidence });
        Ok(sha256)
    }

    fn consume_for_resume(
        &self,
        evidence_sha256: &str,
        spawn: &SpawnRequest,
        policy: &ValidatedModelPolicy,
    ) -> Result<(), String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "route-readiness registry lock is poisoned".to_string())?;
        let registered = state
            .records
            .get(evidence_sha256)
            .ok_or_else(|| "resume readiness is absent, unregistered, or consumed".to_string())?;
        registered.evidence.validate(spawn, policy)?;
        if evidence_sha256 != registered.evidence.canonical_sha256(spawn, policy)? {
            return Err("validated route-readiness hash drifted".to_string());
        }
        state.records.remove(evidence_sha256);
        Ok(())
    }
}

impl SpawnResponse {
    pub fn validate(&self, spawn: &SpawnRequest) -> Result<(), String> {
        validate_session_id(&self.opencode_session_id)?;
        validate_agent_id(&self.observed_agent_id)?;
        if self.observed_agent_id != spawn.runtime_wrapper {
            return Err(
                "observed response agent does not match the spawned runtime wrapper".to_string(),
            );
        }
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

    pub fn canonical_sha256(&self, spawn: &SpawnRequest) -> Result<String, String> {
        self.validate(spawn)?;
        let encoded = crate::json::canonical_json_of(self)?;
        crate::core::SizeLimit::RegisteredRecord.check(encoded.len())?;
        Ok(crate::hashing::sha256_digest(encoded.as_bytes()))
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

pub fn parse_spawn_request(
    json: &[u8],
    policy: &ValidatedModelPolicy,
) -> Result<SpawnRequest, String> {
    if json.len() > SPAWN_REQUEST_MAX_BYTES {
        return Err(format!(
            "spawn request exceeds resumable limit of {SPAWN_REQUEST_MAX_BYTES} bytes"
        ));
    }
    let request: SpawnRequest =
        crate::json::parse_canonical_json(json, crate::core::SizeLimit::RegisteredRecord)
            .map_err(|error| format!("spawn request rejected: {error}"))?;
    request.validate(policy)?;
    Ok(request)
}

pub fn parse_resume_request(
    json: &[u8],
    original_spawn: &SpawnRequest,
    original_response: &SpawnResponse,
    readiness_registry: &RouteReadinessRegistry,
    policy: &ValidatedModelPolicy,
) -> Result<ResumeRequest, String> {
    let request: ResumeRequest =
        crate::json::parse_canonical_json(json, crate::core::SizeLimit::RegisteredRecord)
            .map_err(|error| format!("resume request rejected: {error}"))?;
    request.validate(original_spawn, original_response, policy)?;
    readiness_registry.consume_for_resume(
        &request.resume_evidence_sha256,
        original_spawn,
        policy,
    )?;
    Ok(request)
}

pub fn parse_spawn_response(json: &[u8], spawn: &SpawnRequest) -> Result<SpawnResponse, String> {
    let response: SpawnResponse =
        crate::json::parse_canonical_json(json, crate::core::SizeLimit::RegisteredRecord)
            .map_err(|error| format!("spawn response rejected: {error}"))?;
    response.validate(spawn)?;
    Ok(response)
}
