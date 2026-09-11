# Quality Gates

Gates run in order against one integration candidate. Passing tests do not
replace either review.

## 1. Verification Gate

- Run task-specific checks for every integrated task.
- Run the full relevant test, build, lint, type, and behavior suite in the
  integration workspace.
- Treat project commands as candidate-influenced code: run them only in the
  constrained process sandbox from `handoff-contract.md`, or present them to the
  user and record user-supplied output when that sandbox is unavailable or the
  command requires network or credentials.
- Reproduce and debug failures at their root cause rather than stacking
  speculative fixes.
- Record exact commands, results, evidence, current HEAD, and the exact
  pre-commit candidate files and diff in external `STATE.md`. The commit gate
  later binds unchanged verified bytes to its resulting SHA.
- Treat isolated-worker results as supporting evidence only.

Verification is current only while the tested candidate remains unchanged.

## 2. Documentation Synchronization Gate

- Run documentation synchronization after every integrated task marked as
  having documentation impact.
- Run it unconditionally before final review.
- Inspect the implemented repository, diff, tests, configuration, and existing
  documentation.
- Update only documentation affected by current behavior.
- Verify documented examples and commands against the repository.
- Record changed documentation, checks, current HEAD, and exact pre-commit
  candidate evidence in external `STATE.md`. The commit gate later binds
  unchanged synchronized documentation bytes to its resulting SHA.
- If documentation synchronization changes any candidate byte, rerun every
  affected task-specific check and the full relevant suite against the updated
  integration candidate. Record this verification refresh after the
  documentation evidence; the commit gate requires both to describe the same
  unchanged bytes.

Documentation describes implemented behavior, not planned behavior.

## 3. User-Authorized Commit Gate

After implementation and affected documentation are synchronized and the final
candidate is verified, only the user-facing coordinator may run non-mutating
`COMMIT_PREPARE` to construct the commit context and authorization evidence.
Only after the user explicitly authorizes that specific commit may it enter
mutating `COMMIT`. Isolated writers
cannot create commits or update refs. Plan approval is not commit authorization.

- Require an existing user-created Git repository. Superplanner never runs
  `git init`. Require trusted non-Git parsing of one identity-checked private
  config snapshot to validate the repository-format, object-format, and
  ref-storage tuple from `integration-protocol.md`. Format 0 requires every
  extension absent, including `objectFormat`, `refStorage`, and
  `worktreeConfig`; format 1 permits absent or one exact `files` value.
  Any invalid tuple, URI/payload, duplicate, conflict, or drift blocks before
  namespace snapshots or mutation. The supervisor keeps those attested config
  bytes and paths immutable to untrusted processes through each Git invocation.
- Before asking, bind the commit to the full `base_sha`, require its tree to
  equal the base tree, and require symbolic `HEAD` naming an explicit full branch
  ref at `base_sha`; detached `HEAD` blocks commit authorization. The full ref
  and its OID are the authorized
  update identity; record the `HEAD` symref separately as a required pre/post
  invariant, not as an update alias. Require the target ref's `%(symref)` field
  to be empty. Snapshot the complete shared-ref namespace,
  `HEAD`, and complete reflog namespace. A tree-equivalent commit or different
  ref is not equivalent.
- Require the real index to equal the base tree. From the external
  candidate index and object directory, record the base and candidate tree OIDs
  and persist the authorization patch only through `safe_git_to
  "$AUTHORIZATION_PATCH" external "$CANDIDATE_INDEX" "$CANDIDATE_OBJECTS"
  "$CANDIDATE_ALTERNATES" -- -C
  <registered-candidate-worktree> diff
  --cached --binary --full-index --no-color --no-renames --no-ext-diff
  --no-textconv --src-prefix=a/ --dst-prefix=b/ <base-tree> --`. Never retain
  routing variables in the ambient environment or invoke raw Git. Hash the exact
  patch bytes using the portable procedure in `integration-protocol.md`.
