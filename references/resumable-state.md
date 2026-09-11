# Resumable State

`STATE.md` is a durable operational checkpoint outside candidate content. Only
the orchestrator writes it; workers and reviewers return structured handoffs.

For Git work, resolve the common directory into a registered external leaf with
`safe_git_to <external-output>/git-common-dir plain -- -C <registered-candidate-worktree> rev-parse --path-format=absolute --git-common-dir` and use
`<git-common-dir>/superplanner/<initiative-slug>/STATE.md`. Attempt that form
first with its own one-use operation/output leaves. Only an
unsupported-`--path-format` result permits a fresh `safe_git_to
<external-output>/git-common-dir-relative plain -- -C
<registered-candidate-worktree> rev-parse --git-common-dir` call; resolve its
exact output relative to the candidate worktree first. For non-Git work, require an explicitly configured
absolute state path outside the candidate root. Never fall back to
`docs/superplanner/**/STATE.md` or any other candidate path.

That external path supports discovery, design, feature/task definition, and
planning in a non-Git workspace. Retained `EXECUTE`, coordinator `COMMIT`, SHA-bound
review, and push-command eligibility require a Git repository created by the user. Superplanner
never runs `git init`.

The state directory may also hold orchestrator-owned operational matrices and
persisted handoffs. Coordinator-persisted checkpoints, packaging indices, object
directories, scratch storage, and result bundles use an explicitly configured
absolute worker-artifact root outside the candidate and every Git directory;
state records their paths. Workers emit checkpoint and handoff content only
through supervisor results and cannot write that root. None may be tracked, included
in candidate or review diffs, dirty a worktree, or change the candidate SHA.

## Required Schema

`STATE.md` records:

- Initiative identity, current phase, state update time, and orchestrator ID.
- State path, storage mode, candidate root, and resolved Git common directory or
  explicit non-Git configuration evidence, including the absolute common-dir
  operation or the unsupported-option result and fallback resolved relative to
  the candidate worktree.
- Explicit worker-artifact root and evidence that it is outside the candidate
  root and every resolved Git directory.
- Coordinator launcher path/hash, credential/Git-metadata-masked project
  projection, phase-scoped external mounts, and isolated safe-Git mediator
  attestation.
- Design path, approval status, approver, approval time, evidence, and current
  content ID.
- Feature and task IDs, artifact paths, workflow lifecycles, dependencies, and
  ownership.
- Separate feature, task, Gherkin, and plan approval records, each containing
  `status`, `approver`, `approved_at`, `approval_evidence`, and `content_id`.
- For a quick bounded route, one exact bounded-brief block plus marker
  validation, `status`, `approver`, `approved_at`, `approval_evidence`, recorded
  `content_id`, and recomputed-ID match evidence.
- For every design, feature, task, Gherkin, and plan approval: marker type,
  marker count/order validation, recomputed deterministic content ID, and
  whether it equals the recorded `sha256:<64 lowercase hex>` value.
- A dependency and ownership matrix containing `consumes output from`,
  `expected files`, `shared state`, `exclusive resources`, `verdict`, and
  `reason`, whether embedded or stored at an external path.
- Worktree paths, detached SHAs, harness re-rooting evidence, baselines,
  immutable assignments, literal ownership hashes, external baseline-mirror
  construction command/manifest path/identity/hash and no-hardlink proof-manifest
  path/identity/hash/verdict, complete setup/packaging namespace snapshots with
  exact walker-root and `packed-refs` leaf identities/verdicts, per-domain
  `setup_objects` and exact encoded `setup_alternates`, quarantined packaging
  indices/object directories and result bundles, alternate candidate
  index/object directory, scratch root, empty safe-hooks directory, integration
  packaged/pre/post tree IDs, structured source/setup and phase/context/anchor private
  fixed-config content Git-directory records, and trusted non-Git source/mirror
  repository-format/object-format/`files`-backend config snapshot and
  invocation-lifetime immutability evidence. Each content record includes its
  worktree, real/content Git-directory paths and identities, exact config/HEAD
  hashes, format tuple, anchor OID, allowed index/object/alternate routes,
  persisted absence of `info/alternates` and `info/http-alternates` for every
  object directory and every transitive route member, validation result, and
  active/retired state. For every domain, separately record
  each canonical safe-Git tuple-route manifest path/hash and every per-call
  operation-manifest path/hash. The latter binds the exact context and
  route-policy hash, selected real-or-content Git-directory class/path/identity,
  manifest hash and anchor, exact index/object paths, encoded alternates, destination, tree,
  ownership, patch, input/output descriptors and device/inode identities,
  private result/staging leaves, and command operands. This includes every
  authorized call, without exception. Record its one-use token
  state and only the identity- and hash-attested terminal result-manifest path/hash
  containing exact final mediator status and output hash. A successful primary
  publication retains its manifest-bound staging hard link as safety evidence;
  record both names, their shared device/inode, link count, and byte equality.
  If the primary result
  leaf is preempted, record the manifest-bound staging leaf as the published
  status-`125` recovery result instead. For every unconsumed token, record that
  every reserved output, result, and staging leaf remains absent and unaliased.
