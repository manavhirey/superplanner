# Superplanner State: <initiative-name>

Only the orchestrator edits this external operational file. It must never be
stored under, tracked by, or included in a diff from the candidate workspace.

## State Location

- State path: `<absolute-state-path>`
- Storage mode: `<git-common-dir|explicit-non-git-external>`
- Candidate root: `<absolute-candidate-root>`
- Resolved Git common directory: `<absolute-git-common-dir-or-not-applicable>`
- Git common-directory bootstrap: `<absolute-form-operation-result-or-unsupported-plus-fallback-operation-result-resolved-relative-to-candidate-worktree>`
- Worker artifact root: `<explicit-absolute-path-outside-candidate-and-all-git-directories>`
- Worker artifact root validation: `<yes-or-no-and-evidence>`
- Non-Git configuration evidence: `<explicit-path-configuration-or-not-applicable>`
- Excluded from candidate status, diff, and SHA: `<yes-or-no-and-evidence>`
- Git repository present: `<yes-or-no>`
- Git repository creation evidence: `<user-created-evidence-or-not-applicable>`
- Source/mirror repository-format/object-format/`files` ref-backend config-snapshot identity, SHA-256, tuple-parser, shallow-path absence, namespace-reconstruction, and invocation-lifetime immutability evidence: `<evidence-or-not-applicable>`
- Private fixed-config content Git directory manifests: `<structured-context-phase-anchor-route-records-or-not-applicable>`

For Git, `State path` must equal
`<git-common-dir>/superplanner/<initiative-slug>/STATE.md`. For non-Git, it must
be explicitly configured outside `Candidate root`. Non-Git state supports
discovery, design, and planning. Retained `EXECUTE`, coordinator `COMMIT`, SHA-bound
review, and push-command eligibility require a Git repository created by the user. Superplanner
never runs `git init`. Coordinator-persisted checkpoints, packaging
indices/object directories, scratch storage, and bundles descend from `Worker
artifact root`, never from the state directory or any Git directory. Validated
handoffs and review records may be stored beside this external state file.
Workers emit checkpoint/handoff content through supervisor results and cannot write
either location.

## Initiative

- Initiative slug: `<initiative-slug>`
- Current phase: `<phase-number-and-name>`
- Orchestrator ID: `<agent-id>`
- Updated at: `<iso-8601-timestamp>`
- Design path: `docs/superplanner/<initiative-slug>/design.md`
- Candidate Git SHA: `<git-sha-or-no-candidate>`

## Artifact Approvals

Every row mirrors the durable approval record in the candidate artifact. For
Markdown, remove the exact complete `<!-- superplanner-approval:start -->` and
`<!-- superplanner-approval:end -->` lines and every enclosed byte. For Gherkin,
do the same with `# superplanner-approval:start` and
`# superplanner-approval:end`. Remove each complete marker line including its
line-ending bytes when present. Preserve every remaining exact UTF-8 byte
without normalization, SHA-256 hash it, and encode
`sha256:<64 lowercase hex>`. Missing, multiple, or misordered markers block
approval. Approval updates may change only bytes inside the one existing marker
pair. Workflow lifecycle is operational and exists only in this external state.
`pending` requires `none` for approver, time, and evidence. `approved` requires
a non-`none` approver, valid ISO-8601 time, and explicit user-message evidence
bound to the displayed artifact and exact recomputed content ID.

| Artifact type | Artifact ID | Candidate-relative path | Marker validation | status | approver | approved_at | approval_evidence | content_id | Recomputed ID matches |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `design` | `<initiative-slug>` | `docs/superplanner/<initiative-slug>/design.md` | `<exactly-one-ordered-pair-or-blocked>` | `<pending|approved>` | `<identity-or-none>` | `<iso-8601-or-none>` | `<record-reference-or-none>` | `sha256:<64 lowercase hex>` | `<yes-or-no-and-evidence>` |
| `feature` | `F<feature-number>` | `<feature-path>` | `<exactly-one-ordered-pair-or-blocked>` | `<pending|approved>` | `<identity-or-none>` | `<iso-8601-or-none>` | `<record-reference-or-none>` | `sha256:<64 lowercase hex>` | `<yes-or-no-and-evidence>` |
| `task` | `T<task-number>` | `<task-path>` | `<exactly-one-ordered-pair-or-blocked>` | `<pending|approved>` | `<identity-or-none>` | `<iso-8601-or-none>` | `<record-reference-or-none>` | `sha256:<64 lowercase hex>` | `<yes-or-no-and-evidence>` |
| `gherkin` | `T<task-number>` | `<acceptance-feature-path>` | `<exactly-one-ordered-pair-or-blocked>` | `<pending|approved>` | `<identity-or-none>` | `<iso-8601-or-none>` | `<record-reference-or-none>` | `sha256:<64 lowercase hex>` | `<yes-or-no-and-evidence>` |
| `plan` | `T<task-number>` | `<implementation-plan-path>` | `<exactly-one-ordered-pair-or-blocked>` | `<pending|approved>` | `<identity-or-none>` | `<iso-8601-or-none>` | `<record-reference-or-none>` | `sha256:<64 lowercase hex>` | `<yes-or-no-and-evidence>` |

