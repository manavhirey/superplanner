//! Canonical installation-time model catalog and per-agent selection profile.
//!
//! Trusted installer writes these records outside project control. Launcher,
//! supervisor, and broker independently bind every invocation to their hashes.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::core::{validate_agent_id, validate_digest_form, SizeLimit};
use crate::hashing::sha256_digest;
use crate::json::{canonical_json_of, parse_canonical_json};

pub const MODEL_CATALOG_TAG: &str = "model-catalog-v1";
pub const MODEL_PROFILE_TAG: &str = "model-profile-v1";
pub const MODEL_INSTALLATION_TAG: &str = "model-installation-v1";
pub const AGENT_INSTALLATION_DELTA_TAG: &str = "agent-installation-delta-v1";

pub const REQUIRED_AGENT_IDS: [&str; 10] = [
    "superplanner.adversarial-reviewer",
    "superplanner.brainstormer",
    "superplanner.builder",
    "superplanner.code-reviewer",
    "superplanner.debugger",
    "superplanner.documenter",
    "superplanner.orchestrator",
    "superplanner.orchestrator-glm",
    "superplanner.planner",
    "superplanner.quick",
];

const ADVERSARIAL_CONFLICT_AGENT_IDS: [&str; 4] = [
    "superplanner.builder",
    "superplanner.code-reviewer",
    "superplanner.debugger",
    "superplanner.documenter",
];

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
pub struct ModelRouteRegistration {
    pub model: String,
    pub backend_model: String,
    pub variant: String,
    pub allowed_agent_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelCatalog {
    pub schema: String,
    pub routes: Vec<ModelRouteRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentModelSelection {
    pub agent_id: String,
    pub model: String,
    pub variant: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelProfile {
    pub schema: String,
    pub catalog_sha256: String,
    pub selections: Vec<AgentModelSelection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstalledAgentManifest {
    pub agent_id: String,
    pub source_template_sha256: String,
    pub installed_agent_sha256: String,
    pub installation_delta_sha256: String,
    pub model: String,
    pub variant: String,
}

#[derive(Serialize)]
struct AgentInstallationDelta<'a> {
    schema: &'static str,
    agent_id: &'a str,
    source_template_sha256: &'a str,
    installed_agent_sha256: &'a str,
    changed_fields: [&'static str; 2],
    model: &'a str,
    variant: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelInstallation {
    pub schema: String,
    pub catalog_sha256: String,
    pub profile_sha256: String,
    pub providers: Vec<ProviderManifest>,
    pub agents: Vec<InstalledAgentManifest>,
}

#[derive(Debug, Clone)]
pub struct ValidatedModelPolicy {
    catalog: ModelCatalog,
    profile: ModelProfile,
    installation: ModelInstallation,
    catalog_sha256: String,
    profile_sha256: String,
    installation_sha256: String,
}

impl ModelCatalog {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != MODEL_CATALOG_TAG {
            return Err(format!("unknown model catalog schema: {:?}", self.schema));
        }
        if self.routes.is_empty() {
            return Err("model catalog must register at least one route".to_string());
        }

        let required: BTreeSet<&str> = REQUIRED_AGENT_IDS.into_iter().collect();
        let mut covered = BTreeSet::new();
        let mut previous_route: Option<(&str, &str)> = None;
        for route in &self.routes {
            validate_route_shape(&route.model, &route.variant)?;
            validate_model_id(&route.backend_model)?;
            let route_key = (route.model.as_str(), route.variant.as_str());
            if previous_route.is_some_and(|previous| previous >= route_key) {
                return Err(
                    "model catalog routes must be unique and ordered by model then variant"
                        .to_string(),
                );
            }
            previous_route = Some(route_key);
            if route.allowed_agent_ids.is_empty() {
                return Err("registered route must allow at least one agent".to_string());
            }
            for pair in route.allowed_agent_ids.windows(2) {
                if pair[0] >= pair[1] {
                    return Err(
                        "allowed_agent_ids must be unique and ordered by agent ID".to_string()
                    );
                }
            }
            for agent_id in &route.allowed_agent_ids {
                validate_agent_id(agent_id)?;
                if !required.contains(agent_id.as_str()) {
                    return Err(format!("model catalog names unknown agent {agent_id:?}"));
                }
                covered.insert(agent_id.as_str());
            }
        }
        if covered != required {
            return Err(
                "model catalog must provide at least one route for every agent".to_string(),
            );
        }
        Ok(())
    }

    pub fn allows(&self, agent_id: &str, model: &str, variant: &str) -> bool {
        self.registration_for(model, variant).is_some_and(|route| {
            route
                .allowed_agent_ids
                .binary_search_by(|candidate| candidate.as_str().cmp(agent_id))
                .is_ok()
        })
    }

    pub fn registration_for(&self, model: &str, variant: &str) -> Option<&ModelRouteRegistration> {
        self.routes
            .binary_search_by(|route| {
                (route.model.as_str(), route.variant.as_str()).cmp(&(model, variant))
            })
            .ok()
            .map(|index| &self.routes[index])
    }

    pub fn to_canonical_json(&self) -> Result<String, String> {
        self.validate()?;
        let text = canonical_json_of(self)?;
        SizeLimit::RegisteredRecord.check(text.len())?;
        Ok(text)
    }

    pub fn from_canonical_json(bytes: &[u8]) -> Result<Self, String> {
        let catalog: Self = parse_canonical_json(bytes, SizeLimit::RegisteredRecord)?;
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn canonical_sha256(&self) -> Result<String, String> {
        self.to_canonical_json()
            .map(|text| sha256_digest(text.as_bytes()))
    }
}

impl ModelProfile {
    pub fn validate(&self, catalog: &ModelCatalog) -> Result<(), String> {
        catalog.validate()?;
        if self.schema != MODEL_PROFILE_TAG {
            return Err(format!("unknown model profile schema: {:?}", self.schema));
        }
        validate_digest_form(&self.catalog_sha256)?;
        if self.catalog_sha256 != catalog.canonical_sha256()? {
            return Err("model profile does not bind the supplied catalog".to_string());
        }
        if self.selections.len() != REQUIRED_AGENT_IDS.len() {
            return Err("model profile must select exactly one route for every agent".to_string());
        }
        for (selection, required_agent_id) in self.selections.iter().zip(REQUIRED_AGENT_IDS) {
            validate_agent_id(&selection.agent_id)?;
            validate_route_shape(&selection.model, &selection.variant)?;
            if selection.agent_id != required_agent_id {
                return Err(
                    "model selections must contain every agent exactly once in agent-ID order"
                        .to_string(),
                );
            }
            if !catalog.allows(&selection.agent_id, &selection.model, &selection.variant) {
                return Err(format!(
                    "route {:?} variant {:?} is not registered for agent {:?}",
                    selection.model, selection.variant, selection.agent_id
                ));
            }
        }

        let adversarial = self
            .selection_for("superplanner.adversarial-reviewer")
            .expect("required adversarial selection was checked above");
        let adversarial_backend = catalog
            .registration_for(&adversarial.model, &adversarial.variant)
            .expect("validated selection has a registered route")
            .backend_model
            .as_str();
        for agent_id in ADVERSARIAL_CONFLICT_AGENT_IDS {
            let other = self
                .selection_for(agent_id)
                .expect("required comparison selection was checked above");
            let other_backend = catalog
                .registration_for(&other.model, &other.variant)
                .expect("validated selection has a registered route")
                .backend_model
                .as_str();
            if adversarial_backend == other_backend {
                return Err(format!(
                    "adversarial reviewer backend model must differ from {agent_id} backend model"
                ));
            }
        }
        Ok(())
    }

    pub fn selection_for(&self, agent_id: &str) -> Option<&AgentModelSelection> {
        self.selections
            .binary_search_by(|selection| selection.agent_id.as_str().cmp(agent_id))
            .ok()
            .map(|index| &self.selections[index])
    }

    pub fn to_canonical_json(&self, catalog: &ModelCatalog) -> Result<String, String> {
        self.validate(catalog)?;
        let text = canonical_json_of(self)?;
        SizeLimit::RegisteredRecord.check(text.len())?;
        Ok(text)
    }

    pub fn from_canonical_json(bytes: &[u8], catalog: &ModelCatalog) -> Result<Self, String> {
        let profile: Self = parse_canonical_json(bytes, SizeLimit::RegisteredRecord)?;
        profile.validate(catalog)?;
        Ok(profile)
    }

    pub fn canonical_sha256(&self, catalog: &ModelCatalog) -> Result<String, String> {
        self.to_canonical_json(catalog)
            .map(|text| sha256_digest(text.as_bytes()))
    }
}

impl ProviderManifest {
    pub fn validate(&self) -> Result<(), String> {
        if !valid_route_component(&self.provider) {
            return Err(format!(
                "provider must be a canonical non-empty ID: {:?}",
                self.provider
            ));
        }
        for (label, value) in [
            ("package", self.package.as_str()),
            ("version", self.version.as_str()),
            ("broker_endpoint", self.broker_endpoint.as_str()),
        ] {
            if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_graphic()) {
                return Err(format!(
                    "provider {label} must contain printable non-whitespace ASCII"
                ));
            }
        }
        validate_digest_form(&self.sha256)?;
        validate_broker_endpoint(&self.broker_endpoint)?;
        Ok(())
    }
}

fn validate_broker_endpoint(value: &str) -> Result<(), String> {
    let rest = value
        .strip_prefix("http://127.0.0.1:")
        .or_else(|| value.strip_prefix("http://[::1]:"))
        .ok_or_else(|| "broker endpoint must use an explicit numeric loopback host".to_string())?;
    let (port, path) = rest
        .split_once('/')
        .ok_or_else(|| "broker endpoint must include a port and absolute path".to_string())?;
    if port.is_empty()
        || !port.bytes().all(|byte| byte.is_ascii_digit())
        || (port.len() > 1 && port.starts_with('0'))
    {
        return Err("broker endpoint port must use canonical decimal".to_string());
    }
    let port = port
        .parse::<u16>()
        .map_err(|_| "broker endpoint port must be 1..65535".to_string())?;
    if port == 0
        || path.is_empty()
        || path.contains(['?', '#', '%', '\\'])
        || path
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err("broker endpoint must use port 1..65535 and a fixed path".to_string());
    }
    Ok(())
}

impl InstalledAgentManifest {
    pub fn from_definition_bytes(
        agent_id: &str,
        model: &str,
        variant: &str,
        source_template: &[u8],
        installed_agent: &[u8],
    ) -> Result<Self, String> {
        validate_agent_id(agent_id)?;
        validate_route_shape(model, variant)?;
        require_exact_agent_delta(agent_id, model, variant, source_template, installed_agent)?;
        let mut manifest = Self {
            agent_id: agent_id.to_string(),
            source_template_sha256: sha256_digest(source_template),
            installed_agent_sha256: sha256_digest(installed_agent),
            installation_delta_sha256: String::new(),
            model: model.to_string(),
            variant: variant.to_string(),
        };
        manifest.installation_delta_sha256 = manifest.expected_delta_sha256()?;
        Ok(manifest)
    }

    pub fn validate_definition_bytes(
        &self,
        source_template: &[u8],
        installed_agent: &[u8],
    ) -> Result<(), String> {
        self.validate_manifest()?;
        if self.source_template_sha256 != sha256_digest(source_template)
            || self.installed_agent_sha256 != sha256_digest(installed_agent)
        {
            return Err("installed-agent bytes do not match their manifest hashes".to_string());
        }
        require_exact_agent_delta(
            &self.agent_id,
            &self.model,
            &self.variant,
            source_template,
            installed_agent,
        )
    }

    fn validate_manifest(&self) -> Result<(), String> {
        validate_agent_id(&self.agent_id)?;
        validate_route_shape(&self.model, &self.variant)?;
        validate_digest_form(&self.source_template_sha256)?;
        validate_digest_form(&self.installed_agent_sha256)?;
        validate_digest_form(&self.installation_delta_sha256)?;
        if self.installation_delta_sha256 != self.expected_delta_sha256()? {
            return Err("installed-agent delta evidence does not match its manifest".to_string());
        }
        Ok(())
    }

    fn expected_delta_sha256(&self) -> Result<String, String> {
        let evidence = AgentInstallationDelta {
            schema: AGENT_INSTALLATION_DELTA_TAG,
            agent_id: &self.agent_id,
            source_template_sha256: &self.source_template_sha256,
            installed_agent_sha256: &self.installed_agent_sha256,
            changed_fields: ["model", "variant"],
            model: &self.model,
            variant: &self.variant,
        };
        canonical_json_of(&evidence).map(|json| sha256_digest(json.as_bytes()))
    }
}

fn require_exact_agent_delta(
    agent_id: &str,
    model: &str,
    variant: &str,
    source_template: &[u8],
    installed_agent: &[u8],
) -> Result<(), String> {
    let source = std::str::from_utf8(source_template)
        .map_err(|_| "source agent template must be UTF-8".to_string())?;
    if !source.starts_with("---\n") || source.contains('\r') {
        return Err("source agent template must use LF-delimited frontmatter".to_string());
    }
    let after_open = &source[4..];
    let frontmatter_end = after_open
        .find("\n---\n")
        .ok_or_else(|| "source agent template has no closing frontmatter delimiter".to_string())?;
    let frontmatter = &after_open[..frontmatter_end];
    let body = &after_open[frontmatter_end + 5..];
    let mut rendered = String::with_capacity(source.len() + model.len() + variant.len());
    rendered.push_str("---\n");
    let mut name_count = 0;
    let mut model_count = 0;
    let mut variant_count = 0;
    for (index, line) in frontmatter.split('\n').enumerate() {
        if index > 0 {
            rendered.push('\n');
        }
        if !line.starts_with([' ', '\t']) {
            let key = line
                .split_once(':')
                .map(|(key, _)| key)
                .ok_or_else(|| "source agent has a malformed root frontmatter field".to_string())?;
            if key.is_empty()
                || !key.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || byte == b'_'
                        || byte == b'-'
                })
            {
                return Err(
                    "source agent root frontmatter keys must use canonical bare syntax".to_string(),
                );
            }
        }
        if let Some(value) = line.strip_prefix("name: ") {
            name_count += 1;
            if value != agent_id {
                return Err("source agent name does not match manifest agent ID".to_string());
            }
            rendered.push_str(line);
        } else if line.starts_with("name:") {
            return Err("source agent name must use canonical frontmatter syntax".to_string());
        } else if let Some(value) = line.strip_prefix("model: ") {
            model_count += 1;
            validate_model_id(value)?;
            rendered.push_str("model: ");
            rendered.push_str(model);
        } else if line.starts_with("model:") {
            return Err("source agent model must use canonical frontmatter syntax".to_string());
        } else if let Some(value) = line.strip_prefix("variant: ") {
            variant_count += 1;
            if !valid_route_component(value) {
                return Err("source agent variant is not canonical".to_string());
            }
            rendered.push_str("variant: ");
            rendered.push_str(variant);
        } else if line.starts_with("variant:") {
            return Err("source agent variant must use canonical frontmatter syntax".to_string());
        } else {
            rendered.push_str(line);
        }
    }
    if (name_count, model_count, variant_count) != (1, 1, 1) {
        return Err(
            "source agent frontmatter must contain one name, model, and variant field".to_string(),
        );
    }
    rendered.push_str("\n---\n");
    rendered.push_str(body);
    if rendered.as_bytes() != installed_agent {
        return Err(
            "installed agent must differ from source only by selected model and variant"
                .to_string(),
        );
    }
    Ok(())
}

impl ModelInstallation {
    pub fn validate(&self, catalog: &ModelCatalog, profile: &ModelProfile) -> Result<(), String> {
        profile.validate(catalog)?;
        if self.schema != MODEL_INSTALLATION_TAG {
            return Err(format!(
                "unknown model installation schema: {:?}",
                self.schema
            ));
        }
        validate_digest_form(&self.catalog_sha256)?;
        validate_digest_form(&self.profile_sha256)?;
        if self.catalog_sha256 != catalog.canonical_sha256()? {
            return Err("model installation does not bind the supplied catalog".to_string());
        }
        if self.profile_sha256 != profile.canonical_sha256(catalog)? {
            return Err("model installation does not bind the supplied profile".to_string());
        }
        if self.providers.is_empty() {
            return Err("model installation must pin at least one provider".to_string());
        }
        for pair in self.providers.windows(2) {
            if pair[0].provider >= pair[1].provider {
                return Err(
                    "provider manifests must be unique and ordered by provider ID".to_string(),
                );
            }
        }
        let mut provider_ids = BTreeSet::new();
        for provider in &self.providers {
            provider.validate()?;
            provider_ids.insert(provider.provider.as_str());
        }
        let catalog_provider_ids = catalog
            .routes
            .iter()
            .map(|route| provider_for_model(&route.model))
            .collect::<Result<BTreeSet<_>, _>>()?;
        if provider_ids != catalog_provider_ids {
            return Err(
                "model installation providers must exactly equal catalog providers".to_string(),
            );
        }
        for route in &catalog.routes {
            let provider = provider_for_model(&route.model)?;
            if !provider_ids.contains(provider) {
                return Err(format!(
                    "model installation does not pin provider {provider:?}"
                ));
            }
        }

        if self.agents.len() != REQUIRED_AGENT_IDS.len() {
            return Err(
                "model installation must bind exactly one generated file for every agent"
                    .to_string(),
            );
        }
        for (agent, required_agent_id) in self.agents.iter().zip(REQUIRED_AGENT_IDS) {
            if agent.agent_id != required_agent_id {
                return Err(
                    "installed agents must contain every agent exactly once in agent-ID order"
                        .to_string(),
                );
            }
            agent.validate_manifest()?;
            let selection = profile
                .selection_for(required_agent_id)
                .expect("validated profile contains every required agent");
            if agent.model != selection.model || agent.variant != selection.variant {
                return Err(format!(
                    "installed agent {:?} does not match its selected route",
                    agent.agent_id
                ));
            }
        }
        Ok(())
    }

