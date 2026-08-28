# World Crate Instructions

- Own ECS storage, queries, and schedule execution behind Gridthorn-defined
  types and lifecycle stages.
- Do not expose `bevy_ecs` types or schedule labels through the public API.
- Keep authoritative mutations explicit at schedule boundaries.
- Store schedule tests under `src/schedule/test/` and world-access tests under
  `src/world/test/`.