## Bounded Brief Approval

Use this section only for the quick bounded route; otherwise record
`not-applicable`. The exact approved brief bytes are inside one ordered marker pair.
Hash every byte between the complete marker lines, excluding both marker lines
and their line endings, without any normalization. The result is lowercase
`sha256:<64 lowercase hex>`. Missing, duplicate, nested, or misordered markers
block approval.

<!-- superplanner-bounded-brief:start -->
### Outcome
<exact outcome>

### Current Behavior
<exact current behavior>

### Proposed Behavior
<exact proposed behavior>

### Scope
<exact in-scope files and behavior>

### Exclusions
<exact excluded files and behavior>

### Acceptance Criteria
<observable criteria>

### Tests
<exact verification>

### Documentation Impact
<exact impact or none with reason>

### Material Risks
<risks or none with reason>
<!-- superplanner-bounded-brief:end -->

- Marker validation: `<exactly-one-ordered-pair-or-blocked>`
- status: `pending`
- approver: `none`
- approved_at: `none`
- approval_evidence: `none`
- content_id: `sha256:<64 lowercase hex>`
- recomputed_content_id_matches: `<yes-or-no-or-not-applicable>`

The coordinator presents the exact marked brief bytes to the user before
approval. Any byte change inside the markers changes the content ID and makes
the prior approval stale. Changes elsewhere in operational state do not change
the bounded brief content ID. Approved status requires a non-`none` approver,
valid ISO-8601 time, and explicit user-message evidence for those exact bytes
and content ID.

## Exact Next Action

- Owner: `<orchestrator-user-or-agent-id>`
- Action: `<one concrete operation or decision>`
- Inputs: `<artifact-paths-task-ids-or-none>`
- Preconditions: `<required-state-or-none>`
- Expected evidence: `<result-and-recording-path>`

## Features

| Feature ID | Path | Workflow lifecycle | Outcome evidence | Blocker |
| --- | --- | --- | --- | --- |
| `F<feature-number>` | `docs/superplanner/<initiative-slug>/features/F<feature-number>-<feature-slug>/FEATURE.md` | `<proposed|in-progress|blocked|complete>` | `<path-command-result-or-none>` | `<blocker-or-none>` |

## Tasks

| Task ID | Feature ID | Task path | Acceptance path | Plan path | Dependencies | Ownership domain | Workflow lifecycle |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `T<task-number>` | `F<feature-number>` | `<task-path>` | `<acceptance-path>` | `<plan-path>` | `<task-ids-or-none>` | `<domain-id>` | `<proposed|in-progress|blocked|complete>` |

## Dependency And Ownership

- Matrix location: `<embedded-below-or-absolute-external-operational-path>`
- Required schema validated: `<yes-or-no-and-evidence>`

If a separate matrix is used, the referenced external file must contain every
column below; a path alone without schema validation is not valid state.

| Domain ID | Task IDs | Depends on | Consumes output from | Expected files | Shared state | Exclusive resources | Compared with | Verdict | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<domain-id>` | `<task-ids>` | `<domain-ids-or-none>` | `<outputs-or-none>` | `<exact-paths-or-globs>` | `<state-or-none>` | `<resources-or-none>` | `<domain-ids-or-none>` | `<parallel-or-serialize>` | `<evidence-based-reason>` |

## Worktrees

| Domain ID | Worktree path | Detached SHA | Setup objects/alternates | Materialization index/objects/alternates | Source/setup context | Active content-context record | Tuple-route manifests | Per-call operation/result manifests | Dispatch root evidence | Baseline command and result | Integration status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<domain-id>` | `<absolute-worktree-path>` | `<full-sha>` | `<absolute-per-domain-setup-object-directory-and-exact-git-encoded-baseline-list>` | `<absolute-index-and-object-paths-and-git-encoded-list>` | `<source-setup-content-context-id-path-and-sha256>` | `<content-context-id-or-none>` | `<canonical-route-paths-and-sha256>` | `<operation-and-terminal-result-paths-sha256-policy-effective-git-dir-class-path-manifest-descriptors-operands-status-output-hash-per-call>` | `<harness-proof-or-blocked>` | `<exact-command-and-result>` | `<not-started-active-ready-integrated-or-blocked>` |

