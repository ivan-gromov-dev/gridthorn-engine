# Renderer Crate Instructions

- Own GPU instance, adapter, device, queue, surface configuration, and render
  pass behavior.
- Keep `wgpu` types behind engine-owned renderer types and errors.
- Do not depend on application lifecycle, window event-loop policy, world,
  simulation, or the SDK facade.
- Zero-sized or occluded surfaces must not acquire frames or configure a
  swapchain.
- Keep the window-handle bridge provisional until Milestone 1 defines the
  public platform boundary.
