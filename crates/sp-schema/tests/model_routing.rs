use sp_schema::model_routing::{
    default_model_catalog, default_model_profile, AgentModelSelection, InstalledAgentManifest,
    ModelCatalog, ModelInstallation, ModelProfile, ModelRouteRegistration, ProviderManifest,
    ValidatedModelPolicy, MODEL_INSTALLATION_TAG,
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
    ModelInstallation {
        schema: MODEL_INSTALLATION_TAG.to_string(),
        catalog_sha256: catalog.canonical_sha256().unwrap(),
        profile_sha256: profile.canonical_sha256(catalog).unwrap(),
        providers: vec![provider("openai"), provider("openrouter"), provider("zai")],
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

#[test]
fn default_records_have_fixed_canonical_hashes() {
    let catalog = default_model_catalog();
    let profile = default_model_profile();
    let installation = installation(&catalog, &profile);
    let policy =
        ValidatedModelPolicy::new(catalog.clone(), profile.clone(), installation.clone()).unwrap();
    assert_eq!(
        policy.catalog_sha256(),
        "1fea2c1b3cb67df731be2e5b3f78da260b7c6fcf69a4e2e7be0ba8c5a07519e3"
    );
    assert_eq!(
        policy.profile_sha256(),
        "5a636fc907f4bc66d4b372be30fd36052e824b6200deae08abdb36a6799a7510"
    );
    assert_eq!(
        policy.installation_sha256(),
        "9672d86d500f95db7dd1e644e3789078d3b77b460882d0c8852be8b39af3fc92"
    );

    let catalog_json = catalog.to_canonical_json().unwrap();
    let profile_json = profile.to_canonical_json(&catalog).unwrap();
    let installation_json = installation.to_canonical_json(&catalog, &profile).unwrap();
    assert_eq!(
        ModelCatalog::from_canonical_json(catalog_json.as_bytes()).unwrap(),
        catalog
    );
    assert_eq!(
        ModelProfile::from_canonical_json(profile_json.as_bytes(), &catalog).unwrap(),
        profile
    );
    assert_eq!(
        ModelInstallation::from_canonical_json(installation_json.as_bytes(), &catalog, &profile)
            .unwrap(),
        installation
    );
}

#[test]
fn checked_in_frontmatter_matches_default_profile() {
    let files = [
        (
            "superplanner.adversarial-reviewer",
            include_str!("../../../agents/superplanner.adversarial-reviewer.md"),
        ),
        (
            "superplanner.brainstormer",
            include_str!("../../../agents/superplanner.brainstormer.md"),
        ),
        (
            "superplanner.builder",
            include_str!("../../../agents/superplanner.builder.md"),
        ),
        (
            "superplanner.code-reviewer",
            include_str!("../../../agents/superplanner.code-reviewer.md"),
        ),
        (
            "superplanner.debugger",
            include_str!("../../../agents/superplanner.debugger.md"),
        ),
        (
            "superplanner.documenter",
            include_str!("../../../agents/superplanner.documenter.md"),
        ),
        (
            "superplanner.orchestrator",
            include_str!("../../../agents/superplanner.orchestrator.md"),
        ),
        (
            "superplanner.orchestrator-glm",
            include_str!("../../../agents/superplanner.orchestrator-glm.md"),
        ),
        (
            "superplanner.planner",
            include_str!("../../../agents/superplanner.planner.md"),
        ),
        (
            "superplanner.quick",
            include_str!("../../../agents/superplanner.quick.md"),
        ),
    ];
    let profile = default_model_profile();
    for (agent_id, contents) in files {
        let selection = profile.selection_for(agent_id).unwrap();
        InstalledAgentManifest::from_definition_bytes(
            agent_id,
            &selection.model,
            &selection.variant,
            contents.as_bytes(),
            contents.as_bytes(),
        )
        .unwrap();
        assert_eq!(frontmatter_value(contents, "name"), agent_id);
        assert_eq!(frontmatter_value(contents, "model"), selection.model);
        assert_eq!(frontmatter_value(contents, "variant"), selection.variant);
    }
}

fn frontmatter_value<'a>(contents: &'a str, key: &str) -> &'a str {
    let after_open = contents.strip_prefix("---\n").unwrap();
    let end = after_open.find("\n---\n").unwrap();
    let prefix = format!("{key}: ");
    let mut values = after_open[..end]
        .lines()
        .filter_map(|line| line.strip_prefix(&prefix));
    let value = values.next().unwrap();
    assert!(values.next().is_none(), "duplicate {key} frontmatter field");
    value
}