## Content Git Contexts

| Context ID | Domain/review context | Phase | Worktree | Real Git directory | Content Git directory | Config/HEAD hashes | Repository/object/ref format | Anchor OID | Allowed index/object/alternate routes | Validation result | Lifecycle |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<content-context-id>` | `<domain-id-or-review-sha>` | `<source/setup|integration|commit|review>` | `<canonical-worktree>` | `<canonical-real-git-dir>` | `<canonical-private-content-git-dir>` | `<sha256-values>` | `<version-sha1-or-sha256-files>` | `<full-oid>` | `<canonical-routes-identities-and-transitive-alternate-absence-evidence>` | `<format-head-absence-and-read-only-evidence>` | `<active|retired|blocked>` |

## Mechanical Integration

- Base SHA: `<full-commit-oid>`
- Candidate alternate index: `<absolute-external-path>`
- Candidate external object directory: `<absolute-external-path>`
- Baseline bare mirror/object directory: `<absolute-external-paths>`
- Baseline mirror construction manifest: `<exact-env-argv-source-mirror-identities-source-and-mirror-shallow-absence-clone-namespace-reconstruction-result-and-manifest-path-device-inode-sha256>`
- Baseline mirror no-hardlink proof: `<complete-source-object-to-mirror-identity-scan-manifest-path-device-inode-sha256-and-verdict>`
- Setup/packaging namespace walker manifests: `<pre-post-refs-logs-and-worktree-logs-root-path-identities-verdicts-plus-packed-refs-leaf-path-identity-verdict-and-sha256>`
- Integration scratch root: `<absolute-external-path>`
- Empty safe-hooks directory: `<absolute-external-path>`
- Trusted Git executable: `<canonical-path-device-inode-and-sha256>`
- Trusted env executable: `<canonical-path-device-inode-and-sha256>`
- Trusted transaction-supervisor executable: `<canonical-path-device-inode-and-sha256>`
- Transaction-supervisor source SHA-256: `<sha256-of-exact-source-bytes>`
- Mediator pre-interpreter launch: `<absolute-env-i-bash-argv-environment-and-sha256-evidence>`
- Canonical repository object directory: `<absolute-git-common-object-path>`
- Current candidate tree: `<full-tree-oid>`
- Real index remains at base tree: `<yes-or-no-and-evidence>`
- Object-route transitive alternates absence: `<all-routed-object-directories-and-members-info-alternates-http-alternates-path-identities-and-verdict>`

| Domain ID | Packaging index/objects | Start tree | Packaged tree | Pre-tree | Post-tree | Tuple-route manifests | Per-call operation/result manifests | Owned/patch/manifest immediate rehashes | Result |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<domain-id>` | `<absolute-external-paths>` | `<full-tree-oid>` | `<full-tree-oid>` | `<full-tree-oid>` | `<full-tree-oid>` | `<canonical-paths-and-sha256>` | `<operation-and-terminal-result-paths-sha256-policy-effective-git-dir-class-path-manifest-descriptors-operands-status-output-hash>` | `<three-sha256-values-before-each-consumer-and-after-apply>` | `<integrated-or-blocked-with-evidence>` |

Each row follows `references/integration-protocol.md`. External bundle paths,
replay checks, exact object format, and any blocker are recorded in the matching
invocation/checkpoint evidence.

## Invocations And Checkpoints

