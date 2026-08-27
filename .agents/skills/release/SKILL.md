---
name: release
description: "Prepare the next Gridthorn source release from the current repository slice: bump the version, date the release, reconcile release documentation and AGENTS.md guidance, verify the tree, and generate tag/release text. Use when asked to cut or prepare a release, not for ordinary feature implementation."
---

# Release

Prepare a coherent, verified source release. This workflow updates repository
files but does not create a Git commit, tag, push, or hosted release unless the
user explicitly requests that additional action.

## Read the release scope

1. Inspect Git status, current branch, recent history, and the complete current
   diff. The release covers the current in-scope repository slice; preserve
   unrelated user work and stop if ownership of material changes is ambiguous.
2. Find and read every repository `AGENTS.md`. Apply scoped rules to every file
   touched by the release.
3. Read `Cargo.toml`, `Cargo.lock`, README, CHANGELOG, roadmap, public API
   policy, architecture status, and contributor guidance before deciding the
   release narrative.
4. Derive claims from implemented code and passing tests. Never convert planned
   or provisional behavior into an implemented or stable claim without
   evidence.

## Set version and date

- Read the canonical SemVer from `[workspace.package].version`.
- Unless the user specifies a release level or exact version, increment the
  patch component by one: `X.Y.Z` becomes `X.Y.(Z+1)`.
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
- Repository `AGENTS.md` files: preserve all necessary and scoped instructions,
  remove duplication already supplied by a parent file, eliminate obsolete
  guidance, and tighten wording to reduce token usage without weakening any
  rule. Keep specialized rules near their subtree.

Also update dependency-boundary documentation, contributor guidance, ADRs, or
technology status when the released diff makes them stale.

## Verify the release candidate

Run the platform verification entrypoint: `./scripts/verify.ps1` on Windows or
`sh ./scripts/verify.sh` on macOS/Linux. Run relevant end-to-end workflows and
`git diff --check`. Confirm the lockfile and package metadata report the new
version and search for stale current-version claims.

Do not describe the candidate as ready when a required check fails. Report the
failure and leave commit/tag/publish operations unperformed.

## Generate release output

Build the metadata from the final changelog and verified diff. End the response
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

