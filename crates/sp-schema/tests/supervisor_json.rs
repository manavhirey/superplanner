use std::collections::BTreeSet;

use serde_json::json;
use sp_schema::json::{canonical_digest, canonical_json};
use sp_schema::model_routing::{
    default_model_catalog, default_model_profile, AgentModelSelection, InstalledAgentManifest,
    ModelCatalog, ModelInstallation, ModelProfile, ModelRouteRegistration, ProviderManifest,
    ValidatedModelPolicy, MODEL_INSTALLATION_TAG,
};
use sp_schema::supervisor::{
    parse_resume_request, parse_spawn_request, parse_spawn_response, OperateOperation,
    OperateOutput, OperateResult, OperateStatus,
};

const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn provider(provider: &str) -> ProviderManifest {
    ProviderManifest {
        provider: provider.to_string(),
        package: format!("provider-{provider}"),
        version: "1.0.0".to_string(),
        sha256: DIGEST.to_string(),
        broker_endpoint: format!("http://127.0.0.1:39421/{provider}"),
    }
}

fn installation(catalog: &ModelCatalog, profile: &ModelProfile) -> ModelInstallation {
    let provider_ids: BTreeSet<&str> = catalog
        .routes
        .iter()
        .map(|route| route.model.split('/').next().unwrap())
        .collect();
    ModelInstallation {
        schema: MODEL_INSTALLATION_TAG.to_string(),
        catalog_sha256: catalog.canonical_sha256().unwrap(),
        profile_sha256: profile.canonical_sha256(catalog).unwrap(),
        providers: provider_ids.into_iter().map(provider).collect(),
        agents: profile
            .selections
            .iter()
            .map(|selection| {
                let source = format!(
                    "---\nname: {}\nmodel: {}\nvariant: {}\n---\nAgent body.\n",
                    selection.agent_id, selection.model, selection.variant
                );
                InstalledAgentManifest::from_definition_bytes(
                    &selection.agent_id,
                    &selection.model,
                    &selection.variant,
                    source.as_bytes(),
                    source.as_bytes(),
                )
                .unwrap()
            })
            .collect(),
    }
}

fn policy() -> ValidatedModelPolicy {
    let catalog = default_model_catalog();
    let profile = default_model_profile();
    let installation = installation(&catalog, &profile);
    ValidatedModelPolicy::new(catalog, profile, installation).unwrap()
}

fn spawn_json(policy: &ValidatedModelPolicy) -> serde_json::Value {
    let agent_id = "superplanner.brainstormer";
    let selection = policy.profile().selection_for(agent_id).unwrap();
    let installed_agent = policy.installation().installed_agent_for(agent_id).unwrap();
    let provider_id = selection.model.split('/').next().unwrap();
    let provider = policy.installation().provider_for(provider_id).unwrap();
    json!({
        "agent_id": agent_id,
        "model": selection.model,
        "variant": selection.variant,
        "model_catalog_sha256": policy.catalog_sha256(),
        "model_profile_sha256": policy.profile_sha256(),
        "model_installation_sha256": policy.installation_sha256(),
        "worktree": "/wt/design",
        "private_home": "/roots/design/home",
        "private_roots": {
            "config": "/roots/design/config",
            "data": "/roots/design/data",
            "cache": "/roots/design/cache",
            "state": "/roots/design/state",
            "temp": "/roots/design/tmp"
        },
        "opencode_path": "/opt/homebrew/bin/opencode",
        "opencode_sha256": DIGEST,
        "brief": "Design the login fix.",
        "brief_sha256": DIGEST,
        "source_agent_id": agent_id,
        "source_template_sha256": installed_agent.source_template_sha256,
        "source_agent_sha256": installed_agent.installed_agent_sha256,
        "runtime_wrapper": "sp-runtime-brainstormer",
        "runtime_wrapper_sha256": DIGEST,
        "permission_sha256": DIGEST,
        "skills": [
            {
                "name": "brainstorming",
                "path": "/pack/skills/brainstorming",
                "sha256": DIGEST
            }
        ],
        "provider": provider,
        "credential_manifest": ["/home/real/.zshenv"],
        "sandbox_profile": "userns-v1",
        "isolation": {
            "mode": "kernel-enforced-same-principal",
            "evidence": "probe-battery-sha256:0123"
        }
    })
}