- Dispatch IDs, stable workflow task IDs, generated OpenCode session IDs stored
  as `opencode_session_id`, source agent ID/hash, runtime wrapper name/hash,
  allowed name/mode delta, observed session-agent evidence, model/variant,
  resolved-permission-manifest hash, immutable brief path/hash, exact agent mode,
  scopes, step/execution budgets, milestones, stop conditions, and checkpoint paths.
- Per-invocation private `HOME` plus config/data/cache/state/temp roots; exact XDG,
  `TMPDIR`, pinned `OPENCODE_CONFIG`/`OPENCODE_DB`, required
  `OPENCODE_DISABLE_PROJECT_CONFIG=1` and
  `OPENCODE_DISABLE_EXTERNAL_SKILLS=1`, `OPENCODE_PURE=1`, and
  `OPENCODE_DISABLE_DEFAULT_PLUGINS=1` and
  `OPENCODE_DISABLE_MODELS_FETCH=1`, scrubbed override evidence; resolved OpenCode
  paths/database; and canonical credential file identities plus complete
  symlink/hard-link alias and Git-metadata mask manifest.
- Per-invocation distinct-OS-principal evidence or a kernel-enforced
  same-principal isolation verdict covering filesystem access, process
  inspection/control, inherited descriptors, and privilege escalation. Missing
  or failed evidence is recorded as blocked, never as a degraded sandbox.
- For reviewers, Git mediator client/broker path/hash, literal SHA/range/path
  policy, credential-output exclusion, `--no-replace-objects`,
  `GIT_NO_REPLACE_OBJECTS=1`, `--no-lazy-fetch`, `GIT_NO_LAZY_FETCH=1`, and
  pinned Git-dir/worktree context-manifest evidence.
- Harness monitoring and cancellation capabilities, step budgets, terminal
  handoffs or timeouts, resumes by generated OpenCode session ID, confirmed
  cancellations, replacement agents, and escalations.
- Verification commands, results, evidence, current HEAD and exact pre-commit
  candidate identity, then its binding to the unchanged committed SHA.
- Documentation impact, synchronization status, evidence, pre-commit candidate
  identity, then its binding to the unchanged committed SHA.
- Commit authorization evidence naming the canonical patch hash, candidate tree,
  message hash, full base commit, exact full target ref at the base OID, exact
  author and committer dates, and expected commit OID;
  authorization-manifest path/hash and exact bound context, path-file, patch,
  message, target, identity, exact author/committer dates, precomputed commit OID
  and independent no-write hash check, reason, command operands, and registered output
  leaves; canonical transaction-supervisor executable path/identity/hash and
  exact source hash; caller-read-only Git storage plus mediator unlock/relock evidence;
  source `files`-backend and content-control-directory manifest evidence;
  base/candidate trees; complete pre/post ref/reflog snapshots with exact `refs`,
  `logs`, and every `worktrees/*/logs` walker-root path/identity and walk verdict
  plus optional `packed-refs` leaf path/identity and verdict; exact commit/reflog
  name/email and reflog reason; message
  path/hash; exact committed files; complete-tree attribute and all-untracked-path
  evidence; exact ordered tree/parent/author/committer-only raw-header proof,
  commit-object and explicit-parent evidence; target CAS command; exact
  transaction transcript path/identity/hash, run/not-run flag, raw supervisor
  exit status, and distinct final mediator status; exact
  `prepare: ok` response, deterministic under-lock direct/symbolic/OID-mismatch
  evidence for the full target and, on success, proof it is at
  the authorized base OID, and final transaction `commit: ok` or abort evidence;
  byte-exact expected branch and symbolic-worktree-`HEAD` reflog appends including
  the authorization-bound committer date and numeric timezone; resulting
  tree/message/full SHA and proof that the full SHA equals the authorized expected
  OID; authorization-reserved review-context and result-manifest paths; post-commit
  no-clobber result-manifest path/identity/hash linked to the authorization hash; and the
  blocker preventing coordinator `COMMIT`, when applicable.
