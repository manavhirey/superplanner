---
name: bdd-gherkin
description: Use for every Superplanner task acceptance file to clarify domain behavior and write focused Gherkin scenarios with exactly one When and observable business outcomes.
license: MIT
metadata:
  artifact-layout: docs/superplanner
  methodology: behavior-driven development
---

# BDD Gherkin

Apply this skill to every task acceptance file. Write each task's criteria to `docs/superplanner/<initiative>/features/<feature-id>-<feature-slug>/acceptance/<task-id>-<task-slug>.feature` and link it from the task Markdown.

## Clarify Before Writing

Read the approved design, feature, and task. Do not infer product behavior when any of these are missing or ambiguous:

- capability: what behavior is being accepted;
- actor: who or what performs it;
- value: why the outcome matters;
- rules: business constraints, alternatives, and failure policy;
- preconditions: the state required before the action.

Return the gap for clarification, one material question at a time, rather than encoding a guess. Technical implementation details are not substitutes for domain rules.

## Scenario Rules

- Give each scenario one business behavior and one reason to fail.
- Use domain language visible to stakeholders, not functions, classes, HTTP mechanics, selectors, or database operations unless those are themselves part of the contract.
- Use exactly one `When` step per scenario. Do not hide a second action in `And` beneath it.
- Put context in `Given`, the single trigger in `When`, and observable business outcomes in `Then` and its `And` steps.
- Assert outcomes, not internal calls or storage choices.
- Cover the normal path plus distinct boundary or rejection rules required by the task. Do not enumerate implementation trivia.
- Prefer regular `Scenario` cases. Use `Scenario Outline` only when the same business rule must be demonstrated with several meaningful examples.
- Use a data table when one step naturally carries a structured set of fields or expected values. Do not use a table merely to compress unrelated cases.
- Keep tags minimal. Add only tags consumed by the repository's tooling or needed for a stable domain classification; avoid task-management and redundant tags.
- Use `Background` only for short, shared domain context. It may contain `Given` steps, never the scenario action.

## Shape

```gherkin
Feature: Descriptive business capability
  So that <business value>
  As a <domain actor>
  I want <capability>

  Scenario: One observable rule
    Given <relevant precondition>
    When <the single business action>
    Then <observable business outcome>
    And <another observation of that same outcome>
```

Use the repository's established feature narrative order if it differs, but preserve actor, capability, and value.

## Review Checklist

- The feature maps to one approved task and its filename matches that task.
- Capability, actor, value, rules, and preconditions are explicit.
- Every scenario is independently understandable and single-focused.
- Every scenario contains exactly one `When` keyword and only one action.
- Every `Then` is externally observable and testable.
- Examples cover required rules without speculative cases.
- Scenario outlines, data tables, backgrounds, and tags earn their complexity.
- No unresolved rule has been silently invented.