| Dispatch ID | Workflow task ID | `opencode_session_id` | Source agent ID/hash | Runtime wrapper name/hash | Allowed name/mode delta | Model/variant | Brief path/hash | Domain | Scope | Milestones | Step/execution budget | Stop conditions | Timeout support | Checkpoint path | Invocation state | Handoff status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<dispatch-id>` | `<stable-workflow-task-id>` | `<generated-opencode-session-id-or-none>` | `<canonical-agent-id-and-sha256>` | `<runtime-primary-name-and-sha256>` | `<exact-name-and-subagent-to-primary-delta>` | `<canonical-model-and-variant>` | `<absolute-external-brief-path-and-sha256>` | `<domain-id>` | `<bounded-scope>` | `<ordered-milestones>` | `<step-budget-and-execution-budget-or-none>` | `<exact-stop-conditions>` | `<unsupported-or-supervisor-timeout>` | `<absolute-external-path-or-none>` | `<not-started|running|ended|timed-out|cancellation-confirmed>` | `<complete|blocked|none-yet>` |

## Harness Capabilities

- Live status checks: `<supported|unsupported>`
- Cancellation: `<supported|unsupported>`
- Isolation-supervisor spawn/result bridge: `<supported|unsupported>`
- Coordinator launcher and supervisor-adapter path/SHA-256 plus private startup/resolved-path manifest: `<absolute-path-hash-pairs-and-attestation-or-none>`
- Coordinator credential-identity/alias and Git-metadata projection plus no-lazy/context-pinned safe-Git mediator: `<manifest-path-hash-and-attestation-or-none>`
- Phase-scoped external-mount manifest: `<absolute-path-sha256-and-mounted-state-artifact-verdict-or-none>`
- Target-repository instruction manifest: `<canonical-applicable-path-sha256-pairs-and-embedded-verdict>`
- Private-root re-rooted process launch: `<supported|unsupported>`
- Credential-isolating inference broker: `<supported|unsupported>`
- Trusted provider implementation/version/hash and sole broker endpoint: `<attested-values-or-none>`
- OpenCode session-ID resume: `<supported|unsupported>`
- Process security-principal isolation: `<distinct-principal|kernel-enforced-same-principal|blocked>`
- Principal-isolation verdict/evidence: `<filesystem-process-inspection-and-control-inherited-descriptors-privilege-escalation-attestation>`
- Capability evidence: `<harness-documentation-or-observed-interface>`

Do not poll or stop a foreground invocation. Use live checks or cancellation
only when the matching capability is `supported`.

## Invocation Runtime Envelopes

| Dispatch ID | Private HOME and config/data/cache/state/temp roots | Startup variable/resolved-path manifest | Approval state | Provider/tool provenance | Resolved permission SHA-256 | Skill manifest paths and SHA-256 values | Observed session agent | Credential identity/alias and Git-metadata mask | Reviewer Git mediator path/hash/no-replace/no-lazy/context evidence | Inference-broker evidence | Security-principal isolation evidence | Process sandbox evidence | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<dispatch-id>` | `<six-fresh-absolute-paths>` | `<XDG-vars-TMPDIR-pinned-OPENCODE_CONFIG-DB-OPENCODE_DISABLE_PROJECT_CONFIG=1-OPENCODE_DISABLE_EXTERNAL_SKILLS=1-OPENCODE_PURE=1-OPENCODE_DISABLE_DEFAULT_PLUGINS=1-OPENCODE_DISABLE_MODELS_FETCH=1-scrubbed-overrides-and-resolved-paths>` | `<empty-private-instance>` | `<trusted-provider-manifest-and-stock-built-ins-only-no-duplicates>` | `<sha256-of-resolved-permission-manifest>` | `<all-absolute-external-path-and-sha256-pairs-or-none>` | `<runtime-wrapper-name-hash-and-observed-session-evidence>` | `<canonical-path-file-identity-all-alias-manifest-path-hash-and-masked-verdict>` | `<attested-client-broker-Git-GIT_NO_REPLACE_OBJECTS-GIT_NO_LAZY_FETCH-pinned-context-or-not-applicable>` | `<credential-isolated-canonical-route-and-sole-endpoint>` | `<distinct-principal-or-kernel-enforced-same-principal-filesystem-process-control-descriptor-privilege-verdict-and-evidence>` | `<active-before-startup-and-first-tool>` | `<ready-or-blocked>` |

Each writer row must satisfy the complete envelope in
`references/handoff-contract.md`. Record the selected skill's canonical trusted
path and complete file-manifest hash, not only its discovered name. A reviewer
row records its separate read-only envelope when applicable.
- Reviewer commit-derived projection manifests: `<dispatch-id-reviewed-sha-absolute-manifest-path-sha256-tree-equality-credential-omission-no-extra-or-untracked-entries-and-immutability-evidence>`

## Monitoring And Recovery Events

