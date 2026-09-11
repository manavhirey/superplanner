---
name: using-git-worktrees
description: Use for every implementation-plan execution and every implementation isolation check, even when a new worktree may prove unnecessary, to select a safe workspace and establish a verified baseline.
license: MIT
metadata:
  source: obra/superpowers using-git-worktrees, adapted for Superplanner
  safety: non-destructive
---

# Using Git Worktrees

Run this skill before every implementation-plan execution and whenever implementation isolation is being evaluated. Applying it is mandatory even when the outcome is reuse of existing native or linked-worktree isolation and no new worktree is necessary. Select or establish a safe workspace before implementation, and never initialize a repository silently.

## 1. Detect Current State

Resolve the trusted absolute Git executable and define the exact `safe_git`
profile from `integration-protocol.md`. Confirm this is a Git repository and use
the protocol's trusted non-Git config parser to validate the
repository/object/`files`-backend tuple, require the canonical common
directory's `shallow` path to be absent as every node type, and establish config
and shallow-path immutability before any object read, ref/reflog snapshot, or
worktree operation. Then inspect with that profile;
create and persist the source/setup private content-context path, manifest/hash,
base anchor, and validation result before the first content command;
every command that reads worktree content uses the registered phase/anchor
private fixed-config content-control Git directory and explicit index/object
route rather than repository-local config:

```bash
safe_git_to <external-output>/toplevel plain -- -C <registered-current-worktree> rev-parse --show-toplevel
safe_git_to <external-output>/git-dir plain -- -C <registered-current-worktree> rev-parse --absolute-git-dir
safe_git_to <external-output>/common-dir plain -- -C <registered-current-worktree> rev-parse --path-format=absolute --git-common-dir
safe_git_to <external-output>/superproject plain -- -C <registered-current-worktree> rev-parse --show-superproject-working-tree
safe_git_to <external-output>/gitmodules-stage plain -- -C <registered-current-worktree> ls-files --stage -- .gitmodules
safe_git_to <external-output>/index-stage plain -- -C <registered-current-worktree> ls-files --stage
safe_git_to <external-output>/branch plain -- -C <registered-current-worktree> branch --show-current
```

The mediator's closed bootstrap grammar accepts that exact absolute common-dir
form and this exact fallback only:

```bash
safe_git_to <external-output>/common-dir-relative plain -- -C <registered-current-worktree> rev-parse --git-common-dir
```

Attempt the absolute form first with its own one-use operation/output leaves.
Use a fresh fallback call only when Git reports that `--path-format` is
unsupported, then resolve the returned path relative to
`<registered-current-worktree>`, the registered candidate worktree for this
bootstrap, before canonicalizing it. Canonicalize the Git
directory and common directory before comparing them. Different paths normally
indicate a linked worktree, but a non-empty superproject path indicates a
submodule and must not be mistaken for worktree isolation. Initial detection of
contained submodules is index-only: a tracked `.gitmodules` entry or gitlink in
the parent `ls-files --stage` output is the only initial signal. Do not use
`ls-tree`, attributes, status/diff, or another object read to discover them.
Record initialized, uninitialized, conflicted, and dirty states before selecting
isolation.

- Already in a linked worktree and not a submodule: reuse it for a shell-free
  writer only when it is detached and satisfies the same no-checkout,
  external-index, attribute, and ownership checks. Never reuse a branch-attached
  worktree for an isolated writer.
- In a submodule: treat the submodule repository deliberately and confirm the task belongs there.
- Contains submodules: after the parent index-only detection, validate each
  discovered path as a literal repository-relative path with no traversal or
  symlink escape and parse the tracked `.gitmodules` data with a trusted non-Git
  parser. Stop for user direction before initializing a missing submodule,
  changing a recorded gitlink, or proceeding with a dirty/conflicted nested
  worktree. Do not treat a parent-context submodule command as a cleanliness
  check or context switch. Before any object read for an initialized submodule,
  independently apply the full mirror/bootstrap protocol from
  `integration-protocol.md` to that repository: validate and freeze its own
  repository/object/`files`-backend tuple and routing chain; resolve its common
  directory with the canonical absolute/fallback algorithm above; construct its
  own non-local external baseline mirror and no-hardlink proof; create its own
  source/setup private content context; and register its own external
  index/object/alternate routes and alternate-absence evidence. Parent mirror,
  context, and routes are never reused for the child. Only after that bootstrap,
  select the child context and use `safe_git_to
  <external-output>/submodule-head objects <submodule-setup-objects>
  <encoded-submodule-baseline-objects> -- -C
  <registered-canonical-submodule-root> rev-parse --verify 'HEAD^{commit}'`,
  requiring the result to equal the parent index's recorded gitlink. Compare the
  child index-only `ls-files --stage -z` capture with `safe_git_to
  <external-output>/submodule-tree objects <submodule-setup-objects>
  <encoded-submodule-baseline-objects> -- -C
  <registered-canonical-submodule-root> ls-tree -r -z
  --format='%(objectmode) %(objectname) 0%x09%(path)' <recorded-gitlink>` output,
  including no intent-to-add entry. Require every captured `ls-files -v -z`
  record to carry the ordinary uppercase `H` tag. Then use only the child
  external index/object/alternate tuple for `ls-files -z --others --`, attribute
  checks, status/diff, and every other content or object read. Require no
  untracked path; byte-compare every tracked `.gitattributes` worktree file with
  its mirror-routed blob using a trusted non-Git reader; mechanically reject set
  or valued `filter`, `ident`, `text`, `crlf`, `eol`, or
  `working-tree-encoding` from both named and `--all` `check-attr` forms; and only
  then require cached and worktree `diff --quiet --no-ext-diff --no-textconv
  <recorded-gitlink> --`. Apply this sequence recursively, bootstrapping each
  initialized nested submodule independently before its first object read. Any
  missing equivalent evidence blocks; setup and baseline commands must cover
  every in-scope initialized submodule.