    pub fn installed_agent_for(&self, agent_id: &str) -> Option<&InstalledAgentManifest> {
        self.agents
            .binary_search_by(|agent| agent.agent_id.as_str().cmp(agent_id))
            .ok()
            .map(|index| &self.agents[index])
    }

    pub fn provider_for(&self, provider_id: &str) -> Option<&ProviderManifest> {
        self.providers
            .binary_search_by(|provider| provider.provider.as_str().cmp(provider_id))
            .ok()
            .map(|index| &self.providers[index])
    }

    pub fn to_canonical_json(
        &self,
        catalog: &ModelCatalog,
        profile: &ModelProfile,
    ) -> Result<String, String> {
        self.validate(catalog, profile)?;
        let text = canonical_json_of(self)?;
        SizeLimit::RegisteredRecord.check(text.len())?;
        Ok(text)
    }

    pub fn from_canonical_json(
        bytes: &[u8],
        catalog: &ModelCatalog,
        profile: &ModelProfile,
    ) -> Result<Self, String> {
        let installation: Self = parse_canonical_json(bytes, SizeLimit::RegisteredRecord)?;
        installation.validate(catalog, profile)?;
        Ok(installation)
    }

    pub fn canonical_sha256(
        &self,
        catalog: &ModelCatalog,
        profile: &ModelProfile,
    ) -> Result<String, String> {
        self.to_canonical_json(catalog, profile)
            .map(|text| sha256_digest(text.as_bytes()))
    }
}

impl ValidatedModelPolicy {
    pub fn new(
        catalog: ModelCatalog,
        profile: ModelProfile,
        installation: ModelInstallation,
    ) -> Result<Self, String> {
        installation.validate(&catalog, &profile)?;
        let catalog_sha256 = catalog.canonical_sha256()?;
        let profile_sha256 = profile.canonical_sha256(&catalog)?;
        let installation_sha256 = installation.canonical_sha256(&catalog, &profile)?;
        Ok(Self {
            catalog,
            profile,
            installation,
            catalog_sha256,
            profile_sha256,
            installation_sha256,
        })
    }