- Standard and adversarial review agents, models, variants, evidence, results,
  reviewed SHAs, and immutable commit-derived projection manifests/hashes with
  complete non-credential tree equality and no-extra/untracked/ignored evidence.
- Accepted risks and explicit user decision evidence.
- Push-command eligibility evidence binding the destination remote name as
  provenance, the single closed-grammar HTTPS URL, absence of all URL rewrites,
  full refspec, expected local OID and push-only source-ref equality, explicit
  no-force policy, target-bound registered `push-presentation-request-v1`,
  registered immutable source-context/object-route,
  authorization, coordinator `COMMIT`, both review-result, presentation-request,
  current-verification, documentation-synchronization, and destination-config
  record IDs, service-resolved identities, recomputed hashes, validated
  cross-links, and the supervisor-produced immediate clean-worktree snapshot;
  the registered canonical eligibility manifest ID and recomputed hash;
  registered canonical `push-layout-v1`, `push-execution-closure-v1`, and
  `push-command-v1` IDs, identities, and hashes; source-unavailable layout
  verification; attested absolute `env`/Git paths, private exact `PATH`, and
  closed executable/interpreter/loader/library process graph;
  separate-principal custody-root ancestry and active no-mutation lease; exact
  SHA-1/SHA-256 config-isolation environment including matching `safe.directory`
  and `http.followRedirects=false`; POSIX shell-quoted literal operands; and
  guarded `push-presentation-v1` evidence for the exact user-executed command
  bytes. Keep the lease active until `push-retire` service-resolves and rehashes
  a registered immutable `push-custody-release-v1` trusted-user-decision record
  bound to the lease and eligibility-manifest IDs with no execution or outcome
  field. Record its prepared operation, same-filesystem atomic rename as the
  revocation linearization point, idempotent one-use consumption, and
  `active|retiring|cleanup-failed|retired` state; `cleanup-failed` requires
  inaccessible resumable cleanup. Record that Superplanner has no push route and
  did not execute, broker, retry, observe, or report execution of the command.
- One exact next action, its owner, prerequisites, and expected evidence.

Use stable feature, task, domain, dispatch, agent, checkpoint, and review IDs so
events remain traceable across resume and replacement.

## Approval Validation

Validate every approvable artifact with the canonical algorithm in
`artifact-contracts.md`. Require exactly one ordered marker pair. Remove the
complete marker lines and every enclosed byte, preserve all remaining exact
UTF-8 bytes without normalization, SHA-256 hash those bytes, and encode
`sha256:<64 lowercase hex>`. Missing, multiple, or misordered markers are
blockers. The ID must match `^sha256:[0-9a-f]{64}$`. Approval is current only
when `status` is `approved`, all approval fields are present, and the recomputed
ID exactly equals `content_id`. For `approved`, `approver` and
`approval_evidence` must be non-`none`, `approved_at` must be valid ISO-8601, and
the evidence must bind the explicit user decision to that displayed artifact
and exact content ID. For `pending`, all three fields must be `none`. Mere field
presence never satisfies these invariants.

When recording or correcting approval, change only bytes inside the existing
markers. A change to either marker or any outside byte is an artifact-content
change, not an approval update, and requires a new content ID and approval.

For a quick bounded brief, use the separate markers
`<!-- superplanner-bounded-brief:start -->` and
`<!-- superplanner-bounded-brief:end -->` in external `STATE.md`. Hash only the
exact bytes between the complete marker lines, excluding the marker lines and
their line endings, without normalization. Present those exact bytes to the
user. The approval metadata lives outside the pair and records the same
`sha256:<64 lowercase hex>` value. Any marked-byte change invalidates approval;
unrelated operational-state updates do not.

## Checkpoint Rules

- Update state at phase transitions, approvals, dispatch, meaningful worker
  checkpoints, integration, verification, documentation synchronization,
  reviews, invalidation, recovery, and user escalation.
