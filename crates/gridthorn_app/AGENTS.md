# Application Crate Instructions

- Own application lifecycle, event-loop orchestration, window events, and
  coordination of subsystem services.
- Keep `winit` types inside this crate. Do not expose them through the
  `gridthorn` facade.
- Depend on renderer services through the narrow `gridthorn_render` surface
  contract. Do not access renderer storage or `wgpu` types.
- Expose lifecycle APIs through the facade only when their behavior is tested
  through a complete application workflow.
