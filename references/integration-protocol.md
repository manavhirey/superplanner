# Mechanical Integration Protocol

This protocol transfers uncommitted results from isolated file-writing workers
without worker commits and without orchestrator implementation authorship. A
branch name, changed-file list, or worker worktree alone is not an integration
artifact.

## Portable SHA-256

Every non-Git artifact or evidence checksum in this protocol is computed with
`sha256sum` when available or `shasum -a 256` otherwise. Repository commit,
tree, and blob OIDs instead use that repository's detected SHA-1 or SHA-256
object format. Before dispatch or integration, require at least one checksum
command; if neither exists, stop with a setup blocker. Use this portable shell
helper so generated checksum files have the same conventional
`<hex><two spaces><path>` form:

```bash
sha256_files() {
  if command -v sha256sum >/dev/null 2>&1
  then
    sha256sum "$@"
  elif command -v shasum >/dev/null 2>&1
  then
    shasum -a 256 "$@"
  else
    return 127
  fi
}
```

## Scope And Limits

Use this protocol for every builder, debugger, or documenter whose edits occur
outside the integration workspace. The orchestrator may execute only the exact
validation and replay operations below. It never selects hunks, repairs a patch,
uses three-way fallback, resolves conflicts, or edits implementation. Any failed
check returns to a fresh specialist with evidence.

Git transports regular-file blobs, empty files, binary files, symlink targets,
deletions, type changes, and executable-bit changes. Renames are represented as
one deletion and one addition. Empty directories, ACLs, extended attributes,
hard-link identity, and non-executable permission bits are outside Git's model.
Reject gitlink/submodule changes, sparse worktrees, unsupported symlinks, and
missing Git LFS payloads rather than claiming they were integrated. Reject
shallow and promisor/partial-clone repositories before object reads; no command
may lazy-fetch a missing object. After resolving and canonicalizing the common
Git directory through the closed bootstrap grammar below, a trusted non-Git
inspection must require its `shallow` path to be absent, including as a symlink
or other filesystem node, and bind that absence under the same immutability
boundary. This protocol supports only Git's `files` ref
backend. From one identity-checked private snapshot of the common and worktree
config bytes, a trusted non-Git parser must validate the common config's
`core.repositoryFormatVersion`, `extensions.objectFormat`, and
`extensions.refStorage` as one tuple using Git's origin and precedence rules.
Format fields in per-worktree config block. Format 0 permits no extension,
including `objectFormat`, `refStorage`, or `worktreeConfig`, and implies SHA-1.
Format 1 permits absent
`objectFormat` for SHA-1 or one exact `sha256`, and requires `refStorage` to be
absent or one exact `files`. Reject `reftable`, URI/payload forms, duplicate or
conflicting values, `compatObjectFormat`, `relativeWorktrees`,
`submodulePathConfig`, `partialClone`, and every extension except
`objectFormat`, `refStorage`, and, only at format 1, exact-true
`worktreeConfig`. Inspect
`config.worktree` separately when that accepted extension is active. The trusted
supervisor must bind and keep the `.git` file/directory, `commondir` chain,
attested config paths, parents, and bytes immutable to every untrusted process
from snapshot capture through Git process or direct namespace-walk completion;
hash-before/hash-after checks do not substitute for that exclusion. Complete
this proof before any ref/reflog snapshot, clone, worktree creation, packaging,
authorization, or mutation.

## Immutable Assignment

Before dispatch, require an explicitly configured absolute worker-artifact root
outside the candidate root and every Git directory. Canonicalize every Git
directory and reject case-folded, filesystem-normalized, symlink, hard-link, or
ancestry aliases. Create a full-ref bare, non-local, no-hardlink baseline mirror
under the artifact root from the immutable dispatch repository, verify `base_sha`
and its base tree there, and require the complete shared-ref and reflog
namespaces to retain their recorded identities. Initial trusted context
detection under the same scrubbed, `GIT_OPTIONAL_LOCKS=0` profile plus creating
and validating this mirror are the only bootstrap reads from the repository
object directory. Afterward, all pre-authorization object reads use the external
mirror except for the coordinator-only, non-mutating `COMMIT_PREPARE` inspections,
each explicitly registered with the real index/common-object route and bound
`alternates=none`. This exception permits only the closed read-only forms
required to prove the real index, base, worktree, ref, and commit evidence. It
never permits an object write, lazy fetch, implicit fallback, or an unregistered
route. Reserve new packaging paths under the artifact root, never beside
Git-hosted `STATE.md`, and record:

The closed source/setup discovery forms in `using-git-worktrees` are part of
initial trusted context detection. Their registered real index/common-object
tuple binds repository context, but their grammar permits only ref, index, and
ignore inspection and grants no repository-object read. This setup exception
includes the displayed `rev-parse`, `branch --show-current`, `ls-files --stage`,
`worktree list --porcelain`, and literal `check-ignore --no-index` forms; it does
not extend to any other `plain` content command. The common-directory grammar
accepts exactly `rev-parse --path-format=absolute --git-common-dir` and the
fallback `rev-parse --git-common-dir`. Attempt the first with its own one-use
operation/output leaves. Only an unsupported-`--path-format` result permits a
fresh fallback call; resolve that fallback's exact output relative to the
registered candidate worktree before canonicalization. No ambient working
directory or other base is permitted.

Initial contained-submodule detection is index-only through the displayed
parent `ls-files --stage` forms. For every initialized submodule, independently
complete this protocol's trusted config validation, canonical common-directory
bootstrap, non-local external baseline mirror and no-hardlink proof, private
source/setup content context, alternate-absence checks, and explicit external
index/object routes before any child `ls-tree`, attribute, status/diff, or other
object read. Parent routes and parent mirror evidence never authorize a child
read. Apply the same rule recursively to a nested initialized submodule.