    pub fn catalog(&self) -> &ModelCatalog {
        &self.catalog
    }

    pub fn profile(&self) -> &ModelProfile {
        &self.profile
    }

    pub fn installation(&self) -> &ModelInstallation {
        &self.installation
    }

    pub fn catalog_sha256(&self) -> &str {
        &self.catalog_sha256
    }

    pub fn profile_sha256(&self) -> &str {
        &self.profile_sha256
    }

    pub fn installation_sha256(&self) -> &str {
        &self.installation_sha256
    }
}

pub(crate) fn validate_route_shape(model: &str, variant: &str) -> Result<(), String> {
    validate_model_id(model)?;
    if !valid_route_component(variant) {
        return Err(format!(
            "variant must be a canonical non-empty ID: {variant:?}"
        ));
    }
    Ok(())
}

fn validate_model_id(model: &str) -> Result<(), String> {
    let mut segments = model.split('/');
    let provider = segments.next().unwrap_or_default();
    let model_segments: Vec<&str> = segments.collect();
    if !valid_route_component(provider)
        || model_segments.is_empty()
        || model_segments
            .iter()
            .any(|segment| !valid_route_component(segment))
    {
        return Err(format!(
            "model must be a canonical provider-qualified ID: {model:?}"
        ));
    }
    Ok(())
}

pub(crate) fn provider_for_model(model: &str) -> Result<&str, String> {
    validate_model_id(model)?;
    Ok(model.split('/').next().unwrap())
}

fn valid_route_component(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
}

pub fn default_model_catalog() -> ModelCatalog {
    let agents = |agent_ids: &[&str]| {
        agent_ids
            .iter()
            .map(|agent_id| (*agent_id).to_string())
            .collect()
    };
    ModelCatalog {
        schema: MODEL_CATALOG_TAG.to_string(),
        routes: vec![
            ModelRouteRegistration {
                model: "openai/gpt-5.6-sol".to_string(),
                backend_model: "openai/gpt-5.6-sol".to_string(),
                variant: "high".to_string(),
                allowed_agent_ids: agents(&[
                    "superplanner.adversarial-reviewer",
                    "superplanner.brainstormer",
                    "superplanner.builder",
                    "superplanner.code-reviewer",
                    "superplanner.debugger",
                    "superplanner.documenter",
                ]),
            },
            ModelRouteRegistration {
                model: "openai/gpt-5.6-sol".to_string(),
                backend_model: "openai/gpt-5.6-sol".to_string(),
                variant: "medium".to_string(),
                allowed_agent_ids: agents(&["superplanner.documenter", "superplanner.quick"]),
            },
            ModelRouteRegistration {
                model: "openai/gpt-5.6-sol".to_string(),
                backend_model: "openai/gpt-5.6-sol".to_string(),
                variant: "xhigh".to_string(),
                allowed_agent_ids: agents(&[
                    "superplanner.orchestrator",
                    "superplanner.orchestrator-glm",
                    "superplanner.planner",
                    "superplanner.quick",
                ]),
            },
            ModelRouteRegistration {
                model: "openrouter/moonshotai/kimi-k3".to_string(),
                backend_model: "moonshotai/kimi-k3".to_string(),
                variant: "max".to_string(),
                allowed_agent_ids: agents(&[
                    "superplanner.adversarial-reviewer",
                    "superplanner.brainstormer",
                    "superplanner.builder",
                    "superplanner.code-reviewer",
                    "superplanner.debugger",
                    "superplanner.documenter",
                    "superplanner.planner",
                    "superplanner.quick",
                ]),
            },
            ModelRouteRegistration {
                model: "zai/glm-5.3".to_string(),
                backend_model: "zai/glm-5.3".to_string(),
                variant: "max".to_string(),
                allowed_agent_ids: agents(&REQUIRED_AGENT_IDS),
            },
        ],
    }
}

pub fn default_model_profile() -> ModelProfile {
    let catalog = default_model_catalog();
    let route = |agent_id: &str, model: &str, variant: &str| AgentModelSelection {
        agent_id: agent_id.to_string(),
        model: model.to_string(),
        variant: variant.to_string(),
    };
    ModelProfile {
        schema: MODEL_PROFILE_TAG.to_string(),
        catalog_sha256: catalog
            .canonical_sha256()
            .expect("built-in model catalog must validate"),
        selections: vec![
            route(
                "superplanner.adversarial-reviewer",
                "openrouter/moonshotai/kimi-k3",
                "max",
            ),
            route("superplanner.brainstormer", "openai/gpt-5.6-sol", "high"),
            route("superplanner.builder", "openai/gpt-5.6-sol", "high"),
            route("superplanner.code-reviewer", "openai/gpt-5.6-sol", "high"),
            route("superplanner.debugger", "openai/gpt-5.6-sol", "high"),
            route("superplanner.documenter", "zai/glm-5.3", "max"),
            route("superplanner.orchestrator", "openai/gpt-5.6-sol", "xhigh"),
            route("superplanner.orchestrator-glm", "zai/glm-5.3", "max"),
            route("superplanner.planner", "openai/gpt-5.6-sol", "xhigh"),
            route("superplanner.quick", "zai/glm-5.3", "max"),
        ],
    }
}