- Before any real-index `status`, worktree `diff`, or staging command, enumerate
  every cached and untracked path with `ls-files -z --cached --others --` and no
  exclude option; this does not run clean/process filters. Preserve that complete
  inventory for current-worktree attribute checks. Then enumerate the union of
  every path in the base and candidate trees with the explicit
  external route and repeat
  both mechanically parsed `check-attr --source=<candidate-tree>` rejections from
  `integration-protocol.md`, including `--all`, for `filter`, `ident`, `text`,
  `crlf`, `eol`, and `working-tree-encoding`. Enumerate every `.gitattributes`
  path in that union with the external route. When the candidate contains it,
  byte-compare its worktree regular file to the candidate blob with a trusted
  non-Git reader; when the candidate omits it, require it to be absent from the
  worktree. Any unexpected presence, missing file, type change, or byte difference
  blocks. Then apply both attribute checks to the current worktree rules over the
  complete tree union plus the preserved all-path inventory. Require every
  untracked result in that inventory to be an authorized candidate path absent from the base tree with
  candidate-equal bytes; reject every other untracked path, including an
  ignored path or unrelated `.gitattributes`. This command does not run
  clean/process filters. Byte-compare the complete real-index stage-0 `ls-files
  --stage -z` manifest with `ls-tree -r -z --format='%(objectmode) %(objectname)
  0%x09%(path)' <base-tree>`, whose records exactly match that serialization;
  this rejects extra intent-to-add records even though they display an
  uppercase `H` and are omitted by `write-tree`. Also parse `safe_git_to
  <registered-index-flags-output> plain -- -C <registered-candidate-worktree>
  ls-files -v -z --` and require every tracked record to carry the ordinary
  uppercase `H` tag; any assume-unchanged, skip-worktree, unmerged, or other
  exceptional index state blocks. These checks occur before Git inspects worktree content.
  Only after those checks may scrubbed
  `status` or `diff --no-ext-diff --no-textconv` inspect worktree content.
- Store the proposed UTF-8 message in an external file with exactly one terminal
  newline and hash it before asking. Present the complete patch without omitted
  hunks, its external path and SHA-256, base and candidate trees, exact in-scope
  files, exact message bytes and SHA-256, full base commit, exact full target ref
  at the base OID, explicit commit identity, exact author and committer dates in
  `<unix-seconds> <+|-HHMM>` form, expected full commit OID, and current
  verification/documentation evidence. The user authorization must name the
  patch hash, candidate tree, message hash, base commit, target state, both exact
  dates, and expected commit OID.
- Before asking, resolve the commit identity from explicit repository-local
  `user.name`/`user.email` values or ask the user for it; never inherit global
  configuration. Before asking, select exact author and committer dates in Git's
  `<unix-seconds> <+|-HHMM>` internal form. Canonically serialize the expected
  unsigned commit bytes from the authorized tree, sole parent, identities,
  dates, and message, and compute the repository-format object ID without
  writing an object; require a persisted independently routed `hash-object -t
  commit --stdin` no-write operation/result manifest to agree. Define one exact UTF-8 reflog reason with no
  newline. Present that identity, both dates, expected OID, and reason with the
  authorization evidence; the authorization binds all of them as part of the
  displayed target transition.
- Serialize the exact authorized context, full base commit, base/candidate
  trees, authorized path-file path/device-and-inode identity/hash, canonical
  patch path/identity/hash, message path/identity/hash, direct target ref/base
  OID, expected new commit OID, both exact dates, separately recorded `HEAD` symref,
  complete ref/reflog snapshot hashes, identity, reflog reason, and every
  command input/output, reserved internal scratch operand, canonical Git,
  `env`, and transaction-supervisor executable paths/identities/hashes, and exact
  supervisor-source hash, source `files`-backend config identity/hash/evidence,
  the commit-phase private fixed-config content-control Git-directory manifest,
  and reserved absent review-context and post-commit result-manifest paths into an
  external authorization manifest and hash it. Reserve both paths and every
  other leaf while absent, create each once with no-clobber, retain descriptor
  identity through file writes, and reject any precreated file, directory,
  hard-link, symlink, or dangling-symlink object. Every real-index/object/ref call supplies that
  manifest SHA-256; the mediator reopens the manifest through its retained
  identity, deserializes and compares every field without permitting trailing
  data, then rehashes all bound files through identity-checked descriptors before
  each call. The authorization manifest contains the exact ordered authorized
  calls and, for each call, its classifier-policy hash, selected real-or-content
  Git-directory class/path/identity/hash, content manifest/anchor when
  applicable, and complete index/object/alternate route. Every applicable
  operation manifest binds the same values. The mediator injects one distinct
  one-use operation token for each authorized call after validating the
  authorization token. Persist every authorized call's operation manifest and
  identity- and hash-attested terminal result manifest; each terminal result
  binds the operation hash, argv and policy hashes, status, and output hash.
  Static construction evidence alone never proves runtime routing. Git
  storage remains read-only to the caller and is unlocked only inside the
  mediator after this validation, then relocked before return. Shell redirection
  is forbidden; outputs use separately registered mediator-owned external leaves.
