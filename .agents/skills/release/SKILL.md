---
name: release
description: "Prepare a Gridthorn source release: establish the release range, reconcile version and documentation, verify, and generate release text. Does not publish."
---

# Release

Prepare a coherent, verified source release. This workflow updates repository
files but does not create a Git commit, tag, push, or hosted release unless the
user explicitly requests that additional action.

## Read the release scope

1. Inspect status, branch, release tags, and changelog. Use the user's base when
   supplied; otherwise identify the latest reachable actual release tag by its
   version and history, not an arbitrary recent commit or tag date. If no reliable
   base exists, ask for it or explicit initial-release scope. Record the resolved
   base SHA and candidate HEAD.
2. Review committed changes from base to HEAD, then staged and unstaged diffs
   and relevant untracked files separately. Include only the requested release
   scope; preserve unrelated work and resolve ambiguous ownership before inclusion.
   A clean working tree does not mean an empty release.
3. Load affected instruction chains once. Use
   [context routing](../../../docs/AI_WORKFLOW.md) for current README status,
   changelog, relevant roadmap outcomes, architecture/domain contracts, and public
   API policy. Inspect workspace Cargo metadata, lockfile diffs, and relevant
   package records; read the full lockfile only for dependency investigation.
4. Derive claims from implemented code and passing tests. Never convert planned
   or provisional behavior into an implemented or stable claim without
   evidence.
5. Reconcile Unreleased entries against the complete range. Report a mismatch
   between canonical workspace version and release history before choosing the
   next version; do not silently release below an already published version.

## Set version and date

- Read the canonical SemVer from `[workspace.package].version`.
- Unless the user specifies a release level or exact version, increment the
  patch component by one for compatible changes: `X.Y.Z` becomes `X.Y.(Z+1)`.
- Apply `docs/PUBLIC_API_POLICY.md`: breaking changes before 1.0 require a minor
  bump, and after 1.0 a major bump. Resolve an explicitly requested version that
  conflicts with compatibility policy before changing metadata.
- Use the local calendar date at skill execution time in ISO `YYYY-MM-DD`
  format. Do not reuse an earlier changelog date or infer a date from Git.
- Update every authoritative version occurrence required for a consistent
  workspace and regenerate lockfile metadata through Cargo. Avoid replacing
  historical version references or compatibility examples that should remain
  unchanged.

## Reconcile release documentation

Review and update all of the following as one coherent release change:

- `README.md`: current implementation/release status, supported commands, and
  examples must match the released slice.
- `CHANGELOG.md`: keep an empty `Unreleased` section for future work and move
  the current entries into `## [X.Y.Z] - YYYY-MM-DD`. Describe user-visible
  additions, changes, fixes, and removals from the actual diff.
- `docs/ROADMAP.md`: check only completed outcomes and keep remaining work
  explicit.
- `docs/PUBLIC_API_POLICY.md`: update current compatibility/version statements
  and ensure the release does not promise stability beyond the policy.
- Affected `AGENTS.md` files: update instructions made stale by released behavior.
  Broader instruction optimization belongs in a separate workflow change.

Also update dependency-boundary documentation, contributor guidance, ADRs, or
technology status when the released diff makes them stale.

## Verify the release candidate

Run the platform verification entrypoint: `./scripts/verify.ps1` on Windows or
`sh ./scripts/verify.sh` on macOS/Linux, always in full mode for a release.
Verify affected sibling example packages separately. Run relevant workflows and
`git diff --check`. Confirm the lockfile and package metadata report the new
version and search for stale current-version claims.

Do not describe the candidate as ready when a required check fails. Report the
failure and leave commit/tag/publish operations unperformed.

## Generate release output

State the base SHA, candidate HEAD, and included working-tree scope. Build
metadata from the final changelog and verified release range. End the response
with these copy-ready fields:

```text
Version: vX.Y.Z
Release date: YYYY-MM-DD

Tag message:
Gridthorn vX.Y.Z

Release title:
Gridthorn vX.Y.Z

Release description:
<concise Markdown summary with Highlights, Compatibility, and Verification>
```

The description must mention breaking or compatibility-relevant changes, or
state that none were identified. List the checks actually executed. Do not
invent issue links, contributors, artifacts, checksums, or publication status.
