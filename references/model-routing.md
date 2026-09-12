# Model Routing

Use canonical provider-qualified model IDs and exact non-empty variants. Model
selection is an installation-time trusted action, not a prompt instruction or a
project-controlled OpenCode override.

## Installation Records

The trusted installer presents every registered role-compatible route and asks
the user to select one route for each of the ten shipped agent IDs. It writes three
canonical immutable records outside the target project and every Git directory:

- `model-catalog-v1`: `{ schema, routes }`, where each uniquely ordered route is
  `{ model, backend_model, variant, allowed_agent_ids }`. Every route model and
  immutable backend-model identity is provider-qualified, every route names at
  least one allowed agent, and the catalog provides at least one route for every
  shipped agent. Aliases for the same backend repeat its exact backend identity.
- `model-profile-v1`: `{ schema, catalog_sha256, selections }`, where selections
  contain every shipped agent exactly once in agent-ID byte order and each
  selection is `{ agent_id, model, variant }` authorized by the bound catalog.
- `model-installation-v1`: `{ schema, catalog_sha256, profile_sha256, providers,
  agents }`. Ordered provider manifests pin provider ID, package, version,
  SHA-256, and broker endpoint. Ordered agent manifests bind each agent ID to its
  source-template hash, generated installed-agent hash, canonical
  `agent-installation-delta-v1` evidence hash, model, and variant.

The installer generates the private installed agent registry from the attested
source templates by changing only each selected `model` and `variant`. It
byte-compares the deterministic rewrite against the generated file and records
the source-template, catalog, profile, generated-agent, and canonical delta
identities and hashes.
Project or ambient user configuration cannot add a route or override a selected
route.

The launcher resolves and validates the exact installation, profile, and catalog
records once for a new workflow. Every spawn request carries all three hashes and
the source-template and installed-agent hashes. The supervisor rejects a
requested route, source agent, or provider manifest unless each exactly equals
the prevalidated installation record. The broker accepts only that
provider/model/variant. The supervisor accepts a spawn response only when its
observed agent equals the requested runtime wrapper. For resume, it resolves the
registered original request and response server-side, requires the exact original
request, returned session ID, and canonical response hash, and checks fresh
broker-readiness evidence against the original immutable policy. That canonical
`model-route-readiness-v1` evidence binds the three policy hashes, selected
agent/model/variant, provider-manifest hash, checked-at time, one-use challenge,
probe hash, and passing status. The supervisor registers only a passing broker
result for its pending challenge, resolves it server-side, and atomically consumes
it during resume. A blocked result burns its challenge and cannot later be
upgraded to passing. An explicit
installation-profile update affects new workflows only.

## Built-In Catalog

| Model and variant | Backend-model identity | Allowed agents |
| --- | --- | --- |
| `openai/gpt-5.6-sol` `high` | `openai/gpt-5.6-sol` | `superplanner.adversarial-reviewer`, `superplanner.brainstormer`, `superplanner.builder`, `superplanner.code-reviewer`, `superplanner.debugger`, `superplanner.documenter` |
| `openai/gpt-5.6-sol` `medium` | `openai/gpt-5.6-sol` | `superplanner.documenter`, `superplanner.quick` |
| `openai/gpt-5.6-sol` `xhigh` | `openai/gpt-5.6-sol` | `superplanner.orchestrator`, `superplanner.orchestrator-glm`, `superplanner.planner`, `superplanner.quick` |
| `openrouter/moonshotai/kimi-k3` `max` | `moonshotai/kimi-k3` | `superplanner.adversarial-reviewer`, `superplanner.brainstormer`, `superplanner.builder`, `superplanner.code-reviewer`, `superplanner.debugger`, `superplanner.documenter`, `superplanner.planner`, `superplanner.quick` |
| `zai/glm-5.3` `max` | `zai/glm-5.3` | all ten shipped agents |

## Default Profile

Checked-in agent frontmatter and the built-in catalog provide these defaults:

| Agent | Model ID | Variant |
| --- | --- | --- |
| `superplanner.orchestrator` | `openai/gpt-5.6-sol` | `xhigh` |
| `superplanner.orchestrator-glm` | `zai/glm-5.3` | `max` |
| `superplanner.quick` | `zai/glm-5.3` | `max` |
| `superplanner.brainstormer` | `openai/gpt-5.6-sol` | `high` |
| `superplanner.planner` | `openai/gpt-5.6-sol` | `xhigh` |
| `superplanner.builder` | `openai/gpt-5.6-sol` | `high` |
| `superplanner.debugger` | `openai/gpt-5.6-sol` | `high` |
| `superplanner.documenter` | `zai/glm-5.3` | `max` |
| `superplanner.code-reviewer` | `openai/gpt-5.6-sol` | `high` |
| `superplanner.adversarial-reviewer` | `openrouter/moonshotai/kimi-k3` | `max` |

`superplanner.orchestrator-glm` remains a separate entry handle for compatibility
and defaults to GLM, but its installed route is selected and attested like every
other agent.

## Selection Rules

- Select all ten routes during trusted installation. There is no per-prompt or
  project-config selection.
- A catalog may register models beyond the built-in OpenAI, ZAI, and OpenRouter
  defaults. Each route includes the exact role-compatible model and variant. The
  trusted installation manifest separately pins the corresponding provider
  implementation and broker route required by `handoff-contract.md`.
- The adversarial review backend-model identity must differ from the builder,
  debugger, documenter, and standard-review backend-model identities. A route
  alias or variant-only difference does not provide reviewer independence.
- Classify work as simple or complex before dispatch and state the selected
  coordinator route to the user. Hidden complexity upgrades the workflow but
  does not silently change any agent route.
- Missing model access, provider authentication, catalog entry, or readiness
  evidence blocks dispatch. No automatic, ordered, or silent fallback exists.
- Route changes require an explicit installer action. They never alter or
  migrate an active workflow.

## Availability And Authentication

Before workflow start, the installer and launcher probe every route selected by
the profile through the credential-isolating broker. A failed probe identifies
the exact unavailable agent/model/variant and blocks startup. The user may
update the installation profile explicitly, but runtime never substitutes a
route.

Coordinators, candidate-file specialists, and reviewers receive model responses
only through the broker in `handoff-contract.md`; their OpenCode processes never
receive provider credentials or general provider-network access. Spawn and
resume evidence bind the catalog hash, profile hash, installation hash, selected
route, source template, generated agent definition, provider
implementation/version/hash, and sole broker endpoint.

## Provider Registration

Provider IDs are installation-defined canonical IDs, not aliases discovered
from stock OpenCode catalogs. The launcher's pinned private `OPENCODE_CONFIG`
registers only catalog providers as broker-backed custom providers. Model
metadata is declared in the same pinned configuration because
`OPENCODE_DISABLE_MODELS_FETCH=1` forbids catalog fetching. Provider redirects,
ambient provider overrides, unregistered endpoints, and a provider ID that does
not match the model-ID prefix are invalid. Broker endpoints use an explicit
numeric loopback host, nonzero port, and fixed absolute path; remote hosts,
userinfo, path aliases, percent encoding, backslashes, query strings, and
fragments are invalid.