- Normal checkout: honor an existing user instruction about isolation. Otherwise ask before creating a worktree.
- Not a repository: stop and ask; do not run `git init`.

## 2. Prefer Native Isolation

Use the harness or platform's native worktree tool first when available and only
when it can provide detached, ref-neutral isolation and re-root the shell-free
worker's filesystem/project boundary to that worktree. It owns placement and
cleanup. Do not bypass it with manual Git commands merely for convenience, and
do not let it create a task branch before commit authorization.

Use the fallback only when no native mechanism exists or it explicitly cannot handle the repository.

## 3. Safe Git Fallback

Choose the parent in this order: explicit user or repository instructions,
existing `.worktrees/`, existing `worktrees/`, then `.worktrees/` at the
repository root. Never place a worktree under the separate worker-artifact root,
which only the coordinator writes.

Before creating a project-local worktree, verify the exact parent is ignored:

```bash
safe_git_to <external-output>/ignore-dot-worktrees plain -- -C <registered-current-worktree> check-ignore -q --no-index .worktrees/
safe_git_to <external-output>/ignore-worktrees plain -- -C <registered-current-worktree> check-ignore -q --no-index worktrees/
```

Check only the selected location. If it is not ignored, stop and offer two safe choices: obtain approval to add the exact ignore rule, or use a user-approved external directory. Do not create an unignored project-local worktree and do not commit an ignore change without explicit permission.

Resolve the trusted absolute Git executable and apply the safe Git configuration
profile from `integration-protocol.md`, including its fixed
`worktree.useRelativePaths=false`, to every command below. Choose a unique
path and immutable full base SHA. Allocate and persist the reserved external
`setup_objects`, `materialization_index`, and `materialization_objects` paths and
exact encoded `setup_alternates` and `materialization_alternates` defined by that
protocol. Require and persist the baseline mirror's exact construction manifest
including trusted namespace reconstruction, source/mirror shallow-absence, and
complete no-symlink/no-hardlink proof before registering either route. Create
and canonicalize the two object directories; canonicalize the index parent and
require its leaf to remain absent until `read-tree` creates it. Check existing worktrees before creation, then
create a detached administrative worktree without checkout, branch creation,
filters, or checkout hooks:

Before either command below, register and persist a canonical, SHA-256-attested
tuple route with the safe-Git mediator. The setup tuple binds the exact setup
object directory, encoded baseline alternate, registered current context,
destination path, and full base SHA. After the new context is registered, the
materialization tuple binds its exact index, object directory, encoded
baseline/candidate alternates, worker context, and start tree. Register a
separate hash-attested operation manifest for each closed command form, including
`worktree add`, tree attribute inspection, `read-tree --reset -u`, `ls-files`,
`write-tree`, and `diff`, with exact tree, ownership, patch, input, and output
operands as applicable. Bind its route-policy hash to the exact context and
unique setup or external tuple. Persist the operation-manifest path/hash, token
state, bound staging/final paths, primary-or-recovery result path/hash, terminal
result identity, successful final/stage shared-inode/link-count evidence, mediator
status, and exact input/output hashes for every one-shot call.
Persist every routed object directory and transitive alternate member's canonical
path/identity plus `info/alternates` and `info/http-alternates` absence verdict;
revalidate it before registration, every call, and resume.
Reject duplicate or ambiguous object/alternate/context tuples and a
missing, stale, differently hashed, non-canonical, symlink-aliased, or
Git-directory-overlapping route before invoking Git. A user-approved external
worktree is valid when that exact canonical context and destination are recorded;
it need not be below the repository.

```bash
safe_git_to <external-output>/worktrees plain -- -C <registered-current-worktree> worktree list --porcelain
safe_git_to <external-output>/worktree-add objects <external-setup-objects> <encoded-baseline-objects> -- \
  -C <registered-current-worktree> worktree add --detach --no-checkout <path> <full-base-sha>
```