#[test]
fn catalog_requires_closed_ordered_complete_routes() {
    let mut catalog = default_model_catalog();
    catalog.routes.swap(0, 1);
    assert!(catalog.validate().is_err());

    let mut catalog = default_model_catalog();
    catalog.routes[0].variant.clear();
    assert!(catalog.validate().is_err());

    let mut catalog = default_model_catalog();
    for route in &mut catalog.routes {
        route
            .allowed_agent_ids
            .retain(|agent_id| agent_id != "superplanner.quick");
    }
    assert!(catalog.validate().is_err());

    let mut catalog = default_model_catalog();
    catalog.routes[0]
        .allowed_agent_ids
        .push("superplanner.unknown".to_string());
    catalog.routes[0].allowed_agent_ids.sort();
    assert!(catalog.validate().is_err());
}

#[test]
fn profile_requires_every_agent_and_registered_role_route() {
    let catalog = default_model_catalog();
    let mut profile = default_model_profile();
    profile.selections.pop();
    assert!(profile.validate(&catalog).is_err());

    let mut profile = default_model_profile();
    profile.selections[1].model = "vendor/unregistered".to_string();
    assert!(profile.validate(&catalog).is_err());

    let mut restricted = default_model_catalog();
    let route = restricted
        .routes
        .iter_mut()
        .find(|route| route.model == "openai/gpt-5.6-sol" && route.variant == "high")
        .unwrap();
    route
        .allowed_agent_ids
        .retain(|agent_id| agent_id != "superplanner.brainstormer");
    assert!(default_model_profile().validate(&restricted).is_err());
}

#[test]
fn registered_custom_route_can_be_selected() {
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
    catalog.validate().unwrap();

    let mut profile = default_model_profile();
    profile.catalog_sha256 = catalog.canonical_sha256().unwrap();
    profile.selections[1] = AgentModelSelection {
        agent_id: "superplanner.brainstormer".to_string(),
        model: "vendor/reasoner-v2".to_string(),
        variant: "deep".to_string(),
    };
    profile.validate(&catalog).unwrap();
}

#[test]
fn adversarial_model_must_differ_from_each_protected_role() {
    let catalog = default_model_catalog();
    for conflict_agent in [
        "superplanner.builder",
        "superplanner.code-reviewer",
        "superplanner.debugger",
    ] {
        let mut profile = default_model_profile();
        for selection in &mut profile.selections {
            if [
                "superplanner.builder",
                "superplanner.code-reviewer",
                "superplanner.debugger",
                "superplanner.documenter",
            ]
            .contains(&selection.agent_id.as_str())
                && selection.agent_id != conflict_agent
            {
                selection.model = "zai/glm-5.3".to_string();
                selection.variant = "max".to_string();
            }
        }
        profile.selections[0].model = "openai/gpt-5.6-sol".to_string();
        profile.selections[0].variant = "high".to_string();
        assert!(profile.validate(&catalog).is_err(), "{conflict_agent}");
    }

    let mut documenter_conflict = default_model_profile();
    documenter_conflict.selections[0].model = "zai/glm-5.3".to_string();
    documenter_conflict.selections[0].variant = "max".to_string();
    assert!(documenter_conflict.validate(&catalog).is_err());

    let mut alias_catalog = default_model_catalog();
    alias_catalog.routes.push(ModelRouteRegistration {
        model: "proxy/independent-name".to_string(),
        backend_model: "openai/gpt-5.6-sol".to_string(),
        variant: "max".to_string(),
        allowed_agent_ids: vec!["superplanner.adversarial-reviewer".to_string()],
    });
    alias_catalog
        .routes
        .sort_by(|left, right| (&left.model, &left.variant).cmp(&(&right.model, &right.variant)));
    let mut alias_profile = default_model_profile();
    alias_profile.catalog_sha256 = alias_catalog.canonical_sha256().unwrap();
    alias_profile.selections[0].model = "proxy/independent-name".to_string();
    alias_profile.selections[0].variant = "max".to_string();
    assert!(alias_profile.validate(&alias_catalog).is_err());
}