- After authorization, require the complete ref/reflog snapshot, symbolic
  `HEAD`, target OID, base commit/tree, candidate identity, patch,
  message, and identity to remain exact. The complete reflog snapshot recursively
  enumerates actual files beneath common-dir `logs` and all
  `worktrees/*/logs`, including orphan names not reachable from current refs.
  Retain the config/routing/namespace-root immutability boundary across that
  trusted no-follow, identity-checked recursive walk and its result publication.
  Re-inspect the candidate with explicit
  `safe_git external ... --` paths and repeat the complete-tree attribute and untracked-path
  checks above before real status. Re-run both mechanically parsed
  content-affecting attribute rejections for every authorized path. For every
  attribute, `ls-files`, status, diff, and staging command, the mediator must
  select the base-anchored authorization-bound private content-control Git
  directory and explicit real index/common-object route with bound
  `alternates=none` and absent on-disk alternate files; persist and revalidate
  those absences for every routed object directory and every transitive alternate
  member. It must not expose repository-local config or
  `info/attributes` to Git. Because that fixed config has no filter drivers, a
  raced `.gitattributes` cannot execute a clean/process command, and any built-in
  conversion makes the mandatory patch/tree comparison fail. Then stage exactly the NUL-delimited authorized path
  file with `safe_git_io <registered-add-output> <authorized-paths> authorized
  <authorization-manifest-sha256> -- -C <registered-candidate-worktree> add
  --all --force --pathspec-from-file=- --pathspec-file-nul`. This scrubbed mode
  starts from an empty environment, uses the empty hooks directory, disables
  signing, and is forbidden before exact authorization. Regenerate the same
  canonical `--no-ext-diff --no-textconv` patch from the real index against the
  authorized base tree and require byte equality and the same SHA-256. Require
  `safe_git_to <registered-write-tree-output> authorized <authorization-manifest-sha256> -- -C <registered-candidate-worktree> write-tree` to equal the authorized candidate tree and
  byte-compare the staged index manifest with that candidate tree again so an
  omitted intent-to-add entry cannot survive staging.
- Create the commit object without consulting or advancing `HEAD`: `safe_git_io
  <registered-new-commit-output> <message-file> authorized
  <authorization-manifest-sha256> -- -C <registered-candidate-worktree> -c user.name=<authorized-name> -c
  user.email=<authorized-email> commit-tree <authorized-candidate-tree> -p
  <authorized-base-sha> -F -`. Before changing a ref, parse the new
  commit object and require exactly one parent equal to the authorized base SHA,
  the authorized tree, the presented author/committer identity, and raw message
  bytes equal to the authorized file. For `commit-tree`, the mediator also sets
  exact `GIT_AUTHOR_NAME`, `GIT_AUTHOR_EMAIL`, `GIT_COMMITTER_NAME`, and
  `GIT_COMMITTER_EMAIL`, `GIT_AUTHOR_DATE`, and `GIT_COMMITTER_DATE` values from
  authorization inside `env -i`; these outrank
  repository-local `author.*` or `committer.*` values. The safe profile pins
  commit and log encodings to UTF-8. Require the raw header block to contain only
  the expected tree, sole parent, author, and committer lines in that order;
  reject `encoding`, signature, mergetag, or any other unexpected header. Require
  the parsed full commit OID to equal the pre-authorized expected OID before any
  ref operation; runtime result binding cannot add a new OID to the immutable
  authorization.
- Advance only the authorized target with compare-and-swap and the authorized
  reflog identity: `safe_git_to <registered-update-ref-output> authorized <authorization-manifest-sha256> -- -C <registered-candidate-worktree> -c
  user.name=<authorized-name> -c user.email=<authorized-email> update-ref
  --no-deref --create-reflog -m <authorized-reason> <authorized-full-ref> <authorized-expected-new-commit>
  <authorized-base-sha>`. This is the closed client request, not caller-provided
  stdin. The mediator translates it to a fixed one-ref `update-ref --stdin`
  transaction: `start`, `option no-deref`, the exact update/CAS, and `prepare`.
  The scrubbed `update-ref --stdin` process retains the request's global
  `--create-reflog` and `-m` options; those options are not translated into stdin
  commands or discarded.
  A private transaction supervisor, launched by canonical executable through
  `env -i` with Perl option/library injection absent and its exact source hash
  revalidated, launches the scrubbed Git argv with anonymous
  input/output pipes, drives that fixed grammar, and writes each acknowledgement
  to the registered transcript descriptor. No pathname IPC object exists for
  another process to replace, redirect, or precreate, and Git inherits neither
  mediator artifact descriptors nor caller-controlled stdin.
  After Git reports `prepare: ok` and holds the ref lock, the mediator uses its
  internal pinned read route to require the target is not symbolic and still
  resolves to the base OID; it sends `abort` on either mismatch and `commit` only
  when both pass. Persist the transcript path/identity/SHA-256, raw supervisor
  exit status, distinct final mediator status, acknowledgements,
  and a deterministic direct, symbolic,
  OID-mismatch, or inspection-error outcome. Caller-controlled stdin and additional ref operations are
  impossible. The explicit full branch ref, not dereferenced `HEAD`, is the
  symbolic-worktree target. The mediator sets exact
  authorized `GIT_COMMITTER_NAME`, `GIT_COMMITTER_EMAIL`, and exact
  authorization-bound `GIT_COMMITTER_DATE` values for `update-ref` so neither
  its reflog identity nor timestamp can come from local config or runtime. Require the separately recorded `HEAD` symref text to match before
  the transaction and remain unchanged afterward. After the request passes its
  closed command grammar, consume its authorization before rechecking namespace
  state or starting the transaction. Any snapshot, prepare, type, OID, or transaction mismatch therefore blocks without retrying against
  another parent or ref; any post-command `HEAD`-invariant mismatch blocks review
  and requires explicit user direction without rolling back the authorized ref
  update.
