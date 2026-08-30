# Workspace and Dependency Boundaries

Gridthorn uses a Cargo workspace with resolver version 3. All crates inherit the
edition, MSRV, version, license, authors, and lint policy from the root
`Cargo.toml`.

The workspace contains only the boundaries needed by the current executable
slice and its focused dependency spikes:

| Crate                  | Responsibility                                             | Allowed Gridthorn dependencies                                |
| ---------------------- | ---------------------------------------------------------- | ------------------------------------------------------------- |
| `gridthorn`            | Public SDK facade and curated prelude                      | `gridthorn_app`, `gridthorn_input`, `gridthorn_render`, `gridthorn_simulation`, `gridthorn_world` |
| `gridthorn_app`        | Provisional application and window lifecycle               | `gridthorn_input`, `gridthorn_render`, `gridthorn_simulation`, `gridthorn_world` |
| `gridthorn_cli`        | Thin command-line adapter and project template             | None                                                          |
| `gridthorn_input`      | Engine-owned keyboard and mouse events and frame state     | None                                                          |
| `gridthorn_render`     | Provisional GPU surface and renderer services              | None                                                          |
| `gridthorn_simulation` | Deterministic simulation primitives and state fingerprints | None                                                          |
| `gridthorn_world`      | Provisional ECS storage and lifecycle schedules            | None                                                          |

The CLI deliberately validates project files and delegates builds to Cargo. It
does not depend on SDK internals. The facade re-exports deliberate runtime and
world contracts but contains no hidden runtime behavior. Application
orchestration owns schedule execution through the narrow world runtime API.

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

The minimum supported Rust version is **1.97.1**. Packages declare this through
the workspace `rust-version` field. Local development follows the latest stable
toolchain, while CI separately checks the complete workspace with the exact
MSRV.

Raising the MSRV requires a documented reason, a changelog entry, and a CI
change in the same pull request.