| Time | Dispatch ID | Workflow task ID | `opencode_session_id` | Agent ID | Event | Progress and evidence | Step-budget state | Action | Next action |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<iso-8601>` | `<dispatch-id>` | `<stable-workflow-task-id>` | `<generated-opencode-session-id-or-none>` | `<agent-id>` | `<dispatch|checkpoint|handoff|timeout|resume|live-check|cancel-requested|cancellation-confirmed|invocation-ended|replacement|escalated>` | `<observed-progress-and-absolute-operational-path>` | `<remaining-or-exhausted>` | `<orchestrator-action>` | `<exact-follow-up>` |

`Workflow task ID` is the stable identifier in the canonical handoff. Capture
`OpenCode session ID` from the `superplanner_supervisor` result; only that generated
value may be supplied to the matching resume interface. If none was returned, record
resume as unavailable and do not guess.

For replacement, cite the original invocation's `invocation-ended` or
`cancellation-confirmed` event and record the replacement agent ID:
`<event-reference-and-agent-id>`.

## Verification Evidence

| Candidate state or committed SHA | Scope | Command | Result | Evidence | Current |
| --- | --- | --- | --- | --- | --- |
| `<current-head-and-pre-commit-diff-or-full-git-sha>` | `<task-ids-or-full-relevant-suite>` | `<exact-command>` | `<passed-failed-or-not-run>` | `<observed-result-or-path>` | `<yes-or-no-with-reason>` |

## Documentation Synchronization

- Impacted tasks: `<task-ids-or-none>`
- Candidate state or committed SHA: `<current-head-and-pre-commit-diff-or-full-git-sha>`
- Updated paths: `<paths-or-none-with-reason>`
- Verification evidence: `<commands-inspections-and-results>`
- Status: `<current-stale-or-blocked>`

## Coordinator COMMIT Stage

- Existing user-created Git repository: `<yes-or-no>`
- Revalidated source repository/object/`files`-backend tuple, shallow-path absence, and invocation-lifetime config/path immutability: `<yes-or-no-and-evidence>`
- Verification and documentation current: `<yes-or-no-and-evidence>`
- Non-mutating COMMIT_PREPARE context/evidence: `<base-anchored-content-context-and-complete-preparation-evidence-or-not-run>`
- Explicit user commit authorization: `<message-reference-binding-patch-tree-message-base-target-dates-and-expected-oid-or-not-authorized>`
- Authorization manifest path and SHA-256: `<absolute-external-path-and-sha256-or-not-authorized>`
- Git storage read-only/unlock/relock, no-clobber identity-checked authorized output/scratch leaves, scrubbed and hash-attested transaction supervisor, and registered descriptor evidence: `<exact-attestation-or-not-run>`
- Authorized task or scope: `<task-ids-and-scope>`
- Authorized full base commit/sole parent: `<full-commit-oid-or-not-authorized>`
- Authorized base tree: `<full-tree-oid-or-not-authorized>`
- Authorized candidate tree: `<full-tree-oid-or-not-authorized>`
- Authorized target state: `<direct-full-target-ref-at-base-oid-with-empty-symref-field>`
- Required pre/post `HEAD` attachment invariant: `<exact-symbolic-text>`
- Pre-authorization complete refs/reflogs snapshot: `<absolute-evidence-paths-sha256-exact-refs-logs-and-worktree-logs-root-identities-walk-verdicts-plus-packed-refs-leaf-identity-and-verdict>`
- Presented commit/reflog identity: `<author-committer-and-reflog-name-email>`
- Authorized author/committer dates, with the committer date also binding both reflog appends: `<unix-seconds-and-numeric-timezones>`
- Precomputed expected commit OID and independent no-write hash check: `<full-oid-and-evidence>`
- Expected commit-byte input and no-write hash-object operation/result manifests: `<paths-identities-sha256-route-policy-status-and-output-hash>`
- Every authorized call operation/result manifest: `<ordered-call-id-operation-path-identity-sha256-token-state-terminal-result-path-identity-sha256-status-and-output-hash>`
- Authorized exact reflog reason: `<utf-8-no-newline>`
- Canonical authorization patch and SHA-256: `<absolute-external-path-and-sha256>`
- Authorized message file and SHA-256: `<absolute-external-path-and-sha256>`
- Authorized exact message bytes: `<utf-8-verbatim-one-terminal-newline-evidence>`
- Exact in-scope committed files: `<candidate-relative-paths-or-none>`
- Status and diff inspection: `<exact-commands-and-results>`
- Complete base/candidate-union tree and worktree attribute rejection: `<yes-or-no-and-evidence>`
- Content commands used the authorization-bound fixed-config Git directory and per-call explicit index/object/alternate routes and could not load repository filter drivers: `<yes-or-no-and-evidence>`
- Candidate `.gitattributes` byte equality and candidate-absent paths absent: `<yes-or-no-and-evidence>`
- Real stage-0 index manifest equals the required tree with no intent-to-add entry, and all `ls-files -v -z` records are ordinary uppercase H: `<yes-or-no-and-evidence>`
- All untracked paths, including ignored paths, authorized or absent: `<yes-or-no-and-evidence>`
- Verified staged tree: `<full-tree-oid-or-not-run>`
- Regenerated patch equals authorized bytes/hash: `<yes-or-no-and-evidence>`
- Stage and commit commands: `<exact-non-interactive-commands-or-not-run>`
- Exact authorized author/committer environment: `<GIT_AUTHOR-and-GIT_COMMITTER-name-email-date-values-or-not-run>`
- Full resulting commit SHA: `<full-git-sha-or-none>`
- Resulting commit SHA equals authorized expected OID: `<yes-or-no-and-evidence>`
- Resulting sole parent equals authorized base: `<yes-or-no-and-evidence>`
- Resulting commit tree: `<full-tree-oid-or-none>`
- Raw commit headers are exactly tree, sole parent, author, and committer in order: `<yes-or-no-and-evidence>`
- Resulting tree equals authorized candidate/staged trees: `<yes-or-no-and-evidence>`
- Resulting commit message SHA-256: `<sha256-or-none>`
- Resulting message equals authorized bytes: `<yes-or-no-and-evidence>`
- Target compare-and-swap: `<explicit-full-ref-request-and-prepared-no-deref-transaction-lock-type-oid-commit-evidence>`
- Transaction transcript: `<absolute-path-device-inode-sha256-run-flag-raw-supervisor-status-and-final-mediator-status-or-not-run>`
- Prepared transaction response: `<exact-prepare-ok-evidence-or-not-run>`
- One-shot authorization consumption: `<consumed-before-state-or-transaction-attempt-with-result>`
- Under-lock target check: `<direct-non-symbolic-full-ref-at-authorized-base-oid-evidence-or-not-run>`
- Final transaction response: `<exact-commit-ok-or-abort-evidence-or-not-run>`
- Post-commit complete refs/reflogs snapshot: `<absolute-evidence-paths-sha256-exact-refs-logs-and-worktree-logs-root-identities-walk-verdicts-plus-packed-refs-leaf-identity-and-verdict>`
- Complete refs/reflogs expected delta only: `<yes-or-no-direct-branch-old-new-plus-byte-exact-unconditional-branch-and-symbolic-worktree-head-appends-with-oids-identity-committer-date-timezone-reason-and-evidence>`
- Committed tree matches verified implementation/docs: `<yes-or-no-and-evidence>`
- Verification and documentation bound to resulting SHA: `<yes-or-no-and-evidence>`
- Authorization-reserved absent review-context/result-manifest paths: `<canonical-paths-parent-identities-and-absence-evidence>`
- Post-commit review-context result manifest linked to authorization hash and resulting SHA: `<no-clobber-absolute-path-device-inode-sha256-exact-bytes-authorization-hash-expected-resulting-oid-and-validation-evidence>`
- Superplanner push execution: `prohibited-not-run`

Preparation is non-mutating; do not enter mutating `COMMIT` without Git and
explicit authorization. Never run
`git init`; never include unrelated changes or external state; never execute a
push.

## Review Evidence

Every terminal review attempt, including a blocked and therefore
`not-completed` review, receives a row. Completed reviews use full immutable
commit SHAs, never a branch, `HEAD`, or an abbreviated SHA. A blocked attempt
uses `none` only for a SHA it could not establish and records the blocker in its
evidence.

| Review ID | Type | Agent ID | Model and variant | completion_time | status | review_result | reviewed_sha | Standard record ID/identity/SHA-256 | standard_approval_sha | Finding IDs | Accepted-risk finding IDs | Evidence path | Invalidated and reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<review-id>` | `standard` | `<agent-id>` | `<model-id-and-variant>` | `<iso-8601>` | `<complete|blocked>` | `<approved|findings|not-completed>` | `<full-git-sha-or-none-only-if-blocked>` | `none` | `none` | `<finding-ids-or-none>` | `<accepted-finding-ids-or-none>` | `<absolute-operational-handoff-or-report-path>` | `<no-or-yes-and-reason>` |
| `<review-id>` | `adversarial` | `<agent-id>` | `openrouter/moonshotai/kimi-k3 max` | `<iso-8601>` | `<complete|blocked>` | `<approved|findings|not-completed>` | `<full-git-sha-or-none-only-if-blocked>` | `<registered-standard-review-record-id-path-identity-and-recomputed-sha256-or-none-only-if-blocked>` | `<standard-approved-full-git-sha-or-none-only-if-blocked>` | `<finding-ids-or-none>` | `<accepted-finding-ids-or-none>` | `<absolute-operational-handoff-or-report-path>` | `<no-or-yes-and-reason>` |