- Record the full resulting commit SHA and commit evidence in external
  `STATE.md`, and require that SHA to equal the authorization-bound expected OID.
- Confirm the committed tree contains the exact verified implementation and
  documentation bytes, then bind verification and documentation evidence to the
  resulting SHA. Require the resulting commit tree to equal both the authorized
  candidate tree and verified staged tree. Extract the raw message bytes from
  the commit object and require them to equal the authorized message file. Any
  unexpected staging or commit-time action blocks `COMMIT`: do not review or
  present a push command for that commit, and require explicit user direction before any rollback or newly
  presented patch/message and authorization.
- Compare the complete ref and reflog namespaces with the pre-authorization
  snapshots. Persist and compare the exact path and pre/open/post identity of the
  `refs`, common `logs`, and every `worktrees/*/logs` walker root plus each walk
  verdict, and the optional `packed-refs` leaf path/identity and verdict. No ref
  except the authorized target may change. Its old/new OIDs must be exactly
  base/new. The mandated direct branch update must unconditionally append exactly
  one parsed base-to-new entry to both the authorized branch reflog and the
  symbolically attached worktree's `HEAD` reflog. Byte-compare each complete
  appended line with the authorized base/new OIDs, name/email, exact committer
  date and numeric timezone, and exact reason; all other ref and reflog bytes
  remain identical. Revalidate the bound `files` backend immediately before and
  after this comparison while retaining the same config/routing/namespace-root
  immutability boundary through publication; backend drift is a blocker, never
  a reason to infer an empty or partial namespace.
- Retire the base-anchored content directory after the target update. Create and
  attest a new immutable review content directory for the same context with
  detached `HEAD` anchored to the resulting full commit SHA. All review
  content/diff calls use that directory and their recorded read-only real
  index/common-object route; a pre-commit content manifest is not valid review
  evidence. Construct it only at the authorization-reserved absent context path
  and publish its result manifest once at the separately reserved absent leaf
  with no-clobber semantics. Bind the published path/device/inode identity,
  exact bytes and SHA-256, authorization-manifest hash, equal expected/resulting
  OID, context path, construction evidence, and validation result; review binds
  both hashes rather than pretending the post-commit manifest existed before
  authorization.
- Never push, merge, publish, or open a pull request in `COMMIT`.

Without Git or explicit authorization, stop before commit. Non-Git work may use
external state for discovery, design, and planning, but retained `EXECUTE`, this
gate, SHA-bound review, and push-command eligibility require a Git repository created by the user.
Standard review requires a committed SHA; it never approves an uncommitted
candidate. Require the complete real-index manifest to equal the committed tree;
any unrelated working-tree change, extra intent-to-add record, or non-ordinary
`ls-files -v -z` index tag remains unreviewed and blocks review. Do not discard
or include it merely to make the worktree clean.

## 4. Standard Review Gate

Run the standard code reviewer first in the attested fresh, private, read-only
reviewer envelope from `handoff-contract.md`. Supply the
full committed candidate SHA and diff plus the approved design, feature, task,
separate Gherkin acceptance, implementation plan, verification, and
documentation evidence.

The reviewer permits `git --no-replace-objects --no-pager --no-optional-locks
cat-file commit <exact-full-SHA>` only through the attested Git mediator and only
for the supplied literal object ID. The mediator also sets
`GIT_NO_REPLACE_OBJECTS=1`. It compares bytes after the first empty commit-header
separator through end of object with the authorized external message file. The
required diff/show commands remain executable because the allowlist itself
requires `--no-ext-diff --no-textconv`. Later `*--ext-diff*` and
`*--textconv*` rules reject the positive option spellings; permission-resolution
checks must prove those patterns do not match the required `--no-*` forms.
Reviewer `log` is not allowed. The mediator enforces a closed per-subcommand
token grammar for the exact authorization base, review SHA, and contained paths;
unlisted options or operands are rejected.
The mediator and permission rules reject `--no-index`; every path operand must
canonicalize inside the attested review projection without symlink escape.
They also reject `--format`, `--pretty`, `%G*`, and signature-verification
options; `show` receives final `--format=medium`, `--date=iso-strict`,
`--no-use-mailmap`, `--no-notes`, and no-signature options. The profile pins the
matching pretty/date settings, disables mailmap sources, and sets
`log.showSignature=false`.

