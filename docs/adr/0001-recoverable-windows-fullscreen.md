# ADR 0001: Recoverable Windows fullscreen backend

- Status: Superseded
- Date: 2026-10-05
- Owners: project maintainers
- Supersedes: none
- Superseded by: [ADR 0002](0002-upstream-fullscreen-happy-path.md)

## Context

Exclusive fullscreen is a required engine presentation mode. The selected window
backend asserts on recoverable Windows display-switch failures and publishes
requested fullscreen state before Windows accepts the mode. Catching the panic
cannot safely recover poisoned state or the second failure during teardown.

## Decision drivers

- Keep native code and backend types outside game-facing APIs.
- Preserve the engine workspace prohibition on unsafe code.
- Support source consumers and generated game projects without downstream Cargo
  patch configuration.
- Confirm real desktop mode/refresh changes and recoverable OS rejection.

## Considered options

Disabling exclusive fails the product requirement. Replacing the whole window
backend expands scope. A Cargo registry patch requires configuration in each
consumer's root workspace. A small vendored backend patch can preserve the
existing platform ownership and source-consumer behavior.

## Decision

Use a direct path dependency on the vendored window backend, excluded from engine
workspace membership. Preserve upstream source/licenses; record the minimal
Windows patch and upstream provenance under vendor/winit. The dependency keeps
native FFI; engine crates continue forbidding unsafe code. Add a backend-specific
error mailbox, commit tracked fullscreen state only after accepted mode changes,
and map rejection to an engine-owned typed error with actual state. No backend
types are exported through the SDK. Other platform implementations are unchanged.

## Consequences

The source release includes backend source. Maintainers must rebase and verify
the narrow patch on upgrades, removing the fork when upstream provides equivalent
recovery. This does not stabilize the provisional SDK or guarantee exclusive
scanout independently of Windows/driver presentation policy.

## Validation

Acceptance requires boundary checks, generated-project workflow/full verification,
restricted Windows rejection without panic, ordinary Windows exclusive mode and
refresh readback/restoration, and the GPU settings smoke. All required checks passed on the documented Windows subset.

Evidence: full verify.ps1 and boundary checks passed, including CLI source
consumer builds. Native success switched 144 to 60 Hz and restored 144 Hz;
cross-monitor transfer restored 180/144 Hz desktops. Restricted-token rejection
returned -1 without panic. GPU settings smoke switched 1920 × 1080 / 144 Hz to
1600 × 900 / requested 60 Hz (Windows readback 59 Hz), then restored the desktop.
Known 59/60 alias confirmation is domain-tested; other differences are rejected.
Linux/macOS native acceptance remains deferred and their backend source is
unchanged. Formatting verification covers every engine workspace package and
excludes upstream vendor formatting. Only three Windows vendor source files differ
from the recorded upstream package.
