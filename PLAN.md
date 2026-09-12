# Superplanner Architecture

Superplanner implements a portable OpenCode workflow for carrying a large
initiative from clarified intent to reviewed code. The design uses a thin,
state-owning orchestrator and specialist agents that exchange durable
artifacts rather than relying on conversation history.

## Builder Lineage

The workflow retains the Builder concepts that make long-running work
auditable:

- one orchestrator owns routing, sequencing, integration, and external shared
  state;
- explicit phases produce an artifact for the next phase;
- approved artifacts can resume or skip completed phases;
- canonical handoff and reviewer results identify paths, scope, acceptance
  criteria, evidence, blockers, verdicts, and reviewed SHAs;
- documentation describes the integrated repository, not an intended future;
- user approval gates design, feature/task/Gherkin definition, durable plans,
  implementation, and the exact commit.

Superplanner replaces the parts that do not provide enough control for larger
initiatives:

| Replaced approach | Implemented approach |
| --- | --- |
| One agent carries all context and performs all work | Role-specific agents receive self-contained briefs and return canonical handoff results |
| Plans and acceptance notes remain in chat | Designs, features, tasks, Gherkin criteria, and execution plans are durable candidate files; operational state is external |
| Opportunistic fan-out | A dependency and ownership matrix permits parallelism only for non-conflicting scopes |
| Workers share a mutable checkout | Parallel workers use separate safe worktrees without shell access; the orchestrator packages, verifies, and mechanically applies base/tree-bound binary patch bundles |
| Repeated speculative fixes | A debugger reproduces the failure, tests one hypothesis, and applies one root-cause fix |
| Passing tests imply completion | Verification, documentation synchronization, and two independent reviews gate completion |
| Review approval follows a moving branch | Both approvals identify the same current commit SHA and are invalidated by later changes |

## Phase Ownership

| Phase | Owning role | Output or decision |
| --- | --- | --- |
| 0. Intake and routing | `superplanner.orchestrator`, `superplanner.orchestrator-glm`, or `superplanner.quick` | Spike, bounded, or architectural workflow and selected entry agent |
| 1. Brainstorm and design | `superplanner.brainstormer`, coordinated by a complex orchestrator | Approved `design.md` and visual companion `design.html` |
| 2. Feature and task definition | `superplanner.planner` | Independently valuable feature, task, and Gherkin files; fresh `RECORD_DECOMPOSITION_APPROVAL` invocation after user approval and before `PLAN` |
| 3. Execution planning | `superplanner.builder` in `PLAN` mode | One implementation-ready durable plan; fresh `RECORD_PLAN_APPROVAL` invocation after user approval and before `EXECUTE` |
| 4. Isolated execution | Orchestrator and `superplanner.builder` in `EXECUTE` mode | Worktree-isolated changes from approved plans, integrated in dependency order |
| 5. Monitoring and recovery | Entry orchestrator | Milestone handoffs, recovery decisions, and monitoring events in external `STATE.md` |
| 6. Debugging and verification | `superplanner.debugger` and orchestrator | Root-cause fixes, task checks, and the integrated relevant test suite |
| 7. Documentation synchronization | `superplanner.documenter` | Documentation reconciled with implemented behavior |
| 8. Commit transition | User-facing orchestrator | User authorization itself explicitly naming the canonical patch hash, candidate tree, message hash, full base commit, target state, exact author and committer dates with numeric timezones, and expected full OID, committed by explicit parent and target compare-and-swap as a clean candidate SHA |
| 9. Review and push-command gate | `superplanner.code-reviewer`, then `superplanner.adversarial-reviewer` | Two fresh approvals for one exact SHA; only an exact bound non-force command may be presented after an explicit user request |

The brainstormer inspects the repository before asking one material question
at a time. For architectural work it compares two or three approaches and
recommends the smallest sound design. The planner refuses to invent missing
actors, value, rules, or preconditions. The builder expands only an approved
task and includes exact files, interfaces, tests, commands, expected results,
and documentation effects. The user must explicitly approve the affected
feature, task, and separate Gherkin artifacts before `PLAN`. A fresh planner
`RECORD_DECOMPOSITION_APPROVAL` invocation writes only their approval fields.
After the user approves the durable task plan, a fresh builder
`RECORD_PLAN_APPROVAL` invocation writes only its approval fields before
`EXECUTE`. Artifact content IDs exclude approval fields, so approvals remain
bound to the reviewed content. The orchestrator mirrors both records in external
state, but those mirrors alone cannot authorize either transition.

