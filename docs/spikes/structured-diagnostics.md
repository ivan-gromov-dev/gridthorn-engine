# Structured Diagnostics Spike

## Status

Implemented as a provisional cross-layer Milestone 0 prototype. A CLI-driven
example emits structured events from the CLI process and the application,
renderer, and example lifecycle in the game process. Typed renderer failures
remain identifiable through the application error boundary.

This establishes the diagnostic shape and ownership boundary. It does not yet
define persistent log formats, telemetry export, session identifiers, or the
professional profiling facilities planned for later milestones.

## Question

Can a developer follow one `gridthorn run` operation across CLI delegation,
renderer initialization, application startup, lifecycle completion, and Cargo
process exit using contextual structured events and preserved typed errors?

## Result

- CLI Cargo delegation emits start and completion events with `component`,
  `operation`, `project`, success, and exit-code fields.
- Renderer initialization emits the selected adapter and backend with
  `component="renderer"`.
- Application initialization and failures emit an event name with
  `component="app"`.
- The diagnostics example emits one start and one terminal completion event.
- `ApplicationError` transparently preserves the concrete
  `RenderSurfaceError` variant, verified by a domain-owned test.
- Every layer continues to own its error type. No backend error type is added to
  the public `gridthorn` facade.

The CLI and game are separate processes, so each installs its own tracing
subscriber. Their inherited standard streams provide one ordered development
console. Cross-process correlation identifiers remain deferred until the local
development protocol is designed.

## Run

From the engine repository:

```console
cargo run -p gridthorn_cli -- run ../gridthorn-examples/diagnostics-flow
```

Set `RUST_LOG=debug` for surface resize and occlusion events. The example closes
itself after three lifecycle updates.

## Evidence

The end-to-end Windows run emitted, in order:

1. CLI Cargo-subprocess start.
2. Renderer adapter initialization.
3. Example lifecycle start.
4. Application initialization.
5. Example lifecycle completion.
6. CLI Cargo-subprocess success.

Automated tests cover the terminal lifecycle behavior and renderer error
preservation. Platform runtime checks outside Windows remain delegated to the
CI compile/test matrix until suitable display-backed runners are configured.