- Long-running agents emit checkpoint content after meaningful units of work
  only when the invocation can return checkpoints. The orchestrator persists it
  at the dispatch checkpoint path under the configured worker-artifact root;
  workers never write external checkpoint or state paths.
- A checkpoint records completed scope, remaining scope, changed paths,
  commands and evidence, blockers, and one next action.
- The orchestrator appends monitoring and recovery events; it does not overwrite
  the history needed to explain a replacement.
- Record observed facts and mark assumptions as assumptions.
- On resume, separately rehash every tuple-route, per-call operation manifest,
  and consumed call's terminal result manifest; compare every route-policy hash,
  bound operand, descriptor identity, terminal status, and output hash;
  revalidate the persisted absence of both on-disk alternate files for every
  routed object directory and every transitive alternate member; and
  rehash owned paths, patch, and raw manifest at each recorded pre-consumption
  and post-apply point. Missing or stale evidence blocks rather than being
  reconstructed from a path that happens to exist.

## Monitoring Events

Each event records timestamp, dispatch and agent IDs, stable workflow task ID,
generated OpenCode session ID or `none`, event type, observed progress, evidence or
external checkpoint path, step-budget state, orchestrator action, and next
action. Events can include `dispatch`,
`checkpoint`, `handoff`, `timeout`, `resume`, `live-check`, `cancel-requested`,
`cancellation-confirmed`, `invocation-ended`, `replacement`, and `escalated`.

In standard foreground mode, wait for a `complete` or `blocked` handoff, or a
harness timeout; do not poll, stop, or cancel. A valid shell-free worker
command-evidence checkpoint is a planned `blocked` return: the orchestrator runs
the validated command only in the constrained process sandbox, or asks the user
to run it when the sandbox is unavailable or network/credentials are required,
and resumes the same generated `opencode_session_id` with exact evidence,
repeating within budget. It is not stuck recovery. Normalize timeout to a
blocked handoff and attempt recovery only with the generated `opencode_session_id`
captured from the prior `superplanner_supervisor` result. `live-check`, `cancel-requested`, and
`cancellation-confirmed` are valid only when the recorded harness capabilities
support them. A replacement event is valid only after `invocation-ended` or
`cancellation-confirmed` for the original file-writing invocation.

Never pass the stable workflow `task_id` as a `superplanner_supervisor` resume ID. If no
generated OpenCode session ID was returned, record resume as unavailable rather
than guessing an ID; replacement still requires proof that the invocation ended
or cancellation was confirmed.

## Review Evidence

Store standard and adversarial reviews separately. Each record contains:

- Review ID, type, agent ID, model ID, and variant.
- Exact full `reviewed_sha`, or `none` only for a blocked, not-completed attempt
  that could not establish one, plus evidence or handoff path and
  `completion_time`.
- `review_result` (`approved|findings|not-completed`), findings, and accepted-risk
  entries.
- Each accepted risk's finding ID, exact reviewed SHA and scope, explicit user
  evidence, consequence, and rationale. The referenced finding remains visible
  in the review findings.
- For adversarial review, the exact registered immutable standard-review record
  ID, path identity, recomputed SHA-256, and `standard_approval_sha`, or `none`
  only for a blocked, not-completed attempt that could not establish the named
  field and explains why.
- Invalidation status and reason.
- Verified prepared-transaction evidence: `prepare: ok`, the under-lock direct
  target/base-OID result, and final `commit: ok`, all for the reviewed commit.

The state must make it mechanically clear whether both approvals bind to the
same current SHA. Any candidate change invalidates both approval records and
sets the next action to affected verification, documentation synchronization,
and non-mutating `COMMIT_PREPARE`. The replacement authorization itself must
name the canonical patch hash, candidate tree, message hash, full base commit,
target state, exact author and committer dates in numeric-timezone form, and
expected full OID; coordinator `COMMIT` creates the replacement commit before
review restarts at standard review. External operational-state updates do not
invalidate either review.

## Exact Next Action

Keep exactly one active next action. It names:

- The owner.
- One concrete operation or decision.
- Required input paths or IDs.
- Preconditions.
- Expected evidence and where it will be recorded.

Do not use broad directions such as "continue implementation." If work can
proceed independently, the next action may be one orchestrator dispatch action
that names all approved independent domains.

## Resume Consistency Checks

Before acting on resumed state, the orchestrator verifies:

