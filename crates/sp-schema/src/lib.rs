//! Schemas and canonical validators for every Superplanner record.
//!
//! Normative definitions live in `references/record-grammar.md`; this crate
//! enforces exactly those rules and fails closed on unknown forms.

pub mod approval;
pub mod core;
pub mod handoff;
pub mod hashing;
pub mod json;
pub mod supervisor;

pub use approval::{ApprovalBlock, ApprovalFlavor};
pub use core::{
    HandoffStatus, InvocationState, LeaseState, MonitoringEventType, ReviewResult, Severity,
    SizeLimit,
};
pub use handoff::{
    parse_handoff, AcceptedRisk, AdversarialExtension, Finding, HandoffRecord, IntegrationResult,
    ReviewExtension, Role, Verification,
};
pub use hashing::{approval_content_id, bounded_brief_content_id, MarkerKind, MarkerPair};
pub use supervisor::{
    parse_resume_request, parse_spawn_request, parse_spawn_response, validate_route,
    IsolationEvidence, OperateOperation, OperateResult, OperateStatus, PrincipalVerdict,
    PrivateRoots, ProcessStatus, ProviderManifest, ResumeRequest, SkillEntry, SpawnRequest,
    SpawnResponse,
};
