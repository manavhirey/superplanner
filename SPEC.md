# Superplanner Design Specification

## Purpose

Superplanner is a portable OpenCode workflow pack for turning large product or
software initiatives into approved designs, independently valuable features,
well-defined tasks, executable acceptance criteria, implementation plans, and
reviewed changes.

The workflow preserves the strongest parts of the Builder reference:

- A single orchestrator coordinates explicit phases.
- Each phase writes an auditable artifact for the next phase.
- Existing artifacts can be supplied to resume or skip completed phases.
- Every handoff contains paths, scope, acceptance criteria, and blockers.
- The documentation phase describes the repository as implemented, not merely
  as planned.

## Goals

1. Clarify product and software intent before implementation.
2. Decompose large initiatives into features that deliver observable value.
3. Decompose features into tasks that are independently testable and reviewable.
4. Store each task's acceptance criteria in a separate Gherkin feature file.
5. Create a detailed execution plan for each task before editing code.
6. Use isolated worktrees and parallel workers only when their scopes are safe.
7. Detect and recover subagents that are blocked or making no progress.
8. Debug failures by finding root causes before proposing fixes.
9. Keep documentation synchronized with the current repository.
10. Require user authorization that itself explicitly names the prepared patch
    hash, candidate tree, message hash, full base commit, target state, exact
    author and committer dates with numeric timezones, and expected full OID
    before committing.
11. Prevent Superplanner from executing pushes; after both reviews approve the
    exact current commit and the user requests it, allow only presentation of an
    exact bound non-force command for user execution.

## Non-Goals

- Superplanner never initializes a Git repository; the user must supply one.
- Superplanner does not automatically commit, publish, merge, or open pull
  requests, and never executes a push.
- Superplanner does not infer missing product rules when they affect acceptance
  behavior.
- Superplanner does not parallelize work merely because parallel capacity is
  available.
- Superplanner does not treat passing tests as a substitute for code review.

## Package Form

The repository is a portable workflow pack, not an active OpenCode project.
Consumers copy the agent files into an OpenCode agent directory and register or
copy the skills. Installation instructions are defined in `README.md`.

## Runtime Artifact Layout

For an initiative named `<initiative>`, Superplanner writes:

```text
docs/superplanner/<initiative>/
|-- design.md
|-- design.html
`-- features/
    `-- F001-<feature-slug>/
        |-- FEATURE.md
        |-- tasks/
        |   `-- T001-<task-slug>.md
        |-- acceptance/
        |   `-- T001-<task-slug>.feature
        `-- plans/
            `-- T001-<task-slug>-plan.md
```

The Markdown design is authoritative. The HTML design is its visual companion.
When they disagree, the Markdown design controls and the visual must be
regenerated.

`STATE.md` is operational control-plane data, not generated candidate content.
For a Git repository it lives at
`<git-common-dir>/superplanner/<initiative>/STATE.md`, shared by that
repository's worktrees. A non-Git workspace must have an external state path
explicitly configured before the workflow starts; there is no in-workspace
default. Keeping state outside the candidate tree lets the orchestrator record
review results without changing the reviewed content or commit, which would
otherwise create a review-SHA invalidation loop.

Approval content IDs are deterministic SHA-256 identifiers. Every approvable
Markdown artifact contains exactly one `<!-- superplanner-approval:start -->`
and `<!-- superplanner-approval:end -->` pair. Gherkin uses the equivalent
`# superplanner-approval:start` and `# superplanner-approval:end` lines. Remove
the complete marker lines and every enclosed byte from the exact UTF-8 file
bytes, leaving every byte outside them unchanged. Do not normalize whitespace,
line endings, encoding, or the final newline.

