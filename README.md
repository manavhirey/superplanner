# Superplanner

Superplanner is a portable OpenCode workflow pack for turning a large product
or software initiative into an approved design, independently valuable
features, reviewable tasks, separate Gherkin acceptance criteria, executable
plans, implemented changes, synchronized documentation, and SHA-bound reviews.

It is a workflow pack, not an active OpenCode project. Install its agents and
register its skills and reference directory in the repository where work will
be performed.

## Builder-Inspired Flow

Superplanner keeps Builder's strongest properties: one orchestrator coordinates
explicit phases, every phase leaves an auditable artifact, approved artifacts
can resume a run, canonical handoff and reviewer results carry scope and
evidence, external state tracks operations, and documentation records what was
implemented rather than what was intended.

The full flow is:

1. Classify the request as `spike`, `bounded`, or `architectural`, and as
   `simple` or `complex`.
2. Inspect the repository, clarify intent one material question at a time,
   compare approaches, and obtain explicit design-direction approval.
3. Write authoritative `design.md` and its `design.html` visual companion.
4. Have the user review the written design, then use a fresh brainstormer
   `RECORD_WRITTEN_APPROVAL` invocation to record approval of the exact current
   Markdown content ID; verify the HTML remains a consistent visual companion.
5. Define independently valuable features, reviewable tasks, and one separate
   Gherkin `.feature` file for every task.
6. Have the user review and explicitly approve the feature, task, and Gherkin
   artifacts, then use a fresh planner `RECORD_DECOMPOSITION_APPROVAL`
   invocation to write only their approval fields before planning that task.
7. Produce a durable, exact, TDD-oriented execution plan and obtain explicit
   user approval, then use a fresh builder `RECORD_PLAN_APPROVAL` invocation to
   write only its approval fields before `EXECUTE`.
8. Establish safe isolation and a clean baseline, then execute serially or in
   proven-safe parallel worktrees.
9. Route failures through root-cause debugging and verify integrated results.
10. Synchronize affected documentation with the implemented repository.
11. Run non-mutating `COMMIT_PREPARE` to present the canonical patch hash,
    candidate tree, exact message hash, full base commit, target state, exact
    author and committer dates with numeric timezones, and independently checked
    expected full commit OID. Obtain authorization that itself explicitly names
    the patch hash, candidate tree, message hash, full base commit, target state,
    exact author and committer dates with numeric timezones, and expected full
    OID, then let the
    user-facing coordinator enter `COMMIT` for only those in-scope changes.
12. Run standard review, then Kimi-K3 adversarial review, against the same exact
    commit SHA, which must equal the authorization's expected full OID.
13. After every gate passes and the user authorizes the target-bound presentation
    record, use only the trusted output guard to release the exact registered
    non-force push command bytes for possible user execution.

Spikes stop with a recommendation and retain no implementation. Bounded changes
use a short in-chat design and approval gate. Architectural work uses the full
artifact pipeline. Hidden complexity upgrades the route; work does not
downgrade after execution starts.

Non-Git workspaces support spikes, design, and planning only; planning includes
decomposition and per-task execution plans. Retained implementation, commits,
SHA-bound reviews, and push eligibility require a complete non-shallow Git
repository supplied by the user plus coordinator-attested
repository/object/`files`-backend tuple, common-directory `shallow`-path absence,
and config-immutability evidence. Superplanner never initializes Git and never
commits automatically; it never executes a push.

## Agents

The filename and frontmatter name are the OpenCode agent handle.

