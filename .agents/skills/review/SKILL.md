---
name: review
description: "Review a Gridthorn change or pull request for behavioral defects, architecture violations, compatibility risks, and missing regression coverage. Does not apply fixes."
---

# Review

Review the requested scope without modifying it. Load applicable instructions
and use [context routing](../../../docs/AI_WORKFLOW.md) for affected contracts.

- Establish the comparison first: a specified commit/range, PR base, staged
  change, or working-tree change. For branch review, resolve the intended base
  and use its merge base; do not silently choose an unrelated branch. State the
  resolved base and whether staged, unstaged, or untracked changes are included.
- Read the complete in-scope diff, then inspect callers, tests, and contracts
  needed to evaluate it. For cross-repository changes, establish each base and
  instruction scope separately.
- Check concrete behavioral failures, lifecycle/error handling, dependency
  direction, public defaults/order/formats, and compatibility. Tie missing tests
  to a plausible uncovered failure rather than requesting coverage mechanically.
- Use focused read-only reproduction or tests when needed. Full verification
  is required only if the review also becomes an authorized implementation task.

Report actionable findings first, ordered by severity. Each finding names its
priority, verified file/line, trigger, consequence, and evidence; distinguish a
confirmed defect from an unresolved suspicion. Omit cosmetic preferences unless
they violate a required contract. Finish with scope, checks, and residual gaps;
if no actionable findings exist, say so without claiming the change is proven safe.