If markers are missing, duplicated, nested, or misordered, approval is blocked.
Hash the remaining exact bytes and encode the result as
`sha256:<64 lowercase hex>`, matching `^sha256:[0-9a-f]{64}$`. Candidate and
external-mirror fields `status`, `approver`, `approved_at`,
`approval_evidence`, and `content_id` must match exactly before the next gate.
Pending records require `none` identity, time, and evidence. Approved records
require a non-`none` identity, valid ISO-8601 time, and explicit user-message
evidence bound to the displayed artifact and exact content ID.
SHA-256 is also used for other non-Git artifact and evidence checksums.
Repository commit, tree, and blob OIDs use that repository's detected SHA-1 or
SHA-256 object format instead.

## Workflow

### Phase 0: Intake and Routing

The entry agent classifies work as spike, bounded, or architectural and as
simple or complex. Classification is stated to the user before continuing.

- Spikes produce a recommendation, not retained implementation.
- Bounded changes receive a short in-chat design whose exact bytes are mirrored
  in external state and SHA-256 fingerprinted before an approval gate.
- Architectural work receives the complete artifact pipeline.
- Hidden complexity upgrades the route; work never downgrades mid-task.

A non-Git workspace supports spike, design, and planning only; planning includes
decomposition and per-task execution plans. Retained implementation, commits,
SHA-bound reviews, and push eligibility require a complete non-shallow Git
repository supplied by the user using Git's `files` ref backend. Trusted non-Git
parsing validates the repository/object/ref-format tuple from immutable config
bytes and requires the canonical common Git directory's `shallow` path to be
absent as every node type; `reftable`, URI/payload, invalid-format, conflicting,
shallow, or unknown forms block these phases. Superplanner never initializes Git
on the user's behalf.

### Phase 1: Brainstorm and Design

The brainstormer inspects the current repository before asking questions. It
asks one material question at a time, proposes two or three approaches for
architectural work, recommends the smallest sound option, and waits for explicit
approval.

After approval it writes `design.md` and uses the visual-explainer workflow to
write and open `design.html`. No implementation begins before the user approves
the written design. A fresh approval-record invocation updates only the marked
design record; the orchestrator recomputes its content ID and requires the
candidate record and external mirror to match before feature definition.

### Phase 2: Feature and Task Definition

The planner turns the approved design into independently valuable features. A
feature contains its outcome, actor, value, in-scope behavior, excluded scope,
dependencies, risks, and ordered task index.

Every task contains a single reviewable outcome, explicit inputs and outputs,
scope boundaries, dependencies, likely ownership, documentation impact, and a
link to a separate `.feature` file.

The planner applies the bdd-gherkin skill to every task acceptance file. Each
scenario is single-focused, uses business or domain language, contains exactly
one `When`, and asserts observable outcomes. Missing actors, value, rules, or
preconditions are returned to the orchestrator for clarification rather than
invented.

The user reviews and explicitly approves the affected feature, task, and
separate Gherkin artifacts before the builder may create that task's execution
plan. A fresh planner `RECORD_DECOMPOSITION_APPROVAL` invocation then writes only
the approval fields in those candidate artifacts. Their content IDs exclude
approval fields, so the records bind approval to unchanged artifact content.
The orchestrator mirrors the durable approval in external `STATE.md`, but that
operational mirror alone never authorizes `PLAN`. Immediately before `PLAN`, it
recomputes the design, feature, task, and Gherkin IDs and requires the complete
candidate approval chain and external mirrors to be approved, current, and
identical.

### Phase 3: Execution Planning

The builder applies the writing-plans skill to one user-approved task at a
time. The task plan names exact files, interfaces, test cases, commands,
expected results, documentation changes, and small TDD-oriented steps. Plans
contain no TODO, TBD, or implied implementation.

The task Markdown, Gherkin acceptance file, approved design, and relevant
repository files travel with the execution plan.

