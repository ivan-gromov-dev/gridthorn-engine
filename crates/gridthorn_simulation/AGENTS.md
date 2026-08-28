# Simulation Crate Instructions

These instructions apply to `crates/gridthorn_simulation`.

- Keep authoritative simulation primitives independent from presentation,
  platform, renderer, and ECS implementation details.
- Prefer explicitly specified algorithms and integer encodings so results can
  be reproduced across supported targets.
- Treat changes to random streams or fingerprint encodings as compatibility
  changes and cover them with fixed-vector tests.
- Keep tests beside their domain under `src/<domain>/test/`.