| Agent | Mode and responsibility | Model route |
| --- | --- | --- |
| `superplanner.orchestrator` | Primary full-flow coordinator | `openai/gpt-5.6-sol`, `xhigh` |
| `superplanner.orchestrator-glm` | Alternative complex coordinator; same gates | `zai/glm-5.3`, `max` |
| `superplanner.quick` | Primary spike and bounded-change coordinator | `zai/glm-5.3`, `max` |
| `superplanner.brainstormer` | Repository exploration and approved Markdown/HTML design | `openai/gpt-5.6-sol`, `high` |
| `superplanner.planner` | Features, tasks, and per-task Gherkin acceptance | `openai/gpt-5.6-sol`, `xhigh` |
| `superplanner.builder` | One task in `PLAN`, `RECORD_PLAN_APPROVAL`, or `EXECUTE` mode; no commit/ref authority | `openai/gpt-5.6-sol`, `high` |
| `superplanner.debugger` | Evidence-first root-cause diagnosis and fix | `openai/gpt-5.6-sol`, `high` |
| `superplanner.documenter` | Repository-state documentation synchronization | `zai/glm-5.3`, `max` |
| `superplanner.code-reviewer` | Fresh standard read-only review | `openai/gpt-5.6-sol`, `high` |
| `superplanner.adversarial-reviewer` | Fresh final read-only review after standard approval | `openrouter/moonshotai/kimi-k3`, `max` |

Only the three entry agents are user entry points. The coordinators dispatch
the specialists with self-contained briefs; specialists cannot dispatch their
own subagents or edit external `STATE.md`.

## Prerequisites

- **Platform support (Operational v1).** Linux is required for supervised
  candidate-file specialists, retained implementation, commit, SHA-bound
  reviews, and push-command eligibility; those phases run under
  kernel-enforced user-namespace isolation. macOS supports coordinator
  intake, spike, bounded-design, and planning flows. The same-principal
  four-property isolation verdict cannot be honestly attested on stock
  macOS with public APIs, so supervised candidate-file dispatch and custody
  leases are Linux-only in v1; a failed or missing verdict always blocks
  startup rather than degrading.
- A current OpenCode installation that supports Markdown agents,
  `skills.paths`, named `references`, agent `variant`, and pattern permissions.
- An isolation supervisor implementing the exact spawn, private-`HOME`/XDG/temp
  root and `OPENCODE_CONFIG`/`OPENCODE_DB` binding, required project-config and
  external-skill disable flags, default-plugin disablement with exactly one
  attested private supervisor plugin (pure mode is not used because it blocks
  the required trusted primitive),
  model-catalog fetch disablement, re-root, credential masking, sandbox,
  inference-broker, runtime-wrapper, result, and session-resume interface in
  `references/handoff-contract.md`. Stock OpenCode `Task` alone cannot execute a
  candidate-file specialist or reviewer safely; absence of the supervisor is a
  deliberate blocker, not a degraded mode.
- Each supervised OpenCode process must use a distinct OS security principal or
  attested kernel-enforced same-principal isolation covering filesystem access,
  process inspection/control, inherited descriptors, and privilege escalation.
  Userspace-only isolation or missing evidence blocks startup.
- A trusted coordinator launcher recorded by canonical executable path and
  SHA-256. It disables project/global custom tools, plugins, MCP, and provider
  overrides; pins the trusted provider implementation/version/hash and sole
  broker endpoint; exposes only the provenance-checked
  `superplanner_supervisor` harness primitive with typed `spawn`, `resume`, and
  coordinator-only `operate` requests; masks credential identities,
  every symlink/hard-link alias, and Git metadata from coordinator file tools;
  and routes coordinator Git through the no-lazy-fetch, context-pinned,
  isolated `safe_git` mediator. Bash remains denied; typed `operate` schemas
  cover only sandboxed checks, mediated Git, packaging/integration/verification,
  `COMMIT_PREPARE`, `COMMIT`, `push-prepare`, `push-present`, and `push-retire`,
  with no shell-string or transport route. It launches the coordinator with the same private
  `HOME`/XDG/temp/OpenCode bindings, disable flags, scrubbed overrides, and
  resolved-path attestation required for supervised invocations. Coordinators
  deny built-in `task`.
