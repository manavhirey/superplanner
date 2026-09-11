# <initiative-name>

## Metadata

- Initiative slug: `<initiative-slug>`
- Design owner: `<owner>`
- Last updated: `<yyyy-mm-dd>`

## Problem

`<Describe the observed problem, who experiences it, and why it matters.>`

## Actors And Outcomes

| Actor | Need | Observable outcome | Value |
| --- | --- | --- | --- |
| `<actor>` | `<need>` | `<outcome>` | `<value>` |

## Success Criteria

- `<Measurable or observable success criterion>`
- `<Verification signal and expected result>`

## Scope

### In Scope

- `<Required behavior or system change>`

### Non-Goals

- `<Explicitly excluded behavior or system change>`

## Product Rules And Constraints

- `<Accepted product or domain rule>`
- `<Technical, security, compatibility, or operational constraint>`

## Current System

`<Summarize relevant repository behavior and cite inspected paths.>`

## Considered Approaches

### Option A: <approach-name>

- Approach: `<How this option works>`
- Benefits: `<Material benefits>`
- Costs and risks: `<Material costs and risks>`

### Option B: <approach-name>

- Approach: `<How this option works>`
- Benefits: `<Material benefits>`
- Costs and risks: `<Material costs and risks>`

## Selected Approach

`<Name the smallest sound option and explain why it is selected.>`

## Architecture

### Components And Boundaries

| Component | Responsibility | Boundary or dependency |
| --- | --- | --- |
| `<component>` | `<responsibility>` | `<boundary-or-dependency>` |

### Interfaces

| Interface | Inputs | Outputs | Failure behavior |
| --- | --- | --- | --- |
| `<interface-name-or-signature>` | `<inputs>` | `<outputs>` | `<observable-failure-behavior>` |

### Data And Control Flow

1. `<First concrete interaction>`
2. `<Next concrete interaction>`
3. `<Observable completion or failure>`

## Dependencies

- `<Dependency and required state>`

## Risks And Mitigations

| Risk | Impact | Mitigation or decision |
| --- | --- | --- |
| `<risk>` | `<impact>` | `<mitigation-or-accepted-decision>` |

## Verification Strategy

- `<Behavior to verify, command or method, and expected result>`

## Documentation Impact

- `<Documentation path and required behavioral update, or none with reason>`

## Open Decisions

- `<Decision owner, question, and effect on acceptance, or none>`

## Approval Gate

Implementation may begin only after all material decisions are resolved and the
user explicitly approves this Markdown design. After approval, record:

<!-- superplanner-approval:start -->
- status: `pending`
- approver: `none`
- approved_at: `none`
- approval_evidence: `none`
- content_id: `sha256:<64 lowercase hex>`
<!-- superplanner-approval:end -->

Approval recording may change only bytes inside the markers and does not change
document metadata. Any byte change outside the markers changes `content_id` and
invalidates approval until the new ID is explicitly approved. `approved`
requires non-`none` approver identity and explicit evidence plus a valid
ISO-8601 time, all bound to this exact content ID.
