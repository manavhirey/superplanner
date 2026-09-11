# T<task-number> Implementation Plan: <task-title>

Replace every angle-bracket field with task-specific content before plan
approval. An approved plan contains no unresolved placeholders or product
decisions.

## Sources

- Approved design: `../../../design.md` at content ID `<design-content-id>`
- Approved feature: `../FEATURE.md` at content ID `<feature-content-id>`
- Approved task: `../tasks/T<task-number>-<task-slug>.md` at content ID `<task-content-id>`
- Approved acceptance: `../acceptance/T<task-number>-<task-slug>.feature` at content ID `<gherkin-content-id>`
- Repository baseline SHA: `<full-git-sha-or-none-for-non-git-planning>`

## Plan Approval

<!-- superplanner-approval:start -->
- status: `pending`
- approver: `none`
- approved_at: `none`
- approval_evidence: `none`
- content_id: `sha256:<64 lowercase hex>`
<!-- superplanner-approval:end -->

Implementation is blocked unless `status` is `approved` for the current
`content_id`. Approval recording may change only bytes inside the markers. Any
outside-byte change changes `content_id` and invalidates approval. `approved`
requires non-`none` approver identity and explicit evidence plus a valid
ISO-8601 time, all bound to this exact content ID.

## Outcome

`<Restate the task's single reviewable outcome without expanding scope.>`

## Exact Files

| Action | Exact workspace-root-relative path | Purpose |
| --- | --- | --- |
| `<add-modify-or-delete>` | `<exact/path>` | `<specific change tied to acceptance>` |

## Interfaces

| Interface | Current contract | Planned contract | Callers or consumers |
| --- | --- | --- | --- |
| `<exact-name-or-signature>` | `<current-input-output-and-failure-behavior>` | `<planned-input-output-and-failure-behavior>` | `<exact-callers-or-consumers>` |

## Scenario TDD Cycles

Repeat the complete cycle below once for every scenario in the separate
acceptance `.feature` file, in scenario order. Do not combine scenarios or omit
a cycle.

### Scenario <n>: <exact-scenario-name>

#### 1. Add The Failing Test

- Acceptance scenario: `<exact-scenario-name>`
- Test file: `<exact/test/path>`
- Test case: `<exact-test-name>`
- Setup and input: `<fixtures-preconditions-and-input>`
- Assertion: `<observable expected behavior from this acceptance scenario>`

#### 2. Request Coordinator Evidence Of The Expected Failure

- Command: `<exact-focused-test-command>`
- Expected failure: `<exact assertion error or missing behavior proving the test is effective>`
- Resume rule: `worker remains blocked until the coordinator supplies exact command evidence`
- Unexpected result handling: `<condition requiring correction of the test or plan before continuing>`

#### 3. Implement The Minimum Change

- Files and edits: `<exact minimal edits by path for this scenario>`
- Interface implementation: `<exact signature and behavior>`
- Scope guard: `<how the change avoids excluded behavior>`
- No additional refactor: `<adjacent code explicitly left unchanged>`

#### 4. Request Coordinator Evidence Of The Passing Test

- Command: `<same exact focused-test-command>`
- Expected result: `<test count or exact success condition>`

## Request Coordinator Verification After All Scenario Cycles

| Check | Exact command | Expected result |
| --- | --- | --- |
| `<task-specific-check>` | `<exact-command>` | `<observable-success-result>` |
| `<full-relevant-suite>` | `<exact-command>` | `<observable-success-result>` |

If a check fails unexpectedly, invoke root-cause debugging and update the plan
only after the authoritative artifacts support the change.

## Synchronize Documentation

- Documentation impact: `<yes-or-no>`
- Exact paths: `<documentation-paths-or-none-with-reason>`
- Required content: `<implemented behavior examples configuration or commands>`
- Verification command or inspection: `<exact-command-or-method-and-expected-result>`

Documentation must describe the implemented repository state. Documentation
synchronization runs unconditionally again before final review.

## Refresh Verification After Documentation

- Trigger: `<documentation-changed-candidate-bytes-or-no-change>`
- Affected task checks: `<exact-commands-and-expected-results>`
- Full relevant suite: `<exact-command-and-expected-result>`
- Required evidence: `<verification-and-documentation-bind-to-same-candidate-bytes>`

When documentation changes any candidate byte, rerun the affected task checks
and full relevant suite before commit authorization. If no byte changed, record
that evidence and retain the current verification.

## Inspect And Handoff

- Inspect commands: `<complete-candidate-attribute-and-all-untracked-including-ignored-checks-then-safe-status-and-no-ext-diff>`
- Confirm owned paths: `<exact-file-list-or-globs>`
- Persisted handoff path: `<coordinator-owned-absolute-external-path>`
- Required evidence: `<changed-files-test-output-docs-and-blockers>`