- A target workspace whose setup and verification commands can be run locally.
- A trusted Git whose `clone -h` semantically advertises exactly one
  value-taking `ref-format` option, accepting Git help's
  `--[no-]ref-format <format>` spelling without requiring literal
  `--ref-format=<format>` text, and whose
  scrubbed `clone --mirror --no-local --ref-format=files` accepts `files` and
  produces that backend. The capability probe is mandatory; an older Git or a
  rejected value blocks setup without a default-backend fallback.
- A user-supplied, complete non-shallow Git repository for retained
  implementation, commits, SHA-bound reviews, and push eligibility. It must use
  the `files` ref backend; trusted non-Git parsing validates the
  repository/object/ref-format tuple from immutable config bytes and requires
  the canonical common Git directory's `shallow` path to be absent as every node
  type. Reftable, URI/payload, invalid-format, conflicting, shallow, or unknown
  forms block those phases. Superplanner never initializes one.
- `sha256sum` or `shasum` for byte-exact non-Git artifact and evidence
  checksums; setup blocks if neither command is available. Repository commit,
  tree, and blob OIDs use the repository's detected SHA-1 or SHA-256 format.
- Authenticated broker access to every provider used by the selected flow. The
  launcher, supervised specialists, and reviewers all use the
  credential-isolating inference broker and never receive provider credentials.
  Confirm every canonical route through the launcher before starting.

The canonical routes are:

| Purpose | Route | Variant |
| --- | --- | --- |
| Complex primary | `openai/gpt-5.6-sol` | `xhigh` |
| Complex coordinator alternative | `zai/glm-5.3` | `max` |
| Simple primary | `zai/glm-5.3` | `max` |
| Simple alternative | `openai/gpt-5.6-sol` | `medium` |
| Simple alternative | `openrouter/moonshotai/kimi-k3` | `max` |
| Adversarial reviewer | `openrouter/moonshotai/kimi-k3` | `max` |

OpenCode agent frontmatter has no automatic fallback list. Selecting
`superplanner.orchestrator-glm` changes only the complex coordinator; the
specialists still use the routes shown in the agent table. Override copied
agent definitions deliberately if a provider is unavailable. Do not silently
substitute the adversarial reviewer or weaken its `max` route.

The GLM coordinator is therefore an alternative complex entry, not a complete
provider fallback. OpenAI specialist routes remain required unless each is
separately reconfigured.

The approved OpenAI specialist routes are fixed by role: the brainstormer and
standard code reviewer use `high`, the planner uses `xhigh`, and the builder and
debugger use `high`. These match the installed agent definitions above.

An unmodified implementation flow uses all three providers: OpenAI for most
specialists, ZAI for documentation and the simple/alternative entries, and
Moonshot for final adversarial review. A quick spike can stop before those
specialists are needed.

## Install In A Project

Run these commands from the target workspace root. Replace every
`/absolute/path/to/superplanner` placeholder below with the same absolute path to
this checkout.

```sh
PACK="/absolute/path/to/superplanner"
test -d "$PACK/agents" && test -d "$PACK/skills" && test -d "$PACK/references"
mkdir -p .opencode/agents
cp -i "$PACK"/agents/*.md .opencode/agents/
```

`cp -i` asks before replacing an existing same-named agent. Resolve collisions
intentionally rather than overwriting local customizations.

Next, create `opencode.json` in the target repository if it does not exist. If
`opencode.json` or `opencode.jsonc` already exists, manually merge the `skills`
and `references` entries below into it; do not replace the existing file or its
other keys. Keep every configured skill path already present and avoid adding a
duplicate path.

```json
{
  "$schema": "https://opencode.ai/config.json",
  "skills": {
    "paths": [
      "/absolute/path/to/superplanner/skills"
    ]
  },
  "references": {
    "superplanner": {
      "path": "/absolute/path/to/superplanner",
      "description": "Superplanner workflow contracts, templates, routing, canonical handoff and reviewer results, external state, and quality gates"
    }
  }
}
```