```yaml
base_sha: <full-commit-oid-at-dispatch>
start_tree: <full-tree-oid-materialized-for-worker>
object_format: <sha1|sha256>
owned_paths_file: <absolute-path-to-owned.z>
owned_paths_sha256: <sha256-of-exact-owned.z-bytes>
worker_worktree: <absolute-assigned-worktree>
dispatch_root_evidence: <harness-proof-worker-project-root-equals-worker-worktree>
writer_runtime_evidence: <fresh-private-invocation-envelope-proof>
trusted_git_bin: <canonical-absolute-path-device-inode-and-sha256>
trusted_env_bin: <canonical-absolute-path-device-inode-and-sha256>
trusted_transaction_supervisor_bin: <canonical-absolute-path-identity-and-sha256>
transaction_supervisor_source_sha256: <sha256-of-exact-source-bytes>
packaging_output_dir: <reserved-absolute-external-directory>
packaging_index: <reserved-absolute-new-index-path>
packaging_objects: <reserved-absolute-new-object-directory>
repository_objects: <canonical-absolute-common-object-directory>
baseline_git_dir: <absolute-external-bare-mirror>
baseline_objects: <canonical-absolute-baseline-object-directory>
baseline_construction_manifest: <capability-probe-exact-env-argv-output-status-source-and-mirror-shallow-absence-clone-namespace-reconstruction-source-mirror-identities-result-and-manifest-path-device-inode-sha256>
baseline_no_hardlink_proof: <complete-source-object-to-mirror-identity-scan-manifest-path-device-inode-sha256-and-verdict>
setup_objects: <reserved-absolute-new-per-domain-setup-object-directory>
setup_alternates: <per-domain-git-encoded-baseline-object-path-list>
source_setup_context: <canonical-content-directory-identity-manifest-path-sha256-anchor-and-verdict>
alternate_absence_manifest: <all-routed-object-path-identities-absences-verdicts-manifest-path-device-inode-sha256>
materialization_index: <reserved-absolute-new-per-domain-materialization-index-path>
materialization_objects: <reserved-absolute-new-per-domain-materialization-object-directory>
materialization_alternates: <per-domain-git-encoded-baseline-and-candidate-object-path-list>
candidate_index: <absolute-orchestrator-owned-alternate-index>
candidate_objects: <absolute-orchestrator-owned-external-object-directory>
scratch_root: <absolute-orchestrator-owned-external-scratch-directory>
safe_hooks_dir: <absolute-empty-external-directory>
content_git_dirs: <per-context-and-phase-private-fixed-config-control-directories>
packaging_alternates: <git-encoded-baseline-and-candidate-object-path-list>
scratch_alternates: <git-encoded-baseline-and-candidate-object-path-list>
candidate_alternates: <git-encoded-baseline-object-path>
```

`owned.z` is orchestrator-written, NUL-delimited, and contains exact
repository-relative leaf paths. Every record is non-empty and NUL-terminated.
Reject absolute paths, globs, pathspec magic, trailing slashes, empty, `.` or
`..` components, directory entries, duplicates, ancestor/file collisions, and
case-folded or filesystem-normalized collisions. A rename owns both paths.

After validating and creating the empty safe-hooks directory, create the baseline
mirror before dispatch with the trusted absolute Git and `env` executables and
non-local transport so no object is hard-linked to the source:

Before that Git process, inspect the canonical common config and, only when
enabled there, the per-worktree config with a trusted non-Git parser. Require
the canonical common directory's `shallow` path to be absent as described above.
Bind that verdict, the config byte hashes, identities, origins, the
`.git`/`commondir` chain, and the validated
format tuple into the context manifest. Any `include`/`includeIf`, unsupported
extension, promisor remote, alternate remote object dependency, or source
`objects/info/alternates`/`http-alternates` file blocks. Every
`uploadpack.hideRefs` or `transfer.hideRefs` entry visible to source-side
upload-pack also blocks, regardless of value or Git config scope, including
subsection/scoped spellings and negated or namespace-qualified forms. Match
those key names case-insensitively under Git's config grammar; require
system/global scope suppression and reject them in every local, worktree, or
command scope. Persist the exact path/parent identities and absence verdict for
both alternate names, and
freeze those absences and paths through clone completion. The ref-storage key
name is likewise case-insensitive, but its accepted value is the exact
case-sensitive string `files`; reject URI/payload forms. Do not let Git discover
a missing object or transport while establishing the mirror.

The trusted Git executable must semantically advertise exactly one value-taking
`ref-format` option for `clone`. Before source access, capture its `git clone -h`
output and status under the same exact scrubbed executable/environment profile.
Recognize Git help's `--[no-]ref-format <format>` spelling as that single option;
do not require the literal text `--ref-format=<format>`. Bind the probe
environment, argv, output hash, status, and executable identity into
`baseline_construction_manifest`. The successful clone must then prove that the
exact value `files` is accepted. A missing or ambiguous capability, a rejected
`files` value, or a different resulting backend is a setup blocker; never retry
without `--ref-format=files`, rely on the default backend, or convert the mirror
afterward.

```bash
"$ENV_BIN" -i LC_ALL=C LANG=C GIT_OPTIONAL_LOCKS=0 GIT_ATTR_NOSYSTEM=1 \
  GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TERMINAL_PROMPT=0 \
  GIT_NO_REPLACE_OBJECTS=1 GIT_NO_LAZY_FETCH=1 "$GIT_BIN" \
  --no-replace-objects --no-lazy-fetch \
  -c core.hooksPath="$SAFE_HOOKS_DIR" \
  -c core.alternateRefsCommand= \
  -c protocol.file.allow=always clone --mirror --no-local --ref-format=files \
  "$REPOSITORY_ROOT" "$BASELINE_GIT_DIR"
```

Require the mirror object format to equal `object_format`, its `shallow` path to
be absent as every filesystem-node type, and no mirror file to be a symlink or
share file identity with a repository object. Require the mirror to use the
`files` ref backend too; mirror state never substitutes for validating the
source backend. Under the config and namespace-root immutability boundary, use
the trusted non-Git `files`-backend walker to capture the source's complete
logical shared-ref/symref namespace plus `HEAD` immediately before and after
clone and the mirror's raw namespace immediately after clone. Each canonical
logical manifest is separate from its walker-root identity/verdict evidence. It
uses the `files-ref-namespace-v1` tag, bound object format, canonical decimal
record count, then `HEAD` followed by refs ordered by raw name bytes; every
NUL-delimited record is exactly name, `direct` or `symbolic`, and value, followed
by one terminal NUL and exact EOF. A direct value is one nonzero lowercase OID
of the bound width. A symbolic value is its immediate textual target, never a
recursively resolved target; dangling and chained symrefs are preserved. `HEAD`
uses the same discriminated record, covering detached direct, unborn symbolic,
and chained symbolic forms. Parse canonical loose-ref bytes and optional
`packed-refs` directly, apply loose-over-packed precedence, and reject every
unsupported physical node or malformed physical record. Reject duplicate names,
noncanonical counts or ordering, invalid names/targets/OIDs, path-prefix
file/directory conflicts, a missing terminal delimiter, or trailing data.
Require the two source manifests
to be byte-identical. The raw post-clone mirror manifest is diagnostic evidence,
not an equality gate: Git transport may dereference a non-`HEAD` symref or choose
a symbolic `HEAD` for a detached source.

