# Subagent Handoff Contract

Every subagent returns one structured handoff to the orchestrator. A prose-only
completion message is not a valid handoff.

## Required Fields

Every handoff uses these exact common field names:

```yaml
task_id: <stable-workflow-task-id>
agent_id: <agent-id>
role: <worker|reviewer>
status: <complete|blocked>
worktree: <absolute-worktree-path>
git_sha: <full-git-sha-or-none>
completion_time: <iso-8601-timestamp>
scope:
  - <assigned scope and specific completed portion>
artifacts:
  - <candidate-relative or absolute-operational path>
changed_files:
  - <workspace-root-relative changed file, or none>
verification:
  - command: <exact command or inspection performed>
    result: <passed|failed|not-run>
    evidence: <observable result or evidence path>
blockers:
  - <blocker and required input, or none>
assumptions:
  - <material assumption, or none>
next_action: <one exact recommended action>
```

All common fields are required. `status` has exactly two values: `complete` and
`blocked`. Use `none` as the sole list item when a required list has no entries.
`task_id` is Superplanner's stable workflow identifier, not OpenCode's generated
resume identifier. The orchestrator captures the exact OpenCode session ID
returned by `superplanner_supervisor` separately as `opencode_session_id` in
external state; agents do not invent or substitute it.
Candidate paths are relative to the named worktree root. External state,
checkpoint, matrix, or persisted-handoff paths are absolute. Checkpoints are
written only by the orchestrator under the configured worker-artifact root;
validated handoffs and review records may instead be persisted beside external
state. Workers emit their content through supervisor results and cannot write either
external location. Neither appears in the review diff. Evidence must distinguish
observed output from an unverified claim. A Git SHA is the full immutable commit
ID, not a branch, `HEAD`, or an abbreviated SHA.

## Status Semantics

### `complete`

- The assigned scope and required artifacts are complete.
- Required checks were run, or each omitted check is explicitly reported with
  its reason.
- `blockers` contains only `none`.
- `next_action` tells the orchestrator how to validate or integrate the result.

### `blocked`

- The assigned scope cannot safely continue within the brief, budget,
  permissions, or available inputs.
- `scope` and `artifacts` preserve usable partial progress.
- `blockers` names the observed condition, evidence, and input or decision
  needed.
- `next_action` is one focused recovery action, not a list of speculative fixes.

A blocked agent must not claim completion. A complete agent must not conceal a
failed or skipped required check.

## Isolation Supervisor

Every candidate-file specialist and reviewer requires an enhanced external
supervisor; stock OpenCode `Task` cannot create this boundary. Before dispatch,
the supervisor starts `opencode run --dir <exact-worktree>` through `env -i` in a
new process with a private `HOME`, private config/data/cache/state/temp roots, the
role-specific registry, and the sandbox below. Before exec it sets
`XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_CACHE_HOME`, `XDG_STATE_HOME`, and
`TMPDIR` to those five roots; pins `OPENCODE_CONFIG` and `OPENCODE_DB` to attested
paths beneath them; sets `OPENCODE_DISABLE_PROJECT_CONFIG=1` and
`OPENCODE_DISABLE_EXTERNAL_SKILLS=1`, `OPENCODE_PURE=1`, and
`OPENCODE_DISABLE_DEFAULT_PLUGINS=1`; sets
`OPENCODE_DISABLE_MODELS_FETCH=1` so the inference broker remains the sole
provider-related endpoint; and leaves every other `OPENCODE_*` override unset.
It attests those variables and OpenCode's resolved config, data, cache, state,
temp, config-file, and database paths, not only the requested environment.
Because OpenCode CLI rejects `mode: subagent` for `--agent`, the
supervisor generates a runtime primary wrapper from the exact specialist
definition, changing only `mode` and a collision-free runtime name. It hashes
both definitions and the resolved permission manifest, selects that wrapper
explicitly, and verifies the session's actual agent before accepting output. It
returns the exact OpenCode session ID and terminal result. Persist the source
agent ID/hash, runtime wrapper name/hash, sole allowed name/mode delta,
model/variant, resolved-permission-manifest hash, and observed session-agent
evidence. A resume starts a new process with the same private `HOME`, exact
role-private roots, startup-variable/resolved-path manifest, session ID,
source/wrapper identities, model/variant, and permissions; first proves every
identity remains exact and that no approval entry or provider drift appeared;
and never overlaps the prior invocation. Record that returned session ID as
`opencode_session_id`.