Both values must be absolute paths to the same checkout. The `superplanner`
reference is required because agents resolve shared contracts through
`@superplanner/references/`; they intentionally do not search for those files
inside the target repository. The coordinators embed required reference/template
and skill-support contents into shell-free writer briefs because those writers
deny sensitive external-directory reads.

Preserving existing formatter, LSP, plugin, MCP, custom-tool, skill, or approval
state does not make it safe for writer invocations. Each writer needs a fresh
private `HOME`, private OpenCode config/data/cache/state/temp roots bound through
the actual XDG/temp/OpenCode variables, and a sandbox active before
startup, with no persisted approvals, custom providers, or provider overrides;
the exact trusted provider implementation/version/hash and sole broker endpoint;
stock built-in tool provenance; LSP/formatters/automatic hooks disabled; and only
manifest-and-hash-authenticated allowlisted Superplanner skills. Resolved paths,
the runtime wrapper, permissions, session agent, and canonical credential
identity/alias masks must be
attested and resume-stable. The launched process must also have the passing
distinct-principal or kernel-enforced same-principal verdict above. Its private data root may expose only that
invocation's own truncation output and must be edit-denied. Missing evidence
blocks the writer; do not weaken the agent permissions globally.

The supervisor is not supplied by stock OpenCode 1.18.x and is not emulated by
starting the coordinator in another directory. It must implement the recorded
contract before the full flow can leave coordinator-only intake. This package
intentionally does not claim that an ordinary Task child is a fresh process.

Fully exit and restart OpenCode after installing or changing configuration,
agent files, or skill files. Starting only a new conversation is not a reliable
configuration reload. After restart, verify discovery from the target root:

```sh
opencode debug config
opencode agent list
```

The resolved config must contain the absolute skill path and the named
`superplanner` reference, and the ten agents above must be listed.

## Use The Full Complex Flow

Start the primary complex coordinator from the target workspace through the
required trusted launcher:

```sh
"$SUPERPLANNER_LAUNCHER" --agent superplanner.orchestrator
```

These commands start the coordinator only. Before it dispatches brainstormer,
planner, builder, debugger, documenter, or either reviewer, it must discover and
attest the configured isolation supervisor and inference broker. Without them it
returns a blocker before the specialist's first tool call.

Then describe the initiative, observable outcome, known constraints, and any
existing approved artifact paths. For a non-interactive entry:

```sh
"$SUPERPLANNER_LAUNCHER" run --agent superplanner.orchestrator "Use the full Superplanner flow for <initiative and outcome>. Known constraints: <constraints>."
```

Use `superplanner.orchestrator-glm` in the same commands when explicitly
selecting the alternative complex coordinator. The coordinator states its
classification before proceeding, obtains design approval, records
feature/task/Gherkin approval through a fresh planner
`RECORD_DECOMPOSITION_APPROVAL` invocation, and records each approved plan
through a fresh builder `RECORD_PLAN_APPROVAL` invocation. It maintains external
`STATE.md` mirrors and resumes valid existing artifacts rather than recreating
them. Selecting this entry does not reroute its OpenAI specialists.

After implementation, integrated verification, documentation sync, and any
verification refresh required by documentation changes, the complex coordinator
presents the complete canonical binary patch/hash, candidate tree, exact message
file/hash, full base commit, full target ref at the base OID, exact author and
committer dates in `<unix-seconds> <+|-HHMM>` form, and expected full commit OID
backed by persisted independently routed no-write `hash-object -t commit --stdin`
operation/result evidence. This non-mutating `COMMIT_PREPARE` may not stage,
write a commit object, or update a ref. Only authorization that itself explicitly
names the patch hash, candidate tree, message hash, full base commit, target
state, exact author and committer dates in numeric-timezone form, and expected
full OID permits the coordinator to stage the exact paths, regenerate and compare
the patch/tree, create the explicit-parent commit object, compare-and-swap the
target, and verify the ref/reflog delta. Mutating
`COMMIT` persists a distinct one-use operation and terminal result manifest for
every authorized `add`, `write-tree`, `commit-tree`, and `update-ref` call and
requires the created full commit OID to equal the expected full OID before any
ref mutation or `update-ref`. Standard review starts against that clean matching
SHA.