1. The state path follows the Git-common-dir contract, or the non-Git path is an
   explicitly configured absolute external path; it is not tracked or present
   in candidate status or diffs.
2. The environment is eligible for the recorded phase. Non-Git state may resume
   discovery, design, and planning, but retained `EXECUTE`, `COMMIT`, SHA review,
   and push-command eligibility require a user-created Git repository; never initialize one
   automatically.
3. The initiative slug and artifact paths resolve to the same initiative.
4. Before `DEFINE`, the recorded design has exactly one ordered marker pair, its
   recomputed deterministic content ID equals its approved candidate record, and
   that record equals the external mirror.
5. Feature, task, acceptance, and plan links resolve, IDs agree, each file has
   exactly one ordered marker pair, and each recomputed content ID matches both
   approval copies. Before `PLAN`, this applies to design, feature, task, and
   Gherkin. Before `EXECUTE`, it also applies to the plan, and every upstream ID
   equals the corresponding source ID embedded in that plan.
6. The matrix has every required column and its verdicts still match current
   dependencies, expected files, shared state, and exclusive resources.
7. For Git phases, repository HEAD, worktree status, branches, worktrees, and
    submodules match the recorded integration state, and trusted non-Git config
    parsing still proves the recorded repository/object/`files`-backend tuple,
    source/mirror shallow-path absence, and the supervisor's config-immutability
    boundary remains valid. The persisted baseline-mirror clone/namespace-
    reconstruction/no-hardlink proof, every
    `setup_alternates` value, and every source/setup context remain exact. Every
    worktree-content command remains bound by its per-call result to the active
    phase/anchor private fixed-config content Git directory and exact
    index/object route. A recorded commit gate has
    explicit user authorization for patch/tree/message/base/target, exact author
    and committer dates, and expected commit OID, exact in-scope files,
    complete-tree attribute and all-untracked-path evidence, exact commit/reflog
    identity and reflog reason, a full resulting SHA equal to the authorized
    expected OID with the authorized sole parent/tree/message, exact ordered
    tree/parent/author/committer-only raw-header proof, direct explicit-full-ref no-deref target
    compare-and-swap evidence, authorization-bound `GIT_COMMITTER_DATE`
    environment evidence, and the complete expected direct-ref plus byte-exact
    unconditional branch and symbolic-worktree-`HEAD` reflog appends containing
    old/new OIDs, name/email, Unix seconds, numeric timezone, and reason. Require every
    persisted `refs`, `logs`, and `worktrees/*/logs` walker-root identity and
    verdict plus optional `packed-refs` leaf identity and verdict to match the
    corresponding snapshot.
    Rehash and compare every tuple-route, including the pre-authorization
    no-write commit `hash-object` call; every per-call operation manifest and
    consumed call's terminal result manifest; all immediate pre-consumption and
    post-apply owned-path/patch/raw-manifest hashes; and the authorization
    manifest/token with every bound file and output leaf. Every authorized call
    is included.
    For each unconsumed token, prove it remains unused and all reserved output,
    result, and staging leaves remain absent and unaliased.
    Require caller-read-only Git storage, mediator unlock/relock evidence, exact
    `prepare: ok`, the successful under-lock direct-target/base-OID result, and
    final transaction `commit: ok` evidence. Require the authorization-reserved
    review-context and result-manifest paths and the no-clobber published result's
    path/identity/hash binding to remain exact.
8. No running or not-confirmed-cancelled invocation overlaps a proposed
    ownership domain, and every resumed candidate writer/reviewer still has the
    exact persisted brief, source agent, runtime wrapper, model/variant,
    permissions, observed session agent, private-root/path, credential mask,
    broker, registry, distinct-principal or passing kernel-enforced
    same-principal isolation verdict, and sandbox envelope.
9. Checkpoint files and handoffs use the canonical schema and agree with task
   and invocation state.
10. Verification and documentation evidence bind to the current candidate.
11. Review SHAs match the current candidate, completion times are present, and
    invalidation flags are accurate.
12. Every accepted risk contains the finding ID, exact SHA and scope, explicit
    user evidence, consequence, and rationale; the finding remains present in
    reviewer findings.
13. The exact next action remains safe and its prerequisites are satisfied.

On any mismatch, record the discrepancy and make reconciliation the exact next
action. Do not infer completion, dispatch overlapping work, or rely on stale
approval.