The launched OpenCode process must run either as a distinct OS security
principal from the coordinator, supervisor, broker, mediator, and every process
that can access denied data, or inside attested kernel-enforced same-principal
isolation. For the same-principal case, the attestation must mechanically prove
all four properties for the process lifetime: filesystem access is restricted
to the declared mounts and masks; process enumeration, inspection, tracing,
signaling, and control of protected processes are denied; only the declared
descriptors are inherited and no protected descriptor can be acquired; and
privilege escalation or security-principal transition is denied. Tool
permissions, prompts, userspace path wrappers, and an unattested container do not
satisfy this requirement. Record the selected principal mode, evidence, and
verdict in external state and in the supervisor result; a missing or failed
verdict blocks startup before the first tool call.

The coordinator-facing harness primitive is named `superplanner_supervisor`;
built-in `task` is denied in every coordinator frontmatter. The trusted
coordinator launcher must expose only that attested primitive; disable
project/global custom tools, plugins, MCP, and provider overrides; reject every
duplicate tool ID; install only a trusted provider implementation pinned by
package/version/hash and its sole broker endpoint; and verify the primitive's
executable path and SHA-256 before coordinator
startup. It is not a project OpenCode custom tool and cannot be replaced by one.
It starts the coordinator through `env -i` with the same private `HOME`, five
root bindings, pinned private OpenCode config/database, disable flags, broker-only
provider route, scrubbed overrides, and resolved-path attestation required above.
The launcher also presents every coordinator with a write-through project
projection that masks the credential-identity manifest, every symlink/hard-link
alias of those files, and all Git metadata from
model-facing `read`, `grep`, `glob`, and `list`. It mounts only the exact external
state/artifact paths needed by the current phase. The sole `git` executable is an
attested mediator client whose out-of-process service implements the positional
  `safe_git` modes and complete isolated profile in `integration-protocol.md`;
  neither the coordinator nor a shell sees raw Git storage or credential paths.
That service is started before interpreting mediator shell code through the
attested absolute `/usr/bin/env -i ... /bin/bash --noprofile --norc` boundary;
direct `bash <mediator-script>` startup is rejected and its exact argv/environment
hash is persisted.
Its `spawn` request contains agent ID, canonical model/variant, exact worktree,
private `HOME`, all five private roots, resolved OpenCode path manifest,
immutable brief bytes/hash, source-agent and runtime-wrapper identities,
resolved permission hash, skill manifest, provider manifest, credential-identity/alias
manifest, sandbox profile, and the distinct-principal or kernel-enforced
same-principal isolation evidence;
its `resume` request additionally contains the returned session ID and exact
evidence. Both return the session ID, observed session-agent identity, terminal
handoff bytes, process status, principal-isolation verdict/evidence, and other
attestation evidence.