The execution plan is a durable artifact. After the user reviews and explicitly
approves it, a fresh builder `RECORD_PLAN_APPROVAL` invocation writes only the
plan approval fields. The plan content ID excludes those fields, so the approval
remains bound to the reviewed plan content. The orchestrator mirrors the
approval in external `STATE.md`. A valid candidate plan approval record is
required for `EXECUTE`; external state evidence alone is insufficient.
Immediately before `EXECUTE`, the orchestrator recomputes every design, feature,
task, Gherkin, and plan ID and requires both approval copies to match the source
IDs embedded in the plan.

### Phase 4: Isolated Execution

Retained implementation starts only in a user-supplied complete non-shallow Git
repository. A non-Git run stops after planning and hands the approved artifacts
to the user. Superplanner does not initialize Git to cross this boundary.
Supervised candidate-file execution additionally requires Linux in Operational
v1: the same-principal four-property isolation verdict cannot be honestly
attested on stock macOS, so those dispatches and custody leases are Linux-only
and fail closed elsewhere.

Before implementation, the orchestrator applies the using-git-worktrees skill:

1. Detect existing worktree isolation and submodules.
2. Prefer a harness-native worktree mechanism.
3. Use Git worktrees only when the repository and ignore rules are safe.
4. Run project setup and establish a clean test baseline.
5. Ask before proceeding from a failing baseline.

Bootstrap rejects a shallow source before any object read. The non-local
baseline clone is used for object transfer only; while still private and
supervisor-writable, its exact loose ref/symref namespace and `HEAD` are
reconstructed with a trusted non-Git serializer from byte-identical source
manifests captured immediately before and after clone, then rewalked for exact
logical equality before the mirror is frozen or used.

The orchestrator creates a dependency and ownership matrix before dispatch.
Tasks are parallel only when they do not consume one another's output, edit the
same files, mutate shared state, or require the same exclusive resource.

Parallel workers receive self-contained prompts and separate worktrees. The
harness launcher gives the coordinator a credential/Git-metadata-masked project
projection and routes all Git through the isolated `safe_git` mediator. The
harness must re-root each file-writing invocation's filesystem/project boundary
to its exact worktree; a prompt path is insufficient, and missing capability
blocks retained edits. Each candidate-file specialist uses the required
isolation supervisor and credential-isolating inference broker in
`references/handoff-contract.md`: private `HOME` plus XDG/temp roots, pinned private
`OPENCODE_CONFIG`/`OPENCODE_DB`, required project-config and external-skill
disable flags, default-plugin and model-catalog-fetch disablement plus exactly
one attested private supervisor plugin (pure mode is not set because it blocks
the required trusted primitive),
scrubbed OpenCode overrides, resolved-path attestation, a source-bound runtime
wrapper and permissions/session-agent identity stable across resume,
credential/Git-metadata masking, no persisted approvals, custom providers or
provider overrides, MCP, LSP, formatters, or automatic process hooks;
authenticated allowlisted skills;
stock built-in tool provenance; and a process sandbox active before startup and
the first tool call. The launched process must use a distinct OS security
principal or attested kernel-enforced same-principal isolation covering
filesystem access, process inspection/control, inherited descriptors, and
privilege escalation; missing evidence blocks startup. Brokered project commands
use that sandbox or are delegated
to the user. They do not dispatch their own subagents. Default concurrency is four and the hard
maximum is eight. The orchestrator alone updates shared state and integrates
results. Isolated writers have no shell access and return only canonical handoffs
and changed paths. After an integration-bound builder, debugger, or documenter
invocation returns `complete` with no pending command request, the orchestrator packages its worktree into the base/start-tree-bound
binary patch, raw NUL-delimited mode/blob manifest, literal ownership file, and hashes defined by
`references/integration-protocol.md`, using external index and object storage.
Every Git operation that reads worktree content uses a private fixed-config
control Git directory, so repository-local filter commands cannot execute even
if `.gitattributes` changes between validation and staging.
It replays that bundle against scratch indexes and a separate candidate index;
it never consumes a worker commit, stages the real index, repairs a patch, or
resolves a conflict.
Contained-submodule discovery is initially index-only. Before any `ls-tree`,
attribute, status/diff, or other object read for an initialized submodule, the
orchestrator independently applies the complete external mirror/bootstrap
protocol to that submodule and uses its explicit external object routes.

