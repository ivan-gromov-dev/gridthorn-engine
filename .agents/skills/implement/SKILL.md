---
name: implement
description: "Implement Gridthorn features and behavior changes with domain tests and repository verification. Excludes read-only review and root-cause diagnosis."
---

# Implement

Deliver the requested behavior as a tested change under applicable `AGENTS.md`.

## Establish scope

1. Inspect the working tree and preserve unrelated user changes.
2. Load the instruction chain for affected scopes once; extend it as ownership
   becomes clear instead of loading instructions for every file read.
3. Use [context routing](../../../docs/AI_WORKFLOW.md) for the relevant roadmap
   slice, architecture ownership rules, public API policy, and domain contracts.
4. Resolve minor ambiguity from repository context. Ask only when a missing
   choice would materially change behavior, compatibility, or scope.

## Implement the feature

- Provide contextual diagnostics for relevant failure paths. Do not present
  provisional behavior as stable.

## Test the behavior

- Add or update focused tests for every new observable behavior and important
  failure path.
- Prefer testing through the narrowest stable domain contract. For CLI,
  templates, or process workflows, include an end-to-end test when the external
  behavior would otherwise remain unproven.
- Run targeted tests while iterating, then run all existing repository checks;
  new tests never replace the existing suite.
- When changing a sibling example, load that repository's instructions and
  inspect its working tree separately. Build/test the affected package through
  its manifest and exercise its public workflow or documented smoke mode.

## Verify and report

Run the required platform verification from root `AGENTS.md`.
If the platform entrypoint cannot run, execute its equivalent checks and state
the limitation. Exercise changed CLI/template behavior end to end.

Update README, roadmap, changelog, public API documentation, or ADRs only where
the implemented behavior changes their truth. Mark roadmap work complete only
when its documented definition of done is satisfied.

Use the conditional checkpoint in context routing for long tasks. Report the
outcome, important files, checks, and remaining limits. When both repositories
changed, report their diffs and verification separately.