The same primitive exposes an `operate` request only to a coordinator whose
frontmatter denies Bash. Its operation is one of `safe-git`, `sandbox-command`,
`package`, `integrate`, `verify`, `commit-prepare`, `commit`, or local-only
`push-prepare`/`push-present`/`push-retire`. Each operation
uses a separate typed schema and service-side closed grammar: `safe-git` accepts
only the registered positional modes/routes and Git token grammar from
`integration-protocol.md`; `sandbox-command` accepts a canonical executable and
argv array already approved by the plan/checkpoint plus its constrained sandbox
profile, never a shell string; package/integrate/verify accept only registered
artifact IDs, paths, hashes, trees, ownership manifests, and declared check IDs;
and commit preparation/commit accept only the complete authorization fields and
one-use operation tokens from `quality-gates.md`. Each variant atomically owns
its required trusted non-Git config parsing and immutable snapshots, private
content-context creation, route registration, namespace walking, and no-clobber
evidence publication; callers cannot supply shell or raw filesystem steps for
those suboperations. `push-prepare` accepts only registered immutable record IDs
for the source context/object route, authorization manifest, coordinator
`COMMIT` result, standard/adversarial review results, target-bound
`push-presentation-request-v1`, current verification result,
documentation-synchronization result, trusted destination-config snapshot, and
registered supervisor custody configuration. Approved refs, destination,
object/ref format, paths, and custody root are derived service-side, never
accepted as caller authority. It resolves each ID from the service registry,
reopens and identity-checks the registered path without following links,
recomputes its hash, and validates the authorization-to-commit-to-current-ref and
standard-to-adversarial-review cross-links, verification/tree binding, and
documentation/tree binding required by `quality-gates.md`. It captures and
registers the immediate clean-worktree snapshot itself.
Caller-supplied hashes cannot substitute for a registered record. It creates the
exact loose-object push-only Git directory, verifies it with source paths and
network unavailable, constructs and registers the canonical
`push-layout-v1`, `push-execution-closure-v1`, `push-command-v1`, and
`push-eligibility-v1` records, and returns their registered IDs, identities, and
recomputed hashes plus all resolved bindings, custody-root ancestry evidence,
and a no-mutation lease ID; it does not release editable command bytes.
`push-present` accepts only the registered eligibility ID, service-resolves and
rehashes its complete record graph, captures a fresh clean-worktree snapshot,
performs every final check with no network or transport action and only read-only
safe-Git metadata routes, registers
`push-presentation-v1`, and sends the stored `push-command-v1` bytes unchanged
through the trusted user-response output guard. Missing guard support or any
byte difference blocks presentation. The supervisor retains the lease through
presentation and until successful retirement. `push-retire` accepts only the
registered immutable `push-custody-release-v1` record ID. It service-resolves
and rehashes that trusted-user-decision record, requires its closed payload to
bind the active lease and eligibility-manifest IDs with the literal `retire`
action and no execution/outcome field. Under the lease lock it registers the
prepared no-clobber retire-operation record, then uses the same-filesystem
no-replace rename into its supervisor-only cleanup path as the authoritative
one-use revocation point. Rename failure leaves the release unused and immutable
lease active. After rename, recovery under the same operation ID recognizes the
exact inaccessible cleanup identity, idempotently records release consumption
and `retiring`, and resumes deletion without another rename or release. Failure
there records `cleanup-failed`; a `prepared` operation may retry its rename only
when the exact active identity remains and cleanup is absent, and any nonterminal
operation blocks a different operation ID. Only complete deletion records `retired`. None
of these operations asks or records whether the command ran or
what it returned, has transport or network action, or observes push execution.
No `operate` variant accepts shell syntax, an
arbitrary executable, push/transport/network authority, a raw
Git argv, or a candidate-write path outside its typed phase. The result returns
the operation ID, exact normalized request hash, status, output
paths/identities/hashes, sandbox/mediator evidence, and resulting state
transition. Missing `operate` support blocks coordinator-owned commands, Git,
packaging, integration, verification, `COMMIT`, and push preparation,
presentation, or retirement; Bash is never a fallback.
Any other action or operation, missing field, or resume identity drift is
rejected.

The sandbox exposes a supervisor-owned inference broker over a dedicated local
IPC endpoint. Provider credentials and network sockets remain outside the
OpenCode process; the broker accepts only canonical model requests for the
recorded provider/model/variant and returns model responses. Tools, plugins,
project processes, and model-generated commands cannot access the broker
credential, general network, or another provider endpoint. Missing broker,
spawn, re-root, result, or session-resume support blocks the dispatch.

## Candidate-Writer Invocation Envelope

Every brainstormer, planner, builder, debugger, and documenter runs in a fresh
supervised OpenCode process with a private `HOME` and new private config, data,
cache, state, and temp roots and
no persisted approval state. Start
the process inside the constrained process sandbox before provider discovery or
the first tool call. The instance must disable project/global custom tools and provider overrides,
plugins, MCP servers/resources, LSP servers, formatters, and every automatic
process hook. Its resolved registry must contain only stock built-in tool
implementations, reject duplicate tool IDs, and match the agent's wildcard-deny
allowlist. Its provider implementation package/version/hash and sole broker
endpoint must equal the supervisor's trusted manifest. Never reuse a coordinator
or prior worker permission instance.

