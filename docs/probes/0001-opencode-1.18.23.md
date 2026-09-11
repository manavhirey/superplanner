# OpenCode 1.18.23 Capability Probe Results

Date: 2026-09-11. Environment: macOS 26.5.2 (arm64), OpenCode 1.18.23
(`/opt/homebrew/bin/opencode`), Git 2.50.1 (Apple Git-155). All probes ran
under `env -i` with private `HOME`/XDG/temp roots, a pinned private
`OPENCODE_CONFIG`, a private-config wrapper agent, and a stub
OpenAI-compatible provider at an unroutable endpoint. Findings are
normative for the runtime; each cites where it is used.

## P1 — `mode: subagent` with `--agent` silently falls back

`opencode run --agent sp-probe-sub` prints:

```text
! agent "sp-probe-sub" is a subagent, not a primary agent. Falling back to default agent
> build · stub-model
```

The default `build` agent — a different permission surface — executed the
request. **Consequence:** the runtime-primary-wrapper mechanism plus
observed session-agent verification is the primary defense; the supervisor
never launches a subagent definition directly. (handoff-contract.md
"Isolation Supervisor"; corrected in PR #19.)

## P2 — Mechanical session-agent evidence source

With `--print-logs`, the structured stream emits per-request identity:

```text
message=stream providerID=stub modelID=stub-model session.id=ses_... small=false agent=sp-probe-wrapper mode=primary
```

and session creation emits `created id=ses_...`. The `--format json` event
stream carries `sessionID` on every event but **no agent field** (an
`error` event was verified: `{"type":"error","sessionID":...}`).
**Consequence:** the supervisor captures the generated session ID from JSON
events and the observed session-agent identity from the log stream, then
cross-checks `agent=<expected-wrapper>` before accepting output.
(handoff-contract.md:88-101 "verifies the session's actual agent".)

## P3 — Broker-style custom provider works under the full disable set

A private-config custom provider (`stub` id, `baseURL`, `apiKey`, declared
models) resolved and reached inference under all five disable flags
including `OPENCODE_PURE=1` (`llm.runtime=ai-sdk llm.provider=stub
llm.model=stub-model`). Built-in retry (3 attempts) stayed pinned to the
same provider/model; no rerouting. Title generation uses the same route
(`small=true`, `agent=title`). **Consequence:** the broker design
(`zai`/`openai`/`openrouter` registered via pinned config) is implementable
exactly as specified in model-routing.md "Provider Registration".

## P4 — `OPENCODE_PURE=1` blocks all plugin loading

A plugin in the private config's plugin directory (`<XDG_CONFIG_HOME>/opencode/plugins/`)
that writes a marker on load: marker present without PURE; marker absent
with `OPENCODE_PURE=1` (isolated: the other four flags do not block it).
**Consequence:** the envelope must not set PURE, because the
`superplanner_supervisor` primitive is delivered as a trusted plugin.

## P5 — Four-flag envelope separates trusted from project plugins

Without PURE, with `OPENCODE_DISABLE_PROJECT_CONFIG=1`,
`OPENCODE_DISABLE_EXTERNAL_SKILLS=1`,
`OPENCODE_DISABLE_DEFAULT_PLUGINS=1`, `OPENCODE_DISABLE_MODELS_FETCH=1`:

- Trusted private-config plugin: **loads** (marker present; no
  registration errors in the log stream)
- Project plugin in `<project>/.opencode/plugins/evil.js`: **blocked**
  (no marker)

**Consequence:** the amended envelope (four flags, no PURE, one attested
private plugin) is verified sufficient; project-content plugin injection is
excluded even when workers run re-rooted into candidate worktrees.
(handoff-contract.md "Isolation Supervisor" envelope, amended accordingly.)

## P6 — Skills paths resolve under the disable set

`opencode debug config` under the full flag set (PURE included) shows the
`skills.paths` entry from the pinned private config, and wrapper agents
register from the private config (`sp-probe-sub`, `sp-probe-wrapper`).
**Consequence:** allowlisted skills remain reachable for workers.
Residual: skill *execution* under the envelope needs a working provider and
is covered by the #6/#7 broker integration tests.

## P7 — Generated external-directory exception confirmed

`opencode debug agent` resolves per-invocation `external_directory: allow`
entries for exactly `<data-root>/opencode/tool-output/*` and
`<TMPDIR>/opencode/*`. **Consequence:** the sandbox allowlist for workers
matches the contract's truncation-output exception; everything else in the
private data root stays edit-denied.

## Residual verifications (deferred to #6/#7 with the broker)

1. Model-level exposure of a plugin-registered custom tool (the plugin loads
   and registers without error; an agent tool call needs a working
   provider).
2. Skill execution (not just path resolution) under the envelope.
3. `--session <id>` resume identity stability across a fresh `env -i`
   process.
