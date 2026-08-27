# CLI Crate Instructions

The CLI is a thin adapter over reusable project and SDK services.

- Keep `main.rs` limited to process setup, argument parsing, and top-level error
  return. Keep `lib.rs` limited to module composition and deliberate exports.
- Group implementation by domain: command adapters in `src/commands/`, project
  model and validation in `src/project/`, and process adapters in focused files.
- Each command gets its own source file. Shared behavior must belong to a named
  domain, not a generic helper module.
- Use `anyhow` only at this executable boundary and attach actionable path or
  subprocess context to failures.
- Keep embedded templates under `templates/<template-name>/` and test generated
  files by compiling them.
- Tests belong under the owning source domain, such as `src/commands/test/` or
  `src/project/manifest/test/`. Command tests should exercise the complete
  adapter flow through generated projects when validating user-visible behavior.