### Phase 5: Monitoring and Recovery

Every dispatch defines a stable workflow task ID, captures the generated
OpenCode session ID returned by `superplanner_supervisor`, and defines explicit worker
milestones, scope, expected output, and a step or execution budget. The worker
stops at the budget instead of looping and returns the canonical handoff result with `complete` or
`blocked` status and the latest milestone evidence. A harness timeout remains
an observed monitoring event, but it is normalized into a `blocked` handoff
carrying timeout evidence; `timeout` is not a canonical handoff status.

Foreground supervisor invocations do not expose live status polling or stop
controls unless the supervisor advertises them. The orchestrator therefore waits for the invocation handoff. For a
shell-free worker's planned command-evidence checkpoint, it validates the one
scoped, non-destructive, permitted command and runs it only in the constrained
process sandbox, or asks the user to run it when unavailable or when network or
credentials are required. It resumes the same generated
OpenCode session ID with exact sandbox/user evidence, repeating within budget; this is normal
execution, not stuck recovery. For a genuine blocker or normalized harness
timeout, it detects stalls from milestone and budget evidence and attempts one
focused recovery resume with that generated ID. Only when the current harness explicitly
exposes live status and cancellation may the orchestrator check long-running work every five to ten
minutes and cancel it. An agent is potentially stuck when any of these
capability-aware conditions applies:

- It returns a `blocked` handoff that is not a valid planned command-evidence
  checkpoint.
- It exhausts its step or execution budget and returns a `blocked` handoff with
  budget evidence.
- A harness timeout event is normalized into a `blocked` handoff with timeout
  evidence.
- It reports the same failed action twice without new evidence at a handoff.
- In a live-status-capable harness, it makes no meaningful progress across two
  consecutive checks.
- It waits for unavailable input, permission, or an exclusive resource.

Recovery proceeds in this order:

1. Resume with the same generated OpenCode session ID and one focused next action.
   Never substitute the stable workflow task ID; if no OpenCode ID was returned,
   record resume as unavailable rather than guessing.
2. If recovery fails, wait for the foreground invocation to end or fail; when
   cancellation is supported, cancel and confirm it is inactive.
3. Only then dispatch a fresh agent with the complete brief, milestones, and
   prior evidence.
4. Escalate to the user when recovery still fails or requires destructive,
   security-sensitive, irreversible, or scope-changing action.

The orchestrator never dispatches a replacement file-writing agent while the
original may still be active in the same scope. Unsupported live polling or
stopping is never treated as proof of inactivity. Monitoring and recovery
events are recorded in external `STATE.md`.

### Phase 6: Debugging and Verification

Any bug, failed test, build failure, performance regression, or unexpected
behavior invokes the debugger. The debugger must reproduce the problem, inspect
recent changes, trace data across boundaries, compare working examples, state a
single hypothesis, test it minimally, create a failing regression test when
possible, and then implement one root-cause fix.

After three failed fix attempts, the debugger stops and asks for an
architectural decision. It does not stack speculative fixes.

After integrating work, the orchestrator runs task-specific checks and the full
relevant suite. Parallel results are not considered valid until they pass
together in the integration workspace.

### Phase 7: Documentation Synchronization

The documenter reads the repository, implementation diff, tests, configuration,
and existing documentation. It updates only documentation affected by current
behavior and verifies examples and commands against the repository. It cannot
edit approval-controlled `docs/superplanner/**` artifacts; any required change
returns to the appropriate authoring and user-approval stage.

The documenter runs after every integrated task marked as having documentation
impact and unconditionally before final review. If documentation synchronization
changes any candidate byte, the orchestrator reruns affected task checks and the
full relevant suite so final verification and documentation bind to the same
candidate.

### Phase 8: Commit Gate