The reviewer reports findings and does not edit. If Python files changed, the
reviewer must load `reviewing-py-code` as an additional lens. The adversarial
review cannot start until the standard review has no unresolved, unaccepted
findings and approves the candidate SHA.

Its handoff uses `status: complete` with `review_result: approved|findings`, or
`status: blocked` with `review_result: not-completed`, and records
`completion_time`, findings, and `accepted_risks`. A completed review records the
exact full `reviewed_sha`; a blocked review may record `none` only when no full
SHA could be established and its blocker explains why.

## 5. Kimi Adversarial Review Gate

After standard approval, start a fresh `openrouter/moonshotai/kimi-k3` agent at
variant `max` in a new attested private read-only reviewer envelope and load the
authenticated `adversarial-reviewer` skill. It attempts to disprove:

- Specification and acceptance compliance.
- Functional correctness and failure behavior.
- Design quality and maintainability.
- Data and network efficiency.

The adversarial reviewer is read-only and must review the exact SHA approved by
the standard reviewer. It independently reopens, identity-checks, and rehashes
the registered immutable standard-review record. Its handoff records that exact
record ID, identity, and hash, plus the same SHA as both `reviewed_sha` and
`standard_approval_sha` and its `completion_time`; its status and review-result
enums are identical to standard review.

## SHA Binding And Invalidation

Every review record includes review type, agent, model and variant,
`completion_time`, `review_result` (`approved|findings|not-completed`), evidence
path, findings, `accepted_risks`, and the exact full `reviewed_sha`. The
adversarial record also includes the exact standard-review record ID, identity,
recomputed hash, and `standard_approval_sha`. Approval is valid only when both
reviews approve the same current SHA and the adversarial record points to that
exact registered standard approval.

Any candidate-file change after either review, including code, tests,
configuration, generated output, design or planning artifacts, and
documentation, invalidates all review approvals even when the change appears
unrelated or only addresses a finding. Re-run verification as affected,
synchronize documentation, refresh verification if documentation changes
candidate bytes, and rerun non-mutating `COMMIT_PREPARE`. Present the new
canonical patch hash, candidate tree, message hash, full base commit, target
state, exact author and committer dates in numeric-timezone form, and expected
full OID for explicit user commit authorization, run coordinator `COMMIT`, and restart at
standard review only for the resulting clean new SHA. An uncommitted candidate
working-tree change is unreviewed content and blocks the push-command gate.

External `STATE.md` and operational matrices, checkpoints, and persisted
handoffs are outside candidate content. Updating them does not invalidate
review, alter the review SHA, appear in the review diff, or dirty the candidate
worktree. If an operational file is accidentally placed or tracked under the
candidate root, the state-path contract is violated and review cannot proceed
until it is relocated. Removing tracked candidate state is itself a candidate
change, so it invalidates prior approval and requires review of the new SHA.

## Findings And Accepted Risk

- Resolve a finding with a change, or obtain explicit user acceptance of the
  specific risk.
- Record accepted risk with the finding, severity, consequence, rationale, user
  decision evidence, reviewed SHA, and scope.
- Keep every accepted finding in the reviewer `findings` list and add a matching
  `accepted_risks` entry. Do not replace or suppress the finding with the risk
  record.
- `review_result: approved` may coexist with accepted risks only when each risk
  has explicit user evidence for the exact reviewed SHA and scope and no
  unresolved or unaccepted finding remains.
- User risk acceptance closes only that named finding. It does not constitute a
  reviewer approval, approve another SHA, waive verification or documentation,
  permit hidden working-tree changes, or request a push-command presentation.
- A change made while resolving a finding invalidates prior approvals and
  restarts the sequence.

## Push Command Gate

Superplanner never executes a push, and the Git mediator has no push or
transport route. It may present one exact user-executed command only when all
conditions are true:

1. Verification is current for the candidate.
2. Documentation synchronization is current.
3. The current SHA equals the SHA returned by coordinator `COMMIT` for the exact
   user-authorized patch, candidate tree, and message, and its commit tree and
   message bytes equal the authorized candidate tree and message.