fn spawn_response(spawn: &sp_schema::SpawnRequest) -> sp_schema::SpawnResponse {
    sp_schema::SpawnResponse {
        opencode_session_id: "ses_f6da62264ffedNODzLYU6IRTmN".to_string(),
        observed_agent_id: spawn.runtime_wrapper.clone(),
        handoff_bytes: "task_id: T001...".to_string(),
        process_status: sp_schema::ProcessStatus::Exited,
        principal_verdict: sp_schema::PrincipalVerdict::Pass,
        principal_evidence: "probe-battery passed".to_string(),
    }
}

fn route_readiness_evidence(spawn: &sp_schema::SpawnRequest) -> sp_schema::ModelRouteReadiness {
    sp_schema::ModelRouteReadiness {
        schema: sp_schema::MODEL_ROUTE_READINESS_TAG.to_string(),
        model_catalog_sha256: spawn.model_catalog_sha256.clone(),
        model_profile_sha256: spawn.model_profile_sha256.clone(),
        model_installation_sha256: spawn.model_installation_sha256.clone(),
        agent_id: spawn.agent_id.clone(),
        model: spawn.model.clone(),
        variant: spawn.variant.clone(),
        provider_manifest_sha256: canonical_digest(&serde_json::to_value(&spawn.provider).unwrap())
            .unwrap(),
        checked_at: "2026-09-12T12:00:00Z".to_string(),
        challenge_sha256: DIGEST.to_string(),
        probe_sha256: DIGEST.to_string(),
        status: sp_schema::ModelRouteReadinessStatus::Pass,
    }
}

fn route_readiness(
    spawn: &sp_schema::SpawnRequest,
    policy: &ValidatedModelPolicy,
) -> (sp_schema::RouteReadinessRegistry, String) {
    let registry = sp_schema::RouteReadinessRegistry::default();
    registry
        .issue_challenge(DIGEST, "2026-09-12T12:00:00Z")
        .unwrap();
    let sha256 = registry
        .register_broker_result(route_readiness_evidence(spawn), spawn, policy)
        .unwrap();
    (registry, sha256)
}

#[test]
fn spawn_request_parses_and_validates() {
    let policy = policy();
    let text = canonical_json(&spawn_json(&policy)).unwrap();
    let request = parse_spawn_request(text.as_bytes(), &policy).unwrap();
    assert_eq!(request.agent_id, "superplanner.brainstormer");
    assert_eq!(request.private_roots.temp, "/roots/design/tmp");
}

#[test]
fn spawn_request_rejects_unknown_duplicate_noncanonical_and_oversized_input() {
    let policy = policy();
    let mut value = spawn_json(&policy);
    value
        .as_object_mut()
        .unwrap()
        .insert("extra".to_string(), json!("nope"));
    let text = canonical_json(&value).unwrap();
    assert!(parse_spawn_request(text.as_bytes(), &policy).is_err());

    let spawn_text = canonical_json(&spawn_json(&policy)).unwrap();
    let trimmed = spawn_text.strip_suffix('}').unwrap();
    let duplicated = format!("{trimmed},\"agent_id\":\"superplanner.brainstormer\"}}");
    assert!(serde_json::from_str::<serde_json::Value>(&duplicated).is_ok());
    assert!(parse_spawn_request(duplicated.as_bytes(), &policy).is_err());

    let pretty = serde_json::to_string_pretty(&spawn_json(&policy)).unwrap();
    assert!(parse_spawn_request(pretty.as_bytes(), &policy).is_err());
    let oversized = vec![b' '; sp_schema::supervisor::SPAWN_REQUEST_MAX_BYTES + 1];
    assert!(parse_spawn_request(&oversized, &policy).is_err());
}

