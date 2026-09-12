use sp_schema::handoff::parse_handoff;

fn worker_complete() -> String {
    "task_id: T001-fix-login\n\
     agent_id: superplanner.builder\n\
     role: worker\n\
     status: complete\n\
     worktree: /wt/t001\n\
     git_sha: 0123456789abcdef0123456789abcdef01234567\n\
     completion_time: 2026-09-11T21:00:00Z\n\
     scope:\n  \
     - implement login fix\n\
     artifacts:\n  \
     - src/login.rs\n\
     changed_files:\n  \
     - src/login.rs\n\
     verification:\n  \
     - command: cargo test -p login\n    \
     result: passed\n    \
     evidence: 12 passed\n\
     blockers:\n  \
     - none\n\
     assumptions:\n  \
     - none\n\
     next_action: integrate T001\n"
        .to_string()
}

#[test]
fn worker_complete_parses() {
    let record = parse_handoff(worker_complete().as_bytes()).unwrap();
    assert_eq!(record.role, sp_schema::Role::Worker);
    assert_eq!(record.status, sp_schema::HandoffStatus::Complete);
    assert!(record.blockers.is_empty());
    assert_eq!(record.verification.len(), 1);
    assert_eq!(record.verification[0].result, "passed");
    assert!(record.review.is_none());
}

#[test]
fn blocked_with_timeout_blocker_parses() {
    let text = worker_complete()
        .replace("status: complete", "status: blocked")
        .replace(
            "- none\nassumptions:",
            "- budget exhausted after timeout\nassumptions:",
        );
    let record = parse_handoff(text.as_bytes()).unwrap();
    assert_eq!(record.status, sp_schema::HandoffStatus::Blocked);
    assert_eq!(record.blockers, vec!["budget exhausted after timeout"]);
}