While the new mirror remains supervisor-writable and unavailable to workers,
the trusted non-Git `files`-backend serializer must reconstruct its namespace
from the stable source manifest. Validate every ref name, symbolic target,
direct OID, directory/file conflict, complete manifest, and destination prestate
with the same canonical walker grammar while keeping the manifest descriptor and
destination identities bound. Finish that validation through the terminal NUL
and exact EOF before the first deletion, truncation, directory creation, or ref
write. Then, without copying any source namespace file, remove every post-clone `refs`
descendant and optional `packed-refs`, recreate every source ref as a loose file
containing exactly either `<direct-oid>\n` or `ref: <symbolic-target>\n`, and
write `HEAD` in the matching exact form. No Git ref command performs this
reconstruction. Rewalk the reconstructed mirror and require its logical
namespace plus `HEAD` to equal the stable source manifest byte for byte, with no
extra or missing name, rewritten target, or dereferenced symref. Only then
resolve `base_sha` and its tree, complete the object/no-link checks, and freeze
the mirror. A transport advertisement or equality of reachable objects is not
namespace proof. Persist the exact clone environment/argv and executable
identities, source and mirror roots, construction and reconstruction results,
source-before, source-after, raw post-clone mirror, and reconstructed mirror
namespace manifests and equality verdicts, mirror shallow-absence and
config/object-format/backend evidence, and the complete no-symlink/no-hardlink
identity-scan manifest and verdict.
Resolve an initial `start_tree` from that
mirror; resolve a dependent `start_tree` from `candidate_objects` with the mirror
as alternate. A partial or missing-object mirror blocks dispatch. Recheck the
complete source shared-ref and reflog namespaces after cloning and before mirror
reconstruction.

The baseline mirror, per-context private content-control Git directories, per-domain setup object directories, per-domain
materialization index/object directories, packaging output directory, packaging index/object directory,
candidate index/object directory, scratch root, and empty safe-hooks
directory descend from the
worker-artifact root. Every existing parent is canonical, contains no symlink
component, and is outside the candidate and all Git directories; reserved
per-domain leaves do not exist before use. Recheck those properties after each
directory creation. Create and canonicalize each domain's `setup_objects` and
`materialization_objects` before that domain's first command. Reserve each
`materialization_index`, canonicalize its existing parent and intended leaf, and
require that leaf to remain absent so `read-tree` creates a valid index; never
pre-create an empty index file. Persist `setup_objects`, exact encoded
`setup_alternates`, and all three materialization routing values in external
state. The worker is denied shell and sensitive external-directory
access; its only generated external exception is its own private truncation
output, which remains edit-denied. It may write only its assigned re-rooted workspace, and has no packaging
authority. Before dispatch, reject every workspace symlink that resolves outside
that workspace or into any Git directory or administrative file, and every file
that shares identity with Git administrative storage, regardless of ownership.
Reject any assigned path that case-folds to Git control metadata. Canonicalize
every credential-manifest path, record its filesystem identity, scan every
project file and symlink, and reject or mask every same-identity hard link or
resolving symlink alias before exposing a workspace.
The assigned worktree starts with `HEAD` equal to `base_sha` and bytes equal to
`start_tree`.

Create a separate `content_git_dir` for each context and phase with trusted
non-Git filesystem primitives; never run `git init` and never reuse or mutate a
directory after attestation. Its detached `HEAD` is the full lowercase
`anchor_oid` plus one newline. Create a source/setup context anchored to
`base_sha` before the source's first content command; create each destination's
integration context immediately after administrative worktree creation and
before its first content command; create a fresh commit context anchored to
`base_sha` during the coordinator-only, non-mutating `COMMIT_PREPARE` step before
the authorization evidence is displayed; enter mutating `COMMIT` only after exact
user authorization; and create a fresh review context anchored to
the resulting reviewed SHA. Retire every earlier phase context and invalidate
all of its unconsumed route tokens at each transition.
Persist every source/setup context's canonical path, manifest path/hash, anchor,
route policy, validation result, and lifecycle before its first call.
The config bytes are exactly one of these two forms, including the terminal
newline, and no additional key is permitted:

```ini
[core]
	repositoryformatversion = 0
	bare = false
```

```ini
[core]
	repositoryformatversion = 1
	bare = false
[extensions]
	objectFormat = sha256
```

The directory contains those `HEAD` and `config` files plus required empty
`objects`, `refs`, and `info` directories. When the registered real context has
an `info/exclude`, copy its exact bytes once with trusted non-Git primitives to
the private `info/exclude`; otherwise attest its absence. This snapshot preserves
ignore semantics without exposing executable configuration, and any source
identity/byte drift retires the context. The directory has no
`info/attributes`, hooks, object alternates, reftable data, or other files, and
no candidate-controlled executable configuration. Make its files and
directories read-only to callers; canonicalize and identity/hash-attest the
phase, anchor, every required file/directory, exact config, `HEAD`, and optional
exclude bytes, and every required absence. A closed supervisor-only validation operation supplies
each registered object route, proves the object format, and resolves detached
`HEAD` to `anchor_oid`; ordinary callers cannot invoke this bootstrap form.

The mediator classifies the exact command and option form before selecting a
Git directory. The following matrix is normative; a form absent from it and the
closed operand grammar is rejected:

| Command form | Effective Git directory | Required index/object route |
| --- | --- | --- |
| Current-worktree or `--source` `check-attr`; `check-ignore`; every allowed `ls-files`; `status`; every allowed `diff`, `diff-tree`, or patch-emitting `show`; `add`; every allowed `read-tree` or `apply`; `update-index --refresh` | Phase/context `content_git_dir` | The mode's registered external tuple, or the registered real index/common objects and explicit `alternates=none` for `plain` and authorized staging |
| `rev-parse`; `for-each-ref`; `symbolic-ref`; `reflog`; `branch`; `worktree`; `merge-base`; object-only `ls-tree`, `cat-file`, non-patch `show`, and no-write `hash-object -t commit --stdin`; `write-tree`; `commit-tree`; `update-ref` | Registered real Git directory | The mode's exact registered index/object/alternate tuple; no implicit fallback |

`read-tree` without `-u`, `apply --cached`, cached/two-tree `diff`, `ls-files
--cached`, and `update-index --refresh` are deliberately explicit in the first
row. Packaging `rev-parse HEAD` and `--show-object-format` remain in the second
row so they preserve source context semantics. Every first-row call records the
selected content directory plus its explicit index, object, and encoded
alternate paths; every second-row call records the real directory and its exact
route. Every registered real or external object directory must have
`info/alternates` and `info/http-alternates` absent and frozen; only the explicit
encoded alternate route is allowed, and `none` is a bound value rather than an
omitted field. Apply the same absent-and-frozen check to every object directory
named by that encoded route before registration and before each call; never
permit an explicit alternate root to add a transitive on-disk alternate. Reject
regular files, symlinks, and dangling symlinks at either name. No broad mode such
as `external:*` determines the class.
Persist each checked object directory and transitive member's canonical path,
path/parent identities, both required absences, and verdict; revalidate that
exact evidence on resume and before every call.

Parallel independent workers normally share a start tree. A later dependent
worker receives the current candidate tree materialized mechanically and records
that tree as its `start_tree`.

## Safe Workspace Materialization

Create each detached worktree with `safe_git_to <registered-setup-output>
objects "$SETUP_OBJECTS" "$SETUP_ALTERNATES" -- -C
"$REGISTERED_REPOSITORY" worktree add --detach --no-checkout`, using the external setup object directory and the
Git-encoded baseline mirror object path in `SETUP_ALTERNATES` as its only
alternate. This creates only the user-approved
administrative worktree record and detached per-worktree `HEAD` pseudo-ref;
record that full SHA. It must leave the complete shared-ref and reflog namespaces
byte-identical and must not create an index or run a checkout hook.
The mandatory safe profile pins `worktree.useRelativePaths=false`, so this call
cannot enable `extensions.relativeWorktrees` in the source repository.

