---
name: implement
description: "Implement a requested Gridthorn feature end to end: load scoped AGENTS.md guidance, change domain-owned code, add focused tests for new behavior, and run existing verification. Use for feature additions and behavior changes, not read-only reviews or diagnosis."
---

# Implement

Deliver the requested behavior as a tested repository change.

## Establish scope

1. Inspect the working tree and preserve unrelated user changes.
2. Locate every `AGENTS.md` from the repository root through each directory
   that may be read or edited. Read those files completely before editing that
   scope; more specific guidance supplements or overrides parent guidance.
3. Read the relevant roadmap, architecture, public API, and domain files. Treat
   documented invariants and dependency boundaries as implementation
   requirements.
4. Resolve minor ambiguity from repository context. Ask only when a missing
   choice would materially change behavior, compatibility, or scope.

## Implement the feature

- Keep implementation in the owning domain. Create focused source files rather
  than growing entrypoints, crate roots, generic utilities, or unrelated
  modules.
- Preserve public and dependency boundaries. Add dependencies only when the
  requested feature needs them and update boundary policy in the same change.
- Follow the repository comment policy: declaration-level documentation is
  normal; comments inside executable blocks require a genuinely non-obvious
  invariant or workaround.
- Provide contextual diagnostics for relevant failure paths. Do not present
  provisional behavior as stable.

## Test the behavior

- Add or update focused tests for every new observable behavior and important
  failure path.
- Store tests under the owning `src/<domain>/test/` subtree and register them
  with `#[cfg(test)]`. Do not create crate-wide `tests/` buckets.
- Prefer testing through the narrowest stable domain contract. For CLI,
  templates, or process workflows, include an end-to-end test when the external
  behavior would otherwise remain unproven.
- Run targeted tests while iterating, then run all existing repository checks;
  new tests never replace the existing suite.

## Verify and report

Run `./scripts/verify.ps1` on Windows or `sh ./scripts/verify.sh` on macOS/Linux.
If the platform entrypoint cannot run, execute its equivalent checks and state
the limitation. Exercise changed CLI/template behavior end to end.

Update README, roadmap, changelog, public API documentation, or ADRs only where
the implemented behavior changes their truth. Mark roadmap work complete only
when its documented definition of done is satisfied.

Report the implemented outcome, important files, tests run, and any remaining
risk or deferred work. Do not claim checks that were not executed.