External state updates do not invalidate review. Any candidate-file change does.

## Accepted Risks

Every accepted risk remains linked to a visible reviewer finding.

| Finding ID | Review ID | Exact reviewed SHA | Exact scope | Explicit user evidence | Consequence | Rationale |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `<finding-id>` | `<review-id>` | `<full-git-sha>` | `<accepted-scope>` | `<message-or-record-reference>` | `<accepted-consequence>` | `<acceptance-rationale>` |

When no accepted risk exists, record `none` and add no risk row.

## Push Command Gate

- Verification current: `<registered-immutable-result-id-identity-recomputed-hash-exact-commit-tree-binding-and-passing-status-or-no>`
- Documentation current: `<registered-immutable-synchronization-result-id-identity-recomputed-hash-exact-commit-tree-binding-and-passing-status-or-no>`
- Current SHA equals user-authorized coordinator `COMMIT` SHA: `<yes-or-no-and-evidence>`
- Commit tree equals authorized candidate/staged trees: `<yes-or-no-and-evidence>`
- Commit message equals authorized exact bytes: `<yes-or-no-and-evidence>`
- Standard review approves candidate SHA: `<yes-or-no>`
- Kimi adversarial review approves same SHA: `<yes-or-no>`
- Worktree has no unreviewed changes: `<supervisor-produced-immediate-clean-worktree-snapshot-id-identity-hash-index-tree-equality-and-no-tracked-untracked-or-ignored-difference-or-no>`
- Explicit user push request, authorizing presentation only: `<registered-immutable-push-presentation-request-v1-id-identity-recomputed-hash-authenticated-user-evidence-exact-commit-review-records-destination-refspec-and-no-force-bindings-or-not-requested>`
- Destination remote name and single closed-grammar HTTPS push URL: `<name-and-url-or-not-eligible>`
- Push destination cardinality/rewrite/grammar validation: `<exactly-one-no-insteadof-or-pushinsteadof-helper-free-https-no-userinfo-query-fragment-whitespace-control-or-del-or-not-eligible>`
- Full refspec: `<full-local-ref:full-remote-ref-or-not-eligible>`
- Expected full local OID and push-only source-ref equality: `<repository-format-oid-and-evidence-or-not-eligible>`
- No-force policy: `<leading-plus-and-all-force-options-forbidden-with-evidence>`
- Push-prepare source/evidence bindings: `<registered-immutable-source-context-object-route-authorization-commit-standard-review-adversarial-review-verification-documentation-presentation-request-and-destination-config-record-ids-service-resolved-identities-recomputed-hashes-validated-cross-links-clean-worktree-snapshot-and-registered-canonical-eligibility-manifest-id-and-hash-or-not-eligible>`
- Immutable push-only Git directory: `<registered-push-layout-v1-id-identity-recomputed-hash-separate-principal-custody-path-and-ancestry-identities-hashes-exact-head-config-one-loose-ref-loose-reachable-object-set-empty-info-and-pack-no-extra-entry-or-link-source-unavailable-strict-verification-or-not-eligible>`
- Execution custody lease: `<lease-id-and-active|retiring|cleanup-failed|retired-with-prepared-operation-atomic-rename-linearization-idempotent-one-use-release-consumption-and-cleanup-evidence-or-not-eligible>`
- User custody-release request: `<registered-immutable-push-custody-release-v1-trusted-user-decision-record-id-identity-recomputed-hash-bound-lease-and-eligibility-manifest-ids-literal-retire-action-no-execution-or-outcome-field-and-unused|consumed-state-or-not-requested>`
- Attested Git execution closure: `<registered-push-execution-closure-v1-id-identity-recomputed-hash-absolute-env-and-main-git-private-exec-path-exact-path-variable-closed-process-edges-and-executable-interpreter-loader-library-manifest-including-git-remote-https-and-pack-objects-or-not-eligible>`
- Execution-time config isolation and hooks disabled: `<env-i-path-equals-private-exec-path-null-system-global-config-exact-push-only-local-config-exact-sha1-or-sha256-environment-config-core-hooks-path-dev-null-safe-directory-equals-custody-path-and-http-follow-redirects-false-or-not-eligible>`
- Exact user-executed command record: `<registered-push-command-v1-id-identity-recomputed-hash-exact-environment-argv-posix-bytes-and-all-record-bindings-or-not-presented>`
- Guarded presentation result: `<registered-push-presentation-v1-id-identity-hash-final-revalidation-snapshot-and-byte-exact-output-guard-evidence-or-not-presented>`
- Superplanner push execution/brokering/retry/observation/reporting: `prohibited-not-run`
- Command presentation eligible: `<yes-or-no>`