#[test]
fn maximum_spawn_request_fits_its_resume_envelope() {
    let policy = policy();
    let mut spawn = spawn_json(&policy);
    let base = canonical_json(&spawn).unwrap();
    let current_profile_len = spawn["sandbox_profile"].as_str().unwrap().len();
    let profile_len =
        current_profile_len + sp_schema::supervisor::SPAWN_REQUEST_MAX_BYTES - base.len();
    spawn["sandbox_profile"] = json!("x".repeat(profile_len));

    let spawn_text = canonical_json(&spawn).unwrap();
    assert_eq!(
        spawn_text.len(),
        sp_schema::supervisor::SPAWN_REQUEST_MAX_BYTES
    );
    let original_spawn = parse_spawn_request(spawn_text.as_bytes(), &policy).unwrap();
    let original_response = spawn_response(&original_spawn);
    let (readiness_registry, readiness_sha256) = route_readiness(&original_spawn, &policy);
    let resume = json!({
        "opencode_session_id": "ses_f6da62264ffedNODzLYU6IRTmN",
        "resume_evidence_sha256": readiness_sha256,
        "spawn_response_sha256": original_response.canonical_sha256(&original_spawn).unwrap(),
        "spawn": spawn
    });
    let resume_text = canonical_json(&resume).unwrap();
    assert!(resume_text.len() <= sp_schema::core::REGISTERED_RECORD_MAX_BYTES);
    parse_resume_request(
        resume_text.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy,
    )
    .unwrap();
}

#[test]
fn spawn_request_rejects_unselected_or_malformed_route() {
    let policy = policy();
    let mut value = spawn_json(&policy);
    value["model"] = json!("vendor/unregistered");
    let text = canonical_json(&value).unwrap();
    assert!(parse_spawn_request(text.as_bytes(), &policy).is_err());

    let mut value = spawn_json(&policy);
    value["variant"] = json!("");
    let text = canonical_json(&value).unwrap();
    assert!(parse_spawn_request(text.as_bytes(), &policy).is_err());
}

#[test]
fn spawn_request_binds_profile_installed_agent_and_provider() {
    let policy = policy();
    let mut wrong_provider = spawn_json(&policy);
    wrong_provider["provider"]["package"] = json!("untrusted-provider");
    let text = canonical_json(&wrong_provider).unwrap();
    assert!(parse_spawn_request(text.as_bytes(), &policy).is_err());

    let mut wrong_profile = spawn_json(&policy);
    wrong_profile["model_profile_sha256"] =
        json!("fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210");
    let text = canonical_json(&wrong_profile).unwrap();
    assert!(parse_spawn_request(text.as_bytes(), &policy).is_err());

    let mut wrong_agent = spawn_json(&policy);
    wrong_agent["source_agent_id"] = json!("superplanner.builder");
    let text = canonical_json(&wrong_agent).unwrap();
    assert!(parse_spawn_request(text.as_bytes(), &policy).is_err());

    let mut wrong_source = spawn_json(&policy);
    wrong_source["source_template_sha256"] =
        json!("fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210");
    let text = canonical_json(&wrong_source).unwrap();
    assert!(parse_spawn_request(text.as_bytes(), &policy).is_err());
}

#[test]
fn spawn_request_accepts_explicit_registered_selection() {
    let mut catalog = default_model_catalog();
    catalog.routes.push(ModelRouteRegistration {
        model: "vendor/reasoner-v2".to_string(),
        backend_model: "vendor/reasoner-v2".to_string(),
        variant: "deep".to_string(),
        allowed_agent_ids: vec!["superplanner.brainstormer".to_string()],
    });
    catalog
        .routes
        .sort_by(|left, right| (&left.model, &left.variant).cmp(&(&right.model, &right.variant)));
    let mut profile = default_model_profile();
    profile.catalog_sha256 = catalog.canonical_sha256().unwrap();
    profile.selections[1] = AgentModelSelection {
        agent_id: "superplanner.brainstormer".to_string(),
        model: "vendor/reasoner-v2".to_string(),
        variant: "deep".to_string(),
    };
    let installation = installation(&catalog, &profile);
    let policy = ValidatedModelPolicy::new(catalog, profile, installation).unwrap();
    let text = canonical_json(&spawn_json(&policy)).unwrap();
    parse_spawn_request(text.as_bytes(), &policy).unwrap();
}

