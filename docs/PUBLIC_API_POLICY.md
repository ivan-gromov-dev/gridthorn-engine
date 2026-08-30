# Public API and Release Policy

Gridthorn `0.2.0` is a pre-1.0 source release. No API is stable until the first
explicitly announced stable release.

## Public surface

- The `gridthorn` crate is the default game-facing facade.
- Items in `gridthorn::prelude` receive the same compatibility review as any
  other public API.
- Subsystem crates are public only when independent use is deliberate,
  documented, and tested.
- Experimental APIs are isolated behind opt-in Cargo features and clearly
  marked in their documentation.

Public behavior includes types and functions as well as defaults, ordering,
diagnostics, serialized formats, and side effects.

## Versioning

Releases follow Semantic Versioning. Before `1.0.0`, a minor version may contain
breaking changes; patch releases must remain compatible within that minor
series. After `1.0.0`, breaking public changes require a major release.

Deprecations should include a replacement and remain available for at least one
minor release when practical. Security fixes and corrections to unsound APIs may
use a shorter migration window and must explain the exception in the changelog.

The CLI and SDK use the same workspace version. A project records a SemVer
requirement in `gridthorn.toml`; the CLI rejects a project when its own version
does not satisfy that requirement.

## Changelog

User-visible changes are recorded in the root `CHANGELOG.md` under `Unreleased`
using the Keep a Changelog categories. Release preparation moves those entries
to a dated version section and updates comparison links once a public repository
URL is established.
