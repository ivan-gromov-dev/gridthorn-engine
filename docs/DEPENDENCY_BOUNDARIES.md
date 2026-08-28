# Workspace and Dependency Boundaries

Gridthorn uses a Cargo workspace with resolver version 3. All crates inherit the
edition, MSRV, version, license, authors, and lint policy from the root
`Cargo.toml`.

The workspace contains only the boundaries needed by the current executable
slice and its focused dependency spikes:

| Crate | Responsibility | Allowed Gridthorn dependencies |
| --- | --- | --- |
| `gridthorn` | Public SDK facade and curated prelude | None |
| `gridthorn_app` | Provisional application and window lifecycle | `gridthorn_render` |
| `gridthorn_cli` | Thin command-line adapter and project template | None |
| `gridthorn_render` | Provisional GPU surface and renderer services | None |

The CLI deliberately validates project files and delegates builds to Cargo. It
does not depend on SDK internals. The facade contains no hidden runtime
behavior.

The platform-specific `scripts/check-dependency-boundaries.ps1` and
`scripts/check-dependency-boundaries.sh` entrypoints read Cargo metadata and
fail for an unknown workspace crate or a forbidden project dependency. Adding a
crate requires updating this document and both allowlists. The architectural
rules in [ARCHITECTURE.md](ARCHITECTURE.md#dependency-and-ownership-rules)
remain the source of truth for deciding whether an edge is permitted.

Third-party dependencies are declared centrally under `[workspace.dependencies]`
and opted into by individual crates. Dependencies are added only for the active
milestone.

Runnable examples live in the sibling `gridthorn-examples` repository. They
consume engine crates as external projects and are therefore outside this
workspace's internal dependency allowlist.

## Rust versions

The minimum supported Rust version is **1.85.0**, the first stable release with
Rust 2024 edition support. Packages declare this through the workspace
`rust-version` field. The default development toolchain follows stable so local
formatting and lints receive current fixes; CI separately checks the workspace
with Rust 1.85.0.

Raising the MSRV requires a documented reason, a changelog entry, and a CI
change in the same pull request.