Only allowlisted Superplanner skills may be visible or executable. OpenCode's
built-in `customize-opencode` may remain registered but must be denied. Before
dispatch, resolve each selected skill to the trusted Superplanner installation,
reject every duplicate name including collisions with built-ins, and verify its
complete file manifest and SHA-256 against the coordinator-recorded installation
manifest. The writer's per-skill permission allowlist remains authoritative. The
coordinator embeds all required reference, template, and skill-support bytes in
the brief. Before startup, the trusted launcher also discovers every applicable
target-repository instruction file for the assigned worktree, including
`AGENTS.md`, `CLAUDE.md`, and configured instruction files; canonicalizes and
hashes each one; and embeds its exact bytes in the brief. Since project config is
disabled, workers never rediscover instructions from ambient configuration.
Missing, ambiguous, symlink-aliased, or out-of-scope instruction evidence blocks
dispatch.

The private data root may expose only that invocation's own truncation output
through OpenCode's generated external-directory exception; it must contain no
other session data and remains denied by edit rules. The sandbox, under the
required principal-isolation verdict, confines the whole process, including
startup and reads, to the assigned worktree and private
runtime roots while denying Git storage, parent/peer worktrees, operational
state/artifacts, general network, provider credentials, and every other external
location except the inference-broker IPC. Before startup, the supervisor builds
a credential-identity manifest that includes every `.env`/`.env.*` path except
explicitly non-secret examples, every repository-declared secret path, and every
supplied credential path. Canonicalize those paths, record filesystem identities, scan
every project file and symlink before exposure, and add every same-identity or
resolving alias to the mask; inability to prove complete alias closure blocks.
The sandbox masks every listed path/alias and all Git metadata from
stock `read`, `grep`, `glob`, and `list`; prompt-level read rules are not treated
as enforcement because stock `grep` does not consult them. If the
harness cannot establish and attest this envelope, do not start the worker.

## Command-Evidence Checkpoints

Candidate-file specialists have no shell access. When one needs command
evidence, it returns the full handoff with `status: blocked`, names the missing
evidence in `blockers`, and puts one exact command plus expected result in
`next_action`. The orchestrator checks that the command is scoped,
non-destructive, and permitted. Because tests and build scripts are
worker-influenced code, it never runs them with unrestricted coordinator
authority. Execute only in a process sandbox rooted at the assigned worktree that
denies writes to Git storage, parent/peer worktrees, external state/artifacts and
OpenCode operational paths, and always denies network and credentials. A command
that requires either capability is never brokered; present it to the user and
wait for user-supplied output. If the filesystem/process sandbox is unavailable,
use the same user-run fallback. Record exact output and sandbox/user evidence, then resume the same generated
OpenCode session ID. These planned checkpoints
may repeat within the dispatch budgets and are not stuck-agent recovery. The
worker returns `complete` only after all evidence required for its claim was
supplied and validated.

## Coordinator Packaging Record

After a builder, debugger, or documenter returns `complete` with no pending
command-evidence resume, the orchestrator validates its ordinary handoff and creates the exact
`integration_result` record defined by `integration-protocol.md`. The record
identifies base/start/packaged trees, quarantined external index and object
storage, external baseline and candidate object directories, per-domain
`setup_objects` and exact encoded `setup_alternates`, the `source_setup_context`
manifest/hash, the `baseline_construction_manifest` with clone-capability,
source/mirror shallow-absence, and namespace-reconstruction proof,
the `baseline_no_hardlink_proof`, the `alternate_absence_manifest`, and
`namespace_snapshot_manifests` covering exact source-before/source-after/raw
post-clone mirror/reconstructed mirror ref/symref plus `HEAD` equality and each
required pre/post walker-root and `packed-refs`
leaf identity/verdict. It also records the trusted ownership file/hash, binary
full-index patch/hash, raw NUL-delimited mode/blob/path manifest/hash, and object
format. Use
`integration_result: none` only when that completed task made no
integration-bound edit. Workers never append or claim this record; a worktree
path, branch, or `changed_files` list is not a substitute for coordinator
packaging. Never package a planned `blocked` command-evidence checkpoint.

## Evidence Rules

- Report exact commands and whether each passed, failed, or was not run.
- Summarize the relevant observed output and link a checkpoint or log when one
  exists.
