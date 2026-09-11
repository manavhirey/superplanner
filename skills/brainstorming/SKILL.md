---
name: brainstorming
description: Use before spikes, bounded changes, new features, or architectural work to inspect the repository, clarify intent, compare sound approaches, and obtain explicit design approval before implementation.
license: MIT
metadata:
  source: obra/superpowers brainstorming, adapted for Superplanner
  artifact-layout: docs/superplanner
---

# Brainstorming

Turn a request into an approved design before implementation.

## Hard Gate

Do not write implementation code, scaffold the solution, or invoke an implementation workflow until the user explicitly approves the design. Approval of an earlier request, a list of tasks, or an apparently obvious change is not approval of the current design.

## Classify First

Inspect the repository before asking detailed questions: read relevant code, tests, documentation, configuration, and recent changes when available. Then state one route so the user can correct it:

- **Spike:** A feasibility probe whose retained output is a recommendation, not production code. Describe the question and bounded probe, obtain approval, investigate as cheaply as correctness permits, and label any temporary work throwaway.
- **Bounded:** A small change to an existing, readable flow. Ask only material questions, present a short in-chat design covering behavior, touched areas, and verification, then wait for explicit approval.
- **Architectural:** A new project or subsystem, a cross-component change, or a change to interfaces other consumers rely on. Use the complete design-artifact workflow below.

When uncertain, choose the heavier route. Hidden complexity may upgrade a route; it never downgrades one mid-task. Stop and reclassify before continuing.

## Clarify

Ask one material question per message. Prefer a small set of concrete choices when that helps, while allowing a custom answer. Establish:

- the actor, problem, and intended value;
- in-scope and excluded behavior;
- constraints, dependencies, compatibility needs, and failure policy;
- observable success criteria;
- unresolved product rules that must not be invented.

If the request contains independently valuable subsystems, propose decomposition before refining details. Brainstorm one coherent subsystem at a time.

## Compare Approaches

For architectural work, present two or three materially different approaches. Lead with the recommended option and explain tradeoffs in complexity, coupling, migration, operability, and testability. Apply YAGNI to every option: remove speculative extension points, premature abstractions, and behavior not required by the stated outcome.

Bounded work needs one smallest sound approach unless a real tradeoff requires alternatives. A spike needs a probe plan, not an architecture exercise.

## Present The Design

Present architectural designs in reviewable sections and ask whether each section is correct before proceeding. Scale detail to complexity and cover:

- architecture and component responsibilities;
- interfaces, data flow, and state transitions;
- errors, boundaries, and recovery behavior;
- security, compatibility, and operational constraints where relevant;
- test strategy, acceptance outcomes, and documentation impact;
- explicit exclusions, risks, and unresolved decisions.

Keep units cohesive and independently understandable. Follow repository conventions and include only targeted refactoring required by the design.

## Architectural Artifacts

After the in-chat design is approved:

1. Choose a stable kebab-case `<initiative>` identifier.
2. Write the authoritative design to `docs/superplanner/<initiative>/design.md`.
3. Apply `visual-explainer` to generate `docs/superplanner/<initiative>/design.html` as a faithful visual companion beside it.
4. Self-review both artifacts for placeholders, contradictions, ambiguity, missing decisions, and scope creep. The Markdown controls whenever the artifacts disagree; regenerate the HTML rather than changing intent only in HTML.
5. Ask the user to review the written artifacts and wait for explicit approval.

The Markdown design must identify purpose, scope, architecture, responsibilities, interfaces, flows, failure behavior, testing, rollout or migration when relevant, risks, and exclusions. Do not begin feature decomposition, planning, or implementation until the written design is approved.

## Completion

- A spike ends with evidence and a recommendation; retaining probe code is a new request.
- A bounded change ends this skill at explicit approval of the in-chat design.
- Architectural work ends this skill only after `design.md` and `design.html` exist, agree, pass self-review, and the user explicitly approves the written design.