## Resume Consistency

- State path is external and absent from candidate status and diff: `<yes-or-no-and-evidence>`
- Current phase is permitted by Git availability: `<yes-or-no-and-evidence>`
- Initiative and paths agree: `<yes-or-no-and-evidence>`
- Design candidate and external approvals match its recomputed ID before `DEFINE`: `<yes-or-no-and-evidence>`
- Design, feature, task, and Gherkin approval copies match recomputed IDs before `PLAN`: `<yes-or-no-and-evidence>`
- Complete design, feature, task, Gherkin, and plan approval chain matches recomputed and plan-source IDs before `EXECUTE`: `<yes-or-no-and-evidence>`
- Artifact links, IDs, marker pairs, and recomputed approval IDs agree: `<yes-or-no-and-evidence>`
- Matrix includes every required field and current verdict reasons: `<yes-or-no-and-evidence>`
- Repository and worktrees match recorded state: `<yes-or-no-and-evidence>`
- Source/mirror format/backend tuple, config immutability evidence, and every active phase/anchor content-context record remain exact: `<yes-or-no-and-evidence>`
- Baseline mirror construction/no-hardlink proof, every `setup_alternates` value, and every source/setup context remain exact: `<yes-or-no-and-evidence>`
- Every routed object directory and transitive alternate member retains both required on-disk alternate-file absences: `<yes-or-no-and-evidence>`
- Every safe-Git tuple-route manifest/hash remains current: `<yes-or-no-and-evidence>`
- Every distinct deserialized per-call operation manifest, selected real-or-content Git-directory class/path/identity/manifest hash and anchor, exact index/object/alternate route, primary-or-recovery published result path/hash/identity, successful final/stage shared-inode and link-count evidence, private result staging/final paths, route-policy hash, literal FD-role binding, device/inode descriptor identity, terminal status, and input/output hash remains current: `<yes-or-no-and-evidence>`
- Immediate pre-consumption and post-apply owned-path/patch/raw-manifest rehashes match: `<yes-or-no-and-evidence>`
- No running or unconfirmed-cancellation invocation overlaps scope: `<yes-or-no-and-evidence>`
- Every invocation retains a passing distinct-principal or kernel-enforced
  same-principal isolation verdict covering all four required attack surfaces:
  `<yes-or-no-and-evidence>`