Before writing worktree bytes, generate the complete NUL-delimited path list from
`start_tree` with `ls-tree -r -z --name-only`. Under the external baseline and
candidate object environment, run both `check-attr --source="$START_TREE" -z
--stdin` for `filter`, `ident`, `text`, `crlf`, `eol`, and
`working-tree-encoding` and `check-attr --source="$START_TREE" --all -z --stdin`
against that list. Mechanically reject every set or valued named result and every
appearance of one of those names in the `--all` output. Materialize only after
that check with a new external index/object directory and `safe_git_to
<registered-materialization-output> external
"$MATERIALIZATION_INDEX" "$MATERIALIZATION_OBJECTS" "$MATERIALIZATION_ALTERNATES"
-- -C "$WORKER_WORKTREE" read-tree --reset -u "$START_TREE"`; never omit `-C`,
and never use `checkout`, `switch`, or another porcelain
that invokes checkout hooks. Require resulting bytes to equal `start_tree`.
Byte-compare the complete external stage-0 `ls-files --stage -z` manifest with
`ls-tree -r -z --format='%(objectmode) %(objectname) 0%x09%(path)'
"$START_TREE"`, whose records exactly match that serialization, reject every non-ordinary
`ls-files -v -z` tag, and require `diff --quiet --no-ext-diff --no-textconv` under
that external index. This proves sparse checkout, skip-worktree, intent-to-add,
or omitted worktree bytes did not produce an incomplete materialization. Record
the real linked-worktree index as untouched setup metadata.

Before dispatch, require harness evidence that the file-writing task's
filesystem/project root equals `worker_worktree`. This makes the parent checkout,
peer worktrees, Git common directory, state, and worker-artifact root external to
the agent and mechanically denied. Supplying an absolute path in the prompt is
not re-rooting. If the harness lacks this capability, retained file-writing
execution is blocked; do not weaken `external_directory: deny` or work in place.
Establish and record the complete fresh private worker-invocation envelope from
`handoff-contract.md` before process startup or provider discovery. This includes
no persisted approvals, custom providers, MCP, plugins, LSP, formatters, or
automatic hooks; stock built-in tool provenance with duplicate IDs rejected;
authenticated allowlisted skills; an edit-denied private data root; and a process
sandbox denying Git storage, parent/peer worktrees, operational storage, network,
credentials, and every other sensitive external location. Missing evidence
blocks the invocation before its first tool call.

## Coordinator Packaging

After the isolated task returns `complete` with no pending command-evidence
resume, the coordinator validates its canonical handoff, confirms refs and
`HEAD` did not change, and rejects every worktree change outside `owned.z`. The
worker never runs Git or packages its own result. The coordinator creates the
reserved output and object directories and uses the reserved index.
`GIT_OBJECT_DIRECTORY` quarantines newly written blobs and trees;
`GIT_ALTERNATE_OBJECT_DIRECTORIES` provides read-only access to existing
baseline-mirror and candidate objects. No packaging command writes the repository
object store, real index, commit graph, refs, tags, branches, or reflogs.

Resolve absolute trusted Git and `env` executables before the baseline bootstrap.
The bootstrap clone uses them with the displayed empty environment and
system/global config suppression. Every subsequent pre-authorization Git command
uses those executables and this safe configuration profile; never invoke a
repository script or PATH-local wrapper as Git or `env`:

```bash
safe_git() {
  local mode operation_token index_file object_directory alternates
  local -a routed_environment=("GIT_CONFIG_NOSYSTEM=1")
  test "$#" -ge 2 || return 125
  mode="$1"
  shift
  case "$mode" in
    plain)
      operation_token=${1-}
      shift
      verify_operation_call "$operation_token" plain "$@" || return 125
      test "${1-}" = -- || return 125
      shift
      ;;
    authorized)
      case "${1-}" in
        *[!0-9a-f]*|'') return 125 ;;
      esac
      test "${#1}" -eq 64 && test "${2-}" = -- || return 125
      verify_authorization_manifest "$1" || return 125
      shift 2
      ;;
    objects)
      test "$#" -ge 5 || return 125
      operation_token=$1
      shift
      verify_operation_call "$operation_token" objects "$@" || return 125
      object_directory="$1"
      alternates="$2"
      test -n "$object_directory" && test -n "$alternates" && test "$3" = -- || return 125
      routed_environment=(
        "GIT_OBJECT_DIRECTORY=$object_directory"
        "GIT_ALTERNATE_OBJECT_DIRECTORIES=$alternates"
      )
      shift 3
      ;;
    external)
      test "$#" -ge 6 || return 125
      operation_token=$1
      shift
      verify_operation_call "$operation_token" external "$@" || return 125
      index_file="$1"
      object_directory="$2"
      alternates="$3"
      test -n "$index_file" && test -n "$object_directory" \
        && test -n "$alternates" && test "$4" = -- || return 125
      routed_environment=(
        "GIT_INDEX_FILE=$index_file"
        "GIT_OBJECT_DIRECTORY=$object_directory"
        "GIT_ALTERNATE_OBJECT_DIRECTORIES=$alternates"
      )
      shift 4
      ;;
    *) return 125 ;;
  esac

  "$ENV_BIN" -i LC_ALL=C LANG=C GIT_OPTIONAL_LOCKS=0 GIT_ATTR_NOSYSTEM=1 \
  GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_PAGER=cat \
  GIT_NO_REPLACE_OBJECTS=1 GIT_NO_LAZY_FETCH=1 GIT_LITERAL_PATHSPECS=1 \
  GIT_TERMINAL_PROMPT=0 "${routed_environment[@]}" "$GIT_BIN" \
    --no-replace-objects --no-lazy-fetch \
    -c core.sparseCheckout=false \
    -c core.sparseCheckoutCone=false \
    -c index.sparse=false \
    -c core.splitIndex=false \
    -c core.fsmonitor=false \
    -c core.untrackedCache=false \
    -c worktree.useRelativePaths=false \
    -c core.logAllRefUpdates=false \
    -c core.autocrlf=false \
    -c core.attributesFile=/dev/null \
    -c core.hooksPath="$SAFE_HOOKS_DIR" \
    -c commit.gpgSign=false \
    -c tag.gpgSign=false \
    -c i18n.commitEncoding=UTF-8 \
    -c i18n.logOutputEncoding=UTF-8 \
    -c log.showSignature=false \
    -c format.pretty=medium \
    -c log.date=iso-strict \
    -c log.mailmap=false \
    -c mailmap.file=/dev/null \
    -c mailmap.blob= \
    -c core.alternateRefsCommand= \
    -c core.pager=cat \
    "$@"
}
```