After integrated implementation, verification, and documentation sync, the
orchestrator persists and presents the canonical binary patch and hash, full
base commit, base and candidate trees, and exact full target ref at the base OID,
exact in-scope files, exact commit identity, exact author and committer dates in
numeric-timezone form, precomputed commit OID, and repository-style message file
and hash. Plan approval does not authorize a commit. The authorization itself
must explicitly name the patch hash, candidate tree, message hash, full base
commit, target state, both exact numeric-timezone dates, and expected full OID.
Only after that matching authorization, the coordinator uses the
environment-scrubbed, hookless, unsigned `safe_git authorized <authorization-manifest-sha256> --` profile to
validate the repository/object/`files`-backend tuple and common-directory
`shallow`-path absence, reject content-affecting attributes, and stage only that
scope through a phase/anchor private
fixed-config Git control directory with an explicit real index/object route that
cannot load repository filter drivers. It verifies the
staged patch/tree against the authorization, creates the commit object with the
authorized base as sole parent, requires its full OID to equal the authorized
expected full OID before any ref mutation or `update-ref`, compare-and-swap
advances only the target, and
verifies its parent, tree, raw message bytes, authorization-bound
`GIT_COMMITTER_DATE` environment, and byte-exact branch and symbolic-worktree
`HEAD` reflog appends containing the expected old/new OIDs, name/email, Unix
seconds, numeric timezone, and reason. Standard review starts only from the resulting full SHA
and clean candidate. Isolated writers cannot commit or update refs, and neither
commit is automatic; Superplanner never executes a push.

## Model Routing

Trusted installation selects one registered route for every shipped agent and
records immutable catalog, profile, and generated-installation manifests.
Checked-in frontmatter provides these defaults:

| Route | Model | Variant |
| --- | --- | --- |
| Complex primary (`superplanner.orchestrator`) | `openai/gpt-5.6-sol` | `xhigh` |
| Complex coordinator alternative (`superplanner.orchestrator-glm`) | `zai/glm-5.3` | `max` |
| Simple primary (`superplanner.quick`) | `zai/glm-5.3` | `max` |
| Brainstormer | `openai/gpt-5.6-sol` | `high` |
| Planner | `openai/gpt-5.6-sol` | `xhigh` |
| Builder and debugger | `openai/gpt-5.6-sol` | `high` |
| Standard code reviewer | `openai/gpt-5.6-sol` | `high` |
| Documenter | `zai/glm-5.3` | `max` |
| Adversarial review | `openrouter/moonshotai/kimi-k3` | `max` |

The installer may replace each default with a role-compatible route from its
canonical catalog. It generates and attests the private installed agent files;
project configuration cannot override them. Launcher, supervisor, broker, and
external state bind every workflow to all three record hashes. Missing access
blocks without fallback. Existing resumes keep their original profile. The
adversarial backend-model identity must differ from builder, debugger,
documenter, and standard-review backend identities; aliases are not independent,
and the default independent route is Kimi-K3.

## Artifacts And State

The authoritative design and all downstream artifacts live under
`docs/superplanner/<initiative>/`. Markdown controls if `design.md` and
`design.html` disagree; the HTML companion is regenerated from the Markdown.

`STATE.md` is an operational control-plane ledger outside the candidate tree.
For Git repositories it lives at
`<git-common-dir>/superplanner/<initiative>/STATE.md`; a non-Git workspace must
have an explicit external state path configured before work starts. It records
the current phase, artifact paths and candidate approval-record mirrors, feature
and task status, worktrees, task and invocation IDs, milestones, verification
evidence, documentation status, commit-authorization evidence and resulting
SHA, canonical reviewer results, review SHAs, `accepted_risks`, monitoring
events, and the exact next action. Restricting it to the orchestrator avoids
conflicting transitions. Keeping it external also
prevents recording a review result from modifying candidate content and causing
a review-SHA invalidation loop.

A non-Git workspace can run spike, design, and planning phases, including
decomposition and per-task execution plans, with an explicitly configured
external state path. It cannot retain implementation, commit, run SHA-bound
reviews, or become push-eligible. Those phases require a complete non-shallow
Git repository supplied by the user plus the validated
repository/object/`files`-backend tuple, canonical common-directory
`shallow`-path absence, and config-immutability evidence; Superplanner never
initializes one. Baseline cloning transfers objects, after which a trusted
non-Git serializer reconstructs the private mirror's exact ref/symref namespace
and `HEAD` from stable pre/post-clone source manifests before the mirror is
frozen or used.

