# ADR 0002: Upstream fullscreen happy path

- Status: Accepted
- Date: 2026-10-05
- Owners: project maintainers
- Supersedes: [ADR 0001](0001-recoverable-windows-fullscreen.md)
- Superseded by: none

## Context

The project owner explicitly chose to remove the temporary vendored winit patch
and support only successful exclusive-fullscreen transitions while upstream
recovery is pending. Exclusive fullscreen remains a required presentation mode.

## Decision

Use the registry winit dependency without a fork or native replacement. Remove
the fork-specific error mailbox and rejection-recovery test. Retain inventory
validation, successful transition confirmation and engine-owned window geometry
restoration. Exit exclusive before transferring it between monitors.

## Consequences

This is a temporary, explicitly authorized exception to the architecture rule
that ordinary runtime failures must not panic. Windows may reject mode activation
or restoration after validation; upstream winit can panic and its cached state
can precede OS acceptance. Neither rejection recovery nor teardown recovery is
guaranteed. `NativeRejected` is reserved and currently not emitted for fullscreen.
There is no polling or additional per-frame native work.

Milestone 5 tracks compatible upstream integration, applied-state consistency,
restoration, disconnection and rejected-transition tests. A master-branch fix
alone does not satisfy this task: the engine needs a compatible released backend
or an explicitly reviewed dependency update. Historical native acceptance in
ADR 0001 describes the removed patch, not the current upstream backend.
