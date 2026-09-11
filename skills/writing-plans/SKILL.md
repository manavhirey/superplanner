---
name: writing-plans
description: Use after one Superplanner task and its acceptance file are approved to write an implementation-ready per-task plan with exact per-scenario TDD cycles and an explicit approval-gated commit step.
license: MIT
metadata:
  source: obra/superpowers writing-plans, adapted for Superplanner
  artifact-layout: docs/superplanner
---

# Writing Plans

Create a plan for one approved task at a time. This skill is planning only: inspect the repository and write the plan artifact, but do not edit implementation, test, configuration, or product documentation files. Implementation starts only after the user approves the completed plan.

## Required Inputs

Read all of these before planning:

- `docs/superplanner/<initiative>/design.md`;
- the containing feature's `FEATURE.md`;
- `tasks/<task-id>-<task-slug>.md`;
- `acceptance/<task-id>-<task-slug>.feature`;
- relevant source, test, configuration, and documentation files in the repository.

Stop for clarification if these artifacts conflict, the task is not independently reviewable, acceptance behavior is ambiguous, or implementation depends on an unapproved task. Do not combine multiple task IDs into one plan.

## Output

Write `docs/superplanner/<initiative>/features/<feature-id>-<feature-slug>/plans/<task-id>-<task-slug>-plan.md`.

Start it with:

- task ID, title, and one-sentence outcome;
- exact pointers to the design, feature, task, and acceptance file;
- concise architecture and repository conventions to preserve;
- dependencies, prerequisites, exclusions, and documentation impact.

Then provide an ordered file map. For every created, modified, or tested file, give its exact path, responsibility, and affected symbols or line range when known. Distinguish files that exist from files the task will create.

## Step Requirements

Create a separate, explicitly labeled TDD cycle for every Gherkin acceptance scenario, in acceptance-file order. Map each cycle to its exact scenario name; a single generic cycle cannot stand in for multiple scenarios. For a Scenario Outline, identify every Examples row exercised by its named test. Repeat this full order for every scenario:

1. Add one named failing test with exact setup, action, assertions, and acceptance scenario traced to it.
2. Request the narrowest exact command through the coordinator's constrained process sandbox or user-run fallback and state the expected failure, including the meaningful error or mismatch; implementation resumes only with that evidence.
3. Implement the smallest production change, naming exact files, symbols, signatures, types, control flow, and errors.
4. Request the narrow test through the same sandbox/user-run path and state the expected passing result; implementation resumes only with that evidence.
5. Request the relevant broader checks through that path and state expected counts or success conditions.
6. Update exact affected documentation when the task declares documentation impact.

For every step include:

- exact file paths and symbols;
- complete interface contracts: parameters, return types, errors, side effects, and consumers;
- concrete test names, inputs, fixtures, and assertions;
- copy-pasteable commands run from a stated directory;
- observable expected results, not merely "works" or "passes";
- ordering constraints and the exact output consumed by later steps.

Include concise code or schema snippets when exact signatures or assertions would otherwise be ambiguous. Follow existing repository style and tooling. Do not add speculative abstractions or unrelated cleanup.

## Approval-Gated Commit Handoff

End every generated plan with these explicit workflow handoff steps:

1. After all scenario cycles and documentation work, request the coordinator to
   rerun affected checks and the full relevant suite if documentation changed
   candidate bytes; resume only with exact evidence, then return candidate
   identity, status, changed files, and evidence without committing.
2. In non-mutating `COMMIT_PREPARE`, the orchestrator persists and presents the canonical patch/hash, candidate tree, proposed message file/hash, full base commit, full target ref at the base OID, exact author and committer dates each in Git's `<unix-seconds> <+|-HHMM>` numeric-timezone form, and expected full repository-format commit OID. It persists the independently routed no-write `hash-object -t commit --stdin` operation manifest and terminal result manifest and requires that result to equal the expected OID. It also reserves absent review-context and post-commit result-manifest paths and persists complete pre-commit namespace-walker evidence with exact `refs`, `logs`, and every `worktrees/*/logs` walker-root identity and verdict plus the optional `packed-refs` leaf identity and verdict. It then requests user authorization that itself explicitly names the patch hash, candidate tree, message hash, full base commit, target state, exact author and committer dates in numeric-timezone form, and expected full OID. `COMMIT_PREPARE` may not stage, write a commit object, or update a ref. Detached `HEAD` blocks authorization. Approval of the task, design, or plan is not commit authorization.
3. Only after that authorization, the user-facing orchestrator performs `COMMIT`, persists a distinct one-use operation manifest and terminal result manifest for each authorized `add`, `write-tree`, `commit-tree`, and `update-ref` call, verifies the staged patch/tree and explicit sole parent, and requires both the created object's parsed full OID and returned OID to equal the authorized expected full OID before any ref mutation or `update-ref`. It then performs the compare-and-swap target update with authorization-bound `GIT_COMMITTER_DATE` and retains complete post-commit namespace-walker evidence with exact `refs`, `logs`, and every `worktrees/*/logs` walker-root identity and verdict plus the optional `packed-refs` leaf identity and verdict, recursively enumerating actual files so orphan reflogs are included and proving only the authorized target update and exactly one byte-exact append in both its branch reflog and the symbolic worktree `HEAD` reflog, each containing the expected old/new OIDs, name/email, Unix seconds, numeric timezone, and reason. It constructs the review context only at the authorization-reserved absent path and publishes the reserved result manifest once with no-clobber semantics, binding its path/identity, exact bytes/hash, authorization-manifest hash, and equal expected/resulting full OID, then records the verified tree/message, full matching SHA, and clean candidate status.

The plan never auto-commits, and Superplanner never executes, brokers, retries,
observes, or reports a push. It must not include a push step. After both reviews
verify that the exact current SHA equals the authorization's expected full OID
and the user authorizes the target-bound presentation record, the separate gate
may only use the trusted output guard to release the exact registered
eligibility-bound non-force command bytes for possible user execution.

## Forbidden Plan Content

Generated plans must not contain `TODO`, `TBD`, ellipses standing for omitted work, "implement as needed", "add validation", "handle errors", "write tests", "similar to above", or any other placeholder. Never cite a type, helper, fixture, command, or file without defining it or identifying its existing location.

Do not perform implementation, create a worktree, dispatch workers, commit, or push while using this skill. The approval-gated handoff above reserves both user coordination and the exact commit transition for the user-facing orchestrator, never the plan executor or another isolated writer.

## Self-Review

Before presenting the plan:

1. Trace every design requirement and Gherkin scenario to its own explicitly labeled, repeated TDD cycle and named test.
2. Check that every changed file is necessary and every required output, configuration change, migration, and documentation update is covered.
3. Verify all symbol names, signatures, types, paths, and cross-task interfaces are internally consistent with the repository.
4. Scan for placeholders, implied work, broad commands, missing expected results, and untestable assertions; replace them with specifics.
5. Confirm each TDD cycle proves failure before production changes and includes narrow plus integrated verification.
6. Confirm the plan ends by returning evidence to the orchestrator, separates non-mutating `COMMIT_PREPARE` from authorized `COMMIT`, reserves both for the user-facing orchestrator, requires the independent no-write hash operation/result, all four authorized per-call operation/result manifests, pre-ref-mutation expected-OID equality, complete namespace-walker and unconditional dual-reflog evidence, authorization-reserved review-context/result publication, no push step, no push execution, and no automatic commit behavior.
7. Re-read the task's scope and remove unrelated work and speculative flexibility.

Fix review failures in the plan itself. Then report the plan path, unresolved risks if any, and request explicit approval. Stop without implementing.
