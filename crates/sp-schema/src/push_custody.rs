//! Push-custody schema foundations per `references/quality-gates.md`: closed
//! HTTPS/refspec validation plus `push-presentation-request-v1` and
//! `push-eligibility-v1`. Layout, execution, command, presentation, and
//! retirement records land with push runtime work.

use crate::commit::AuthorizationTuple;
use crate::core::{
    validate_digest_form, validate_git_sha_width, validate_lease_id, validate_sp_id,
};
use crate::git_mediation::{validate_full_ref_name, ObjectFormat};
use crate::identity::{FilesystemIdentity, RegistryIdentity};
use crate::push_encoding::{decode, encode, Decoded, Field};

// ---------------------------------------------------------------------------
// Destination grammar
// ---------------------------------------------------------------------------

/// Validate the closed helper-free HTTPS URL grammar:
/// `https://<dns-host>[:<1-65535>]/<path>` with no userinfo, query,
/// fragment, backslash, whitespace, control byte, DEL, `::`, or IPv6
/// literal.
pub fn validate_push_url(url: &str) -> Result<(), String> {
    let rest = url
        .strip_prefix("https://")
        .ok_or_else(|| format!("destination must use the https scheme: {url:?}"))?;
    if url
        .bytes()
        .any(|b| b == b'\\' || b == 0x7f || b.is_ascii_control())
    {
        return Err("destination carries backslash or control bytes".to_string());
    }
    if url.contains("::") {
        return Err("destination carries an IPv6 literal or doubled colon".to_string());
    }
    if url.chars().any(|c| c.is_whitespace()) {
        return Err("destination carries whitespace".to_string());
    }
    let (authority, path) = rest
        .split_once('/')
        .ok_or_else(|| format!("destination must carry a path after the host: {url:?}"))?;
    if path.is_empty() {
        return Err("destination path must not be empty".to_string());
    }
    if authority.contains('@') {
        return Err("destination must not carry userinfo".to_string());
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (host, Some(port)),
        None => (authority, None),
    };
    validate_dns_host(host)?;
    if let Some(port) = port {
        if port.is_empty() || !port.bytes().all(|b| b.is_ascii_digit()) || port.starts_with('0') {
            return Err(format!("port must be canonical decimal digits: {port:?}"));
        }
        let value: u32 = port
            .parse()
            .map_err(|_| format!("unparsable port: {port:?}"))?;
        if !(1..=65535).contains(&value) {
            return Err(format!("port out of range: {port:?}"));
        }
    }
    if host.contains('[') || host.contains(']') {
        return Err("destination must not use IPv6 literals".to_string());
    }
    if path.contains('?') || path.contains('#') {
        return Err("destination must not carry a query or fragment".to_string());
    }
    Ok(())
}

fn validate_dns_host(host: &str) -> Result<(), String> {
    if host.is_empty() || host.len() > 253 {
        return Err(format!("invalid host length: {host:?}"));
    }
    for label in host.split('.') {
        if label.is_empty() || label.len() > 63 {
            return Err(format!("invalid DNS label: {label:?}"));
        }
        if !label
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err(format!("invalid DNS label bytes: {label:?}"));
        }
        if label.starts_with('-') || label.ends_with('-') {
            return Err(format!("invalid DNS label placement: {label:?}"));
        }
    }
    Ok(())
}

/// Validate a rendered refspec operand: exactly one full local ref, one
/// colon, one full remote ref, no leading `+`, and no control bytes or DEL
/// in any rendered operand (`references/quality-gates.md`).
pub fn validate_push_refspec(refspec: &str) -> Result<(), String> {
    if refspec.starts_with('+') {
        return Err("the no-force policy rejects a leading +".to_string());
    }
    let (local, remote) = refspec.split_once(':').ok_or_else(|| {
        format!("refspec must be `<full-local-ref>:<full-remote-ref>`: {refspec:?}")
    })?;
    validate_full_ref_name(local)?;
    validate_full_ref_name(remote)?;
    Ok(())
}

