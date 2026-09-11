---
name: dispatching-parallel-agents
description: Use when two or more implementation or investigation tasks may be independent to prove safe ownership, dispatch focused agents concurrently, and integrate and recover their work without conflicting writes.
license: MIT
metadata:
  source: obra/superpowers dispatching-parallel-agents, adapted for Superplanner
  default-concurrency: "4"
---

# Dispatching Parallel Agents

Parallelism is allowed only when task dependencies and ownership prove it safe. The orchestrator owns coordination, `STATE.md`, integration, and final verification.

## Prove Independence

Build the canonical dependency and ownership matrix before dispatch:

| Domain ID | Task IDs | Depends on | Consumes output from | Expected files | Shared state | Exclusive resources | Compared with | Verdict | Reason |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `<domain-id>` | `<task-ids>` | `<domain-ids-or-none>` | `<outputs-or-none>` | `<exact-paths-or-globs>` | `<state-or-none>` | `<resources-or-none>` | `<domain-ids-or-none>` | `<parallel-or-serialize>` | `<evidence-based-reason>` |

Tasks are not parallel-safe when either one consumes the other's output, they may edit the same file, they mutate the same database or shared environment, they need the same exclusive resource, or resolving one may invalidate the other's assumptions. Group related failures under one investigator until distinct root causes are established. Run dependent or conflicting tasks sequentially.

Assign every writable candidate path to exactly one active agent. Read-only
overlap is acceptable. Reserve shared files, integration files, lockfiles, and
generated indexes for the orchestrator unless one worker receives exclusive
ownership. The resolved external operational `STATE.md` is never assignable and
always remains orchestrator-owned.

## Concurrency

- Default to at most four active agents.
- Never exceed eight active agents, even when capacity is available.
- Use fewer agents when integration cost, repository size, external rate limits, or monitoring capacity makes that safer.
- Parallel dispatch means multiple independent `superplanner_supervisor` adapter
  calls in the same orchestrator message/turn, never stock Task calls and never
  one multi-agent call. Calls issued later form later waves.
- Give every agent fresh context. Do not rely on inherited conversation history.
- Forbid child subagents explicitly. Workers complete their assigned scope themselves.

## Workspaces

Every file-writing agent receives a separate detached, ref-neutral worktree
established through `using-git-worktrees`, and harness evidence that its
filesystem/project root is that exact worktree. A prompt path is insufficient.
If re-rooting is unavailable, block retained file-writing dispatch. Never give
two active writers the same worktree. Read-only investigators may share a clean
source view only when they cannot mutate it.

Do not dispatch a replacement writer while the original agent may still be active in that ownership scope.

## Self-Contained Prompt

Every worker gets explicit milestones and a step/budget contract. The contract defines objective evidence for each milestone, the maximum steps or execution budget, and the conditions that require the worker to stop and return a blocked handoff rather than continue speculatively.

Each prompt must include:

- task ID, one outcome, and why it matters;
- exact design, feature, task, acceptance, and plan artifact paths;
- relevant repository facts, errors, and prior evidence;
- prerequisites and fixed interface contracts;
- exclusively owned files and explicitly forbidden files or shared state;
- required TDD or debugging procedure;
- exact commands and expected results;
- detached worktree path, immutable base/start-tree identity, harness re-rooting
  evidence, and the attested candidate-writer supervisor/broker envelope;
- milestones, coordinator-owned checkpoint ID/path, and execution or step
  budget; the worker emits checkpoint content through supervisor results and never
  writes the external path;
- instruction not to spawn subagents;
- required handoff: status, scope completed, artifacts, changed files, verification evidence, blockers, assumptions, and next action.

Keep one prompt to one coherent problem domain. Do not say only "implement the task" or "fix the tests"; include the concrete behavior, test names or scenarios, constraints, and evidence needed to start without this session's context.

## Monitor And Recover

Monitoring is capability-aware. A foreground isolation-supervisor adapter call cannot be live-polled, stopped, or inspected unless the supervisor explicitly advertises those controls. The orchestrator regains control only when the call completes, returns blocked, or times out; any continuation or recovery then resumes that worker by the generated OpenCode session ID captured from the supervisor result, not by the stable workflow task ID in its handoff. A shell-free worker may make a planned blocked return requesting one exact command; validate it, then run worker-influenced code only in the constrained process sandbox from `handoff-contract.md`, or ask the user to run it when unavailable or when network/credentials are required. Resume with exact sandbox/user evidence, repeating within budget. This is not stuck recovery. Require the worker's own step/budget contract to make it return genuinely blocked with milestone evidence when it exhausts its budget, repeats the same failed action twice without new evidence, cannot make meaningful progress, or waits on unavailable input, permission, or an exclusive resource.

Only use live status or cancellation when the active harness explicitly exposes that capability. Respect its notification and no-poll contract: when it promises notifications or prohibits polling, wait for its notification and do not poll. Do not infer live-control capabilities from task IDs, checkpoint paths, or support in another harness.

Recover in order after control returns or a supported live notification arrives:

1. Resume the same worker by its generated OpenCode session ID with the observed evidence and one focused next action. If none was returned, record resume as unavailable rather than guessing.
2. If recovery fails, confirm that invocation has ended. In a harness with supported cancellation, request cancellation and wait for confirmation that it is inactive.
3. Replace a worker only after the prior invocation ended or a supported cancel confirms inactivity. Give the replacement the complete original brief, checkpoints, and accumulated evidence.
4. Escalate to the user when recovery needs destructive, irreversible, security-sensitive, or scope-changing action, or a replacement also fails.

The orchestrator records dispatches, checkpoints, blockers, recovery, and agent status in the initiative's `STATE.md`.

## Integrate

On return, reject every handoff that is not `status: complete`, has a pending
command-evidence resume, contains out-of-scope edits or conflicting interfaces,
or lacks required verification. For each accepted integration-bound result, the
coordinator creates the canonical `integration_result`, independently replays its
bundle in scratch indexes, and mechanically applies it through the candidate
index under `integration-protocol.md`. Never inspect and directly apply a worker
diff. Integrate complete bundles in dependency order, then run task-specific
checks and the full relevant suite in the integration workspace. Parallel results
are not valid merely because they passed in separate worktrees.