The helper starts Git with an empty environment, so inherited routing,
configuration, attribute-source, external-diff, tracing/output, pager/editor,
transport, and credential variables cannot affect it. The wrapper registers and
injects a distinct operation-manifest token for each call. Its direct inner
forms are `safe_git plain <operation-token> -- ...`, `safe_git objects
<operation-token> "$OBJECT_DIRECTORY" "$ALTERNATES" -- ...`, and `safe_git
external <operation-token> "$INDEX_FILE" "$OBJECT_DIRECTORY" "$ALTERNATES" --
...`.
The only literal-path environment exception is the fallback's fixed
`check-ignore -q --no-index .worktrees/` or `check-ignore -q --no-index
worktrees/` form: Git versions that reject `GIT_LITERAL_PATHSPECS=1` for `check-ignore` receive an exact
mediator-injected `GIT_LITERAL_PATHSPECS=0` after the grammar has accepted one of
those two complete literal operands. No caller-selected path reaches that
exception.
`verify_operation_call` and `verify_authorization_manifest` deserialize the
effective Git-directory class and complete index/object route before this inner
profile runs. The mediator injects those values into `routed_environment` and
prepends the registered `--git-dir`/`--work-tree` arguments. For a content-class
`plain` call it injects the registered real index and common object directory;
for a content-class `external` call it preserves the supplied external tuple;
authorized `add` receives the authorization-bound real index and common object
directory. The first and third routes bind an explicit empty
`GIT_ALTERNATE_OBJECT_DIRECTORIES` value and require both on-disk alternate files
to remain absent. No content command uses the private directory's empty index or
object store.
Before authorization, that real-index/common-object `plain` route is available
only to the coordinator's closed, non-mutating `COMMIT_PREPARE` inspections. All
other post-bootstrap pre-authorization object reads remain mirror-routed, and no
pre-authorization route may write the repository object directory.
Pass every canonical path positionally; never retain it in the coordinator's
ambient environment. `authorized` has the same scrubbed profile as `plain` but
is reserved for the exact post-authorization staging and commit commands.
The mediator enforces capabilities before Git executes: `plain` is read-only;
`objects` permits only object reads and the exact ref-neutral worktree setup;
`external` permits reads plus index/worktree mutation only through the supplied
external index/object route; and `authorized` permits only the displayed
path-file `add`, argument-free `write-tree`, explicit-parent/file-message
`commit-tree`, and one-ref compare-and-swap `update-ref` forms. Every form has a
closed operand grammar bound to recorded paths, OIDs, identity, and reason;
`update-ref --stdin`, extra refs, unlisted subcommands, and cross-mode commands
are rejected from callers. There is no push form or transport route in any mode;
Superplanner never executes `git push`. For the accepted one-ref update request, the mediator
itself drives the fixed prepared transaction from `quality-gates.md`, inspects
target type/OID while Git holds the lock, and commits or aborts it.
Every `plain`, `external`, or `objects` request must carry a one-use SHA-256 token
for a persisted, NUL-serialized per-call manifest and exactly match a
pre-registered, hash-attested index/object/alternate/worktree tuple whose
writable paths are under the external artifact root and outside every Git
directory. Its route-policy hash binds the registered context-manifest hash,
  exact effective Git-directory class/path/identity/hash, content-control
  manifest hash and anchor when applicable, exact index/object/alternate paths
  and identities, unique route identities, tree, owned-path
  file/hash, patch/hash, and integration-policy hash. Owned-path grammar validation and hashing consume one
  private, unlinked byte snapshot captured through an identity-checked source
  descriptor; patch and policy artifacts are
likewise identity-bound before their hashes enter the route policy. An
`external` call cannot fall through to an object
route; an object-only read may use either its exact setup route or the unique
external object/alternate/context tuple. Reject duplicate or ambiguous tuples.
The exact argv, selected real-or-content Git-directory class/path/manifest hash,
content anchor, index/object/alternate route,
stdin leaf/hash/device-and-inode identity, output leaf identity,
literal mediator FD-role bindings (`6,8` for input and `9` for output), private
result/staging leaves, and route-policy
hash must equal the operation manifest. The mediator reopens and deserializes
every manifest field, rehashes the manifest and stdin through identity-checked
descriptors, recomputes policy, consumes the token before Git executes, and
rejects reuse, substitution, stale bytes, extra fields, or a changed policy.
The mediator injects the same one-use operation token internally for every exact
`authorized` call after verifying the authorization-manifest token; the caller
cannot select or reuse that operation token. Its manifest additionally binds the
authorization hash, ordered-call policy hash, and current authorization phase.
Persist the operation-manifest path/identity/hash and terminal primary-or-recovery
result path/identity/hash for every authorized call without exception.
After consumption it writes the complete NUL-serialized terminal result with the
operation-manifest hash, argv and route-policy hashes, final mediator status, and
exact output SHA-256 to a no-clobber staging leaf,
  publishes it to the reserved final leaf with the exact `link(2)` syscall,
  validates both hard-link identities, link count, and bytes, records their
  shared device/inode plus the primary result path/hash,
  and retains the validated staging hard link as independently named safety
  evidence for the recorded primary result.
  `link(2)` treats a directory or symlink-to-directory final
operand as an occupied exact leaf rather than traversing it. If final publication
is preempted, the mediator rewrites the still identity-bound stage with final
status `125`, retains that stage as the published recovery result, and records
its path/hash. A request
  that consumes its token but fails closed grammar or later policy checks still
  gets one identity- and hash-attested failure result and cannot be retried. Using the real
