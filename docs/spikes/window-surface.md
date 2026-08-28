# Window and GPU Surface Spike

## Status

Implemented as a provisional Milestone 0 prototype. Automated lifecycle tests
and an executable Windows smoke path validate the current design. Runtime
validation on macOS and Linux remains explicitly deferred to Milestone 1; CI
continues to compile and test the code on all three operating systems.

## Question

Can Gridthorn own a native `winit` event loop and a `wgpu` surface behind
domain-specific packages while handling resize, minimize, restore, redraw, and
close without exposing those backends through the `gridthorn` facade?

## Result

- `gridthorn_app` owns window creation, event-loop composition, and application
  lifecycle hooks.
- `gridthorn_render` owns adapter, device, queue, surface configuration, frame
  acquisition, clearing, and presentation.
- `gridthorn_example_window_surface` depends only on `gridthorn_app` and keeps
  executable-only smoke sequencing and diagnostics under
  `../gridthorn-examples/window-surface/`.
- Public errors use Gridthorn-owned types. Backend types are not re-exported by
  the `gridthorn` facade.
- The cross-crate surface-target constructor remains a provisional internal
  bridge constrained by `wgpu::WindowHandle`. Stabilizing or replacing that
  bridge is deferred until the renderer API enters the Milestone 1 public
  facade.

The smoke lifecycle requests a resize, minimize, restore, and close through
Gridthorn-owned control commands. Unit tests cover the state transitions without
requiring a display server.

## Run

Run the interactive example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_window_surface
```

Run the self-closing lifecycle smoke path:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_window_surface -- --smoke
```

Set `RUST_LOG=debug` when detailed application and renderer diagnostics are
needed.

## Deferred measurements

Combined clean-build and release binary-size baselines are recorded in the
[Milestone 0 review](milestone-0-review.md). Platform runtime validation remains
required on macOS and Linux; the normal CI matrix continues to compile and test
the packages on all three operating systems.
