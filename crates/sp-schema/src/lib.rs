//! Schemas and canonical validators for every Superplanner record.
//!
//! Normative definitions live in `references/record-grammar.md`; this crate
//! enforces exactly those rules and fails closed on unknown forms.

pub mod approval;
pub mod core;
pub mod hashing;

pub use approval::{ApprovalBlock, ApprovalFlavor};
pub use core::{
    HandoffStatus, InvocationState, LeaseState, MonitoringEventType, ReviewResult, Severity,
    SizeLimit,
};
pub use hashing::{approval_content_id, bounded_brief_content_id, MarkerKind, MarkerPair};
