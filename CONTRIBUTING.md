# Contributing to Gridthorn

Gridthorn uses Rust 1.99.0 for local development and as its MSRV.
`rust-toolchain.toml` selects that toolchain with Clippy and rustfmt; rustup
installs it automatically. CI also verifies the latest stable toolchain.

## Common commands

Run the complete local CI workflow from PowerShell:

```powershell
./scripts/verify.ps1
```

On macOS or Linux, run the equivalent POSIX shell workflow:

```console
sh ./scripts/verify.sh
```

The POSIX boundary check uses `python3` to parse Cargo metadata.

For changes containing only Markdown prose or agent instructions, use
`./scripts/verify.ps1 -DocsOnly` or `sh ./scripts/verify.sh --docs-only`.
This mode rejects non-Markdown staged, unstaged, and untracked changes and checks
tracked diff whitespace. Review links and claims separately, validate changed
skills with the system skill validator, and exercise any changed executable
snippets. Code, configuration, scripts, and release candidates require full
verification; CI continues running full checks on all supported platforms.
The docs-only gate examines local changes, not already committed branch changes.

Individual checks are also available:

```console
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Exercise the CLI from this checkout:

```console
cargo run -p gridthorn_cli -- new ../sample-game --engine-path ./crates/gridthorn
cargo run -p gridthorn_cli -- check ../sample-game
cargo run -p gridthorn_cli -- run ../sample-game
```

Do not commit generated `target` directories. Add dependencies at workspace
scope, then opt into them from the crate that uses them. Any new workspace crate
must add an explicit rule to `docs/DEPENDENCY_BOUNDARIES.md` and
`scripts/check-dependency-boundaries.ps1`.

Architectural decisions that constrain multiple subsystems follow the process
in [docs/adr/README.md](docs/adr/README.md). User-visible changes belong in
`CHANGELOG.md`.

## Repository structure

Read the applicable `AGENTS.md` chain before changing a subtree. The root file defines
repository-wide organization rules; nested files add crate, test, documentation,
script, or CI-specific constraints. [AI workflow](docs/AI_WORKFLOW.md) maps domains
to their contracts and checks; load the relevant scope rather than every document.

Production code is grouped by domain. Crate roots and `mod.rs` files compose
modules and public exports rather than storing unrelated implementation. Tests
live under the domain they validate, such as `src/commands/test/`, in focused
files. Add a focused source module instead of growing generic `common`, `utils`,
or entrypoint files.

Comments document declarations and contracts: methods, structures, modules, and
implementations. Executable blocks should remain comment-free unless they use a
genuinely tricky technique whose invariant or portability constraint cannot be
expressed through naming and extraction.
