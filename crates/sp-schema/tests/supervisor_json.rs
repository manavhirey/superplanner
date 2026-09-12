use serde_json::json;
use sp_schema::json::{canonical_digest, canonical_json};
use sp_schema::supervisor::{
    parse_resume_request, parse_spawn_request, parse_spawn_response, validate_route,
    OperateOperation, OperateOutput, OperateResult, OperateStatus,
};

fn spawn_json() -> serde_json::Value {
    json!({
        "agent_id": "superplanner.brainstormer",
        "model": "openai/gpt-5.6-sol",
        "variant": "high",
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
        "opencode_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "brief": "Design the login fix.",
        "brief_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "source_agent_id": "superplanner.brainstormer",
        "source_agent_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "runtime_wrapper": "sp-runtime-brainstormer",
        "runtime_wrapper_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "permission_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "skills": [
            {
                "name": "brainstorming",
                "path": "/pack/skills/brainstorming",
                "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            }
        ],
        "provider": {
            "provider": "openai",
            "package": "provider-impl",
            "version": "1.0.0",
            "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "broker_endpoint": "http://127.0.0.1:39421/v1"
        },
        "credential_manifest": ["/home/real/.zshenv"],
        "sandbox_profile": "userns-v1",
        "isolation": {
            "mode": "kernel-enforced-same-principal",
            "evidence": "probe-battery-sha256:0123"
        }
    })
}

#[test]
fn spawn_request_parses_and_validates() {
    let text = canonical_json(&spawn_json()).unwrap();
    let request = parse_spawn_request(&text).unwrap();
    assert_eq!(request.agent_id, "superplanner.brainstormer");
    assert_eq!(request.private_roots.temp, "/roots/design/tmp");
}

#[test]
fn spawn_request_unknown_field_rejected() {
    let mut value = spawn_json();
    let map = value.as_object_mut().unwrap();
    map.insert("extra".to_string(), json!("nope"));
    let text = canonical_json(&value).unwrap();
    assert!(parse_spawn_request(&text).is_err());
}

#[test]
fn spawn_request_duplicate_field_rejected() {
    let spawn_text = canonical_json(&spawn_json()).unwrap();
    let trimmed = spawn_text.strip_suffix('}').unwrap();
    let duplicated = format!("{trimmed},\"agent_id\":\"superplanner.brainstormer\"}}");
    assert!(serde_json::from_str::<serde_json::Value>(&duplicated).is_ok());
    assert!(parse_spawn_request(&duplicated).is_err());
}

#[test]
fn non_canonical_route_rejected() {
    let mut value = spawn_json();
    value["variant"] = json!("ultra");
    let text = canonical_json(&value).unwrap();
    assert!(parse_spawn_request(&text).is_err());
}

#[test]
fn unknown_model_rejected() {
    assert!(validate_route("openai/gpt-4", "high").is_err());
    assert!(validate_route("openai/gpt-5.6-sol", "max").is_err());
    assert!(validate_route("zai/glm-5.3", "max").is_ok());
    assert!(validate_route("openrouter/moonshotai/kimi-k3", "max").is_ok());
}

#[test]
fn relative_worktree_rejected() {
    let mut value = spawn_json();
    value["worktree"] = json!("wt/design");
    let text = canonical_json(&value).unwrap();
    assert!(parse_spawn_request(&text).is_err());
}

#[test]
fn resume_requires_session_id() {
    let spawn = parse_spawn_request(&canonical_json(&spawn_json()).unwrap()).unwrap();
    let resume = sp_schema::ResumeRequest {
        spawn,
        opencode_session_id: "ses_f6da62264ffedNODzLYU6IRTmN".to_string(),
        resume_evidence_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .to_string(),
    };
    let text = serde_json::to_string(&resume).unwrap();
    assert!(parse_resume_request(&text).is_ok());

    let mut value: serde_json::Value = serde_json::from_str(&text).unwrap();
    value.as_object_mut().unwrap().remove("opencode_session_id");
    let broken = canonical_json(&value).unwrap();
    assert!(parse_resume_request(&broken).is_err());
}

#[test]
fn resume_rejects_stable_task_id_as_session() {
    let spawn = parse_spawn_request(&canonical_json(&spawn_json()).unwrap()).unwrap();
    let resume = sp_schema::ResumeRequest {
        spawn,
        opencode_session_id: "T001-fix-login".to_string(),
        resume_evidence_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .to_string(),
    };
    let text = serde_json::to_string(&resume).unwrap();
    assert!(parse_resume_request(&text).is_err());
}

#[test]
fn spawn_response_validates() {
    let response = sp_schema::SpawnResponse {
        opencode_session_id: "ses_f6da62264ffedNODzLYU6IRTmN".to_string(),
        observed_agent_id: "sp-runtime-brainstormer".to_string(),
        handoff_bytes: "task_id: T001...".to_string(),
        process_status: sp_schema::ProcessStatus::Exited,
        principal_verdict: sp_schema::PrincipalVerdict::Pass,
        principal_evidence: "probe-battery passed".to_string(),
    };
    let text = serde_json::to_string(&response).unwrap();
    assert!(parse_spawn_response(&text).is_ok());
}

#[test]
fn failing_principal_verdict_rejected() {
    let response = sp_schema::SpawnResponse {
        opencode_session_id: "ses_f6da62264ffedNODzLYU6IRTmN".to_string(),
        observed_agent_id: "sp-runtime-brainstormer".to_string(),
        handoff_bytes: "task_id: T001...".to_string(),
        process_status: sp_schema::ProcessStatus::Exited,
        principal_verdict: sp_schema::PrincipalVerdict::Fail,
        principal_evidence: "probe-battery failed property 2".to_string(),
    };
    let text = serde_json::to_string(&response).unwrap();
    assert!(parse_spawn_response(&text).is_err());
}

#[test]
fn operate_envelope_and_result_validate() {
    let result = OperateResult {
        operation_id: "sp-operation-0123456789abcdef0123456789abcdef".to_string(),
        request_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .to_string(),
        status: OperateStatus::Ok,
        outputs: vec![OperateOutput {
            path: "/artifacts/op1.json".to_string(),
            identity: "dev=1;ino=2".to_string(),
            sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
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
}

#[test]
fn canonical_digest_rejects_floats_and_null() {
    let with_float = json!({"steps": 1.5});
    assert!(canonical_digest(&with_float).is_err());
    let with_null = json!({"evidence": null});
    assert!(canonical_digest(&with_null).is_err());
}