#### Phase 8a: `COMMIT_PREPARE` (Non-Mutating)

After implementation is integrated, verification is current, and documentation
is synchronized, the orchestrator revalidates the complete approval chain or
bounded brief. From the external candidate index it records the base and
candidate trees and persists the canonical binary full-index, no-color,
no-renames, no-external-diff, no-textconv patch and SHA-256. It stores the
proposed UTF-8 message with one terminal newline and hashes it before presenting
the complete patch, trees, exact files, explicit commit identity/dates,
and message to the user. It selects the exact author and committer dates, each in
Git's `<unix-seconds> <+|-HHMM>` form with a numeric timezone, canonically
serializes the expected commit bytes, and computes the expected full
repository-format commit OID without writing an object. Persisted independently
routed no-write `hash-object -t commit --stdin` operation and terminal result
manifests must agree with that OID. `COMMIT_PREPARE` may create external evidence
but may not stage, write a commit object, or update a ref. Plan
or implementation approval is not commit authorization. The authorization
itself must explicitly name that patch hash, candidate tree, message hash, full
base commit, target state, exact author and committer dates in numeric-timezone
form, and expected full OID; detached `HEAD` blocks authorization, and any change
requires new authorization.

#### Phase 8b: `COMMIT` (Authorized Mutation)

Only the user-facing coordinator performs `COMMIT`; all isolated writers are
mechanically denied commit and ref mutation. It uses the environment-scrubbed,
hookless, unsigned `safe_git authorized <authorization-manifest-sha256> --` profile to reject content-affecting
attributes and stage only the authorized paths through the same private
fixed-config content-control Git directory, regenerates and byte-compares the
canonical patch and hash, requires the staged tree to equal the authorized
candidate tree, creates and verifies a commit object whose explicit sole parent
is the authorized base, then compare-and-swap advances only the authorized target
full target ref. The resulting parent, tree, identity, and raw message
bytes must match authorization, and complete ref/reflog comparison must show only
the expected base-to-new target delta under a revalidated `files` backend.
`update-ref` receives the authorization-bound `GIT_COMMITTER_DATE`; both the
branch and symbolic-worktree `HEAD` reflog appends must byte-match the exact
old/new OIDs, name/email, Unix seconds, numeric timezone, and reason. Unexpected
commit-time mutation blocks review and push-command presentation and requires explicit user direction
before rollback or renewed authorization.
Every authorized `add`, `write-tree`, `commit-tree`, and `update-ref` call must
have its own persisted one-use operation manifest and terminal result manifest;
aggregate or static route evidence is insufficient. After `commit-tree`, the
created object's parsed full OID and returned OID must equal the authorization's
expected full OID before `update-ref` or any other ref mutation. A mismatch
blocks all ref mutations.
Superplanner never commits automatically and never executes a push. Standard review cannot start
from an uncommitted working tree; it binds to the resulting clean commit SHA.

### Phase 9: Review and Push Command Gate

The standard code reviewer runs first in the fresh private read-only reviewer
envelope from `references/handoff-contract.md`, with provider/skill provenance and a process
sandbox established before startup. It requires fresh attested coordinator
recomputation of the complete approval chain or bounded brief, independently
compares that evidence with the candidate and external approval records, checks
the full reviewed SHA equals the authorization's expected full commit OID,
checks the authorized and committed trees match, and reviews the diff
against the approved design, feature, task, Gherkin criteria, and execution plan.
If Python files changed, it must load the reviewing-py-code skill as an
additional review lens.

After standard review is clear, a fresh Kimi-K3 agent in the same kind of
private read-only envelope using
`openrouter/moonshotai/kimi-k3` at maximum reasoning loads the
adversarial-reviewer skill. It tries to disprove specification compliance,
correctness, design quality, and data or network efficiency.
Both reviewers independently require their supplied and parsed full reviewed SHA
to equal the authorization's expected full commit OID.