index/object directory or an unregistered route blocks before Git. Read formats
are likewise closed literals; no caller-supplied `%G*`, `--ext-diff`, textconv,
unsafe apply, or mutating symbolic-ref/reflog form reaches Git.
Every object-only request likewise matches a registered object/alternate/context
tuple and, for setup, the exact destination and base OID. Registration
canonicalizes every component and rejects lexical, normalized, symlink,
case-folded, hard-link, ancestry, or identity overlap with a Git directory. A
user-approved external worktree is accepted through its registered canonical
context identity, not by requiring repository ancestry.
Callers cannot use shell redirection around the mediator. The trusted service
launcher starts its canonical script directly through absolute `/usr/bin/env -i`
with only the attested fixed environment and absolute `/bin/bash --noprofile
--norc`; invoking it as `bash <script>` is forbidden because that would bypass
the pre-interpreter scrub and permit `BASH_ENV` or imported-function injection.
It starts as a private process with FDs 3-9 closed, then exclusively owns those fixed internal
descriptors; this avoids both caller collisions and Bash's reserved descriptors
above 9. Each request supplies an exact pre-registered external stdout leaf
through a separate mediator argument. The mediator creates it once with
no-clobber semantics, keeps fixed FD 9 open, and checks that descriptor and the
final path retain the registered device/inode identity before and after FD 9
closes. Commands with input use fixed FD 8 and require its identity to equal the
registered, hash-attested input leaf. Git receives only stdin/stdout duplicates;
the exec closes every mediator-only FD 3-9. A regular-file stderr descriptor is
rejected. Before authorization, the process sandbox makes
every Git directory and repository index read-only, so the shell cannot pre-open
or truncate Git storage before mediator validation. The examples below use
`safe_git_to <output> ...` and `safe_git_io <output> <input> ...` for this
out-of-band descriptor contract. Every output, result, authorization-manifest,
and authorization scratch leaf is reserved absent, created once without
following a dangling symlink, and identity-checked after its descriptor closes.
The trusted mediator surrounding this inner profile requires an exact registered
context on every call, classifies the closed command before execution, and
prepends that class's attested real or content-control `--git-dir` and the
context's `--work-tree` plus a final matching `-c core.worktree`; bare contexts
omit the worktree options. The request's one leading `-C` is a context selector and must
exactly equal that registered canonical worktree; the mediator preserves it as
Git's process directory for commands such as `apply` and `read-tree -u`. It
rejects every additional command-supplied context or configuration override; the only command-supplied `-c` values are
one ordered pair containing the exact authorized `user.name` followed by
`user.email` during an identity-bearing `COMMIT` command. `commit-tree` requires
that pair and receives the same identity through exact `GIT_AUTHOR_*` and
`GIT_COMMITTER_*` variables plus exact authorization-bound `GIT_AUTHOR_DATE` and
`GIT_COMMITTER_DATE` inside `env -i`; `update-ref` requires the pair and receives
exact authorization-bound `GIT_COMMITTER_NAME`, `GIT_COMMITTER_EMAIL`, and
`GIT_COMMITTER_DATE` values. The authorization's precomputed commit OID is
the only accepted `update-ref` new value. Other commands reject the pair. Build the
immutable context manifest by canonical file-level inspection during the narrow
bootstrap exception, hash it, and revalidate it before each call. Parse and hash
the repository-format/object-format/ref-backend tuple only from one private
unlinked common-config snapshot captured through an identity-checked descriptor;
inspect per-worktree config separately when enabled, and reject format keys
there. Do not separately reopen a pathname for parse and hash. Reinspect all
local config files, the `.git`/`commondir` routing chain, optional
`info/exclude`, and required alternate-file absences through the same snapshot
procedure on every call; block any `include`/`includeIf` rather than following
it. The supervisor keeps all those paths, parents, bytes, and namespace roots
immutable to every untrusted process until the Git invocation or direct
namespace walk and result publication finish. Git's repository-format parser
therefore consumes the same immutable common-config bytes that were attested;
command-line `-c` is not a backend override. Bracket each direct files-backend
ref/reflog namespace walk with matching context/backend attestation while
retaining that exclusion boundary. Repository-local config can therefore
neither reach a content command, redirect the worktree, trigger a filter, nor
race the mediated ref backend; missing local objects block.

Before `add`, run both `check-attr -z --stdin` for `filter`, `ident`, `text`,
`crlf`, `eol`, and `working-tree-encoding` and `check-attr --all -z --stdin`
against every NUL-delimited owned path under the same environment. Mechanically
parse the first output and reject every set or valued content-affecting
attribute; parse the second and reject every appearance of one of those names.
The second check distinguishes absence from a literal value such as
`filter=unspecified`, which the named query renders ambiguously. This check uses the final worker bytes, so a changed
`.gitattributes` is detected before normal staging. A concurrent attribute race
still cannot execute a clean/process filter because content commands never read
repository-local filter configuration; any built-in byte conversion changes the
staged tree and fails the mandatory patch/tree equality checks. Reject rather
than execute any external filter. The safe profile disables split
indexes, sparse checkout, fsmonitor commands, untracked-cache side files,
automatic reflog creation, checkout hooks, global/system config and attributes,
non-UTF-8 commit/log encoding, and implicit CRLF conversion.

```bash
test ! -e "$PACKAGING_OUTPUT_DIR"
test ! -e "$PACKAGING_INDEX"
test ! -e "$PACKAGING_OBJECTS"
mkdir -p "$PACKAGING_OUTPUT_DIR" "$PACKAGING_OBJECTS" "$CANDIDATE_OBJECTS"

safe_git_to "$PACKAGING_OUTPUT_DIR/base-sha" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" rev-parse --verify 'HEAD^{commit}'

safe_git_to "$PACKAGING_OUTPUT_DIR/object-format" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" rev-parse --show-object-format

safe_git_to "$PACKAGING_OUTPUT_DIR/read-tree" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" read-tree "$START_TREE"

safe_git_to "$PACKAGING_OUTPUT_DIR/all-paths.z" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" ls-files -z --cached --others --

safe_git_io "$PACKAGING_OUTPUT_DIR/attributes.raw.z" \
  "$PACKAGING_OUTPUT_DIR/all-paths.z" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" check-attr -z --stdin \
  filter ident text crlf eol working-tree-encoding

safe_git_io "$PACKAGING_OUTPUT_DIR/all-attributes.raw.z" \
  "$PACKAGING_OUTPUT_DIR/all-paths.z" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" check-attr --all -z --stdin

safe_git_to "$PACKAGING_OUTPUT_DIR/tracked-changes.z" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" diff --name-only -z \
  --no-ext-diff --no-textconv --

safe_git_to "$PACKAGING_OUTPUT_DIR/untracked-changes.z" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" ls-files -z --others --

safe_git_io "$PACKAGING_OUTPUT_DIR/add-output" "$OWNED_PATHS_FILE" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" add \
  --all --force \
  --pathspec-from-file=- \
  --pathspec-file-nul

safe_git_to "$PACKAGING_OUTPUT_DIR/change.patch" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" diff \
  --cached --binary --full-index --no-color --no-renames --no-ext-diff --no-textconv \
  --src-prefix=a/ --dst-prefix=b/ "$START_TREE" --

safe_git_to "$PACKAGING_OUTPUT_DIR/manifest.raw.z" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" diff \
  --cached --raw -z --full-index --abbrev=64 --no-color --no-renames \
  --no-ext-diff --no-textconv "$START_TREE" --

safe_git_to "$PACKAGING_OUTPUT_DIR/packaged-tree" \
  external "$PACKAGING_INDEX" "$PACKAGING_OBJECTS" \
  "$PACKAGING_ALTERNATES" -- \
  -C "$WORKER_WORKTREE" write-tree

sha256_files "$OWNED_PATHS_FILE" \
  "$PACKAGING_OUTPUT_DIR/change.patch" \
  "$PACKAGING_OUTPUT_DIR/manifest.raw.z" \
  >"$PACKAGING_OUTPUT_DIR/SHA256SUMS"
```

