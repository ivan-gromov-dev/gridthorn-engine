# Gridthorn Repository Instructions

These instructions apply to the entire repository. A nested `AGENTS.md` may add
more specific rules for its subtree.

## Working agreement

- Follow `docs/ROADMAP.md` in risk-reduction order and keep every increment
  buildable, tested, and documented.
- Treat `docs/ARCHITECTURE.md` invariants as requirements. Record durable
  exceptions or difficult-to-reverse decisions through the ADR process.
- Preserve a clean dependency direction. Run the boundary checker whenever a
  crate or local dependency edge changes.
- Do not present planned behavior as implemented. Update README and roadmap
  status in the same change that implements or removes a capability.
- Keep changes focused. Do not add dependencies or subsystems before the active
  milestone needs them.

## Source organization

- Organize source files by product domain and responsibility, not by incidental
  type or vague utility categories.
- Keep crate roots, binary entrypoints, and `mod.rs` files small. They should
  declare modules, define the public surface, or perform composition only.
- Do not accumulate implementation in generic files such as `lib.rs`,
  `main.rs`, `common.rs`, `utils.rs`, or `helpers.rs` unless the code genuinely
  has that single cross-cutting responsibility.
- Prefer a new focused module when code has a distinct lifecycle, data owner,
  error boundary, or reason to change.
- Tests belong to the domain they validate under `src/<domain>/test/`, split
  into focused files. Do not create crate-wide `tests/` buckets or collect
  unrelated tests in one generalized test module.

## Comment policy

- Prefer clear names, small methods, and domain-focused types over explanatory
  comments.
- Put documentation comments on declarations: methods, functions, structures,
  enums, traits, modules, and implementation contracts.
- Do not place narrative comments inside function, method, closure, loop,
  conditional, or script blocks when the code can express the intent directly.
- An in-block comment is allowed only for a genuinely tricky approach whose
  constraint, safety property, platform behavior, or non-obvious invariant
  cannot be made clear through naming or extraction.
- When a tricky comment is necessary, explain why the approach is required,
  not what the next line mechanically does.
- Apply the same policy to production code, tests, scripts, templates, and
  configuration files.

## Required verification

Run `./scripts/verify.ps1` on Windows or `sh ./scripts/verify.sh` on macOS/Linux
before handing off a completed change. For CLI or template changes, also
exercise the affected command end to end.