Record the shared ref namespace and complete reflog namespace before and after
creation and require byte-identical equality. Persist the exact `refs`, common
`logs`, and every `worktrees/*/logs` walker-root path, pre/open/post
identity, and verdict plus the optional `packed-refs` leaf path/identity/verdict
for both snapshots. The new detached administrative
record and its per-worktree `HEAD` pseudo-ref are setup metadata approved when
the user accepts worktree creation; record that full SHA and require it to remain
unchanged. No checkout, linked-worktree index, shared ref, or reflog is created
by this command. Before materializing
bytes, use the external baseline/candidate object environment to inspect every
path in the assigned start tree and reject `filter`, `ident`, `text`, `crlf`,
`eol`, or `working-tree-encoding` using both named and `--all` checks. Then materialize with
`safe_git_to <external-output>/materialize external <materialization-index> <materialization-objects>
<encoded-baseline-candidate-alternates> -- -C <worker-worktree> read-tree
--reset -u <start-tree>`; never omit `-C` or `<start-tree>`, and never use
`checkout`, which can invoke filters and hooks. Immediately byte-compare
`ls-files --stage -z` with `ls-tree -r -z
--format='%(objectmode) %(objectname) 0%x09%(path)' <start-tree>`, require every
`ls-files -v -z` record to have uppercase `H`, and require external-index `diff
--quiet --no-ext-diff --no-textconv`. If sandbox permissions block
creation, report the failure and ask whether to use another approved location.
Do not silently abandon isolation, work in place, or create a branch as fallback.

## 4. Set Up The Project

In the selected workspace, read repository setup documentation and CI
configuration. Before any command that inspects worktree contents, use `safe_git_to
<external-output>/setup-paths external <materialization-index> <materialization-objects>
<encoded-baseline-candidate-alternates> -- -C <registered-worker-worktree>
ls-files -z --cached --others --` with
no exclude option and mechanically
reject active content-affecting attributes under the current worktree rules as
specified by `integration-protocol.md`. Use lockfiles and existing scripts to
select the exact package manager, runtime version, dependency installation,
generated-code, environment, and bootstrap commands. Do not guess commands from
language alone, overwrite local secrets, or upgrade dependencies during setup.
Run repository-provided setup and baseline commands only in the constrained
process sandbox from `handoff-contract.md`; if unavailable or if they require
network or credentials, present them to the user and wait for supplied output.

Before setup, require `safe_git_to <external-output>/setup-tree external <materialization-index>
<materialization-objects> <encoded-baseline-candidate-alternates> -- -C
<registered-worker-worktree> write-tree`
to equal the assigned `start_tree`. Pass those three explicit paths again after
setup to `safe_git_to <external-output>/setup-diff external ... -- -C <registered-worker-worktree> diff --quiet
--no-ext-diff --no-textconv` and
to an explicit NUL-delimited diff when needed to compare worktree bytes with the
external index. Also require `safe_git_to <external-output>/setup-untracked
external ... -- -C <registered-worker-worktree> ls-files -z --others --`
with no exclude option to return no
untracked path; any result is a setup change and blocks for user direction.
Revalidate the tuple-route hash and that call's distinct operation-manifest hash
and exact operands before every one of these calls; a path being unchanged is not a substitute for its
recorded route-manifest identity.
Explain any generated or modified file before implementation begins. Never use
the linked worktree's real index or porcelain `status` to establish cleanliness;
a dependent `start_tree` can differ from `HEAD` by design.

Before dispatching a file-writing worker, require harness evidence that its
filesystem/project root is this exact worktree, so `external_directory: deny`
makes the parent checkout and peer worktrees inaccessible. A prompt containing a
path is not evidence of re-rooting. If the active supervisor cannot re-root the
invocation, block retained file-writing execution until an attested supervisor
with re-root support is available. Changing the coordinator's directory is not
equivalent; never dispatch into the parent project.

## 5. Establish The Baseline

Run the repository's narrow smoke check and full relevant baseline suite, including build, lint, type, or test commands required by its documented workflow. Record exact commands and results.

- If setup leaves unexplained changes, stop and resolve them with the user.
- If baseline checks pass and the worktree is clean, report the path, detached
  full commit, and evidence.
- If any baseline check fails, report the failure and ask the user to choose whether to investigate it, proceed with the known failure recorded, or stop. Never attribute a pre-existing failure to later implementation.

## Non-Destructive Lifecycle

When the orchestrator applies this skill, record the worktree path, detached full
SHA, start tree, status, and evidence in `STATE.md`. A builder or other
non-orchestrator returns those values in its canonical handoff so the
orchestrator can record them; non-orchestrators never edit state. Never
force-remove a worktree, delete a worktree directory manually, discard
uncommitted changes, or prune active metadata as automatic cleanup. At
completion, report status and let the user authorize any cleanup; preserve work
when state is uncertain.