Candidate design, feature, task, Gherkin, and plan artifacts contain durable
approval records. Markdown surrounds each record with exactly one
`<!-- superplanner-approval:start -->` and
`<!-- superplanner-approval:end -->` pair; Gherkin uses the equivalent
`# superplanner-approval:start` and `# superplanner-approval:end` lines. Remove
the complete marker lines and every enclosed byte from the exact UTF-8 file
bytes, preserving every outside byte without normalization, then hash the
remainder. Missing, duplicated, nested, or misordered markers block approval.
SHA-256 output uses exact form
`sha256:<64 lowercase hex>`, matching `^sha256:[0-9a-f]{64}$`. Candidate and
external fields `status`, `approver`, `approved_at`, `approval_evidence`, and
`content_id` must match exactly. The orchestrator validates both records before
`PLAN` or `EXECUTE`; an external mirror is evidence, not authorization.
Pending records use `none` identity, time, and evidence; approved records require
a non-`none` identity, valid ISO-8601 time, and explicit user-message evidence
bound to the displayed artifact and exact content ID.
These SHA-256 values cover non-Git artifacts and evidence. Repository commit,
tree, and blob OIDs use the repository's detected SHA-1 or SHA-256 object format.

Every non-review specialist returns the canonical handoff result:

- `complete` or `blocked` status, stable task ID, and scope completed;
- artifact paths and changed files;
- milestone and verification evidence;
- blockers and assumptions;
- recommended next action.

Review specialists return the canonical reviewer result with verdict, reviewed
SHA and scope, findings and evidence, `accepted_risks`, and verification
context. Every `accepted_risks` entry contains explicit user decision evidence
and the exact SHA and scope to which it applies. It closes only that risk and
does not skip a fresh reviewer invocation that returns approval. The
orchestrator validates these results, updates external `STATE.md`, and decides
whether the next phase can start. Existing approved artifacts are inputs to
resumed runs, so a restart does not require reconstructing intent from chat.

## Parallelism And Monitoring

Parallel execution is a safety decision, not a throughput default. Before
dispatch, the orchestrator maps task dependencies, file ownership, shared
state, and exclusive resources. It serializes tasks that consume one another's
output, touch the same files, mutate shared state, or need the same exclusive
resource.

Parallel-safe workers receive self-contained prompts and separate worktrees,
with harness evidence that each file-writing invocation is re-rooted to its exact
worktree and uses the required isolation supervisor and credential-isolating
inference broker in `references/handoff-contract.md`. The sandbox is active before process startup and provider discovery; persisted
approvals, custom providers, automatic processes, and unauthenticated skills are
absent. The process uses either a distinct OS security principal or attested
kernel-enforced same-principal isolation covering filesystem access, process
inspection/control, inherited descriptors, and privilege escalation. A
prompt-only path or missing runtime evidence blocks retained edits;
unsandboxed commands go to the user rather than running with coordinator authority.
Workers cannot dispatch subagents, and only the orchestrator integrates their
results. Default concurrency is four; eight is the hard maximum. Integrated
results must pass together in the integration workspace even when every worker
passed independently. After an integration-bound worker returns complete, the orchestrator packages
its isolated result using the literal ownership, external index/object storage,
binary patch, raw mode/blob manifest, and packaged/pre/post tree protocol in
`references/integration-protocol.md`; workers never commit or package results,
and the orchestrator never authors or repairs implementation hunks.
Contained-submodule detection is initially index-only. Each initialized
submodule independently completes the external mirror/bootstrap protocol and
uses explicit child object routes before any child object, attribute, or
status/diff read.