- Checkpoints and canonical handoffs agree with invocation state: `<yes-or-no-and-evidence>`
- Verification and documentation bind to candidate: `<yes-or-no-and-evidence>`
- Commit authorization including exact dates/expected OID, exact files, and resulting full-SHA equality are valid before review: `<yes-or-no-and-evidence>`
- Every authorized call has its exact persisted operation and terminal result manifest: `<yes-or-no-and-evidence>`
- Authorization manifest/token, every bound file/output/scratch leaf and identity, transaction-supervisor executable/source hashes and transcript evidence, and caller-read-only Git storage plus unlock/relock evidence are current: `<yes-or-no-and-evidence>`
- Complete base/candidate-union tree/worktree attribute, `.gitattributes`, exact-index-manifest/flags, and all-untracked-path checks are valid: `<yes-or-no-and-evidence>`
- Authorized candidate/staged trees equal resulting commit tree before review: `<yes-or-no-and-evidence>`
- Authorized message bytes equal resulting commit message before review: `<yes-or-no-and-evidence>`
- Resulting sole parent equals the authorized full base commit: `<yes-or-no-and-evidence>`
- Resulting raw commit has exactly the authorized tree/parent/author/committer headers and no extra header: `<yes-or-no-and-evidence>`
- Authorized explicit-full-ref target CAS, `HEAD` invariant, complete direct-ref plus byte-exact unconditional branch/symbolic-worktree-`HEAD` reflog delta including the authorized committer date, and exact snapshot walker-root identities/verdicts remain valid: `<yes-or-no-and-evidence>`
- Authorization-reserved review-context/result paths and no-clobber result-manifest identity/hash remain valid: `<yes-or-no-and-evidence>`
- Prepared transaction has `prepare: ok`, a successful under-lock direct-ref/base-OID check, and final `commit: ok` evidence: `<yes-or-no-and-evidence>`
- Review completion times, exact SHAs, and invalidation flags are current: `<yes-or-no-and-evidence>`
- Accepted risks have all required fields and retain their findings: `<yes-or-no-and-evidence>`
- Push-command evidence binds service-resolved registered source/authorization/
  commit/review/verification/documentation/presentation-request/destination-config
  record identities, recomputed hashes, validated cross-links, immediate clean-
  worktree snapshot, canonical layout/execution-closure/command/eligibility
  record IDs, identities, and hashes,
  remote-name provenance, one closed-grammar HTTPS URL, no URL rewrites,
  exact config-isolated hookless redirect-disabled private-`PATH` `env -i` prefix,
  POSIX shell-quoted
  exec-path/push-only-Git-directory/URL/refspec, exact loose object/ref/config
  state, private executable/helper/loader/library closure, active
  separate-principal custody lease, target-bound request, guarded
  `push-presentation-v1`, expected local OID, push-only source-ref equality,
  no-force policy, and typed atomic-rename-linearized revocation, idempotent
  one-use release consumption, and cleanup states;
  Superplanner has no execution, brokering, retry, observation, or reporting route:
  `<yes-or-no-or-not-requested-and-evidence>`
- Exact next action remains safe: `<yes-or-no-and-evidence>`
