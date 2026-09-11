# F<feature-number>: <feature-title>

## Metadata

- Feature ID: `F<feature-number>`
- Feature slug: `<feature-slug>`
- Initiative: `<initiative-slug>`
- Approved design: `../../design.md`
- Design approval evidence: `<message-or-record-reference>`

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

## Outcome

`<State one independently observable outcome.>`

## Actor And Value

- Actor: `<actor>`
- Value: `<value delivered when the outcome is achieved>`

## In-Scope Behavior

- `<Observable behavior included in this feature>`

## Excluded Scope

- `<Behavior or change intentionally excluded>`

## Dependencies

| Dependency | Required output or state | Evidence |
| --- | --- | --- |
| `<feature-system-or-decision>` | `<required-output-or-state>` | `<path-or-record>` |

## Likely Affected Domains

- `<subsystem-or-file-domain>`

## Risks

| Risk | Effect on outcome | Mitigation or decision |
| --- | --- | --- |
| `<risk>` | `<effect>` | `<mitigation-or-decision>` |

## Ordered Task Index

| Order | Task | Reviewable outcome | Acceptance |
| --- | --- | --- | --- |
| 1 | [`T<task-number>-<task-slug>`](tasks/T<task-number>-<task-slug>.md) | `<single outcome>` | [`acceptance`](acceptance/T<task-number>-<task-slug>.feature) |

## Feature Completion Evidence

- Observable result: `<user-visible-or-system-visible result>`
- Verification: `<command-or-inspection and expected result>`
- Documentation: `<updated-paths-or-none-with-reason>`
