# Subagent Handoff

Use every common field exactly as named. Candidate paths are relative to the
named worktree; external operational paths are absolute.

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
  - <candidate-relative-artifact-or-absolute-operational-path-or-none>
changed_files:
  - <candidate-relative-changed-file-or-none>
verification:
  - command: <exact-command-or-inspection>
    result: <passed|failed|not-run>
    evidence: <observable-result-or-evidence-path>
blockers:
  - <observed-blocker-required-input-and-evidence-or-none>
assumptions:
  - <material-assumption-and-impact-or-none>
next_action: <one-exact-action-owner-and-expected-evidence>
```

Only after validating a `complete` integration-bound builder, debugger, or
documenter handoff with no pending command-evidence resume, the orchestrator records:

```yaml
integration_result:
  base_sha: <full-commit-oid>
  start_tree: <full-tree-oid>
  packaged_tree: <full-tree-oid>
  packaging_index: <absolute-external-index-path>
  packaging_objects: <absolute-external-object-directory>
  setup_objects: <absolute-external-setup-object-directory>
  setup_alternates: <exact-git-encoded-baseline-object-path-list>
  source_setup_context: <canonical-content-directory-manifest-path-sha256-anchor-and-verdict>
  baseline_objects: <absolute-external-baseline-object-directory>
  baseline_construction_manifest: <capability-probe-source-and-mirror-shallow-absence-clone-and-namespace-reconstruction-manifest-absolute-path-device-inode-and-sha256>
  baseline_no_hardlink_proof: <absolute-path-device-inode-sha256-and-verdict>
  alternate_absence_manifest: <absolute-path-device-inode-sha256-and-verdict>
  namespace_snapshot_manifests: <source-before-source-after-raw-post-clone-mirror-reconstructed-mirror-ref-symref-head-equality-and-pre-post-root-packed-refs-identities-verdicts-paths-sha256>
  candidate_objects: <absolute-external-candidate-object-directory>
  owned_paths_file: <absolute-owned.z-path>
  owned_paths_sha256: <sha256>
  patch_file: <absolute-change.patch-path>
  patch_sha256: <sha256>
  manifest_file: <absolute-manifest.raw.z-path>
  manifest_sha256: <sha256>
  object_format: <sha1|sha256>
```

Use `integration_result: none` when that completed builder, debugger, or
documenter task made no integration-bound edit. Brainstormer and planner
handoffs do not receive this extension. Follow `references/integration-protocol.md`;
workers never create this record, a planned `blocked` checkpoint is never
packaged, and a branch or changed-file list alone is not an integration result.
The orchestrator accepts any handoff only with supervisor-returned evidence of a
distinct OS security principal or a passing kernel-enforced same-principal
isolation verdict covering filesystem access, process inspection/control,
inherited descriptors, and privilege escalation. A field or claim authored by
the agent is not that evidence.

For a timeout, use `status: blocked` and identify timeout evidence in
`blockers`. Do not add a timeout status. Reviewers report `none` under
`changed_files` and append:

```yaml
review_result: <approved|findings|not-completed>
reviewed_sha: <full-git-sha-or-none-when-blocked>
findings:
  - <severity-file-line-criterion-and-evidence-or-none>
accepted_risks:
  - finding_id: <finding-id>
    reviewed_sha: <full-git-sha>
    scope: <exact-accepted-scope>
    user_evidence: <explicit-user-decision-reference>
    consequence: <accepted-consequence>
    rationale: <user-accepted-rationale>
```

Use `none` as the sole `accepted_risks` list item when absent. An accepted
finding remains in `findings`; each accepted-risk entry must identify the same
exact `reviewed_sha`, its precise scope, explicit user evidence, consequence,
and rationale.

```yaml
accepted_risks:
  - none
```

An adversarial reviewer also appends:

```yaml
standard_approval_record_id: <registered-standard-review-record-id-or-none-when-blocked>
standard_approval_record_identity: <registered-path-identity-or-none-when-blocked>
standard_approval_record_sha256: <full-sha256-or-none-when-blocked>
standard_approval_sha: <full-git-sha-or-none-when-blocked>
```

For completed review, use `status: complete` and `review_result: approved` or
`findings`. For a blocked review, use `status: blocked` and
`review_result: not-completed`; `git_sha`, `reviewed_sha`, the three
`standard_approval_record_*` fields, and `standard_approval_sha` may be `none`
only for values that could not be established, with the reason in `blockers`.
An adversarial approval requires the exact registered standard-review record ID,
identity, and recomputed hash plus identical `git_sha`, `reviewed_sha`, and
`standard_approval_sha`. Approval may
coexist with accepted risks only when no unresolved or unaccepted finding
remains and all accepted findings stay visible.

The handoff `task_id` is a stable workflow identifier. The orchestrator captures
the generated OpenCode session ID from the `superplanner_supervisor` result
separately in external state and uses only that generated value for resume.