Reviewers report findings but never edit reviewed code. Findings are resolved
or explicitly accepted by the user. Each canonical reviewer result includes
`accepted_risks`; every entry contains explicit user decision evidence and the
exact reviewed SHA and accepted scope. Acceptance closes only that named risk.
It does not itself approve the review, transfer to another SHA or scope, or skip
a fresh reviewer invocation that returns approval.

Any candidate-file or candidate-content change after either review, including
code, tests, configuration, documentation, generated output, or approval-bound
planning artifacts, invalidates all approvals. The workflow reruns affected verification and
documentation, then reruns non-mutating `COMMIT_PREPARE`. The new authorization
itself names the patch hash, candidate tree, message hash, full base commit,
target state, exact author and committer dates in numeric-timezone form, and
expected full OID. Coordinator `COMMIT` then runs, and review restarts at
standard code review for the new clean SHA.

Each reviewer returns the canonical reviewer result with its verdict, reviewed
commit SHA and scope, findings and evidence, `accepted_risks` with explicit user
evidence bound to that exact SHA and scope, and verification context.
The orchestrator records that result in external `STATE.md`; this operational
write does not alter candidate content and therefore does not invalidate the
reviewed SHA.

Approval records include the reviewed Git commit SHA. Superplanner never
executes a push and the Git mediator has no push route. It may present an exact
user-executed push command only when:

1. Verification is current.
2. Documentation synchronization is current.
3. The current SHA is the SHA returned by coordinator `COMMIT` for the exact
   user-authorized patch, candidate tree, and message, and its tree and message
   bytes equal the authorized candidate tree and message.
4. Standard code review approves the current SHA.
5. Kimi-K3 adversarial review approves the same SHA.
6. The worktree contains no unreviewed changes.
7. After both exact review records exist, the user authorizes a target-bound
   `push-presentation-request-v1`; it authorizes command presentation only.
8. Eligibility evidence binds the destination remote name as provenance, exactly
   one closed-grammar HTTPS URL, absence of URL rewrites, the attested absolute
   `env`/Git paths, private-only `PATH`, and executable/helper/loader/library
   closure, the exact loose-object
   push-only Git directory under an active separate-principal no-mutation custody
   lease, service-resolved registered verification/documentation/source/
   authorization/commit/review records, recomputed hashes, validated cross-links,
   an immediate clean-worktree snapshot, canonical layout/execution-closure/
   command/eligibility records, the exact SHA-1/SHA-256 config-isolation
   environment with matching `safe.directory` and redirects disabled, the full refspec and POSIX
   shell-quoted operands, the expected local OID and push-only source-ref
   equality, and a policy forbidding every force form.

The only presented form is the config-isolated, hookless `<absolute-env> -i ...
'<absolute-git>' '--exec-path=<absolute-attested-git-exec-path>'
--no-replace-objects --no-lazy-fetch
'--git-dir=<absolute-push-only-git-directory>' push --no-force --
'<effective-https-url>' '<full-local-ref>:<full-remote-ref>'` command defined in
`quality-gates.md`. The config snapshot must yield one HTTPS URL and no
`insteadOf`/`pushInsteadOf` entry; execution suppresses system/global config,
never loads candidate-repository local config, and loads only the independently
attested immutable push-only directory's allowlisted local config plus the fixed
object/ref-format, `core.hooksPath=/dev/null`, matching `safe.directory`, and
`http.followRedirects=false` overrides. Every operand rejects control bytes and
uses POSIX single-quote escaping. A local-only `push-present` operation repeats
the final checks and sends registered exact command bytes through a trusted output
guard that the model cannot edit. Superplanner does not execute, broker, retry,
observe, or report it; the user may execute it outside the workflow while the
supervisor retains immutable custody. Retirement one-use consumes a registered
trusted-user-decision release record bound to the lease and eligibility manifest
only after an atomic same-filesystem rename linearizes presented-path revocation;
idempotent recovery then uses explicit
`retiring|cleanup-failed|retired` cleanup states. The release record has no
execution or outcome field.