Require the first two command outputs to equal `BASE_SHA` and `OBJECT_FORMAT`
and record the final output as `packaged_tree`. Before and after packaging,
require the real index tree, refs, and `HEAD` to have identical recorded
identities. Also require equality of the complete shared ref namespace with
symbolic targets and the complete reflog namespace plus each reflog's exact-byte
SHA-256. Enumerate actual files recursively beneath common-dir `logs` and every
`worktrees/*/logs` directory rather than deriving names from current refs; this
must include orphan and unregistered linked-worktree reflogs. This filesystem
enumeration is valid only after the source and current context manifests prove
the `files` ref backend; backend drift blocks before Git or snapshot access.
Walk `refs`, optional regular `packed-refs`, `logs`, and every
`worktrees/*/logs` root with a trusted non-Git recursive walker using no-follow
file opens and pre/open/post identity checks while those identity-pinned roots
remain immutable to untrusted processes. Reject symlinks,
non-directory traversal components, non-regular leaves, hard-linked leaves,
escapes, duplicate identities, or a changed root/directory identity. Keep the
format/config/routing and namespace roots immutable to untrusted processes until
the complete snapshot is published, then revalidate them before releasing that
boundary.
Persist each exact `refs`, `logs`, and `worktrees/*/logs` walker-root path,
absent-or-present state, pre/open/post device/inode identity, recursive-walk
verdict, and snapshot hash, plus the optional `packed-refs` leaf path, state,
identity, and verdict. A namespace byte hash without these identities and
verdicts is incomplete evidence.
Canonicalize
every index, output, scratch, mirror, and object path and its existing parents;
reject equality, ancestry, hard-link, case-folded, normalized, or symlink aliasing
with the candidate or any Git directory. Every Git command must carry the safe
profile and displayed explicit index/object environment; an omitted or
overridden value blocks packaging.
Before any command that compares worktree content, obtain the complete cached
and all-untracked path inventory without exclusions and run both attribute
checks over that inventory. Only after they pass, parse both change files as
NUL-delimited paths and require every record to occur exactly in `owned.z`; do
not use ignore rules. Immediately parse `manifest.raw.z` as NUL-delimited raw-diff records and
enforce the status, mode, object-ID, and exact ownership grammar below. In
particular, reject `160000` gitlinks and embedded repositories rather than
silently packaging a submodule change.

The orchestrator treats `owned_paths_sha256` from its assignment as authority.
The raw manifest must use only `A`, `D`, `M`, or `T`, only modes
`000000`, `100644`, `100755`, or `120000`, full repository-format object IDs,
and paths contained in `owned.z`. Require every header to begin with `:`, contain
exactly the space-separated old mode, new mode, old OID, new OID, and one status,
and end with one NUL; exactly one path and its NUL follow that header. Reject a
trailing partial record and duplicate path. Both OIDs use the detected common format and lowercase hex. Mode
`000000` requires the all-zero OID, while every nonzero mode requires a nonzero
OID. Require `A` to have a zero old side and nonzero new side, `D` the inverse,
and `M`/`T` two nonzero sides. Define regular-file modes as `100644` and
`100755`: `M` requires the same type on both sides and a changed mode/OID tuple,
while `T` requires a regular-file-to-symlink or symlink-to-regular-file type
change. Human-readable
`changed_files` is secondary.

The coordinator records this canonical packaging result beside the validated
worker handoff in external operational state:

```yaml
integration_result:
  base_sha: <full-commit-oid>
  start_tree: <full-tree-oid>
  packaged_tree: <full-tree-oid>
  packaging_index: <absolute-external-index-path>
  packaging_objects: <absolute-external-object-directory>
  setup_objects: <absolute-external-setup-object-directory>
  setup_alternates: <exact-git-encoded-baseline-object-path-list>
  source_setup_context: <canonical-content-directory-manifest-path-sha256-anchor-and-verdict>
  baseline_objects: <absolute-external-baseline-object-directory>
  baseline_construction_manifest: <capability-probe-source-and-mirror-shallow-absence-clone-and-namespace-reconstruction-manifest-absolute-path-device-inode-and-sha256>
  baseline_no_hardlink_proof: <absolute-path-device-inode-sha256-and-verdict>
  alternate_absence_manifest: <absolute-path-device-inode-sha256-and-verdict>
  namespace_snapshot_manifests: <source-before-source-after-raw-post-clone-mirror-reconstructed-mirror-ref-symref-head-equality-and-pre-post-root-packed-refs-identities-verdicts-paths-sha256>
  candidate_objects: <absolute-external-candidate-object-directory>
  owned_paths_file: <absolute-owned.z-path>
  owned_paths_sha256: <sha256>
  patch_file: <absolute-change.patch-path>
  patch_sha256: <sha256>
  manifest_file: <absolute-manifest.raw.z-path>
  manifest_sha256: <sha256>
  object_format: <sha1|sha256>
```

Record `integration_result: none` only after a completed task made no
integration-bound edit. This field is a coordinator record, never a worker claim;
never create it for a planned `blocked` command-evidence checkpoint.

## Candidate Identity

The integration candidate is identified by both `base_sha` and the current
candidate tree OID. `HEAD` remains at `base_sha` while uncommitted domains are
integrated. The orchestrator initializes one external alternate candidate index
from the base tree using `candidate_objects` as `GIT_OBJECT_DIRECTORY` and only
`baseline_objects` as its encoded alternate. It records each accepted `pre_tree`
and `post_tree`. Git's real index and repository object directory must remain at
the base state until user commit authorization.

```bash
safe_git_to <candidate-init-output> \
  external "$CANDIDATE_INDEX" "$CANDIDATE_OBJECTS" \
  "$CANDIDATE_ALTERNATES" -- \
  -C "$INTEGRATION_WORKTREE" read-tree "$BASE_TREE"
```

Before each integration, require the integration worktree bytes, candidate
index, recorded current candidate tree, and regular index to have no unexplained
drift. Rehash the bundle immediately before replay and require the worker task to
be complete with no pending command-evidence resume. Here the bundle means all
three recorded artifacts: `owned_paths_file`, `patch_file`, and `manifest_file`;
each current SHA-256 must equal its recorded value before every consuming call
and again immediately after apply before manifest comparison.

## Independent Replay

First replay the patch with `--cached` in a scratch index seeded from
`start_tree`. Use both `git apply --cached --check` and `git apply --cached` with
all of these restrictions:

```text
--no-3way --no-reject --no-unsafe-paths --no-allow-overlap
--no-ignore-space-change --no-ignore-whitespace --whitespace=nowarn
```

For each replay, reserve a new scratch index path and scratch object directory
under `scratch_root`; require both leaves not to exist, create only the object
directory, and repeat every canonical/alias check. Every scratch Git command,
including `read-tree`, both `apply` commands, manifest generation, and
`write-tree`, must use that explicit scratch path as `GIT_INDEX_FILE`, the
scratch directory as `GIT_OBJECT_DIRECTORY`, `scratch_alternates` containing
only baseline and candidate object directories, and the safe Git profile in
`external` mode. The
path list uses Git's platform syntax and quoting; no scratch command may read or
write the repository object directory or real index.

For each start/pre tree, register new output leaves and the exact scratch route,
then use these closed forms; `<patch-file>` is the identity/hash-bound canonical patch path, never
shell redirection or ambient stdin:

