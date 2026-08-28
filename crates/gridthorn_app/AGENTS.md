# Application Crate Instructions

- Own application lifecycle, event-loop orchestration, window events, and
  coordination of subsystem services.
- Keep `winit` types inside this crate. Do not expose them through the
  `gridthorn` facade.
- Depend on renderer services through the narrow `gridthorn_render` surface
  contract. Do not access renderer storage or `wgpu` types.
- Keep experimental lifecycle APIs out of the facade until Milestone 1 proves
  their public behavior.

