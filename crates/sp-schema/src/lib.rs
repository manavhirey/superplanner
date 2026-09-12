//! Schemas and canonical validators for every Superplanner record.
//!
//! Normative definitions live in `references/record-grammar.md`; this crate
//! enforces exactly those rules and fails closed on unknown forms.

pub mod approval;
pub mod commit;
pub mod core;
pub mod git_mediation;
pub mod handoff;
pub mod hashing;
pub mod identity;
pub mod json;
pub mod push_custody;
pub mod push_encoding;
pub mod supervisor;
pub mod verification;

pub use approval::{ApprovalBlock, ApprovalFlavor};
pub use commit::{AuthorizationTuple, CommitPrepare, COMMIT_AUTHORIZATION_TAG, COMMIT_PREPARE_TAG};
pub use core::{
    HandoffStatus, InvocationState, LeaseState, MonitoringEventType, ReviewResult, Severity,
    SizeLimit,
};
pub use git_mediation::{
    parse_checksum_line, parse_owned_z, parse_raw_manifest, validate_full_ref_name,
    NamespaceManifest, NamespaceRecord, ObjectFormat, RawDiffRecord, RefValue,
    FILES_REF_NAMESPACE_TAG,
};
pub use handoff::{
    parse_handoff, AcceptedRisk, AdversarialExtension, Finding, HandoffRecord, IntegrationResult,
    ReviewExtension, Role, Verification,
};
pub use hashing::{
    approval_content_id, bounded_brief_content_id, sha256_digest, MarkerKind, MarkerPair,
};
pub use identity::{FilesystemIdentity, RegistryIdentity};
pub use push_custody::{
    bind_request_to_authorization, validate_push_refspec, validate_push_url, PushEligibility,
    PushPresentationRequest, ResolvedInput,
};
pub use push_encoding::{decode, encode, Decoded, Field};
pub use supervisor::{
    parse_resume_request, parse_spawn_request, parse_spawn_response, validate_route,
    IsolationEvidence, OperateOperation, OperateResult, OperateStatus, PrincipalVerdict,
    PrivateRoots, ProcessStatus, ProviderManifest, ResumeRequest, SkillEntry, SpawnRequest,
    SpawnResponse,
};
pub use verification::{
    bind_verification_to_documentation, DocumentationRecord, DocumentationStatus,
    VerificationCheck, VerificationRecord, DOCUMENTATION_TAG, VERIFICATION_TAG,
};