- Identify the Git SHA used for review or verification when results are
  SHA-sensitive.
- Separate pre-existing baseline failures from failures introduced by the work.
- Never report integration success based only on an isolated worker worktree.
- Accept a handoff only when its supervisor result carries the recorded passing
  distinct-principal or kernel-enforced same-principal isolation verdict and
  evidence; agent-authored claims are not attestation.

## Reviewer Handoffs

Reviewers operate in a separate supervised OpenCode process with a private
`HOME`, private config/data/cache/state/temp roots, the exact environment/path
binding above, and no persisted approvals, project/global custom tools,
plugins, MCP, provider overrides, LSP, formatters, or automatic process hooks.
Require the exact trusted provider implementation/version/hash and sole broker
endpoint. Before startup, place
the entire process in a read-only sandbox that denies general network, provider
credentials, and writes outside its own private output while permitting only the
inference-broker IPC. Mount a credential-identity-masked review projection,
authorization evidence, and required Superplanner references read-only. Git
metadata and every credential-manifest path remain absent from model-facing
`read`, `grep`, `glob`, and `list` access. Build the review projection only from
the exact reviewed commit tree with
the external no-lazy-fetch object route, never from ambient worktree traversal.
For every non-credential tree entry, attest its path, mode, object ID, projected
type, bytes or symlink target, and filesystem identity; credential-manifest
entries are absent rather than replaced with ambiguous bytes. Reject every
untracked or ignored leaf, extra directory entry, symlink escape, hard-link
alias, or tree/projection mismatch. Publish and hash the complete projection
manifest, mount it immutable for the entire review, and bind its hash and the
reviewed SHA into dispatch and result evidence. Model-facing file tools may read
only that projection.
Expose a supervisor-owned,
path/hash-attested Git mediator client as the sole `git` entry in a sanitized
PATH; it talks over dedicated local IPC to trusted Git outside the reviewer
sandbox. The mediator accepts only the frontmatter commands, literal authorized
SHAs/ranges, and canonical non-credential paths contained by the attested review
projection with no symlink escape. It implements a closed token grammar for each
listed subcommand: no `log`; exact `status --short`; `diff`, `diff-tree`, and
`show` only for the supplied authorization base/review SHA and validated path
terminator/list; `rev-parse --verify` only for a supplied literal object; and
fixed-operand `merge-base`, `ls-files`, `ls-tree`, and `cat-file commit` forms.
Every option or operand not in that grammar is rejected, including `--no-index`,
`--alternate-refs`, config overrides, absolute/out-of-root operands, pathspec
magic, `--format`, `--pretty`, every `%G*` placeholder, and all
signature-verification options. The mediator appends final `--format=medium`,
`--date=iso-strict`, `--no-use-mailmap`, `--no-notes`, and
`--no-show-signature` to `show` and
rejects object/path expressions and any diff/show whose complete output path set
intersects the credential manifest; and invokes
Git with `--no-replace-objects --no-lazy-fetch` through the complete `env -i` safe profile from
`integration-protocol.md`, including `GIT_NO_REPLACE_OBJECTS=1`, system/global
config suppression, disabled sparse checkout, `core.fsmonitor=false`, disabled split index/untracked cache,
an attested empty `core.hooksPath`, disabled signing and
`log.showSignature=false`, `format.pretty=medium`,
`log.date=iso-strict`, `log.mailmap=false`, `mailmap.file=/dev/null`,
`mailmap.blob=` and `core.alternateRefsCommand=` disabled, UTF-8 commit/log
encoding, neutral attributes/CRLF,
and omitted trace/output, external-diff, attribute-source, transport,
pager/editor, credential, and configuration-routing variables. For every request,
the supervisor proves the source repository-format/object-format/ref-backend
tuple from an identity-checked config snapshot and keeps those bytes and paths
immutable to untrusted processes until Git exits; command-line `-c` is not a
backend override. The mediator ignores repository-selected worktree routing and
classifies the closed command before supplying canonical paths. `status`, every
allowed `diff`/`diff-tree`, and patch-emitting `show` use a fresh
object-format-correct private fixed-config review Git directory whose detached
`HEAD` is the reviewed SHA. It exists only at the authorization-reserved context
path and is accepted only with the no-clobber post-commit result-manifest
path/identity/hash binding the authorization hash and equal expected/reviewed
OID. Object-only `cat-file`, `ls-tree`, `merge-base`,
`rev-parse`, and ref/admin requests use the real registered Git directory. Both
classes receive the canonical `--work-tree` plus a final matching `-c
core.worktree=<canonical-worktree>` and an explicit read-only real
index/common-object route with bound `alternates=none` and attested absent
on-disk alternate files for every routed object directory and transitive member
from immutable, hash-attested context and operation
manifests. Each request binds the selected class/path, review anchor, static
content manifest, exact index/object route, argv, and terminal result; a static
directory manifest alone does not prove runtime routing. Revalidate and persist
those exact alternate-file absence identities and verdicts before every
request. Bare contexts omit
`--work-tree`. An unregistered or mismatched context,
shallow or promisor/partial-clone state, missing object under no-lazy-fetch, or
attempted context override blocks before Git executes.
Allow only the reviewer's manifest-and-hash-authenticated allowlisted skill and
stock built-in tools; deny the registered `customize-opencode` built-in and
reject every duplicate skill or tool ID. Missing envelope evidence
blocks review. Reviewers report findings and never edit reviewed code, tests,
configuration, documentation, plans, or state.