4. Standard review approves the current SHA.
5. Kimi adversarial review approves the same SHA.
6. The worktree contains no unreviewed changes.
7. After both exact review records exist, the user explicitly authorizes the
   target-bound `push-presentation-request-v1` record described below. That
   request authorizes command presentation only, not Superplanner execution.
8. Eligibility evidence binds the destination remote name as provenance, exactly
   one effective push URL, the full `<full-local-ref>:<full-remote-ref>` refspec,
   the expected full local OID in the repository's object format, an immutable
   push-only Git directory and execution-custody lease described below, the
   attested Git executable/loader/helper closure, and a no-force policy
   forbidding a leading `+`, `--force`, `--force-with-lease`, and every other
   force form.
9. Immediately before presentation, the bound full local ref still resolves to
   the expected local OID and every preceding condition remains current.

Resolve the destination from one trusted non-Git parsed, identity-checked config
snapshot. Use all `remote.<name>.pushurl` values when present, otherwise all
`remote.<name>.url` values, and require exactly one value. Reject every
`url.*.pushInsteadOf` and `url.*.insteadOf` entry rather than trying to emulate
runtime rewriting. The selected value must match the closed helper-free HTTPS
grammar `https://<dns-host>[:<1-65535>]/<path>`, with no userinfo, query,
fragment, backslash, whitespace, control byte, DEL, `::`, IPv6 literal, or
unknown scheme. Record the remote name as provenance and this literal URL; a
missing, multiple, rewritten, helper-based, non-HTTPS, or changed destination
blocks presentation. A blocked presentation never prevents the user from
choosing and running their own command outside Superplanner.

Every named `push-*-v1` record below uses the same canonical encoding. It starts
with its ASCII schema tag as a scalar, then an eight-byte unsigned big-endian
top-level field count. Each scalar field is byte `0x00`, an eight-byte unsigned
big-endian byte length, and exactly that many raw bytes. Each collection field is
byte `0x01`, an eight-byte unsigned big-endian item count, then its items; each
item starts with an eight-byte unsigned big-endian scalar-field count followed by
that schema's fixed-order scalar fields. Nested collections, optional fields,
extra fields, duplicate set items, and noncanonical ordering are invalid. An
empty collection has count zero.

After both approvals and destination resolution, the trusted user-decision
recorder may create a registered immutable `push-presentation-request-v1` only
from an authenticated explicit user action. Its closed deterministic payload
binds the user-decision record ID/hash, exact commit-result ID/hash and full OID,
standard-review record ID/identity/hash, adversarial-review record
ID/identity/hash, destination-snapshot ID/hash, remote name, literal URL, full
source and destination refs, full refspec, and no-force policy. The adversarial
record must itself bind the exact standard-review record ID, identity, and hash.
The request is valid only after both bound reviews approved that OID. A generic,
stale, earlier, differently targeted, or caller-created request blocks
presentation.

Validate both refs with the closed Git mediator's `check-ref-format`, reject a
leading `+`, and reject NUL, CR, LF, every other ASCII control byte, and DEL in
every rendered operand. Through the supervisor's local-only `push-prepare`
operation, create a new push-only Git directory outside the candidate and every
repository Git directory. The typed request accepts registered immutable record
IDs, not caller-asserted hashes, for the source context, object route,
authorization manifest, coordinator `COMMIT` result, both review results,
current verification result, documentation-synchronization result, explicit user
presentation request, and trusted destination-config snapshot.
For each ID, the supervisor resolves the registry's canonical path, expected
identity, and expected hash; opens the path without following links; revalidates
its ancestry and immutability; and recomputes its hash. It then validates that
the authorization's base/tree/message/dates/target/expected OID equal the
`COMMIT` inputs and result, both reviews approve that same full OID, the
adversarial result binds the exact standard-review record ID/identity/hash, the
source ref still equals that OID, the canonical source common directory's
`shallow` path remains absent as every node type under its registered immutable
boundary, verification passed for the exact commit tree,
documentation is synchronized to that tree, and the presentation request binds
those exact records and destination evidence. It derives the approved refs,
object/ref format, destination, and paths from resolved records. The custody root
comes only from registered supervisor configuration selected by service policy,
never a caller path. Every record must be current. The supervisor itself also
captures and registers an immediate clean-worktree snapshot proving the real
index tree equals the commit tree and the complete non-credential candidate
projection has no tracked, untracked, or ignored difference. A mismatch blocks
preparation. Ambient coordinator state, standalone booleans, and unresolved
caller-provided digests are not evidence.