#[test]
fn installation_pins_provider_and_generated_agent_manifests() {
    let catalog = default_model_catalog();
    let profile = default_model_profile();
    let mut record = installation(&catalog, &profile);
    record.providers.swap(0, 1);
    assert!(record.validate(&catalog, &profile).is_err());

    let mut record = installation(&catalog, &profile);
    record.agents[1].installed_agent_sha256 = "bad".to_string();
    assert!(record.validate(&catalog, &profile).is_err());

    let mut record = installation(&catalog, &profile);
    record.agents[1].model = "zai/glm-5.3".to_string();
    assert!(record.validate(&catalog, &profile).is_err());

    let mut record = installation(&catalog, &profile);
    record.providers[0].broker_endpoint = "https://api.example.com/v1".to_string();
    assert!(record.validate(&catalog, &profile).is_err());

    let mut record = installation(&catalog, &profile);
    record.providers[0].broker_endpoint = "http://[::1]:39421/openai".to_string();
    assert!(record.validate(&catalog, &profile).is_ok());

    for endpoint in [
        "http://localhost:39421/openai",
        "http://127.0.0.1:039421/openai",
        "http://127.0.0.1:0/openai",
        "http://127.0.0.1:65536/openai",
        "http://127.0.0.1:39421",
        "http://127.0.0.1:39421//openai",
        "http://127.0.0.1:39421/./openai",
        "http://127.0.0.1:39421/../openai",
        "http://127.0.0.1:39421/%2e/openai",
        "http://127.0.0.1:39421/open\\ai",
        "http://127.0.0.1:39421/openai?route=other",
        "http://127.0.0.1:39421/openai#other",
    ] {
        let mut record = installation(&catalog, &profile);
        record.providers[0].broker_endpoint = endpoint.to_string();
        assert!(record.validate(&catalog, &profile).is_err(), "{endpoint}");
    }

    let mut record = installation(&catalog, &profile);
    record.providers[0].package = "provider\u{2028}openai".to_string();
    assert!(record.validate(&catalog, &profile).is_err());

    let mut record = installation(&catalog, &profile);
    record.providers.push(provider("unused"));
    record
        .providers
        .sort_by(|left, right| left.provider.cmp(&right.provider));
    assert!(record.validate(&catalog, &profile).is_err());
}

#[test]
fn installed_agent_attests_only_model_and_variant_delta() {
    let source = b"---\nname: superplanner.builder\nmode: subagent\nmodel: openai/gpt-5.6-sol\nvariant: high\n---\nTrusted body.\n";
    let installed = b"---\nname: superplanner.builder\nmode: subagent\nmodel: zai/glm-5.3\nvariant: max\n---\nTrusted body.\n";
    let manifest = InstalledAgentManifest::from_definition_bytes(
        "superplanner.builder",
        "zai/glm-5.3",
        "max",
        source,
        installed,
    )
    .unwrap();
    manifest
        .validate_definition_bytes(source, installed)
        .unwrap();

    let altered = b"---\nname: superplanner.builder\nmode: primary\nmodel: zai/glm-5.3\nvariant: max\n---\nTrusted body.\n";
    assert!(InstalledAgentManifest::from_definition_bytes(
        "superplanner.builder",
        "zai/glm-5.3",
        "max",
        source,
        altered,
    )
    .is_err());

    for duplicate in [
        b"---\nname: superplanner.builder\nmodel: openai/gpt-5.6-sol\n\"model\": zai/glm-5.3\nvariant: high\n---\nTrusted body.\n".as_slice(),
        b"---\nname: superplanner.builder\nmodel: openai/gpt-5.6-sol\nmodel : zai/glm-5.3\nvariant: high\n---\nTrusted body.\n".as_slice(),
    ] {
        assert!(InstalledAgentManifest::from_definition_bytes(
            "superplanner.builder",
            "zai/glm-5.3",
            "max",
            duplicate,
            installed,
        )
        .is_err());
    }
}

#[test]
fn profile_rejects_catalog_drift_and_noncanonical_json() {
    let catalog = default_model_catalog();
    let mut profile = default_model_profile();
    profile.catalog_sha256 =
        "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210".to_string();
    assert!(profile.validate(&catalog).is_err());

    let profile = default_model_profile();
    let pretty = serde_json::to_string_pretty(&profile).unwrap();
    assert!(ModelProfile::from_canonical_json(pretty.as_bytes(), &catalog).is_err());
}
