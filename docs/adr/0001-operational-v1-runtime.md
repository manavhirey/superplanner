# ADR 0001: Operational v1 Runtime Architecture

- Status: Accepted
- Date: 2026-09-11
- Deciders: Repository owner; research synthesis from seven parallel tracks
- References: issue #2; `README.md` prerequisites; `references/handoff-contract.md`;
  `references/integration-protocol.md`; `references/quality-gates.md`;
  `references/record-grammar.md`

## Context

The repository ships a declarative OpenCode workflow pack (agents, skills,
contracts, templates) whose contracts require an external trusted runtime:
coordinator launcher, isolation supervisor, credential-isolating inference
broker, typed operation service, `safe_git` mediator, evidence registry, and
output guard. None existed. Live research verified the OpenCode 1.18.23 and
Git 2.50.1 behaviors the contracts depend on, found two factual spec errors,
and established that the four-property same-principal isolation verdict cannot
be honestly attested on stock macOS.

## Decisions

### D1: Implementation language — Rust

All trusted runtime components are implemented in Rust.

- **Rust (chosen):** memory-safe systems language with no runtime GC pauses;
  zero-dependency first-pass crates keep the attestation surface tiny; strong
  `std` coverage for process spawn, FDs, filesystem identity, Unix sockets,
  and hashing; first-class error typing suits fail-closed validation; static
  linking yields a single pinned, SHA-256-attestable launcher binary per
  platform.
- **Go (rejected):** excellent process/IPC ergonomics and single-binary
  output, but a garbage-collected runtime with broader runtime surface and
  less precise control over descriptor/privilege edge cases demanded by the
  four-property verdicts.
- **Node/TypeScript (rejected):** matches the OpenCode ecosystem, but the
  trusted runtime would inherit a large interpreter and dependency graph,
  making path-and-hash attestation of "the trusted implementation" impractical.
- **Python (rejected):** fastest to prototype, weakest distribution and
  attestation story for a trust boundary.

Exception: the `safe_git` mediator **client** remains the bash script mandated
by `integration-protocol.md:431-516, 617-621`, started through
`/usr/bin/env -i ... /bin/bash --noprofile --norc`. That script must remain
bash-3.2-safe. The out-of-process mediator **service** behind it is Rust.

### D2: Platforms — Linux-first v1

- Linux (x86_64, aarch64): full flows. Supervised candidate-file workers run
  under user namespace + mount namespace + PID namespace + `no_new_privs` +
  seccomp, which mechanically delivers all four isolation properties; custody
  roots require an admin-provisioned distinct principal.
- macOS: coordinator-only, spike, bounded-design, and planning flows. The
  same-principal four-property verdict cannot honestly pass (same-uid Mach
  task ports and `ptrace` cannot be comprehensively denied with public
  APIs), so supervised candidate-file dispatch and custody leases are
  Linux-only in v1 and fail closed elsewhere.

### D3: Process and component map

```
user -> sp-launcher (Rust bin; CLI: --agent <id> | run --agent <id> "<prompt>")
          |-- attests and starts sp-runtime daemon:
          |     sp-supervisor  typed spawn/resume/operate lifecycle
          |     sp-operate     closed typed operations
          |     sp-safegit     safe_git service (bash client + Rust service)
          |     sp-evidence    immutable registry, no-clobber link(2) publication
          |     sp-guard       trusted output guard
          |-- starts sp-broker (sole provider-credential holder;
          |     local OpenAI-compatible endpoint; per-invocation tokens)
          '-- env -i exec: OpenCode coordinator -> (workers/reviewers
                via supervisor, model traffic via broker only)
```

Supporting crates: `sp-schema` (validators, canonical forms),
`sp-gix` (trusted non-Git Git-format parsers), `sp-state` (external STATE.md
validation), `sp-installer` (install + readiness probes).

### D4: Trust boundaries and principals

| Data | Reader/writer | Denied to |
| --- | --- | --- |
| Provider credentials | broker only | launcher, supervisor, coordinator, workers, mediator |
| Raw Git metadata/storage | safe_git service under the scrubbed profile; registry identities | model-facing tools; workers |
| Candidate files | workers in their re-rooted worktree; reviewers via commit-tree projections; coordinator via masked write-through projection | peers, parent worktrees, operational storage |
| Operational state (`STATE.md`) | coordinator only | workers, reviewers |
| Immutable evidence | registry consumers with rehash | coordinator (IDs only), workers |

### D5: Dependency and toolchain policy

Pinned stable Rust toolchain via `rust-toolchain.toml`; committed
`Cargo.lock`; minimal external dependencies, each justified in the ADR that
introduces it; reproducible builds verified twice from clean checkouts; CI
runs fmt + clippy (-D warnings) + tests + release build on Linux and macOS.

The schema crate uses `serde`/`serde_json` for strict typed JSON and canonical
serialization, RustCrypto `sha1`/`sha2` for Git object IDs and SHA-256 record
identities, and the small `unicode-casefold`/`unicode-normalization` crates to
reject ownership-path aliases under full Unicode case folding and canonical
normalization. These replace incomplete handwritten implementations at security
boundaries.

## Open Verification Items (probed 2026-09-11; see docs/probes/0001-opencode-1.18.23.md)

1. ~~Mechanical session-agent identity source~~ — **Resolved:** the
   `--print-logs` stream carries `agent=<name> mode=<mode>` per request and
   `created id=ses_...` for the session; JSON events carry `sessionID` only,
   so the supervisor cross-checks both streams.
2. ~~Trusted-plugin tool injection under disable flags~~ — **Resolved:**
   `OPENCODE_PURE=1` blocks all plugins, so the envelope drops PURE; with the
   four granular flags the attested private plugin loads while a project
   `.opencode/plugins/` injection is blocked. Contracts amended accordingly.
3. Skill visibility under `OPENCODE_DISABLE_EXTERNAL_SKILLS=1` —
   config-path resolution verified; skill execution under the envelope is
   covered by the #6/#7 broker integration tests.

Residual model-level checks (need a working provider): plugin-tool exposure
to an agent, skill execution, `--session` resume identity stability.

## Consequences

- The scaffold lands one empty crate per trusted component with smoke tests;
  no speculative functionality before its owning issue.
- Issue #3's schema work consumes `references/record-grammar.md` as the
  normative gap resolution.
- The early demo gate (after #7) is a quick-agent spike flow through the real
  launcher on Linux.
- The declarative pack itself is unchanged by this ADR except for the
  verified factual corrections already merged into the contracts.