#[test]
fn relative_worktree_rejected() {
    let policy = policy();
    let mut value = spawn_json(&policy);
    value["worktree"] = json!("wt/design");
    let text = canonical_json(&value).unwrap();
    assert!(parse_spawn_request(text.as_bytes(), &policy).is_err());
}

#[test]
fn route_readiness_registry_requires_issued_passing_probe() {
    let policy = policy();
    let spawn_text = canonical_json(&spawn_json(&policy)).unwrap();
    let spawn = parse_spawn_request(spawn_text.as_bytes(), &policy).unwrap();
    let registry = sp_schema::RouteReadinessRegistry::default();

    let evidence = route_readiness_evidence(&spawn);
    assert!(registry
        .register_broker_result(evidence.clone(), &spawn, &policy)
        .is_err());
    registry
        .issue_challenge(DIGEST, "2026-09-12T12:00:00Z")
        .unwrap();
    let mut blocked = evidence;
    blocked.status = sp_schema::ModelRouteReadinessStatus::Blocked;
    assert!(registry
        .register_broker_result(blocked, &spawn, &policy)
        .is_err());
    let mut passing = route_readiness_evidence(&spawn);
    passing.status = sp_schema::ModelRouteReadinessStatus::Pass;
    assert!(registry
        .register_broker_result(passing, &spawn, &policy)
        .is_err());
}

#[test]
fn resume_requires_original_spawn_and_session_id() {
    let policy = policy();
    let spawn_text = canonical_json(&spawn_json(&policy)).unwrap();
    let original_spawn = parse_spawn_request(spawn_text.as_bytes(), &policy).unwrap();
    let original_response = spawn_response(&original_spawn);
    let (readiness_registry, readiness_sha256) = route_readiness(&original_spawn, &policy);
    let resume = sp_schema::ResumeRequest {
        spawn: original_spawn.clone(),
        opencode_session_id: "ses_f6da62264ffedNODzLYU6IRTmN".to_string(),
        spawn_response_sha256: original_response.canonical_sha256(&original_spawn).unwrap(),
        resume_evidence_sha256: readiness_sha256,
    };
    let text = canonical_json(&serde_json::to_value(&resume).unwrap()).unwrap();
    parse_resume_request(
        text.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy,
    )
    .unwrap();

    assert!(parse_resume_request(
        text.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy
    )
    .is_err());

    let mut wrong_session = resume.clone();
    wrong_session.opencode_session_id = "ses_AnotherValidSession".to_string();
    let wrong_session = canonical_json(&serde_json::to_value(&wrong_session).unwrap()).unwrap();
    assert!(parse_resume_request(
        wrong_session.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy
    )
    .is_err());

    let mut wrong_response = resume.clone();
    wrong_response.spawn_response_sha256 = DIGEST.to_string();
    let wrong_response = canonical_json(&serde_json::to_value(&wrong_response).unwrap()).unwrap();
    assert!(parse_resume_request(
        wrong_response.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy
    )
    .is_err());

    let mut missing_session: serde_json::Value = serde_json::from_str(&text).unwrap();
    missing_session
        .as_object_mut()
        .unwrap()
        .remove("opencode_session_id");
    let broken = canonical_json(&missing_session).unwrap();
    assert!(parse_resume_request(
        broken.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy
    )
    .is_err());

    let mut changed_spawn = resume.clone();
    changed_spawn.spawn.brief = "Different brief".to_string();
    let changed = canonical_json(&serde_json::to_value(&changed_spawn).unwrap()).unwrap();
    assert!(parse_resume_request(
        changed.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy
    )
    .is_err());
}

#[test]
fn resume_rejects_replacement_profile_and_stable_task_id() {
    let policy = policy();
    let spawn_text = canonical_json(&spawn_json(&policy)).unwrap();
    let original_spawn = parse_spawn_request(spawn_text.as_bytes(), &policy).unwrap();
    let original_response = spawn_response(&original_spawn);
    let (readiness_registry, readiness_sha256) = route_readiness(&original_spawn, &policy);

    let mut bad_session = sp_schema::ResumeRequest {
        spawn: original_spawn.clone(),
        opencode_session_id: "T001-fix-login".to_string(),
        spawn_response_sha256: original_response.canonical_sha256(&original_spawn).unwrap(),
        resume_evidence_sha256: readiness_sha256,
    };
    let text = canonical_json(&serde_json::to_value(&bad_session).unwrap()).unwrap();
    assert!(parse_resume_request(
        text.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy
    )
    .is_err());

    bad_session.opencode_session_id = "ses_f6da62264ffedNODzLYU6IRTmN".to_string();
    bad_session.spawn.model_profile_sha256 =
        "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210".to_string();
    let text = canonical_json(&serde_json::to_value(&bad_session).unwrap()).unwrap();
    assert!(parse_resume_request(
        text.as_bytes(),
        &original_spawn,
        &original_response,
        &readiness_registry,
        &policy
    )
    .is_err());
}

