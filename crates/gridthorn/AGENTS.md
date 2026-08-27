# SDK Facade Instructions

The `gridthorn` crate is the default game-facing facade.

- Keep this crate free of hidden runtime behavior.
- Re-export deliberate subsystem APIs; do not reproduce their implementation.
- Keep `prelude` curated and conservative. Broad names require public API
  compatibility review.
- Do not expose provisional backend types by accident.
- Put facade contract tests under the source domain they validate, one focused
  file per public capability.