## User-Authorized Commit Gate: Coordinator `COMMIT`

- Git precondition: `a user-created Git repository exists at COMMIT entry`
- Ref-storage precondition: `trusted non-Git parsing validates the repository/object/files-backend tuple and common-directory shallow-path absence from identity-checked snapshots and the supervisor keeps that config/path state immutable through each Git call`
- Verification precondition: `implementation and affected documentation are verified before authorization`
- Planned gate state: `not-authorized`
- Preparation step: `coordinator-only non-mutating COMMIT_PREPARE creates and attests the base-anchored commit context and all displayed authorization evidence`
- Authorization requirement: `explicit user authorization after verification and documentation binding the patch, tree, message, base, target, exact author/committer dates, and expected commit OID`
- Runtime authorization destination: `external STATE.md`
- Canonical patch profile: `binary, full-index, no-color, no-renames, no-ext-diff, no-textconv, fixed a/ and b/ prefixes`
- Runtime full base commit/sole parent and target state: `<full-base-sha-and-full-target-ref-at-base>`
- Runtime complete refs/reflogs snapshots: `<external-state-evidence-paths-and-sha256>`
- Runtime base/candidate trees and patch path/hash: `<external-state-fields-populated-after-verified-implementation>`
- Runtime message file/hash: `<external-state-fields-populated-before-commit-authorization>`
- Runtime explicit commit/reflog identity, authorization-bound committer date, and reason: `<author-committer-reflog-name-email-exact-numeric-timezone-date-and-reason>`
- Runtime exact author/committer dates and precomputed commit OID: `<unix-seconds-timezones-full-oid-independent-no-write-hash-check-and-presented-authorization-binding>`
- Exact in-scope changed files: `<candidate-relative-paths>`
- Base/candidate-union tree/worktree attributes, `.gitattributes` bytes, exact stage-0 index manifest/flags, all-untracked, status, and diff inspection: `<exact-safe_git-commands-and-expected-scope>`
- Content-control evidence: `<phase-and-anchor-private-fixed-config-manifest-plus-per-call-class-index-object-route-and-terminal-result-proving-repository-filter-config-is-not-consumed>`
- Authorized-call evidence: `<persisted-operation-and-identity-hash-attested-terminal-result-manifest-for-every-authorized-call>`
- Stage command: `<safe_git-authorized-add-all-force-with-nul-authorized-path-file-and-content-control-route>`
- Staged patch/tree comparison: `<exact-regeneration-byte-hash-and-tree-checks>`
- Commit-object command: `<safe_git-authorized-commit-tree-with-explicit-parent-message-and-exact-identity-date-environment>`
- Commit-object verification: `<exact-ordered-tree-parent-author-committer-header-only-and-raw-message-checks>`
- Target update command: `<safe_git-authorized-one-ref-request-with-mediator-prepared-no-deref-lock-type-oid-compare-and-swap>`
- Ref/reflog verification: `<complete-namespace-one-direct-ref-and-byte-exact-unconditional-branch-plus-symbolic-worktree-HEAD-reflog-append-check-with-old-new-oids-name-email-unix-seconds-numeric-timezone-reason-authorization-bound-git-committer-date-and-create-reflog-preserved-on-the-stdin-transaction>`
- Review-context publication: `<authorization-reserved-absent-context-and-result-paths-plus-no-clobber-identity-hash-bound-post-commit-result-manifest>`
- Commit message: `<repository-style-commit-message>`
- Committed tree check: `<exact-command-proving-committed-bytes-match-verified-candidate>`
- Evidence binding: `<verification-and-documentation-bound-to-full-resulting-sha>`
- Expected result: `<full-new-commit-sha-equal-to-authorized-expected-oid-and-no-unreviewed-candidate-changes>`

Run non-mutating `COMMIT_PREPARE` only after verified implementation and
documentation. Enter mutating `COMMIT` only
when the user explicitly authorizes the canonical patch hash, candidate tree,
message hash, full base commit, full target ref at the base OID, exact author and
committer dates, and expected commit OID. Plan approval is not commit
authorization. Only the user-facing coordinator stages and commits the exact
authorized in-scope files; never include
unrelated changes or external operational state. If Git is absent, do not run
`git init`; the user must create the repository. If authorization is absent,
report that no commit was made. Record runtime authorization and commit evidence
only in external state; do not edit this approved plan to record them, because
any outside-marker edit changes its content ID and invalidates plan approval.
Never commit automatically, never execute a push, and never include a push in
`COMMIT`. Standard review starts only with the full committed SHA. After current
verification and documentation, both reviews approving that same SHA, no
unreviewed candidate changes, and a target-bound explicit user presentation
request, the push-command gate may only use the local `push-present` output guard
to release the exact registered non-force command bytes from `quality-gates.md`.
Superplanner never executes, brokers, retries, observes, or reports the push.
