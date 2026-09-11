# Model Routing

Use canonical provider-qualified model IDs and exact variants.

| Route | Model ID | Variant | Use |
| --- | --- | --- | --- |
| Complex coordinator entry | `openai/gpt-5.6-sol` | `xhigh` | Default coordinator for the full complex pipeline |
| Complex coordinator entry | `zai/glm-5.3` | `max` | Explicit alternate coordinator for the same full complex pipeline |
| Simple coordinator entry | `zai/glm-5.3` | `max` | Spikes and simple, bounded work |
| Simple coordinator alternative | `openai/gpt-5.6-sol` | `medium` | Explicit installation-time or user-selected simple route |
| Simple coordinator alternative | `openrouter/moonshotai/kimi-k3` | `max` | Explicit installation-time or user-selected simple route |

Coordinator selection does not reroute specialists. Each specialist uses its
configured route:

| Specialist | Model ID | Variant |
| --- | --- | --- |
| Brainstormer | `openai/gpt-5.6-sol` | `high` |
| Planner | `openai/gpt-5.6-sol` | `xhigh` |
| Builder | `openai/gpt-5.6-sol` | `high` |
| Debugger | `openai/gpt-5.6-sol` | `high` |
| Documenter | `zai/glm-5.3` | `max` |
| Standard code reviewer | `openai/gpt-5.6-sol` | `high` |
| Adversarial reviewer | `openrouter/moonshotai/kimi-k3` | `max` |

## Routing Rules

- Classify work as simple or complex before dispatch and state the route to the
  user.
- Route complex work through either configured complex coordinator entry,
  defaulting to `openai/gpt-5.6-sol` at `xhigh` unless the GLM entry was
  explicitly selected.
- Route simple work to `zai/glm-5.3` at `max` by default. A listed simple
  alternative requires explicit selection or installation-time substitution.
- Dispatch every specialist at its configured `high`, `xhigh`, or `max` route;
  the coordinator's provider does not replace specialist routing.
- Hidden complexity upgrades a simple route. Do not downgrade work mid-task.
- The adversarial route is fixed to `openrouter/moonshotai/kimi-k3` at `max`; a standard
  reviewer or an available general-purpose model does not substitute for it.

## Availability And Authentication

OpenCode agent frontmatter has no automatic model fallback list. The GLM complex
entry is an alternate coordinator, not a full-provider fallback: OpenAI
brainstormer, planner, builder, debugger, and standard-review routes remain in
the pipeline. Never claim that selecting GLM or encountering an OpenAI provider
failure automatically reroutes those specialists.

Before dispatch, confirm that every provider required by the selected
coordinator and the upcoming specialists is configured and authenticated for
its canonical model ID. Missing provider credentials or model access is a
blocker. Record an unavailable route and any explicit substitution in external
`STATE.md`; do not silently change providers, model IDs, or variants.

Coordinators, candidate-file specialists, and reviewers receive model responses
only through the credential-isolating inference broker in
`handoff-contract.md`; their OpenCode processes never receive provider
credentials or general provider-network access. The launcher attests the trusted
provider implementation/version/hash and sole broker endpoint.

## Provider Registration

The canonical provider ids above (`zai`, `openai`, `openrouter`) are
installation-defined ids, not stock OpenCode catalog ids. Stock OpenCode
catalogs may spell a provider differently (for example `z-ai`) or omit one
entirely; those spellings never substitute. The launcher's pinned private
`OPENCODE_CONFIG` registers exactly the canonical ids as broker-backed
custom providers, so agent frontmatter and these routes resolve through the
broker without depending on the stock catalog. Model metadata for those
custom providers is declared in the same pinned configuration because
`OPENCODE_DISABLE_MODELS_FETCH=1` forbids catalog fetching.