## Model Routing

- Complex primary: `openai/gpt-5.6-sol`, variant `xhigh`.
- Complex coordinator alternative: `zai/glm-5.3`, variant `max`.
- Simple primary: `zai/glm-5.3`, variant `max`.
- Simple alternatives: `openai/gpt-5.6-sol` at `medium` or
  `openrouter/moonshotai/kimi-k3` at `max`.
- Brainstormer: `openai/gpt-5.6-sol`, variant `high`.
- Planner: `openai/gpt-5.6-sol`, variant `xhigh`.
- Builder and debugger: `openai/gpt-5.6-sol`, variant `high`.
- Standard code reviewer: `openai/gpt-5.6-sol`, variant `high`.
- Documenter: `zai/glm-5.3`, variant `max`.
- Adversarial reviewer: `openrouter/moonshotai/kimi-k3`, variant `max`.

OpenCode does not provide an automatic model fallback list in agent frontmatter.
The GLM entry is an explicit alternative for the complex coordinator, not a
complete provider fallback. Selecting it does not reroute OpenAI specialists;
their routes remain in effect unless their copied agent definitions are
separately reconfigured.

## State and Handoffs

External `STATE.md` is written only by the orchestrator. It records the
initiative, approved artifact and plan paths, mirrors of candidate approval
records, current phase, feature and task statuses, worktree paths, task and
invocation IDs, milestones, verification evidence, documentation status,
commit-authorization evidence and the resulting SHA, canonical reviewer results,
review SHAs, `accepted_risks`, monitoring events, and the exact next action. The
orchestrator validates candidate approval records;
their external mirrors cannot independently advance the workflow.

Every non-review specialist returns the canonical handoff result containing
`complete` or `blocked` status, task ID, scope completed, artifact paths,
changed files, milestone and verification evidence, blockers, assumptions, and
recommended next action. Harness timeouts are recorded as events and normalized
to `blocked` results with timeout evidence.
Review specialists return the canonical reviewer result described in Phase 9.

## Acceptance

Superplanner is complete when a clean OpenCode installation plus the required
trusted coordinator launcher, isolation supervisor, credential-isolating
inference broker, and isolated `safe_git` mediator can:

- Route simple and complex work to the configured models.
- Produce approved Markdown and HTML design artifacts.
- Produce features, task definitions, and separate Gherkin acceptance files,
  then require candidate approval records from a fresh
  `RECORD_DECOMPOSITION_APPROVAL` invocation before per-task planning.
- Produce implementation-ready plans without placeholders and require durable
  candidate approval records from a fresh `RECORD_PLAN_APPROVAL` invocation
  before execution.
- Distinguish parallel-safe work from dependent or conflicting work.
- Detect and recover stuck agents through capability-aware monitoring, resume
  the generated OpenCode session ID, and replace without duplicate writes.
- Keep operational state outside candidate content in Git and non-Git
  workspaces.
- Stop non-Git work after spike, design, or planning, including decomposition;
  require a user-supplied complete non-shallow Git repository plus the validated
  repository/object/`files`-backend tuple, common-directory `shallow`-path
  absence, and config-immutability evidence for retained implementation and
  later gates.
- Route technical failures through systematic debugging.
- Update documentation from implemented repository state.
- Refuse to commit until the user authorization itself explicitly names the
  canonical patch hash, candidate tree, message hash, full base commit, target
  state, exact numeric-timezone author and committer dates, and expected full OID
  prepared without mutation, then
  commit only through coordinator `COMMIT` with per-call operation/result
  manifests and pre-ref-mutation created-OID equality.
- Invoke the Python review skill whenever Python changes are reviewed.
- Refuse adversarial review until standard review passes.
- Never execute a push. Refuse to present the exact bound non-force user command
  unless both reviews approve the same current commit and the user explicitly
  requests it.
