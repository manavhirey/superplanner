# Superplanner Artifact Contracts

These contracts define the runtime artifacts produced for architectural work. A
bounded change may use an approved in-chat design, but any retained artifacts
must follow the same authority and traceability rules.

## Runtime Layout

Candidate artifacts live in the repository:

```text
docs/superplanner/<initiative-slug>/
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

Operational state is never candidate content. In Git, resolve the common
directory from the candidate worktree into a registered external output leaf with
`safe_git_to <external-output>/git-common-dir plain -- -C <registered-candidate-worktree> rev-parse --path-format=absolute --git-common-dir` and store state at:

```text
<git-common-dir>/superplanner/<initiative-slug>/STATE.md
```

Attempt that absolute form first with its own one-use operation/output leaves.
Only an unsupported-`--path-format` result permits a fresh
`safe_git_to <external-output>/git-common-dir-relative plain -- -C <registered-candidate-worktree> rev-parse --git-common-dir` call; resolve its exact output relative to the candidate worktree before appending
`superplanner/<initiative-slug>/STATE.md`. In a non-Git workspace, the user or
harness must explicitly configure an absolute state path outside the candidate
root; there is no in-repository default.

External state supports discovery, design, feature/task definition, and
implementation planning in a non-Git workspace. Retained `EXECUTE`, coordinator
`COMMIT`, SHA-bound standard or adversarial review, and push-command eligibility
require a Git repository created by the user. Superplanner never runs `git init`;
repository creation is always a user action.

The state file and any operational matrix, checkpoint, or persisted handoff
beside it must be untracked and excluded from candidate diffs. Writing
operational state must not dirty a worktree, appear in a review diff, create a
candidate file change, or change the candidate review SHA.

## Initiative Contract

The initiative is the directory-level record at
`docs/superplanner/<initiative-slug>/`; it does not require a separate
initiative Markdown file. It consists of:

- An approved `design.md` defining intent, scope, constraints, and the selected
  approach.
- A generated `design.html` visual companion.
- One directory per independently valuable feature under `features/`.

The orchestrator-owned external `STATE.md` records execution and review state.
The initiative slug and identity must agree in `design.md`, external `STATE.md`,
and all child artifact metadata. The initiative is ready for feature
decomposition only after explicit user approval of the written design is
recorded and bound to the current design content.

## Design Contract

`design.md` must contain:

- Initiative identity and document metadata. Approval is not a document status.
- Problem, actors, intended outcomes, and observable success criteria.
- In-scope behavior, non-goals, constraints, and material product rules.
- Considered approaches and the reason for the selected approach.
- Components, boundaries, interfaces, data flow, and relevant failure behavior.
- Dependencies, risks, documentation impact, and verification strategy.
- Open decisions that block approval, or an explicit statement that none remain.
- The user approval record.

`design.html` must be generated from the current Markdown design and opened for
visual inspection. It must not introduce requirements absent from `design.md` or
copy mutable approval metadata; any displayed stage label is approval-independent.
No implementation starts until the written design is explicitly approved.

## Approval Record And Content ID Contract

`design.md`, every `FEATURE.md`, task Markdown, Gherkin acceptance file, and
implementation plan each contain exactly one approval marker pair. Markdown
artifacts use these exact complete lines, with no leading or trailing bytes:

```text
<!-- superplanner-approval:start -->
<!-- superplanner-approval:end -->
```

Gherkin uses these exact comment lines:

```text
# superplanner-approval:start
# superplanner-approval:end
```

The approval block between the markers contains only `status`, `approver`,
`approved_at`, `approval_evidence`, and `content_id`. Approval status is exactly
`pending` or `approved` and exists only in this marked block.

The status-dependent invariants are mandatory:

- `pending` requires `approver: none`, `approved_at: none`, and
  `approval_evidence: none`.
- `approved` requires a non-`none` approver identity, a valid ISO-8601 timestamp,
  and non-`none` explicit user-message evidence that identifies the displayed
  artifact and its exact recomputed content ID.
- `content_id` is always the current deterministic ID. A present field with
  `none`, malformed time, generic assent, or evidence for another ID does not
  satisfy approval.

Compute every `content_id` deterministically:

1. Read the complete file as exact UTF-8 bytes.
2. Find the exact complete start-marker line and exact complete end-marker line.
3. Require exactly one of each and require the start marker to precede the end
   marker. Missing, duplicate, or misordered markers are blockers.
4. Remove the complete marker lines, including their line-ending bytes when
   present, and remove every byte enclosed by them.
5. Leave every byte outside the removed region unchanged. Do not normalize line
   endings, whitespace, Unicode, or a trailing newline.
6. SHA-256 hash the remaining exact bytes and encode the result as
   `sha256:<64 lowercase hex>`, matching `^sha256:[0-9a-f]{64}$`.

Approval recording or correction may change only bytes enclosed by the existing
markers. It must not change either marker line or any byte outside them. Any
outside-byte change produces a different content ID and invalidates the prior
approval; the marked record must remain or become `pending` until the new ID is
explicitly approved.

Approval is a chain, not an isolated plan flag. Before `DEFINE`, recompute the
design ID and require its approved candidate record to equal the external
mirror. Before `PLAN`, do the same for the design, feature, task, and Gherkin
records. Before `EXECUTE`, do the same for design, feature, task, Gherkin, and
plan, and require each upstream ID to equal the corresponding source ID embedded
in the plan. Any mismatch or stale upstream record blocks the transition.

## Feature Contract

Each `FEATURE.md` must contain:

- Feature ID, slug, title, initiative, and approved design path. Operational
  workflow lifecycle lives only in external `STATE.md`.
- One observable outcome, its actor, and the value delivered.
- In-scope behavior and explicitly excluded scope.
- Dependencies, risks, and likely affected domains.
- An ordered task index with task and acceptance-file links.
- A durable approval record inside its exact marker pair.

A feature must deliver independently observable value. It must not be merely a
technical layer or a grouping chosen only to enable parallel work.

Recording approval changes only its marked record. The approval is current only
when its deterministic content ID matches the file.

## Task Contract

Each task Markdown file must contain:

- Task ID, slug, title, feature path, and approved design path. Operational
  workflow lifecycle lives only in external `STATE.md`.
- A single reviewable outcome.
- Explicit inputs, outputs, scope boundaries, and excluded work.
- Dependencies and the outputs consumed from those dependencies.
- Likely file or subsystem ownership and documentation impact.
- A relative link to its separate Gherkin acceptance file.
- Completion and verification evidence expected from implementation.
- Its own durable approval record inside its exact marker pair.

A task must be independently testable and reviewable. It must not duplicate its
Gherkin scenarios inline or silently define behavior that conflicts with its
feature or design. Recording approval changes only its marked record.

## Gherkin Acceptance Contract

Every task has exactly one separate acceptance file at
`acceptance/<task-id>-<task-slug>.feature`. The planner applies the
`bdd-gherkin` skill when producing it.

The file must:

- Identify the actor, desired capability, and value where those are material.
- Express product or domain behavior rather than implementation details.
- Keep each scenario focused on one behavior.
- Use exactly one `When` step in each scenario.
- State preconditions as `Given` steps and observable outcomes as `Then` steps.
- Cover accepted rules and edge cases without inventing missing behavior.
- Carry a comment-only durable approval record inside its exact marker pair.

Missing actors, value, rules, or preconditions are blockers returned to the
orchestrator for clarification. A changed scenario, rule, or other byte outside
the approval markers changes the deterministic content ID and invalidates the
approval.

## Implementation Plan Contract

Each task plan may be created only after the feature, task, and separate Gherkin
file are approved for their current content IDs. It must contain:

- Links to the approved design, feature, task, and acceptance file.
- Exact files to add or modify and the interfaces affected.
- Exact test cases and commands.
- A complete failing-test, expected-failure, minimum-implementation, and
  focused-passing-test cycle repeated for every acceptance scenario.
- Broader passing-test steps with expected results after all scenario cycles.
- Required documentation changes and verification commands.
- A durable plan approval record inside its exact marker pair.
- A commit gate that runs only with explicit user authorization for that commit.

The plan travels with its source artifacts and relevant repository files. A
filled plan must contain no unresolved placeholders, implied implementation, or
unresolved product decisions. Plan approval status is exactly `pending` or
`approved`; implementation cannot begin unless the approval applies to the
  current plan content ID. Commit is never automatic, and Superplanner never
  executes a push.

## Coordinator COMMIT Stage Contract

The coordinator's non-mutating `COMMIT_PREPARE` step and mutating `COMMIT`
transition are separate from isolated `PLAN` and `EXECUTE` work. Preparation may
run after implementation and affected documentation are verified; mutating
`COMMIT` may run only after exact user authorization and only when all of these
conditions hold:

- The workspace is a user-created Git repository. Superplanner never runs
  `git init`. Trusted non-Git config parsing validates the repository-format,
  object-format, and `files` ref-backend tuple from one identity-checked
  snapshot; format 0 requires every extension absent, including `objectFormat`,
  `refStorage`, and `worktreeConfig`, while format 1 permits absent or one exact
  `files` value. Any invalid tuple, duplicate,
  conflict, or URI/payload form blocks. The canonical common Git directory's
  `shallow` path must be absent as every node type before any object read. The
  supervisor keeps the attested config and shallow-path absence immutable to
  untrusted processes through each Git invocation.
- The user explicitly authorizes this specific commit after verification and
  documentation synchronization. Plan approval is not commit authorization.
- Before asking, the coordinator persists the baseline-mirror clone, trusted
  namespace-reconstruction, source/mirror shallow-absence, and no-hardlink
  proof, the canonical binary full-index patch defined by
  `quality-gates.md`, its SHA-256, the base and candidate tree OIDs, the full base
  commit, exact full target ref at the base OID, complete ref/reflog snapshots
  with exact `refs`, `logs`, and every `worktrees/*/logs` walker-root identity
  and verdict plus optional `packed-refs` leaf identity and verdict, explicit commit identity,
  exact author and committer dates, precomputed expected commit OID with an
  independent no-write hash check, exact in-scope files, an external
  authorized-message candidate ending in one newline with its SHA-256, and
  reserved absent review-context and post-commit result-manifest paths.
- The user authorization names that patch hash, candidate tree, exact UTF-8
  message hash, full base commit, target state, exact author and committer dates,
  and expected commit OID. A file list, ordinary display diff, tree-equivalent
  parent, or later-staged tree alone is not sufficient.
- The coordinator re-inspects status and candidate identity, then uses the
  environment-scrubbed `safe_git authorized <authorization-manifest-sha256> --` profile from
  `quality-gates.md` to reject content-affecting attributes and stage only those
  exact files through an authorization-bound private fixed-config content Git
  directory and persist a one-use operation manifest and identity- and
  hash-attested terminal result manifest for every authorized call, proving Git
  used the explicit real
  index/common-object route without repository-local filter drivers,
  exact in-scope files, exclude unrelated changes and external operational
  state, and require the regenerated canonical staged patch and staged tree to
  equal the authorized patch bytes/hash and candidate tree before creating one
  hookless, unsigned commit object with the authorized base as its explicit sole
  parent and compare-and-swap advancing only the authorized target.
- The resulting full commit OID, commit tree, and raw message bytes must equal the
  authorized expected OID, candidate tree, and message. Any unexpected
  commit-time mutation blocks review and push-command presentation; rollback or a new authorization
  requires explicit user direction.
- The complete ref/reflog namespace must show only the exact authorized target
  base-to-new direct branch update and exactly one unconditional append in both
  its branch reflog and the symbolic worktree `HEAD` reflog. Enumerate actual
  files recursively beneath common-dir `logs`
  and every `worktrees/*/logs` directory so orphan reflogs are included. Each
  append is byte-equal to the expected base/new OIDs, authorized name/email,
  exact committer date with numeric timezone, and reason. Retain the supervisor's config/routing/namespace-root
  immutability boundary across the trusted no-follow identity-checked walk and result
  publication, then revalidate the `files` backend before releasing it.
- The coordinator constructs the post-commit review context only at its
  authorization-reserved absent path and publishes the reserved result manifest
  once with no-clobber semantics, binding its path and identity, exact bytes and
  hash, authorization-manifest hash, and expected/resulting equal commit OID.
- The coordinator records the full resulting commit SHA and commit evidence in
  external operational state.

The `COMMIT` transition never pushes, merges, publishes, or opens a pull request;
Superplanner has no later push-execution route either.
If Git is unavailable, authorization is absent, verification or documentation is
stale, or exact commit scope is ambiguous, return `status: blocked` without
committing. Standard review requires the resulting committed SHA; an
uncommitted candidate is not reviewable.

## Naming And Path Rules

- Use lowercase ASCII kebab-case for `<initiative-slug>`, `<feature-slug>`, and
  `<task-slug>`.
- Use three-digit, zero-padded feature and task IDs such as `F001` and `T001`.
- Keep an assigned ID stable; do not renumber existing artifacts to close gaps.
- Use `F<nnn>-<feature-slug>` for the feature directory.
- Use `T<nnn>-<task-slug>.md`, `T<nnn>-<task-slug>.feature`, and
  `T<nnn>-<task-slug>-plan.md` for a task's three artifacts.
- Use the exact uppercase filename `FEATURE.md` and exact root filenames
  `design.md` and `design.html`. `STATE.md` uses the external path contract
  above and is not an initiative-root candidate artifact.
- Use candidate-workspace-root-relative paths for candidate artifacts in state
  and handoffs, absolute paths for worktrees and external operational files,
  and relative links between candidate artifacts so the initiative directory
  remains portable.
- Keep a task's Markdown, acceptance, and plan basenames aligned. A rename must
  update all candidate links and external `STATE.md` in the same orchestrator
  operation.

## Authoritative Sources

Authority is scoped by concern; a narrower artifact cannot silently override a
broader approved rule.

1. The approved `design.md` is authoritative for initiative intent, scope,
   constraints, and architecture. `design.html` is only its visual companion;
   regenerate HTML whenever they disagree.
2. `FEATURE.md` is authoritative for the feature outcome and feature boundary,
   subject to the approved design.
3. The task Markdown is authoritative for task scope, dependencies, ownership,
   and expected inputs and outputs, subject to its feature and design.
4. The separate `.feature` file is authoritative for the task's observable
   acceptance behavior. It may refine, but not contradict, approved product
   rules.
5. The implementation plan is authoritative for the approved execution steps,
   exact files, commands, and interfaces. It cannot redefine scope or acceptance
   behavior.
6. The integrated repository, tests, configuration, and observed command output
   are authoritative for current implemented behavior. Documentation must
   describe this state rather than planned behavior.
7. External `STATE.md` is authoritative for workflow progress, assignments,
   checkpoints, evidence, and the next action. It is operational metadata, not
   candidate content or a product-requirements source.
8. Review approval applies only to the recorded Git commit SHA. A working-tree
   description or approval for another SHA is not authoritative for the current
   candidate.

When sources conflict, stop the dependent work, record the conflict in
external `STATE.md`, and return it to the orchestrator. Product behavior is
clarified by the user and then changed in the highest applicable authoritative
artifact before downstream artifacts are synchronized.