Only after those checks, the supervisor creates the complete
`push-eligibility-v1` manifest. Its top-level fields contain, in order, a resolved-
input collection whose items each contain record type, ID, identity, and
recomputed SHA-256 hash in request-schema order;
the generated clean-worktree snapshot ID, identity, and hash; approved full SHA
and source ref; remote-name provenance, URL, and refspec; object/ref format;
no-force policy; registered `push-layout-v1`, `push-execution-closure-v1`, and
`push-command-v1` record IDs, identities, and recomputed hashes; custody-root
identity; and lease ID. The typed result returns the registered eligibility ID
and recomputed hash plus those resolved bindings, but command bytes are released
only through `push-present` below.

The push-only directory has exactly `HEAD`, `config`, one loose
`<full-local-ref>` file, `objects/`, `objects/info/`, `objects/pack/`, `refs/`,
the directories needed by that one ref, and loose object files named
`objects/<first-two-hex>/<remaining-hex>`. `HEAD` contains exactly `ref:
<full-local-ref>` followed by one LF; the ref file contains exactly the expected
full OID followed by one LF;
`objects/info/` and `objects/pack/` are empty. `packed-refs`, `shallow`, grafts,
replace refs, hooks, alternate files, remotes, and every unlisted entry are
absent. The canonical `config` bytes, including one terminal newline, are:

```gitconfig
[core]
repositoryformatversion = 0
bare = true
logallrefupdates = false
```

for SHA-1, and:

```gitconfig
[core]
repositoryformatversion = 1
bare = true
logallrefupdates = false
[extensions]
objectformat = sha256
refstorage = files
```

for SHA-256. No other config byte is allowed. Walk the source commit with
replacement objects and lazy fetching disabled, materialize every reachable
object by verified type and raw bytes as a new loose object, and require each
resulting OID to equal the source OID. Enumerate all destination objects and
require exact set equality with that reachable closure, with no extras. Reject
thin packs, packs of any kind, symlinks, hard links, special nodes, and both
alternate-file forms. Finally verify the ref, commit, complete ancestry, trees,
blobs, and strict object integrity in a sandbox where every source-repository
path and network access is unavailable.

Register the generated directory evidence as deterministic `push-layout-v1`.
Its top-level fields contain, in order, object/ref format; Git-directory
path/identity; exact `HEAD`, config, and loose-ref paths/identities/bytes/hashes;
the directory collection ordered by raw path bytes with each item containing
path, identity, and mode; the object collection ordered by OID bytes with each
item containing path, identity, type, size, OID, and raw-content hash; source
reachable-closure hash;
exact-set/no-extra/no-link verdicts; and source-unavailable, network-denied,
ancestry, and strict-integrity verification record IDs/hashes. Register its
canonical path, identity, and recomputed SHA-256; a bare caller-supplied layout
hash is invalid.

Place the Git directory and a private Git exec-path under a supervisor-owned
custody root whose owner is a distinct OS principal. The user, OpenCode, and
candidate processes have read/execute access only; every ancestor, directory,
config/ref/object, executable, loader, and library is identity/hash attested,
link-free, and not writable by those principals. The exec-path manifest is a
closed allowlist containing the attested main `git` and every possible local
child executable for this exact Git build and HTTPS push, including at minimum
`git-remote-https`, its local delegate if any, and `git-pack-objects`. Put an
attested `git` executable in that private exec directory and set execution
`PATH` to exactly that directory, so Git cannot append or search a compiled
default path. Reject any process edge outside the allowlist, any unmanifested
helper, shell fallback, or script whose interpreter is not also in the closure.

Register this evidence as deterministic `push-execution-closure-v1`. Its
top-level fields contain, in order,
attested `env` and main-Git paths/identities/hashes; private exec-path identity;
exact `PATH`; a node collection ordered by raw path bytes with each item
containing kind, path, identity, mode, and hash; an edge collection ordered by
parent-node ID then child-node ID with each item containing parent-node ID,
child-node ID, and invocation mode; exact Git
build/version identity; and the closed-process-graph verification ID/hash.
Register its canonical path, identity, and recomputed SHA-256. A bare exec-path
or helper hash is invalid. The supervisor holds a no-replace/no-mutation custody
lease from final validation until retirement. Because no workflow process observes
execution, the lease retains the bundle unchanged after presentation. Retirement
requires a registered immutable `push-custody-release-v1` record produced by the
trusted user-decision recorder from an authenticated explicit user action, not
by the `push-retire` caller. Its closed payload contains only the schema tag,
user-decision record ID/hash, eligibility-manifest ID, lease ID, and literal
`retire` action; it has no execution or outcome field. Under the lease's exclusive
lock, `push-retire` resolves and rehashes that record, validates both bound IDs
and the user evidence, and first registers a no-clobber
`push-retire-operation-v1` in `prepared` state containing the operation ID,
release-record ID/hash, lease and eligibility IDs, exact active and cleanup
paths/identities, and expected state. The authoritative one-use linearization
point is an atomic no-replace rename of the bundle on the same filesystem from
the presented path into that operation's supervisor-only cleanup path. A failed
rename leaves the lease `active`, release `unused`, and bundle unchanged. Once
the rename succeeds, the presented path is revoked and logical state is
`retiring` even if the process stops before metadata publication. Recovery under
the same operation ID either retries the rename when state is `prepared`, the
active path has its exact identity, and the cleanup path is absent, or recognizes
the absent active path plus exact cleanup-path identity, idempotently compare-and-
swaps the release to `consumed` and lease to `retiring`, and continues deletion
without another rename or release. Any existing nonterminal operation for that
lease or release must be resumed; a different operation ID is rejected.
Interruption, metadata failure, or deletion failure after rename records or
recovers as `cleanup-failed`, with the bundle inaccessible; retry under that
operation ID resumes cleanup. Only complete deletion records `retired`. No
release can name another lease or operation, and
retiring/cleanup-failed/retired state blocks presentation.