```bash
safe_git_to <read-tree-output> \
  external "$SCRATCH_INDEX" "$SCRATCH_OBJECTS" "$SCRATCH_ALTERNATES" -- \
  -C "$INTEGRATION_WORKTREE" read-tree <start-or-pre-tree>
safe_git_to <apply-check-output> \
  external "$SCRATCH_INDEX" "$SCRATCH_OBJECTS" "$SCRATCH_ALTERNATES" -- \
  -C "$INTEGRATION_WORKTREE" apply --cached --check \
  --no-3way --no-reject --no-unsafe-paths --no-allow-overlap \
  --no-ignore-space-change --no-ignore-whitespace --whitespace=nowarn \
  <patch-file>
safe_git_to <apply-output> \
  external "$SCRATCH_INDEX" "$SCRATCH_OBJECTS" "$SCRATCH_ALTERNATES" -- \
  -C "$INTEGRATION_WORKTREE" apply --cached \
  --no-3way --no-reject --no-unsafe-paths --no-allow-overlap \
  --no-ignore-space-change --no-ignore-whitespace --whitespace=nowarn \
  <patch-file>
```

Regenerate a raw, NUL-delimited, full-index, no-renames manifest from that
scratch result and require byte equality with `manifest.raw.z`. This proves that
the patch produces the declared modes and blobs from the assigned start tree.

Next seed a second scratch index from the current recorded candidate `pre_tree`,
replay the patch with the same restrictions, and write `expected_tree`. Generate
the raw diff from `pre_tree` to `expected_tree` and require it to equal the
coordinator-packaged manifest byte-for-byte. This rejects overlap or stale
assumptions from previously integrated domains even when textual context would
apply.

Before accepting `expected_tree`, use the same explicit scratch route to generate
its complete NUL-delimited path list with `ls-tree -r -z --name-only`, then run
both `check-attr --source="$EXPECTED_TREE" -z --stdin` for `filter`, `ident`,
`text`, `crlf`, `eol`, and `working-tree-encoding` and `check-attr
--source="$EXPECTED_TREE" --all -z --stdin` against every path. Mechanically
apply the two rejection rules above. This post-tree check is mandatory even when
`.gitattributes` is outside the current ownership domain; it prevents a changed
attribute file from activating conversion or a process filter on an unchanged
candidate path.

## Mechanical Apply

Only after both scratch replays pass, apply to the orchestrator-owned candidate
index and integration worktree with `git apply --index --check`, followed by
`git apply --index`, using exactly the restrictions above. Before checking the
patch, require candidate index/tree equality with recorded `pre_tree`.

```bash
safe_git_to <candidate-apply-check-output> \
  external "$CANDIDATE_INDEX" "$CANDIDATE_OBJECTS" \
  "$CANDIDATE_ALTERNATES" -- \
  -C "$INTEGRATION_WORKTREE" apply --index --check \
  --no-3way --no-reject --no-unsafe-paths --no-allow-overlap \
  --no-ignore-space-change --no-ignore-whitespace --whitespace=nowarn \
  <patch-file>
safe_git_to <candidate-apply-output> \
  external "$CANDIDATE_INDEX" "$CANDIDATE_OBJECTS" \
  "$CANDIDATE_ALTERNATES" -- \
  -C "$INTEGRATION_WORKTREE" apply --index \
  --no-3way --no-reject --no-unsafe-paths --no-allow-overlap \
  --no-ignore-space-change --no-ignore-whitespace --whitespace=nowarn \
  <patch-file>
```
Before exact worktree-byte comparison, use `ls-files -z --cached --others --`
with no exclude option to inventory every tracked and untracked path, then run
both current-worktree attribute checks over that complete inventory. Also use
`--source="$PRE_TREE"` to perform the same two mechanically parsed
`filter`/`ident`/`text`/`crlf`/`eol`/`working-tree-encoding` rejections under the
candidate environment for the union of every path in `pre_tree` and every
incoming path from the validated raw manifest/`owned.z`. Only after all four
checks pass may exact worktree-byte equality with that tree be established. Run `update-index --refresh`
only after that rejection, to populate stat data without changing entries.
Require:

Every command against the candidate index must set `GIT_INDEX_FILE` to that
index, `GIT_OBJECT_DIRECTORY` to `candidate_objects`, and alternates to the
encoded `candidate_alternates` containing only `baseline_objects`. Deliberately
omit `packaging_objects`: this forces `git apply` and `write-tree` to materialize
all new candidate blobs and trees in `candidate_objects` rather than resolving
them from a transient packaging store. This environment and the safe Git profile
in `external` mode apply to candidate status, refresh, raw diff, `apply`, and
`write-tree` commands. Outside the coordinator's explicitly registered read-only
`COMMIT_PREPARE` exception, no post-bootstrap pre-authorization command may read
Git's repository object directory; no pre-authorization command may write it.

- resulting candidate tree equals `expected_tree`;
- raw `pre_tree` to `post_tree` manifest equals the coordinator-packaged
  manifest;
- changed paths are exactly within assigned ownership;
- Git's real index still equals the base tree;
- no unrelated worktree path changed.

Record domain ID, `base_sha`, `start_tree`, `packaged_tree`, `pre_tree`,
`post_tree`, baseline/candidate object paths, all three artifact hashes,
application time, and checks in external state. Generate any cumulative
candidate patch from the candidate alternate index/object directory with only
the baseline mirror as alternate, never by staging into the real index.

If replay, manifest, ownership, mode, object, or tree checks fail, integration is
blocked. Never use `--3way`, `--reject`, whitespace repair, unsafe paths, manual
edits, or conflict resolution. A corrected domain replaces its prior bundle;
rebuild the candidate mechanically from the base by replaying accepted bundles
in deterministic dependency order.

## Authority Boundary

Builder, debugger, and documenter permissions deny all shell and sensitive
external-path access plus every case spelling of `.git`; assignment validation rejects every
Git-path and symlink alias across the workspace. The
orchestrator owns workspace setup, command execution,
persisted checkpoints, packaging, integration, verification, and commits; it
records complete ref/symbolic-ref and complete reflog identities before and
after dispatch and never consumes a worker commit or branch as the result. Its
own direct edit permission remains denied; every mechanical Git or command action
uses the typed, attested `superplanner_supervisor` `operate` interface from
`handoff-contract.md`, never Bash or a permission-prompted shell.

The final coordinator `COMMIT` transition is separate from isolated work. Only
after exact user authorization does the user-facing coordinator use `safe_git
authorized <authorization-manifest-sha256> --` to stage the
integrated candidate into Git's real index and verify that the authorized
candidate tree equals both the staged and resulting commit trees. It creates the
commit object with the authorized base commit as its explicit sole parent, then
compare-and-swap advances only the explicit authorized full ref. Complete
ref/reflog comparison must prove the expected base-to-new ref update and exact
unconditional branch-plus-symbolic-worktree-`HEAD` reflog appends,
including byte equality for the old/new OIDs, authorized name/email,
authorization-bound `GIT_COMMITTER_DATE` Unix seconds and numeric timezone, and
reason.
