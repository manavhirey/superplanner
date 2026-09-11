# T<task-number>: <task-title>

## Metadata

- Task ID: `T<task-number>`
- Task slug: `<task-slug>`
- Feature: `../FEATURE.md`
- Approved design: `../../../design.md`
- Acceptance file: [`../acceptance/T<task-number>-<task-slug>.feature`](../acceptance/T<task-number>-<task-slug>.feature)
- Implementation plan: `../plans/T<task-number>-<task-slug>-plan.md`

## Approval

<!-- superplanner-approval:start -->
- status: `pending`
- approver: `none`
- approved_at: `none`
- approval_evidence: `none`
- content_id: `sha256:<64 lowercase hex>`
<!-- superplanner-approval:end -->

Approval recording may change only bytes inside the markers. Any byte change
outside the markers changes `content_id` and invalidates approval. Operational
lifecycle is tracked only in external `STATE.md`. `approved` requires non-`none`
approver identity and explicit evidence plus a valid ISO-8601 time, all bound to
this exact content ID.

## Single Reviewable Outcome

`<State one outcome that can be implemented, tested, and reviewed independently.>`

## Inputs

| Input | Source | Required state |
| --- | --- | --- |
| `<input>` | `<path-interface-or-dependency>` | `<required-state>` |

## Outputs

| Output | Consumer | Observable result |
| --- | --- | --- |
| `<output>` | `<actor-system-or-following-task>` | `<observable-result>` |

## In Scope

- `<Exact behavior or change owned by this task>`

## Out Of Scope

- `<Explicitly excluded behavior or change>`

## Dependencies

| Dependency | Consumed output | Completion evidence |
| --- | --- | --- |
| `<task-id-system-or-decision>` | `<specific-output>` | `<artifact-path-or-command-result>` |

## Likely Ownership

- Files or globs: `<workspace-root-relative-paths-or-globs>`
- Subsystem: `<subsystem>`
- Shared state or exclusive resources: `<resource-or-none>`

## Documentation Impact

- Impact: `<yes-or-no>`
- Paths and required update: `<paths-and-behavior-to-document-or-none-with-reason>`

## Acceptance Source

Observable acceptance is defined only in
[`../acceptance/T<task-number>-<task-slug>.feature`](../acceptance/T<task-number>-<task-slug>.feature).
This task does not duplicate or override those scenarios.

## Completion Evidence

- Focused verification: `<exact-command-and-expected-result>`
- Integrated verification: `<exact-command-and-expected-result>`
- Required handoff artifacts: `<paths>`