Revalidate the complete manifest, verification/documentation records, clean
worktree state, source ref, executable closure, ancestry, and active custody
lease immediately before presentation. Bind the attested absolute
`env` and Git executables. Render every dynamic operand as a POSIX single-quoted
shell word, escaping an embedded single quote as `'\''`.
The presented command must start the exact Git binary through `<absolute-env>
-i`, set `PATH=<absolute-attested-private-exec-path>`, `LC_ALL=C`, `LANG=C`,
`GIT_CONFIG_NOSYSTEM=1`,
`GIT_CONFIG_SYSTEM=/dev/null`, `GIT_CONFIG_GLOBAL=/dev/null`,
and `GIT_TERMINAL_PROMPT=1`, and supply only these environment config entries:

- SHA-1: `GIT_CONFIG_COUNT=4`, then `core.repositoryformatversion=0`,
  `core.hookspath=/dev/null`,
  `safe.directory=<absolute-push-only-git-directory>`, and
  `http.followRedirects=false`.
- SHA-256: `GIT_CONFIG_COUNT=6`, then `core.repositoryformatversion=1`,
  `core.hookspath=/dev/null`, `extensions.objectformat=sha256`,
  `extensions.refstorage=files`,
  `safe.directory=<absolute-push-only-git-directory>`, and
  `http.followRedirects=false`.

Use the indexed `GIT_CONFIG_KEY_<n>`/`GIT_CONFIG_VALUE_<n>` environment form in
that exact order. The canonical safe-directory path must equal the
eligibility-bound custody path. This suppresses system and global config,
disables hooks and redirects, and
loads only the separately attested push-only directory's exact local config;
the candidate repository's local config is never in the execution path. The
only presented operation is `'<absolute-git>'
'--exec-path=<absolute-attested-git-exec-path>' --no-replace-objects
--no-lazy-fetch '--git-dir=<absolute-push-only-git-directory>' push --no-force --
'<effective-https-url>' '<full-local-ref>:<full-remote-ref>'` inside that fixed
environment prefix and active custody lease. The user may execute the complete
command outside Superplanner.

`push-prepare` serializes that exact environment vector, argv vector, and POSIX
rendering into registered deterministic `push-command-v1`. Its top-level fields
contain, in order, an environment collection whose items contain ordinal, key,
and value; an argv collection whose items contain ordinal and raw argument; the
exact POSIX-rendered bytes; and all
canonical paths, `PATH`, indexed config entries, URL/refspec operands, lease ID,
and the source, presentation-request, layout, and execution-closure record
IDs/hashes. Its registered ID, canonical path identity, and recomputed hash are
included in `push-eligibility-v1`.

Only the supervisor's local-only `push-present` operation may release the command
to the user. It accepts the eligibility-record ID, resolves and rehashes the full
record graph, captures a fresh clean-worktree snapshot, repeats every immediate
pre-presentation check, and performs no network or transport action; any Git
metadata check uses only the read-only safe-Git mediator routes.
On success it registers deterministic `push-presentation-v1` whose top-level
fields contain eligibility and command record IDs/identities/hashes; fresh clean-
worktree snapshot ID/identity/hash; recomputed complete-record-graph hash;
the final-check collection in fixed check-name order with each item containing
check name, verdict, and evidence-record ID/hash; output-guard identity/hash; and
exact released-byte hash.
It passes the exact stored command bytes through that trusted output guard
directly to the user-facing response. The model receives no editable command field; the
coordinator may request this operation but cannot render, truncate, prefix,
append, or otherwise alter the guarded bytes. Missing output-guard support or
any byte mismatch blocks presentation. Never add a mediator push route or
execute, broker, retry, observe, or report execution of that command. Approval
of a design, plan, commit, review finding, or risk is not a request to present it.
