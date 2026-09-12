use sp_schema::core::{
    validate_agent_id, validate_content_id_form, validate_digest_form, validate_git_date,
    validate_iso8601_z, validate_session_id, validate_sp_id, validate_task_id, HandoffStatus,
    InvocationState, LeaseState, MonitoringEventType, ReviewResult, Severity, SizeLimit,
};

#[test]
fn content_id_forms() {
    assert!(validate_content_id_form(
        "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    )
    .is_ok());
    assert!(validate_content_id_form(
        "sha256:0123456789ABCDEF0123456789abcdef0123456789abcdef0123456789abcdef"
    )
    .is_err());
    assert!(validate_content_id_form(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    )
    .is_err());
    assert!(validate_content_id_form("sha256:short").is_err());
}

#[test]
fn digest_forms() {
    assert!(validate_digest_form(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    )
    .is_ok());
    assert!(validate_digest_form(
        "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    )
    .is_err());
    assert!(validate_digest_form("0123").is_err());
}

#[test]
fn iso8601_profile() {
    assert!(validate_iso8601_z("2026-09-11T20:54:39Z").is_ok());
    assert!(validate_iso8601_z("2026-13-11T20:54:39Z").is_err());
    assert!(validate_iso8601_z("2026-09-11T24:54:39Z").is_err());
    assert!(validate_iso8601_z("2026-09-11T20:54:39+00:00").is_err());
    assert!(validate_iso8601_z("2026-09-11 20:54:39Z").is_err());
    assert!(validate_iso8601_z("2026-09-11T20:54:39.123Z").is_err());
}

#[test]
fn git_date_profile() {
    assert!(validate_git_date("1789099003 -0400").is_ok());
    assert!(validate_git_date("1750001000 +0000").is_ok());
    assert!(validate_git_date("1750001000 -0000").is_err());
    assert!(validate_git_date("1750001000 +2400").is_err());
    assert!(validate_git_date("1750001000 +0060").is_err());
    assert!(validate_git_date("1750001000+0000").is_err());
}

#[test]
fn sp_id_grammar() {
    assert!(validate_sp_id("operation", "sp-operation-0123456789abcdef0123456789abcdef").is_ok());
    assert!(validate_sp_id("finding", "sp-finding-0123456789abcdef0123456789abcdef").is_ok());
    assert!(validate_sp_id("review", "sp-review-0123456789ABCDEF0123456789abcdef").is_err());
    assert!(validate_sp_id("user", "sp-user-0123456789abcdef0123456789abcdef").is_err());
    assert!(validate_sp_id("review", "sp-review-0123").is_err());
}

#[test]
fn agent_and_session_and_task_ids() {
    assert!(validate_agent_id("sp-probe-wrapper").is_ok());
    assert!(validate_agent_id("superplanner.builder").is_ok());
    assert!(validate_agent_id("superplanner..builder").is_err());
    assert!(validate_agent_id("superplanner.").is_err());
    assert!(validate_agent_id(".superplanner").is_err());
    assert!(validate_agent_id("Build").is_err());
    assert!(validate_agent_id("").is_err());
    assert!(validate_session_id("ses_f6da62264ffedNODzLYU6IRTmN").is_ok());
    assert!(validate_session_id("task_123").is_err());
    assert!(validate_task_id("F001/T001").is_ok());
    assert!(validate_task_id("T001-fix-login").is_ok());
    assert!(validate_task_id("T001").is_ok());
    assert!(validate_task_id("T01").is_err());
    assert!(validate_task_id("X001").is_err());
}

#[test]
fn enums_reject_unknown_and_timeout_status() {
    assert_eq!(
        HandoffStatus::parse("complete").unwrap(),
        HandoffStatus::Complete
    );
    assert_eq!(
        HandoffStatus::parse("blocked").unwrap(),
        HandoffStatus::Blocked
    );
    assert!(HandoffStatus::parse("timeout").is_err());
    assert!(HandoffStatus::parse("complete; rm -rf").is_err());
    assert_eq!(
        ReviewResult::parse("approved").unwrap(),
        ReviewResult::Approved
    );
    assert!(ReviewResult::parse("timeout").is_err());
    assert!(Severity::parse("critical").is_err());
    assert!(Severity::parse("blocker").is_ok());
    assert!(InvocationState::parse("running").is_ok());
    assert!(InvocationState::parse("finished").is_err());
    assert!(MonitoringEventType::parse("live-check").is_ok());
    assert!(MonitoringEventType::parse("poll").is_err());
    assert!(LeaseState::parse("cleanup-failed").is_ok());
    assert!(LeaseState::parse("expired").is_err());
}

#[test]
fn size_limits_enforce_record_grammar() {
    assert!(SizeLimit::Handoff.check(256 * 1024).is_ok());
    assert!(SizeLimit::Handoff.check(256 * 1024 + 1).is_err());
    assert!(SizeLimit::Brief.check(4 * 1024 * 1024).is_ok());
    assert!(SizeLimit::Brief.check(4 * 1024 * 1024 + 1).is_err());
    assert!(SizeLimit::State.check(8 * 1024 * 1024 + 1).is_err());
}