Each dispatch defines a stable workflow task ID, captures the generated
OpenCode session ID returned by `superplanner_supervisor`, and defines explicit worker
milestones, scope, expected output, and a step or execution budget. The
foreground supervisor invocation cannot be live-polled or stopped unless the
supervisor advertises those controls. The worker therefore enforces its budget
and returns a canonical `complete` or `blocked` handoff carrying its latest
milestone evidence. A shell-free worker's planned command-evidence checkpoint is
a `blocked` return that the orchestrator services by running the validated
command only in the constrained process sandbox, or by asking the user to run it
when the sandbox is unavailable or network/credentials are required,
then resuming the same generated OpenCode session ID with exact evidence;
this may repeat within budget and is not stuck recovery. A harness timeout
remains an observed monitoring event and is normalized into a `blocked` handoff
carrying timeout evidence; `timeout` is not a canonical status. The orchestrator
attempts one focused recovery resume for a genuine blocker or timeout. It never
substitutes the stable workflow task ID; when no generated ID was returned,
resume is unavailable rather than guessed.

Only a harness that explicitly exposes live status and cancellation may add
five-to-ten-minute status checks, treat two checks without progress as a stuck
signal, or cancel an invocation. A replacement is dispatched only after the
foreground invocation ends or fails, or supported cancellation confirms it is
inactive. It receives the complete brief and prior evidence. The orchestrator
escalates when recovery still fails or would require a destructive,
irreversible, security-sensitive, or scope-changing action. This preserves
stuck-agent recovery without allowing overlapping file-writing agents.

## Documentation Synchronization

Documentation is downstream of implementation. The documenter reads the
integrated repository, diff, tests, configuration, and existing documentation,
then changes only material affected by current behavior and verifies examples
and commands. It runs after each integrated task marked with documentation
impact and always runs immediately before final review. Any documentation change
to candidate bytes triggers affected task checks and the full relevant suite
again before commit authorization.

## Review Invalidation

Review follows the explicit commit transition and is ordered and read-only:

1. A fresh standard reviewer checks the current diff against the approved
   design, feature, task, Gherkin criteria, and execution plan. Python changes
   add the `reviewing-py-code` lens and it returns the canonical reviewer result.
2. A finding is fixed or explicitly accepted for its exact SHA and scope. Risk
   acceptance is not approval; a fresh standard reviewer must still approve.
3. Only after standard review is clear, a fresh profile-selected independent
   reviewer loads `adversarial-reviewer` and tries to disprove specification
   compliance, correctness, design quality, and data or network efficiency.
4. Adversarial findings follow the same accepted-risk rule and require a fresh
   adversarial approval. Reviewers never edit the reviewed code.

Any candidate-file or candidate-content change after either review, including
code, tests, configuration, documentation, generated output, or approval-bound
planning artifacts, invalidates all approvals. Verification and documentation synchronization are
refreshed, and the new authorization itself explicitly names the canonical patch
hash, candidate tree, message hash, full base commit, target state, exact author
and committer dates in numeric-timezone form, and expected full OID. Coordinator `COMMIT` then creates
the new clean candidate, and review restarts at the
standard reviewer.
A push-command presentation is eligible only when the exact commit was
user-authorized, both reviews approve the same current commit SHA, the worktree
has no unreviewed changes, and the user authorizes the exact commit/reviews/
destination/refspec in `push-presentation-request-v1`. Superplanner never
executes, brokers, retries, observes, or reports a push. It may present only the config-isolated, hookless
`<absolute-env> -i ... '<absolute-git>'
'--exec-path=<absolute-attested-git-exec-path>' --no-replace-objects --no-lazy-fetch
'--git-dir=<absolute-push-only-git-directory>' push
--no-force -- '<effective-https-url>' '<full-local-ref>:<full-remote-ref>'` form
from `quality-gates.md`, after binding the remote name as provenance, one
closed-grammar HTTPS URL, no URL rewrites, the immutable attested push-only Git
directory's exact loose-object layout and separate-principal custody lease, the
service-resolved registered verification/documentation/source/authorization/
commit/review evidence with recomputed hashes and validated cross-links, an
immediate clean-worktree snapshot, the canonical eligibility manifest,
registered layout/exact-command and closed attested executable/helper/loader/
library manifests, private-only `PATH`, SHA-1/SHA-256 environment config with
matching `safe.directory` and redirects disabled, POSIX shell-quoted operands,
expected local OID/push-only-source-ref equality, and no-force policy. The local
`push-present` output guard alone releases stored command bytes after final
revalidation. A manifest-bound release record drives an atomic same-filesystem
rename as the revocation point, followed by idempotent one-use consumption and
explicit retiring/cleanup-failed/retired cleanup states. Recording canonical
reviewer results in external `STATE.md` does not modify candidate content and
therefore does not invalidate the reviewed SHA.
