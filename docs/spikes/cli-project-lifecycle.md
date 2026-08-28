# CLI Project Lifecycle Spike

## Status

Implemented and validated as the Milestone 0 CLI workflow. The CLI generates a
standalone Cargo project, validates its Gridthorn manifest and engine
compatibility, delegates compilation to Cargo, and runs the resulting program.

This spike validates the current minimal project template. It does not imply
that the planned Milestone 1 runtime template or registry-published SDK is
already available.

## Question

Can the `gridthorn` CLI own a complete `new → check → run` workflow while
remaining a thin adapter over Cargo and an embedded, testable project template?

## Result

- `gridthorn new` validates the destination name, renders the embedded minimal
  template, writes project files, and removes an incomplete destination after a
  write failure.
- `gridthorn check` validates `Cargo.toml`, `gridthorn.toml`, project naming,
  manifest format, and engine compatibility before delegating to `cargo check`.
- `gridthorn run` repeats project validation and delegates to `cargo run`.
- The lifecycle test exercises the public Clap parser and `Cli::execute` for all
  three commands, then compiles and runs the generated process.
- A focused failure-path test verifies that an empty directory reports the
  missing `Cargo.toml` together with the affected project path.

No persistent example directory is appropriate for this spike: generation of a
fresh standalone project is the behavior under test. Both automated and manual
end-to-end runs use isolated temporary directories and clean them afterward.

## Run

From the engine repository:

```console
cargo run -p gridthorn_cli -- new ../cli-spike-game --engine-path ./crates/gridthorn
cargo run -p gridthorn_cli -- check ../cli-spike-game
cargo run -p gridthorn_cli -- run ../cli-spike-game
```

The local `--engine-path` is required while the SDK is consumed from this
unpublished checkout. The generated program currently prints the Gridthorn
version, proving that the project links and executes against the selected SDK.

## Evidence

A process-level run created a fresh project, completed Cargo validation,
compiled the binary, and printed its Gridthorn greeting. The repository test
suite repeats the lifecycle through the public CLI adapter on every run.

Registry publication, richer runtime generation, and project migration are
outside this spike. Compile-time and generated-binary measurements remain part
of the combined Milestone 0 spike review.