## Use The Quick Flow

For a spike or bounded change:

```sh
"$SUPERPLANNER_LAUNCHER" --agent superplanner.quick
```

Or run:

```sh
"$SUPERPLANNER_LAUNCHER" run --agent superplanner.quick "Handle this bounded change: <outcome, scope, and constraints>."
```

A spike returns evidence and a recommendation without retained implementation.
A bounded change receives a short in-chat design with scope, exclusions,
observable acceptance criteria, tests, documentation impact, and risks. Its
exact bytes are stored between bounded-brief markers in external state and
SHA-256 hashed without normalization; the agent presents those exact bytes and
ID and waits for explicit matching approval before dispatching implementation. If the
work proves architectural or develops cross-cutting risk, the quick agent stops
and upgrades to a complex coordinator.

For bounded Git work, after implementation, integrated verification,
documentation sync, and any verification refresh required by documentation
changes, the quick coordinator presents the exact diff and proposed commit
message as a canonical patch/hash, candidate tree, message file/hash, full base
commit, full target ref at the base OID, exact numeric-timezone author/committer
dates, and independently checked expected full commit OID during non-mutating
`COMMIT_PREPARE`. Only matching user authorization that itself explicitly names
the patch hash, candidate tree, message hash, full base commit, target state,
exact author and committer dates in numeric-timezone form, and expected full OID
permits coordinator `COMMIT` to stage the exact in-scope changes, persist per-call
operation/result manifests, create the explicit-parent commit object, require its
full OID to equal the authorized expected OID before any ref mutation or
`update-ref`, and compare-and-swap that target.
Standard review then binds to that clean commit SHA. A non-Git quick flow stops
after spike or bounded design; requests for decomposition or execution planning
route to a complex coordinator. It never dispatches retained implementation,
commit, SHA-bound review, or push-command eligibility work without Git.

## Generated Artifacts

The full complex route writes:

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

`design.md` is authoritative. If the HTML companion disagrees, regenerate
`design.html`.

Operational `STATE.md` is outside this candidate tree. For Git repositories its
required location is `<git-common-dir>/superplanner/<initiative>/STATE.md`, so
all worktrees share it without tracking it as candidate content. Before a
non-Git flow starts, an absolute external state path must be explicitly
configured; there is no in-workspace default. Only the orchestrator writes it.
It records mirrors of artifact and plan approval records, task status,
worktrees, task and invocation IDs, milestones, verification, documentation,
commit-authorization evidence and the resulting SHA, canonical handoff and
reviewer results, review SHAs, `accepted_risks`, monitoring events, and the exact
next action. External state prevents recording a review result from changing the
reviewed content and immediately invalidating its own SHA approval.

Coordinator-persisted checkpoints, packaging artifacts, and raw integration
evidence use a separately configured absolute artifact root outside both the
candidate and every Git directory. Validated handoffs and review records may be
stored beside external `STATE.md`; workers emit their content only through
supervisor results. After each integration-bound builder, debugger, or documenter
returns `complete` with no pending command-evidence resume, the orchestrator
creates quarantined packaging indices, object directories, and integration
bundles under that root. A non-local full-ref bare baseline mirror prevents
pre-authorization Git operations from freshening repository objects. Its exact
logical ref/symref namespace and `HEAD` are reconstructed from stable trusted
source manifests rather than accepted from clone transport. `STATE.md`
records those paths, but workers never run shell commands or write outside their
assigned workspace.

