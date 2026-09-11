# Parallel Execution

Parallelism is allowed only when dependency and ownership analysis shows that
workers cannot interfere. Available capacity alone is not a reason to dispatch.

## Dependency And File-Ownership Matrix

Create the matrix before dispatch and record its path or contents in
external `STATE.md`. If stored separately, the matrix is an external operational
artifact beside `STATE.md`, not candidate content. Every embedded or separate
matrix uses all of these columns:

| Domain ID | Task IDs | Depends on | Consumes output from | Expected files | Shared state | Exclusive resources | Compared with | Verdict | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<domain-id>` | `<task-ids>` | `<domain-ids-or-none>` | `<outputs-or-none>` | `<exact-added-modified-deleted-paths-or-globs>` | `<state-or-none>` | `<resource-or-none>` | `<domain-ids-or-none>` | `<parallel-or-serialize>` | `<evidence-based-reason>` |

`Expected files` defines ownership, including generated files. Add one row for
each domain comparison needed to justify the batch; a bare `parallel` verdict
without a reason is invalid.

Two domains are parallel-safe only when all answers below are `no`:

| Conflict check | Required decision |
| --- | --- |
| Does either domain consume the other's output? | Serialize |
| Can both domains add, edit, rename, or delete the same file? | Serialize or redefine exclusive ownership |
| Can their globs overlap, including generated files? | Serialize or redefine exclusive ownership |
| Do both mutate the same database, lockfile, cache, fixture, migration sequence, or other shared state? | Serialize |
| Do both require the same exclusive service, device, port, credential lease, or environment? | Serialize |
| Can one domain change an interface assumed by the other? | Add the dependency and serialize |

Group closely coupled files and tasks into one independent ownership domain.
Issue one dispatch per independent domain, not one dispatch per file and not
multiple competing dispatches for the same scope.

## Concurrency

- Default to at most four active workers.
- Eight active workers is the hard maximum.
- Use fewer than four whenever isolation, review capacity, integration cost, or
  resource limits make that safer.
- Never split a dependent chain merely to reach the default.

## Dispatch Contract

Every worker receives a fresh, self-contained prompt containing:

- Stable dispatch and task IDs.
- The complete objective and bounded scope.
- Approved design, feature, task, acceptance, and plan paths and their relevant
  contents or accessible locations.
- Evidence that the feature, task, Gherkin file, and implementation plan are
  approved for their current content IDs.
- Exact owned literal leaf paths, including anticipated generated files, and
  explicit exclusions. Globs may inform conflict analysis but must be expanded,
  collision-checked, and approved as exact leaves before dispatch.
- Dependencies already satisfied and inputs supplied.
- Required commands, expected output, and completion evidence.
- Worktree path, harness evidence that the file-writing invocation's
  filesystem/project root equals that exact worktree, resolved inherited-tool
  denial and automatic-edit-process sandbox evidence, coordinator-owned
  checkpoint ID/path, milestones, step budget, and any harness-supported
  timeout. Workers emit checkpoint content through supervisor results and never write
  external paths. A prompt path without re-rooting evidence blocks dispatch.
- Supervisor-returned evidence for either a distinct OS security principal or
  kernel-enforced same-principal isolation covering filesystem access, process
  inspection/control, inherited descriptors, and privilege escalation.
- The structured handoff contract and exact return destination.

Do not rely on hidden conversation history. Workers may not dispatch subagents,
change another domain's files, update external `STATE.md`, integrate branches,
commit, update refs, or push even with user authorization; exact commit authority
belongs only to the user-facing coordinator after its separate gate. The
orchestrator owns dispatch, shared state, persisted checkpoints/handoffs,
recovery, and integration. It stores checkpoints and integration artifacts under
the configured worker-artifact root; validated handoffs and review records may
be stored beside external state. Workers cannot write either location. Neither
location may dirty or change the candidate.

## Worktree Isolation

- Detect existing worktree isolation and submodules first.
- Prefer a harness-native worktree mechanism.
- Use Git worktrees only when the repository and ignore rules are safe.
- Assign each parallel domain a separate worktree and identify it in every
  handoff path.
- Require the harness to re-root each file-writing invocation to its exact
  assigned worktree so parent and peer worktrees are external and denied. If the
  harness cannot do this, block retained file-writing dispatch rather than
  relying on prompt-only ownership.
- Require resolved writer permissions to deny inherited/MCP/custom execution
  tools and actual OpenCode output/temp edit exceptions. Disable automatic
  formatter/LSP/plugin/custom edit processes, or confine them and every brokered
  project command with the process sandbox defined in `handoff-contract.md`.
- Run project setup and establish a clean test baseline in the intended
  environment before implementation.
- Stop for user direction before continuing from a failing baseline.

## Monitoring And Recovery

Monitoring is capability-aware. Record the harness capabilities before
dispatch. Do not poll or attempt to stop a foreground invocation. Issue a live
status request or cancellation only when the harness explicitly supports that
operation.

### Standard Foreground Mode

- Give each invocation a step budget and wait for its terminal result. Do not
  poll or attempt to stop a foreground invocation.
- The terminal result is a `complete` handoff, a `blocked` handoff, or a harness
  timeout. Normalize a timeout to the common handoff schema with
  `status: blocked` and a timeout blocker; do not add a third status value.
- A shell-free worker's valid command-evidence checkpoint is a planned
  `blocked` return. Validate its one scoped, non-destructive, permitted command;
  run it only in the constrained process sandbox or ask the user to run it when
  the sandbox is unavailable or network/credentials are required, then resume
  the same generated OpenCode session ID with exact evidence. Repeat within budget;
  this is not stuck-agent recovery.
- Treat self-reported budget exhaustion, repeated failure without new evidence,
  unavailable input, or unavailable resources as blockers in that handoff.
- Recover a genuinely blocked or timed-out invocation by its generated OpenCode session ID,
  captured separately from the stable workflow task ID, with prior evidence and
  one focused next action. Preserve the same ownership domain and brief.
- Dispatch a replacement only after the prior invocation has ended. A malformed
  or missing handoff does not imply that a still-running invocation ended.

### Harness-Supported Live Mode

- Use event-driven waits rather than rapid polling. Live status checks are
  allowed only when the harness advertises status capability.
- Request cancellation only when the harness advertises cancellation. Do not
  infer cancellation from silence or from sending a request.
- Resume by generated OpenCode session ID when supported. A replacement may start only after the
  invocation ends or the harness positively confirms cancellation.

After a terminal blocker or timeout, resume the same generated OpenCode session ID
first. Never substitute the stable workflow task ID. Replace only
when resume is unavailable or fails after the prior invocation has ended or
cancellation is confirmed. Escalate when recovery requires destructive,
security-sensitive, irreversible, or scope-changing action. Never overlap two
file-writing invocations in one ownership domain. Record capabilities, terminal
outcomes, timeout evidence, OpenCode session-ID resumes, cancellation confirmation,
replacement IDs, and escalation in external `STATE.md`.

## Merge And Integration Checks

The orchestrator alone integrates completed domains by applying
`integration-protocol.md`. A branch, worktree, changed-file list, or worker
claim is not a transferable result. Before accepting a worker result:

1. Validate the ordinary terminal handoff, immutable assignment, ended
   invocation, and absence of a pending command-evidence resume.
2. Create the coordinator-owned `integration_result` package from that validated
   handoff, then validate its external bundle hashes, base SHA, start tree,
   object format, and raw manifest.
3. Confirm every NUL-delimited literal path falls within the assigned ownership
   domain and has no path, case-fold, normalization, or ancestor collision.
4. Replay independently against both the assigned start tree and current
   candidate tree; require exact mode/blob manifest equality.
5. Apply mechanically with the alternate candidate index and forbidden fallback
   options; a failed replay returns to a specialist and is never hand-resolved.
6. Confirm dependency outputs and generated files are present and current.
7. Run each task-specific check in the integrated workspace.
8. Run the full relevant suite with all parallel results present together.
9. Record bundle hashes, pre/post tree IDs, commands, results, and remaining
   blockers in external `STATE.md`.

Passing checks in isolated worker worktrees are evidence, but results are not
valid for the initiative until they pass together after integration.

Coordinator commit preparation or mutation is never parallel with active
file-writing work. After integrated implementation and documentation
verification, run coordinator-only non-mutating `COMMIT_PREPARE`; it may create
external evidence but may not stage, write a commit object, or update a ref. It
persists the canonical patch/hash, candidate tree, message file/hash, full base
commit, target state, exact author and committer dates in numeric-timezone form,
and expected full OID, plus an independent routed no-write
`hash-object -t commit --stdin` operation manifest and terminal result manifest
whose OID equals that expected OID. It also persists complete pre-commit
namespace-walker evidence with exact `refs`, `logs`, and every
`worktrees/*/logs` walker-root identity and verdict plus the optional
`packed-refs` leaf identity and verdict, and reserves absent review-context and
post-commit result-manifest paths.

The user authorization itself must explicitly name that patch hash, candidate
tree, message hash, full base commit, target state, exact author and committer
dates in numeric-timezone form, and expected full OID. Only after that
authorization may the coordinator enter mutating `COMMIT`. Persist a distinct
one-use operation manifest and terminal result manifest for each authorized
`add`, `write-tree`, `commit-tree`, and `update-ref` call. Verify the staged
patch/tree and explicit sole parent, then require the created object's parsed
full OID and returned OID to equal the authorized expected full OID before any
ref mutation or `update-ref`.

Compare-and-swap advance only the authorized target. Complete post-commit
namespace-walker evidence with exact `refs`, `logs`, and every
`worktrees/*/logs` walker-root identity and verdict plus the optional
`packed-refs` leaf identity and verdict must recursively enumerate actual files
so orphan reflogs are included and prove only that target update and exactly one
unconditional append in both its branch reflog and the symbolic worktree `HEAD`
reflog. Both appends must byte-match the authorized committer date and numeric
timezone and the complete expected old/new OIDs, name/email, and reason.
Construct the review context only at the authorization-reserved absent
path and publish the reserved result manifest once with no-clobber
semantics, binding its path/identity, exact bytes/hash, authorization-manifest
hash, and equal expected/resulting full OID. Record the verified tree/message,
full matching SHA, and clean candidate status; never push from this transition.

Superplanner never executes, brokers, retries, observes, or reports a later push.
After both ordered reviews pass and the user authorizes the exact commit/review/
destination/refspec tuple in `push-presentation-request-v1`, the coordinator may only present the exact
non-force user command from `quality-gates.md`, with eligibility evidence binding
the destination remote name as provenance, one closed-grammar HTTPS URL, absence
of URL rewrites, the config-isolated hookless private-`PATH` `env -i` execution
prefix with matching `safe.directory` and redirects disabled, POSIX
shell-quoted literal exec-path/push-only-Git-directory/URL/refspec words, reviewed
registered verification/documentation/source/authorization/commit/review
bindings resolved and rehashed by the supervisor with validated cross-links, an
immediate clean-worktree snapshot, the independently attested exact
loose-object push-only context, registered exact-command record, and closed
executable/helper/loader/library process graph, an active
separate-principal no-mutation custody lease retained until retirement atomically
renames the bundle to an inaccessible namespace as the revocation point, then
idempotently records one-use consumption of a registered trusted-user-decision
release bound to that lease and eligibility manifest and explicit cleanup state,
expected local OID/push-only-source-ref equality, and no-force policy.
Only the local `push-present` output guard releases the registered command bytes
after final revalidation; the coordinator cannot render or edit them.