/// Every rendered operand rejects control bytes and DEL.
pub fn validate_rendered_operand(value: &str) -> Result<(), String> {
    if value.bytes().any(|b| b.is_ascii_control() || b == 0x7f) {
        return Err(format!("rendered operand carries control bytes: {value:?}"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// push-presentation-request-v1
// ---------------------------------------------------------------------------

pub const PUSH_PRESENTATION_REQUEST_TAG: &str = "push-presentation-request-v1";

/// The closed deterministic payload created only by the trusted
/// user-decision recorder from an authenticated explicit user action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushPresentationRequest {
    pub user_decision_record_id: String,
    pub user_decision_record_sha256: String,
    pub commit_result_id: String,
    pub commit_result_sha256: String,
    pub commit_full_oid: String,
    pub standard_review_id: String,
    pub standard_review_identity: String,
    pub standard_review_sha256: String,
    pub adversarial_review_id: String,
    pub adversarial_review_identity: String,
    pub adversarial_review_sha256: String,
    pub destination_snapshot_id: String,
    pub destination_snapshot_sha256: String,
    pub remote_name: String,
    pub literal_url: String,
    pub source_ref: String,
    pub destination_ref: String,
    pub refspec: String,
    pub no_force_policy: String,
}

impl PushPresentationRequest {
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        self.validate()?;
        encode(
            PUSH_PRESENTATION_REQUEST_TAG,
            &[
                Field::scalar_str(&self.user_decision_record_id),
                Field::scalar_str(&self.user_decision_record_sha256),
                Field::scalar_str(&self.commit_result_id),
                Field::scalar_str(&self.commit_result_sha256),
                Field::scalar_str(&self.commit_full_oid),
                Field::scalar_str(&self.standard_review_id),
                Field::scalar_str(&self.standard_review_identity),
                Field::scalar_str(&self.standard_review_sha256),
                Field::scalar_str(&self.adversarial_review_id),
                Field::scalar_str(&self.adversarial_review_identity),
                Field::scalar_str(&self.adversarial_review_sha256),
                Field::scalar_str(&self.destination_snapshot_id),
                Field::scalar_str(&self.destination_snapshot_sha256),
                Field::scalar_str(&self.remote_name),
                Field::scalar_str(&self.literal_url),
                Field::scalar_str(&self.source_ref),
                Field::scalar_str(&self.destination_ref),
                Field::scalar_str(&self.refspec),
                Field::scalar_str(&self.no_force_policy),
            ],
        )
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let decoded: Decoded = decode(bytes)?;
        if decoded.tag != PUSH_PRESENTATION_REQUEST_TAG {
            return Err(format!("unknown tag: {:?}", decoded.tag));
        }
        if decoded.fields.len() != 19 {
            return Err(format!(
                "push-presentation-request-v1 must carry 19 fields, found {}",
                decoded.fields.len()
            ));
        }
        let scalars: Vec<String> = decoded
            .fields
            .iter()
            .map(|f| f.as_scalar_str())
            .collect::<Result<_, _>>()?;
        let request = PushPresentationRequest {
            user_decision_record_id: scalars[0].clone(),
            user_decision_record_sha256: scalars[1].clone(),
            commit_result_id: scalars[2].clone(),
            commit_result_sha256: scalars[3].clone(),
            commit_full_oid: scalars[4].clone(),
            standard_review_id: scalars[5].clone(),
            standard_review_identity: scalars[6].clone(),
            standard_review_sha256: scalars[7].clone(),
            adversarial_review_id: scalars[8].clone(),
            adversarial_review_identity: scalars[9].clone(),
            adversarial_review_sha256: scalars[10].clone(),
            destination_snapshot_id: scalars[11].clone(),
            destination_snapshot_sha256: scalars[12].clone(),
            remote_name: scalars[13].clone(),
            literal_url: scalars[14].clone(),
            source_ref: scalars[15].clone(),
            destination_ref: scalars[16].clone(),
            refspec: scalars[17].clone(),
            no_force_policy: scalars[18].clone(),
        };
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<(), String> {
        for (value, name) in [
            (&self.user_decision_record_sha256, "user decision hash"),
            (&self.commit_result_sha256, "commit result hash"),
            (&self.standard_review_sha256, "standard review hash"),
            (&self.adversarial_review_sha256, "adversarial review hash"),
            (
                &self.destination_snapshot_sha256,
                "destination snapshot hash",
            ),
        ] {
            validate_digest_form(value).map_err(|e| format!("{name}: {e}"))?;
        }
        for (id, name) in [
            (&self.user_decision_record_id, "user_decision_record_id"),
            (&self.commit_result_id, "commit_result_id"),
            (&self.standard_review_id, "standard_review_id"),
            (&self.adversarial_review_id, "adversarial_review_id"),
            (&self.destination_snapshot_id, "destination_snapshot_id"),
        ] {
            validate_sp_id("record", id).map_err(|e| format!("{name}: {e}"))?;
        }
        for (identity, hash, name) in [
            (
                &self.standard_review_identity,
                &self.standard_review_sha256,
                "standard_review_identity",
            ),
            (
                &self.adversarial_review_identity,
                &self.adversarial_review_sha256,
                "adversarial_review_identity",
            ),
        ] {
            let parsed = RegistryIdentity::from_canonical_scalar(identity)
                .map_err(|error| format!("{name}: {error}"))?;
            if parsed.sha256 != *hash {
                return Err(format!("{name} does not bind its record hash"));
            }
        }
        if self.remote_name.is_empty() {
            return Err("remote name must not be empty".to_string());
        }
        validate_rendered_operand(&self.remote_name)?;
        validate_push_url(&self.literal_url)?;
        validate_full_ref_name(&self.source_ref)?;
        validate_full_ref_name(&self.destination_ref)?;
        validate_push_refspec(&self.refspec)?;
        let expected_refspec = format!("{}:{}", self.source_ref, self.destination_ref);
        if self.refspec != expected_refspec {
            return Err("refspec must exactly bind the source and destination refs".to_string());
        }
        if self.no_force_policy != "no-force" {
            return Err("the request must carry the no-force policy".to_string());
        }
        let format = if self.commit_full_oid.len() == 40 {
            ObjectFormat::Sha1
        } else if self.commit_full_oid.len() == 64 {
            ObjectFormat::Sha256
        } else {
            return Err("commit_full_oid must be a full 40- or 64-hex OID".to_string());
        };
        validate_git_sha_width(&self.commit_full_oid, format, "commit_full_oid")?;
        validate_rendered_operand(&self.refspec)?;
        validate_rendered_operand(&self.literal_url)?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// push-eligibility-v1
// ---------------------------------------------------------------------------

pub const PUSH_ELIGIBILITY_TAG: &str = "push-eligibility-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedInput {
    pub record_type: String,
    pub record_id: String,
    pub record_identity: String,
    pub record_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushEligibility {
    pub resolved_inputs: Vec<ResolvedInput>,
    pub snapshot_id: String,
    pub snapshot_identity: String,
    pub snapshot_sha256: String,
    pub approved_full_sha: String,
    pub source_ref: String,
    pub remote_name: String,
    pub literal_url: String,
    pub refspec: String,
    pub object_ref_format: ObjectFormat,
    pub no_force_policy: String,
    pub layout_record_id: String,
    pub layout_record_identity: String,
    pub layout_record_sha256: String,
    pub closure_record_id: String,
    pub closure_record_identity: String,
    pub closure_record_sha256: String,
    pub command_record_id: String,
    pub command_record_identity: String,
    pub command_record_sha256: String,
    pub custody_root_identity: String,
    pub lease_id: String,
}

impl PushEligibility {
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        self.validate()?;
        encode(
            PUSH_ELIGIBILITY_TAG,
            &[
                Field::Collection(
                    self.resolved_inputs
                        .iter()
                        .map(|input| {
                            vec![
                                Field::scalar_str(&input.record_type),
                                Field::scalar_str(&input.record_id),
                                Field::scalar_str(&input.record_identity),
                                Field::scalar_str(&input.record_sha256),
                            ]
                        })
                        .collect(),
                ),
                Field::scalar_str(&self.snapshot_id),
                Field::scalar_str(&self.snapshot_identity),
                Field::scalar_str(&self.snapshot_sha256),
                Field::scalar_str(&self.approved_full_sha),
                Field::scalar_str(&self.source_ref),
                Field::scalar_str(&self.remote_name),
                Field::scalar_str(&self.literal_url),
                Field::scalar_str(&self.refspec),
                Field::scalar_str(match self.object_ref_format {
                    ObjectFormat::Sha1 => "sha1/files",
                    ObjectFormat::Sha256 => "sha256/files",
                }),
                Field::scalar_str(&self.no_force_policy),
                Field::scalar_str(&self.layout_record_id),
                Field::scalar_str(&self.layout_record_identity),
                Field::scalar_str(&self.layout_record_sha256),
                Field::scalar_str(&self.closure_record_id),
                Field::scalar_str(&self.closure_record_identity),
                Field::scalar_str(&self.closure_record_sha256),
                Field::scalar_str(&self.command_record_id),
                Field::scalar_str(&self.command_record_identity),
                Field::scalar_str(&self.command_record_sha256),
                Field::scalar_str(&self.custody_root_identity),
                Field::scalar_str(&self.lease_id),
            ],
        )
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let decoded: Decoded = decode(bytes)?;
        if decoded.tag != PUSH_ELIGIBILITY_TAG {
            return Err(format!("unknown tag: {:?}", decoded.tag));
        }
        if decoded.fields.len() != 22 {
            return Err(format!(
                "push-eligibility-v1 must carry 22 fields, found {}",
                decoded.fields.len()
            ));
        }
        let inputs_collection = decoded.fields[0].as_collection()?;
        let mut resolved_inputs = Vec::with_capacity(inputs_collection.len());
        for item in inputs_collection {
            if item.len() != 4 {
                return Err("each resolved input must carry 4 scalar fields".to_string());
            }
            let values: Vec<String> = item
                .iter()
                .map(|f| f.as_scalar_str())
                .collect::<Result<_, _>>()?;
            resolved_inputs.push(ResolvedInput {
                record_type: values[0].clone(),
                record_id: values[1].clone(),
                record_identity: values[2].clone(),
                record_sha256: values[3].clone(),
            });
        }
        let scalars: Vec<String> = decoded.fields[1..]
            .iter()
            .map(|f| f.as_scalar_str())
            .collect::<Result<_, _>>()?;
        let eligibility = PushEligibility {
            resolved_inputs,
            snapshot_id: scalars[0].clone(),
            snapshot_identity: scalars[1].clone(),
            snapshot_sha256: scalars[2].clone(),
            approved_full_sha: scalars[3].clone(),
            source_ref: scalars[4].clone(),
            remote_name: scalars[5].clone(),
            literal_url: scalars[6].clone(),
            refspec: scalars[7].clone(),
            object_ref_format: parse_object_ref_format(&scalars[8])?,
            no_force_policy: scalars[9].clone(),
            layout_record_id: scalars[10].clone(),
            layout_record_identity: scalars[11].clone(),
            layout_record_sha256: scalars[12].clone(),
            closure_record_id: scalars[13].clone(),
            closure_record_identity: scalars[14].clone(),
            closure_record_sha256: scalars[15].clone(),
            command_record_id: scalars[16].clone(),
            command_record_identity: scalars[17].clone(),
            command_record_sha256: scalars[18].clone(),
            custody_root_identity: scalars[19].clone(),
            lease_id: scalars[20].clone(),
        };
        eligibility.validate()?;
        Ok(eligibility)
    }

    pub fn validate(&self) -> Result<(), String> {
        let required_inputs = [
            "source-context",
            "object-route",
            "authorization",
            "commit-result",
            "standard-review",
            "adversarial-review",
            "presentation-request",
            "verification",
            "documentation",
            "destination-snapshot",
        ];
        if self.resolved_inputs.len() != required_inputs.len()
            || !self
                .resolved_inputs
                .iter()
                .zip(required_inputs)
                .all(|(input, expected)| input.record_type == expected)
        {
            return Err(
                "resolved inputs must appear exactly once in request-schema order".to_string(),
            );
        }
        for input in &self.resolved_inputs {
            validate_digest_form(&input.record_sha256)?;
            validate_sp_id("record", &input.record_id)?;
            let identity = RegistryIdentity::from_canonical_scalar(&input.record_identity)?;
            if identity.sha256 != input.record_sha256 {
                return Err("resolved input identity does not bind its record hash".to_string());
            }
        }
        for index in 0..self.resolved_inputs.len() {
            let current = &self.resolved_inputs[index];
            if self.resolved_inputs[..index]
                .iter()
                .any(|prior| prior.record_id == current.record_id)
            {
                return Err("resolved input record IDs must be unique".to_string());
            }
            if self.resolved_inputs[..index]
                .iter()
                .any(|prior| prior.record_identity == current.record_identity)
            {
                return Err("resolved input identities must be unique".to_string());
            }
        }
        validate_git_sha_width(
            &self.approved_full_sha,
            self.object_ref_format,
            "approved_full_sha",
        )?;
        validate_push_url(&self.literal_url)?;
        validate_full_ref_name(&self.source_ref)?;
        validate_push_refspec(&self.refspec)?;
        let (local_ref, _) = self.refspec.split_once(':').expect("validated refspec");
        if local_ref != self.source_ref {
            return Err("refspec local ref must equal source_ref".to_string());
        }
        if self.remote_name.is_empty() {
            return Err("remote name must not be empty".to_string());
        }
        validate_rendered_operand(&self.remote_name)?;
        if self.no_force_policy != "no-force" {
            return Err("eligibility must carry the no-force policy".to_string());
        }
        for hash in [
            &self.snapshot_sha256,
            &self.layout_record_sha256,
            &self.closure_record_sha256,
            &self.command_record_sha256,
        ] {
            validate_digest_form(hash)?;
        }
        for (id, name) in [
            (&self.snapshot_id, "snapshot_id"),
            (&self.layout_record_id, "layout_record_id"),
            (&self.closure_record_id, "closure_record_id"),
            (&self.command_record_id, "command_record_id"),
        ] {
            validate_sp_id("record", id).map_err(|e| format!("{name}: {e}"))?;
        }
        for (identity, hash) in [
            (&self.snapshot_identity, &self.snapshot_sha256),
            (&self.layout_record_identity, &self.layout_record_sha256),
            (&self.closure_record_identity, &self.closure_record_sha256),
            (&self.command_record_identity, &self.command_record_sha256),
        ] {
            let parsed = RegistryIdentity::from_canonical_scalar(identity)?;
            if parsed.sha256 != *hash {
                return Err("eligibility record identity does not bind its hash".to_string());
            }
        }
        FilesystemIdentity::from_canonical_scalar(&self.custody_root_identity)?;
        validate_lease_id(&self.lease_id)?;
        Ok(())
    }
}

fn parse_object_ref_format(value: &str) -> Result<ObjectFormat, String> {
    match value {
        "sha1/files" => Ok(ObjectFormat::Sha1),
        "sha256/files" => Ok(ObjectFormat::Sha256),
        other => Err(format!(
            "object/ref format must be sha1/files or sha256/files, not {other:?}"
        )),
    }
}

/// The cross-check tying the presentation request to the authorization: the
/// authorization's expected OID must equal the request's commit OID.
pub fn bind_request_to_authorization(
    request: &PushPresentationRequest,
    authorization: &AuthorizationTuple,
    format: ObjectFormat,
) -> Result<(), String> {
    request.validate()?;
    authorization.validate(format)?;
    crate::core::validate_git_sha_width(
        &request.commit_full_oid,
        format,
        "request.commit_full_oid",
    )?;
    if request.commit_full_oid != authorization.expected_commit_oid {
        return Err(format!(
            "the presentation request binds commit {} but the authorization expects {}",
            request.commit_full_oid, authorization.expected_commit_oid
        ));
    }
    if request.source_ref != authorization.target_ref {
        return Err(format!(
            "the presentation request uses source ref {} but the authorization names {}",
            request.source_ref, authorization.target_ref
        ));
    }
    Ok(())
}