#[test]
fn timeout_is_not_a_status() {
    let text = worker_complete().replace("status: complete", "status: timeout");
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn missing_field_fails() {
    let text = worker_complete().replace("next_action: integrate T001\n", "");
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn out_of_order_fields_fail() {
    let text = worker_complete().replacen("agent_id:", "task_id_x:", 0);
    let swapped = text.replacen("task_id: T001-fix-login\n", "", 1).replacen(
        "agent_id: superplanner.builder\n",
        "agent_id: superplanner.builder\ntask_id: T001-fix-login\n",
        1,
    );
    assert!(parse_handoff(swapped.as_bytes()).is_err());
}

#[test]
fn empty_list_fails() {
    let text = worker_complete().replacen("scope:\n  - implement login fix\n", "scope:\n", 1);
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn complete_with_blockers_fails() {
    let text = worker_complete().replacen(
        "- none\nassumptions:",
        "- something blocked\nassumptions:",
        1,
    );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn worker_with_reviewer_extension_fails() {
    let text = format!(
        "{}review_result: approved\nreviewed_sha: 0123\n",
        worker_complete()
    );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn integration_result_none_on_builder_parses() {
    let text = format!("{}integration_result: none\n", worker_complete());
    let record = parse_handoff(text.as_bytes()).unwrap();
    assert_eq!(
        record.integration_result,
        Some(sp_schema::IntegrationResult::None)
    );
}

#[test]
fn integration_result_on_brainstormer_fails() {
    let text = worker_complete().replace(
        "agent_id: superplanner.builder",
        "agent_id: superplanner.brainstormer",
    ) + "integration_result: none\n";
    assert!(parse_handoff(text.as_bytes()).is_err());
}

fn reviewer_approved() -> String {
    worker_complete()
        .replacen("role: worker", "role: reviewer", 1)
        .replacen("changed_files:\n  - src/login.rs", "changed_files:\n  - none", 1)
        .replacen(
            "next_action: integrate T001\n",
            "next_action: record standard approval\nreview_result: approved\nreviewed_sha: 0123456789abcdef0123456789abcdef01234567\nfindings:\n  \
             - none\naccepted_risks:\n  \
             - none\n",
            1,
        )
}

#[test]
fn reviewer_approved_parses() {
    let record = parse_handoff(reviewer_approved().as_bytes()).unwrap();
    let review = record.review.as_ref().unwrap();
    assert_eq!(review.review_result, sp_schema::ReviewResult::Approved);
    assert!(review.findings.is_empty());
    assert!(review.accepted_risks.is_empty());
}

#[test]
fn reviewer_changed_files_fails() {
    let text = reviewer_approved().replacen(
        "changed_files:\n  - none",
        "changed_files:\n  - src/login.rs",
        1,
    );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn complete_review_sha_must_equal_git_sha() {
    let text = reviewer_approved().replacen(
        "reviewed_sha: 0123456789abcdef0123456789abcdef01234567",
        "reviewed_sha: fedcba9876543210fedcba9876543210fedcba98",
        1,
    );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn approved_with_unaccepted_finding_fails() {
    let text = reviewer_approved()
        .replacen(
            "findings:\n  - none",
            "findings:\n  - finding_id: sp-finding-0123456789abcdef0123456789abcdef\n    severity: minor\n    file: src/a.rs\n    line: 3\n    criterion: SPEC a-05\n    evidence: duplicated helper",
            1,
        );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn accepted_risk_must_reference_visible_finding() {
    let text = reviewer_approved()
        .replacen(
            "review_result: approved",
            "review_result: findings",
            1,
        )
        .replacen(
            "accepted_risks:\n  - none",
            "accepted_risks:\n  - finding_id: sp-finding-0123456789abcdef0123456789abcdef\n    reviewed_sha: 0123456789abcdef0123456789abcdef01234567\n    scope: src/a.rs:3\n    user_evidence: user message 77\n    consequence: accepted duplication\n    rationale: isolated helper",
            1,
        );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn accepted_risk_binds_handoff_reviewed_sha() {
    let text = reviewer_approved()
        .replacen("review_result: approved", "review_result: findings", 1)
        .replacen(
            "findings:\n  - none",
            "findings:\n  - finding_id: sp-finding-0123456789abcdef0123456789abcdef\n    severity: minor\n    file: src/a.rs\n    line: none\n    criterion: SPEC a-05\n    evidence: duplicated helper",
            1,
        )
        .replacen(
            "accepted_risks:\n  - none",
            "accepted_risks:\n  - finding_id: sp-finding-0123456789abcdef0123456789abcdef\n    reviewed_sha: fedcba9876543210fedcba9876543210fedcba98\n    scope: src/a.rs:3\n    user_evidence: user message 77\n    consequence: accepted duplication\n    rationale: isolated helper",
            1,
        );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn blocked_review_requires_not_completed() {
    let text = reviewer_approved()
        .replacen("status: complete", "status: blocked", 1)
        .replacen(
            "- none\nassumptions:",
            "- evidence unavailable\nassumptions:",
            1,
        );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn blocked_not_completed_review_with_none_shas_parses() {
    let text = reviewer_approved()
        .replacen("status: complete", "status: blocked", 1)
        .replacen(
            "git_sha: 0123456789abcdef0123456789abcdef01234567",
            "git_sha: none",
            1,
        )
        .replacen("review_result: approved", "review_result: not-completed", 1)
        .replacen(
            "reviewed_sha: 0123456789abcdef0123456789abcdef01234567",
            "reviewed_sha: none",
            1,
        )
        .replacen(
            "- none\nassumptions:",
            "- projection unavailable\nassumptions:",
            1,
        );
    let record = parse_handoff(text.as_bytes()).unwrap();
    assert_eq!(
        record.review.as_ref().unwrap().review_result,
        sp_schema::ReviewResult::NotCompleted
    );
}

fn adversarial_complete() -> String {
    reviewer_approved().replacen(
        "accepted_risks:\n  - none\n",
        "accepted_risks:\n  - none\nstandard_approval_record_id: sp-record-0123456789abcdef0123456789abcdef\nstandard_approval_record_identity: /state/reviews/std.json\nstandard_approval_record_sha256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\nstandard_approval_sha: 0123456789abcdef0123456789abcdef01234567\n",
        1,
    )
}

#[test]
fn adversarial_complete_parses_and_binds_sha() {
    let record = parse_handoff(adversarial_complete().as_bytes()).unwrap();
    let adversarial = record.adversarial.as_ref().unwrap();
    assert_eq!(
        adversarial.standard_approval_sha,
        "0123456789abcdef0123456789abcdef01234567"
    );
}

#[test]
fn adversarial_sha_must_match_reviewed_sha() {
    let text = adversarial_complete().replacen(
        "standard_approval_sha: 0123456789abcdef0123456789abcdef01234567",
        "standard_approval_sha: fedcba9876543210fedcba9876543210fedcba98",
        1,
    );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn adversarial_blocked_allows_none_fields() {
    let text = reviewer_approved()
        .replacen("status: complete", "status: blocked", 1)
        .replacen("git_sha: 0123456789abcdef0123456789abcdef01234567", "git_sha: none", 1)
        .replacen("review_result: approved", "review_result: not-completed", 1)
        .replacen("reviewed_sha: 0123456789abcdef0123456789abcdef01234567", "reviewed_sha: none", 1)
        .replacen(
            "accepted_risks:\n  - none\n",
            "accepted_risks:\n  - none\nstandard_approval_record_id: none\nstandard_approval_record_identity: none\nstandard_approval_record_sha256: none\nstandard_approval_sha: none\n",
            1,
        )
        .replacen("- none\nassumptions:", "- standard record unavailable\nassumptions:", 1);
    let record = parse_handoff(text.as_bytes()).unwrap();
    let adversarial = record.adversarial.unwrap();
    assert_eq!(adversarial.standard_approval_sha, "none");
}

#[test]
fn adversarial_without_review_extension_fails() {
    let text = format!(
        "{}standard_approval_record_id: sp-record-0123456789abcdef0123456789abcdef\nstandard_approval_record_identity: /x\nstandard_approval_record_sha256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\nstandard_approval_sha: 0123\n",
        worker_complete()
    );
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn trailing_garbage_fails() {
    let text = format!("{}extra: field\n", worker_complete());
    assert!(parse_handoff(text.as_bytes()).is_err());
}

#[test]
fn abbreviated_git_sha_fails() {
    let text = worker_complete().replacen(
        "git_sha: 0123456789abcdef0123456789abcdef01234567",
        "git_sha: 0123456",
        1,
    );
    assert!(parse_handoff(text.as_bytes()).is_err());
}