Every reviewer appends this extension to the common fields:

```yaml
review_result: <approved|findings|not-completed>
reviewed_sha: <full-git-sha-or-none-when-blocked>
findings:
  - <severity, file and line, criterion, and evidence, or none>
accepted_risks:
  - finding_id: <finding-id>
    reviewed_sha: <full-git-sha>
    scope: <exact-accepted-scope>
    user_evidence: <explicit-user-decision-reference>
    consequence: <accepted-consequence>
    rationale: <user-accepted-rationale>
```

Use `none` as the sole `accepted_risks` list item when no risk was accepted.
Every accepted-risk entry requires all six fields shown above. Its `finding_id`
must remain present in `findings`; acceptance never deletes, downgrades, or
conceals the finding. Its `reviewed_sha` must equal the reviewer handoff's exact
`reviewed_sha`, and its scope and user evidence must identify exactly what the
user accepted.

```yaml
accepted_risks:
  - none
```

The adversarial reviewer also appends:

```yaml
standard_approval_record_id: <registered-standard-review-record-id-or-none-when-blocked>
standard_approval_record_identity: <registered-path-identity-or-none-when-blocked>
standard_approval_record_sha256: <full-sha256-or-none-when-blocked>
standard_approval_sha: <full-git-sha-or-none-when-blocked>
```

No other values are valid for `review_result`. For a reviewer handoff:

- `role` is `reviewer`.
- `changed_files` contains only `none`.
- `scope` names the reviewed design, feature, task, acceptance file, plan, diff,
  and exact SHA.
- `verification` records review evidence and any inspection commands.
- Findings include severity, file and line where applicable, violated criterion,
  and supporting evidence.
- `accepted_risks` contains only explicitly user-accepted findings for this
  exact SHA and scope. A reviewer may return `review_result: approved` with
  accepted risks only when every such finding remains visible in `findings` and
  no unresolved or unaccepted finding remains.
- `status` is `complete` when the review itself completed, even if it found
  issues; `review_result` is then `approved` or `findings`.
- `status` is `blocked` only when the review could not be completed, and its
  `review_result` is `not-completed`.
- For a completed review, `reviewed_sha` equals the exact commit inspected and
  equals `git_sha`. For `status: blocked` with
  `review_result: not-completed`, both may be `none` only when no full SHA could
  be established; blockers must state why.
- For a completed adversarial review, the three `standard_approval_record_*`
  fields identify the exact registered immutable standard-review record that was
  independently reopened, identity-checked, and rehashed; its result is
  `approved`, and `standard_approval_sha`, `reviewed_sha`, and `git_sha` are the
  same exact commit. A blocked, not-completed adversarial handoff may use `none`
  only for a record field or SHA it could not establish and must explain why.
- `completion_time` records when the reviewer reached the reported result.

The orchestrator alone updates external `STATE.md`, resolves assignments,
integrates changes, and decides whether the next gate may start. Reviewers never
edit candidate content or operational state.
