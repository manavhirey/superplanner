---
name: systematic-debugging
description: Use for every bug, test or build failure, performance regression, flaky test, or unexpected behavior to establish root cause through evidence before proposing or applying one verified fix.
license: MIT
metadata:
  source: obra/superpowers systematic-debugging, adapted for Superplanner
  methodology: four-phase root-cause debugging
---

# Systematic Debugging

## Iron Law

No fix, workaround, or fix proposal is allowed before root-cause investigation. Treat symptom patches and stacked guesses as failures of the process.

Complete each phase in order.

When the debugging agent has no shell access, every reproduction, diagnostic,
test, or verification command becomes a planned `status: blocked`
command-evidence checkpoint. The agent requests one exact command and expected
result. The coordinator runs worker-influenced code only in the constrained
process sandbox from `handoff-contract.md`; if unavailable or if network or
credentials are required, it asks the user to run the command. It then resumes
the same generated OpenCode session ID with exact
sandbox/user output. The agent never substitutes an unrun command claim.

## Phase 1: Root Cause Investigation

1. Read the complete error, warnings, stack trace, paths, and error codes.
2. Reproduce with exact steps and the narrowest reliable command. If intermittent, collect timing and state evidence instead of guessing.
3. Inspect relevant recent diffs, commits, dependency, configuration, environment, and data changes.
4. At each component boundary, capture inputs, outputs, state, and configuration propagation to locate the first divergence.
5. Trace invalid values and unexpected state backward to their origin using [Root Cause Tracing](root-cause-tracing.md). Fixing the point where a symptom surfaces is not enough.

Success means being able to explain what fails, where the first incorrect state appears, and why it appears.

## Phase 2: Pattern Analysis

1. Find the closest working example in the same repository.
2. Read the relevant implementation and contract completely.
3. List every difference between working and failing paths, including data, ordering, lifecycle, environment, and dependencies.
4. Identify which difference is consistent with the evidence. Do not dismiss a difference without testing it.

For timing failures, replace guessed sleeps with the technique in [Condition-Based Waiting](condition-based-waiting.md), unless elapsed time is the behavior under test.

## Phase 3: Hypothesis And Test

State one falsifiable hypothesis: "X is the root cause because evidence Y explains observation Z." Test it with the smallest diagnostic or one-variable change. Do not combine candidate fixes.

- If confirmed, continue to Phase 4.
- If rejected, remove the diagnostic change, record the evidence, and form a new hypothesis from Phase 1.
- If evidence is insufficient, say what is unknown and gather it. Do not present confidence as proof.

## Phase 4: Root-Cause Fix

1. Add the smallest failing regression test or reproducible check before production changes when feasible.
2. Implement one minimal fix at the source of the defect. Avoid unrelated cleanup and bundled refactoring.
3. Add proportionate boundary protection where the same invalid state can enter through realistic alternate paths; use [Defense In Depth](defense-in-depth.md) after, not instead of, the root-cause fix.
4. Re-run the reproducer, narrow tests, and full relevant suite. Verify the original behavior, adjacent failure paths, and absence of regressions.
5. Remove temporary diagnostics unless they provide intentional ongoing observability.

If the fix fails, do not stack another change on top. Revert only that attempted fix, preserve evidence, and return to Phase 1. After three failed fix attempts, stop and ask the user for an architectural decision; do not attempt a fourth speculative fix.

## Completion Evidence

Report the root cause, evidence that proved it, the single fix, the regression coverage, exact verification commands and results, and residual risks. Passing tests without a root-cause explanation does not complete this workflow.