Design, feature, task, Gherkin, and plan approvals are durable candidate
artifact records. Markdown surrounds each approval record with exactly one
`<!-- superplanner-approval:start -->` and
`<!-- superplanner-approval:end -->` pair. Gherkin uses the equivalent
`# superplanner-approval:start` and `# superplanner-approval:end` lines. Remove
the complete marker lines and every enclosed byte from the exact UTF-8 file
bytes, preserving every outside byte without line-ending, whitespace, encoding,
or final-newline normalization, then hash the remainder. Missing, duplicate,
nested, or misordered markers block approval. The content ID is lowercase
`sha256:<64 lowercase hex>` and must match `^sha256:[0-9a-f]{64}$`. After user
approval, a fresh planner `RECORD_DECOMPOSITION_APPROVAL` invocation writes only
decomposition approval fields, or a fresh builder `RECORD_PLAN_APPROVAL`
invocation writes only plan approval fields. Candidate and external-mirror
values for `status`, `approver`, `approved_at`, `approval_evidence`, and
`content_id` must match exactly. `PLAN` and `EXECUTE` validate both copies; an
external state mirror alone cannot authorize either transition.
Pending records require `none` identity, time, and evidence. Approved records
require a non-`none` identity, valid ISO-8601 time, and explicit user-message
evidence bound to the displayed artifact and exact content ID.

## Isolation And Parallel Work

Before implementation, the full coordinator detects existing isolation and
submodules, prefers a harness-native worktree mechanism, and uses Git worktrees
only when repository and ignore rules are safe. It runs project setup and a
clean test baseline, and asks before continuing from a failing baseline.
Contained-submodule discovery is initially index-only. Every initialized
submodule independently completes the full external mirror/bootstrap and
explicit object-routing protocol before any child object, attribute, or
status/diff read.

Parallelism requires a dependency and ownership matrix. Tasks stay serial when
they consume one another's output, edit common files, mutate shared state, or
need the same exclusive resource. Parallel-safe builders receive separate
worktrees and self-contained prompts. The harness must re-root each file-writing
invocation to its exact worktree so parent and peer worktrees are sensitive
external locations and denied. The isolation supervisor and credential-isolating
inference broker from `references/handoff-contract.md` must be active before process startup and the first tool:
no persisted approvals, custom providers, MCP, plugins, LSP, formatters, or
automatic hooks; stock built-in tool provenance; authenticated allowlisted
skills; an edit-denied private data root; and a distinct-principal or passing
kernel-enforced same-principal isolation verdict covering filesystem access,
process inspection/control, inherited descriptors, and privilege escalation.
Brokered project commands must be
process-sandboxed away from Git and operational storage, network, and credentials; otherwise commands are
presented to the user for execution. A prompt path alone blocks retained edits. Default concurrency is four
and the hard maximum is eight. A standard Task call does not prove re-rooting or
create a private process; starting the coordinator in the isolated worktree does
not satisfy the envelope. Stop before candidate-file dispatch when the supervisor
cannot attest it. Integration-bound worker results are not valid until the
orchestrator packages, verifies, and mechanically applies a
base/start-tree-bound binary patch, literal ownership, raw mode/blob manifest,
and checksums through external object/index storage. Workers never commit or
package results, and the orchestrator never
repairs patches or stages the real index before authorization. The combined
relevant suite must then pass in the integration workspace.

## Stuck-Agent Monitoring

Every dispatch names a stable workflow task ID, captures the generated OpenCode
session ID from the supervisor result, and defines explicit worker milestones, scope,
expected output, and a step or execution budget. The worker must stop at the
budget and return the canonical handoff result as `complete` or `blocked`, with
its latest milestone evidence. A harness timeout remains an observed event but
is normalized into a `blocked` handoff carrying timeout evidence; it is not a
third handoff status.

