---
name: diagnose
description: "Reproduce and diagnose Gridthorn failures, regressions, or unexpected CLI/runtime behavior; apply a focused fix when requested. Excludes general feature development."
---

# Diagnose

Use applicable `AGENTS.md` and [context routing](../../../docs/AI_WORKFLOW.md)
to investigate the failing domain without widening the task.

1. Capture the expected/observed behavior, exact command or scenario, platform,
   relevant versions, and useful diagnostics. Preserve unrelated working changes.
   Request missing reproduction inputs only when they block meaningful progress.
2. Reproduce with the smallest existing test, CLI project, scenario, or example.
   For window/device failures, preserve the real platform conditions and use
   documented smoke paths where applicable; a headless pass cannot prove them.
3. Narrow the boundary and test a specific hypothesis. Separate observed evidence
   from inference; avoid repeated broad logs, speculative rewrites, or retries
   against unchanged inputs. If reproduction fails, record what was tried and
   the evidence needed next rather than claiming a root cause.
4. For an authorized fix, change the owning domain, add a regression test that
   demonstrates the failure when feasible, rerun reproduction, and complete
   required verification. Diagnosis-only requests end with findings and a
   concrete proposed fix, without applying it.

Report the cause and evidence, reproduction outcome, any fix, checks, and
remaining platform limits. Use a checkpoint for investigations spanning sessions.