#[test]
fn spawn_response_validates() {
    let policy = policy();
    let spawn_text = canonical_json(&spawn_json(&policy)).unwrap();
    let spawn = parse_spawn_request(spawn_text.as_bytes(), &policy).unwrap();
    let response = spawn_response(&spawn);
    let text = canonical_json(&serde_json::to_value(&response).unwrap()).unwrap();
    parse_spawn_response(text.as_bytes(), &spawn).unwrap();

    let mut wrong_agent = response;
    wrong_agent.observed_agent_id = "sp-runtime-builder".to_string();
    let text = canonical_json(&serde_json::to_value(&wrong_agent).unwrap()).unwrap();
    assert!(parse_spawn_response(text.as_bytes(), &spawn).is_err());
}

#[test]
fn failing_principal_verdict_rejected() {
    let policy = policy();
    let spawn_text = canonical_json(&spawn_json(&policy)).unwrap();
    let spawn = parse_spawn_request(spawn_text.as_bytes(), &policy).unwrap();
    let mut response = spawn_response(&spawn);
    response.principal_verdict = sp_schema::PrincipalVerdict::Fail;
    response.principal_evidence = "probe-battery failed property 2".to_string();
    let text = canonical_json(&serde_json::to_value(&response).unwrap()).unwrap();
    assert!(parse_spawn_response(text.as_bytes(), &spawn).is_err());
}

#[test]
fn operate_envelope_and_result_validate() {
    let result = OperateResult {
        operation_id: "sp-operation-0123456789abcdef0123456789abcdef".to_string(),
        request_sha256: DIGEST.to_string(),
        status: OperateStatus::Ok,
        outputs: vec![OperateOutput {
            path: "/artifacts/op1.json".to_string(),
            identity: "dev=1;ino=2".to_string(),
            sha256: DIGEST.to_string(),
        }],
        evidence: "terminal result manifest".to_string(),
        state_transition: "packaged".to_string(),
    };
    result.validate().unwrap();
    assert_eq!(OperateOperation::SafeGit, OperateOperation::SafeGit);

    let operations: Vec<String> = [
        "safe-git",
        "sandbox-command",
        "package",
        "integrate",
        "verify",
        "commit-prepare",
        "commit",
        "push-prepare",
        "push-present",
        "push-retire",
    ]
    .iter()
    .map(|s| serde_json::to_string(&serde_json::json!(s)).unwrap())
    .collect();
    let parsed: Vec<OperateOperation> = operations
        .iter()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(parsed.len(), 10);
    assert!(serde_json::from_str::<OperateOperation>("\"push\"").is_err());
    assert!(serde_json::from_str::<OperateOperation>("\"rm -rf\"").is_err());
}

#[test]
fn canonical_digest_is_order_independent() {
    let a = json!({"agent_id": "sp.wrap", "variant": "high"});
    let b = json!({"variant": "high", "agent_id": "sp.wrap"});
    assert_eq!(canonical_digest(&a).unwrap(), canonical_digest(&b).unwrap());
    assert_eq!(
        canonical_digest(&a).unwrap(),
        "41adfa22306ea5df5764a3b437e06eddd1f6b67801c0f60967ab8d613fb2c4a9"
    );
}

#[test]
fn canonical_digest_rejects_floats_and_null() {
    let with_float = json!({"steps": 1.5});
    assert!(canonical_digest(&with_float).is_err());
    let with_null = json!({"evidence": null});
    assert!(canonical_digest(&with_null).is_err());
}