Foreground supervisor invocations cannot be live-polled or stopped unless the supervisor advertises those controls. The
orchestrator waits for a `complete` or `blocked` handoff. For a shell-free
worker's planned command-evidence checkpoint, it validates the one scoped,
non-destructive, permitted command and runs it only in the constrained process
sandbox, or asks the user to run it when unavailable or when network or
credentials are required. It resumes the same generated
OpenCode session ID with exact sandbox/user evidence, repeating within budget. This is normal
execution, not stuck recovery. It normalizes any harness timeout event to
`blocked` and attempts one focused recovery resume with that generated ID. Only when the current harness explicitly
exposes live status and cancellation may the orchestrator check a long-running
invocation every five to ten minutes and cancel it; those controls must not be
assumed.

An agent is potentially stuck when its handoff is `blocked` for a reason other
than a valid planned command-evidence checkpoint, including budget or normalized
harness-timeout evidence; it reports the same failed action twice without new
evidence; or it waits for unavailable input, permission, or an exclusive
resource. Two checks without meaningful progress are a signal only in a harness
that exposes live status.

Recovery order is mandatory: resume the same generated OpenCode session ID with the
observed blocker and one focused action; never substitute the stable workflow
task ID, and record resume as unavailable when no generated ID was returned. If
that fails, wait for the foreground invocation to
end or fail, or use supported cancellation and confirm inactivity; only then
dispatch a fresh replacement with the full brief and prior evidence; then
escalate unresolved or destructive, security-sensitive, irreversible, or
scope-changing action. A replacement writer never overlaps an active writer in
the same scope. Unsupported polling or stopping is not evidence of inactivity.
Events are recorded in external `STATE.md`.

## Commit, Review, And Push Command Gate

Gate order cannot be reversed or combined:

1. After integrated verification, documentation sync, and any verification
   refresh required by documentation changes, run coordinator-only,
   non-mutating `COMMIT_PREPARE`; it may not stage, write a commit object, or
   update a ref. Persist and present the complete canonical binary patch and
   hash, full base commit, base/candidate trees, full target ref at the base OID,
   exact paths, commit identity, exact author and committer dates each in Git's
   `<unix-seconds> <+|-HHMM>` numeric-timezone form, expected full
   repository-format commit OID, and UTF-8 message file/hash. Persist the
   independently routed no-write `hash-object -t commit --stdin` operation and
   terminal result manifests and require their OID to agree. Obtain explicit
   authorization that itself explicitly names the patch hash, candidate tree,
   message hash, full base commit, target state, exact author and committer dates
   in numeric-timezone form, and expected full OID; plan approval is not
   authorization.
2. The user-facing coordinator uses the environment-scrubbed, hookless, unsigned
   `safe_git authorized <authorization-manifest-sha256> --` profile after proving
   the repository/object/`files`-backend tuple and common-directory
   `shallow`-path absence from immutable evidence. It
   stages only the authorized paths through a private fixed-config
   content-control Git directory and explicit real index/object route that cannot
   load repository filter drivers, regenerates and
   byte-compares the canonical patch/hash, requires the staged tree to equal the
   authorized candidate tree, creates a commit object with the authorized base
   as its sole parent, verifies it, and compare-and-swap advances only the
   authorized target. Every authorized `add`, `write-tree`, `commit-tree`, and
   `update-ref` call has its own persisted one-use operation manifest and
   terminal result manifest. Before any ref mutation or `update-ref`, the created
   object's full OID and returned OID must equal the authorization's expected
   full OID. It
   verifies the resulting tree and raw message bytes, proves `update-ref`
   received the authorization-bound `GIT_COMMITTER_DATE`, and byte-compares both
   complete branch and symbolic-worktree `HEAD` reflog appends with the expected
   old/new OIDs, name/email, Unix seconds, numeric timezone, and reason.
   Unexpected commit-time mutation blocks review. A changed
   candidate or message requires new authorization. Isolated writers cannot
   commit or update refs, no commit is automatic, and no push is executed.
