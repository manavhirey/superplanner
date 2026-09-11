---
name: reviewing-py-code
description: Use when reviewing Python files or Python pull requests to report evidence-backed Clean Code, SOLID, DRY, code-smell, refactoring, and design-pattern findings before summaries or praise.
license: MIT
metadata:
  source: User-provided reviewing-py-code skill, made self-contained
  category: code-review
  stack: Python
---

# Reviewing Python Code

Review Python changes skeptically and constructively. Inspect the changed code, surrounding contracts, callers, tests, and repository conventions. Report concrete risks, not preferences. This skill produces findings only: never edit reviewed code, tests, configuration, documentation, or other candidate files, even if asked.

Return findings in the response or harness handoff only; do not write review evidence into `STATE.md` or any other candidate repository file. When used in Superplanner, bind the review to the exact committed SHA supplied by the standard-review context, include `Reviewed SHA: <sha>` in the output, and stop without approval if that SHA cannot be verified or candidate files change during review.

## Findings First

When this skill is loaded by `superplanner.code-reviewer`, the agent's canonical
YAML reviewer handoff remains the outer response contract. Put the Markdown
finding structure below inside its `findings` entries, and record missing tests,
assumptions, and review evidence in the corresponding canonical fields; do not
replace the envelope.

Start with findings ordered by severity. Use this format for each:

```markdown
**F1 - High - `path/file.py:20-31` - SOLID/SRP**
- **Problem:** The concrete defect or design issue.
- **Evidence:** The relevant control flow, dependency, duplication, or contract.
- **Impact:** The realistic correctness, maintainability, testability, security, or performance cost.
- **Remediation:** A specific direction that preserves intended behavior.
```

Use stable IDs and precise line references. Include a short code example only when it makes the remediation unambiguous; make it valid Python and consistent with repository style. After findings, list missing tests and unresolved assumptions. Put a brief summary and strengths last. If there are no findings, say `No substantiated findings` and identify residual risks or checks not run.

## Severity

| Severity | Meaning |
| --- | --- |
| Critical | A realistic security breach, data loss, or system-wide failure. |
| High | Materially incorrect behavior, broken public contract, or architecture that makes required change unsafe. |
| Medium | Bounded defect, significant coupling or duplication, weak error handling, or meaningful testability cost. |
| Low | Localized maintainability or readability cost with concrete impact. |

Formatting or personal taste alone is not a finding. Do not inflate a heuristic, such as function length, into a defect without showing its cost.

## Review Passes

### 1. Clean Code And Python Semantics

Check intent-revealing module, class, function, and variable names; cohesive functions; readable control flow; specific exceptions; resource lifetime; mutability and default arguments; iterator and async behavior; context managers; public API type hints and docstrings; useful comments that explain why; named domain constants; PEP 8 and repository formatting; and testable dependency boundaries.

Flag bare or overly broad exception handling, swallowed failures, misleading names or comments, ambiguous return contracts, hidden side effects, import-time work, mutable defaults, resource leaks, and magic values when they create a concrete risk.

### 2. SOLID

- **Single Responsibility:** A unit has one cohesive reason to change; identify mixed policy, persistence, transport, and presentation responsibilities.
- **Open/Closed:** Required variants can extend a stable abstraction without repeated central conditionals. Do not demand abstraction for hypothetical variants.
- **Liskov Substitution:** Subtypes preserve accepted inputs, outputs, invariants, exceptions, and side effects promised by the base type.
- **Interface Segregation:** Consumers do not depend on methods or data they cannot use.
- **Dependency Inversion:** High-level policy is not needlessly coupled to concrete I/O, clocks, globals, or framework construction; dependencies are injectable where tests or variants require it.

Prefer composition over inheritance when it reduces coupling. Apply SOLID to actual change pressure, not as ceremony.

### 3. DRY And Knowledge Duplication

Look for repeated business rules, condition sets, transformations, schemas, error mapping, and test setup that can drift independently. Similar syntax is not automatically duplication, and premature unification of coincidental code can be worse. Recommend extraction only when copies represent the same knowledge and should change together.

### 4. Code Smells

Evaluate these categories and connect any finding to evidence:

- **Bloaters:** long method, large class, long parameter list, primitive obsession, data clumps.
- **Object-orientation misuse:** repeated type switches, temporary fields, refused bequest, and inheritance used only for reuse.
- **Change preventers:** divergent change, shotgun surgery, parallel inheritance hierarchies.
- **Dispensables:** comments compensating for unclear code, duplicate code, lazy class, dead code, speculative generality.
- **Couplers:** feature envy, inappropriate intimacy, message chains, middle man, globals, and hidden service locators.

Do not report pre-existing smell outside the requested scope unless the change depends on it and the impact is direct.

### 5. Design Patterns

Recognize both useful and misapplied Gang of Four patterns:

- **Creational:** Factory Method, Abstract Factory, Builder, Prototype, Singleton.
- **Structural:** Adapter, Bridge, Composite, Decorator, Facade, Flyweight, Proxy.
- **Behavioral:** Chain of Responsibility, Command, Iterator, Mediator, Memento, Observer, State, Strategy, Template Method, Visitor.

Recommend a pattern only when it solves observed variation, coupling, lifecycle, or composition pressure more simply than direct Python. Account for Python-native alternatives such as callables, protocols, iterators, decorators, context managers, dataclasses, and module-level factories. Flag patterns that add indirection without reducing real cost.

### 6. Refactoring And Tests

When warranted, give a narrow refactoring direction such as Extract Function/Class, Introduce Parameter Object, Replace Magic Value with Named Constant, Replace Conditional with Polymorphism or a strategy mapping, Move Method, Encapsulate Collection, or replace inheritance with composition. Preserve behavior and request characterization tests before risky structural change.

Check that tests cover public behavior, boundaries, failures, state transitions, side effects, and dependency interactions without overfitting internals. Identify the exact missing test and the defect it would catch.

## Final Check

- Every finding has location, evidence, impact, severity, and actionable remediation.
- Findings distinguish defects from optional improvements.
- Advice follows current repository patterns and supported Python versions.
- Suggested abstractions remove demonstrated duplication or coupling and satisfy YAGNI.
- No unverified claim is presented as fact.
- No reviewed or candidate file was edited, and any Superplanner result names the exact reviewed SHA.