3. Run `superplanner.code-reviewer` in the fresh private read-only reviewer
   envelope from `references/handoff-contract.md` against the approved
   artifacts, complete diff, verification, documentation status, and exact
   current commit SHA. It independently requires that full SHA to equal the
   authorization's expected full OID. If Python changed, it must load
   `reviewing-py-code`. It returns the canonical reviewer result.
4. Resolve every finding or have the user explicitly accept its exact risk for
   the reviewed SHA and scope, then run a fresh standard reviewer until it
   approves. Risk acceptance is not reviewer approval. Any candidate change
   returns through verification, docs sync, non-mutating `COMMIT_PREPARE`, user
   authorization naming the new patch/tree/message/base/target, exact
   numeric-timezone author and committer dates, and expected full OID,
   coordinator `COMMIT`, and fresh standard review.
5. Only after standard approval, run `superplanner.adversarial-reviewer` in a new
   private read-only reviewer envelope on Kimi-K3 `max` with the same SHA and
   independently require it to equal the authorization's expected full OID.
6. Resolve or explicitly accept its findings for the exact SHA and scope, then
   run a fresh adversarial reviewer until it approves. Any later candidate
   change invalidates both approvals and repeats that complete replacement
   `COMMIT_PREPARE` authorization tuple, coordinator `COMMIT`, and standard-first
   review sequence.

Every canonical reviewer result includes `accepted_risks`. Each accepted-risk
entry records the finding, explicit user decision evidence, and exact reviewed
SHA and scope. Acceptance applies only to that named risk and never substitutes
for fresh reviewer approval.

Superplanner never executes a push and the mediator has no push route. Only when
integrated verification and documentation are current, the exact commit was
user-authorized, both reviews approve the same current SHA, the worktree has no
unreviewed changes, and the user separately authorizes the exact commit/reviews/
destination/refspec in `push-presentation-request-v1`, Superplanner rechecks the
gate and may request guarded release of the exact config-isolated, hookless `<absolute-env> -i
... '<absolute-git>' '--exec-path=<absolute-attested-git-exec-path>'
--no-replace-objects --no-lazy-fetch
'--git-dir=<absolute-push-only-git-directory>' push --no-force --
'<effective-https-url>' '<full-local-ref>:<full-remote-ref>'` form from the
quality gate for user execution. Eligibility binds the remote name as provenance,
exactly one closed-grammar HTTPS URL, no URL rewrites, the SHA-1/SHA-256
environment config with private-only `PATH`, matching `safe.directory`, and
redirects disabled, reviewed source/authorization/commit/review evidence, an
exact loose-object push-only Git directory under a separate-principal immutable
custody lease, a closed attested executable/helper/interpreter/loader/library
process graph, POSIX
shell-quoted operands, expected local OID/push-only-source-ref equality, and the
policy forbidding every force form. The supervisor resolves registered immutable
verification, documentation, source, authorization, commit, review, request, and
destination records; recomputes their hashes; validates their cross-links;
captures an immediate clean-worktree snapshot; and creates registered canonical
layout, execution-closure, exact-command, and eligibility records. Caller-
asserted hashes are insufficient. A local-only `push-present` final check sends
the stored bytes through a trusted output guard; the model cannot render or edit
them. The custody lease remains active until retirement validates a registered
immutable trusted-user-decision release record bound to the lease and manifest
and atomically renames the bundle into an inaccessible cleanup namespace as the
revocation point. It then idempotently records one-use consumption and
`retiring`, `cleanup-failed`, or `retired` cleanup state. That record has no
execution or outcome field. Superplanner never executes, brokers, retries,
observes, or reports command execution.
Canonical reviewer results are stored in external `STATE.md`, so recording them
does not modify candidate content or trigger the SHA invalidation rule.

## License

Superplanner is available under the [MIT License](LICENSE). Pinned source
attribution is in [THIRD_PARTY.md](THIRD_PARTY.md), with complete notices in
[THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).

See [PLAN.md](PLAN.md) for the implementation architecture.
