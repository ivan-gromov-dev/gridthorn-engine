# Milestone 2 General-purpose 2D SDK Review

## Status

Milestone 2 completed on 2026-10-02 for the supported subsets below. The
maintainer confirmed Crystal Trail works as intended after its vertical input
mapping was corrected. Public APIs remain provisional; completion does not
stabilize them or broaden their existing compatibility guarantees.

## Supported capabilities and evidence

| Capability | Implemented subset and evidence |
| --- | --- |
| Scenes and game states | Deferred state stack and scene-owned entity replacement; Crystal Trail tests pause, victory cleanup, and restart. |
| Sprite batching and animation | Adjacent shared-texture batches, normalized atlas regions, uniform-frame playback; renderer domain tests and game presentation. |
| Text and runtime UI | Bitmap text, colored panels, mouse buttons; playable menu/pause/victory screens and a public-input mouse-start test. |
| Audio | PCM16 WAV, ordered voice commands, opt-in native output; game music/pickups, pause/resume, shutdown/join, and silent fallback diagnostics. Backend command behavior has mock tests. |
| Basic 2D collision | Circle/AABB overlap and minimum translation queries; domain tests and game wall/pickup tests. |
| Asset dependencies and hot reload | Raw-source and PNG/PNM identities, dependencies, atomic reload batches, background worker; `asset-reload` file-edit, rollback, recovery, and shutdown smoke. |
| Reflection | Explicit registration, scalar metadata, read-only snapshots; domain tests and `reflection-basics`. |
| Versioned scene serialization | Schema 1 scalar TOML, compatibility validation, prepared loading, migrations; domain tests and `scene-serialization`. |
| Development CLI | Compilation-input content watching and validated release builds; CLI lifecycle, watch, and build tests. |
| Small traditional 2D game | `classic_2d` Crystal Trail provides start, movement, collection, pause, victory, and replay with included graphics and audio assets. |

## Verification

- Engine `scripts/verify.ps1` passed, including formatting, workspace check,
  Clippy, tests, documentation tests, and dependency-boundary enforcement.
- The examples workspace Clippy and tests passed after the game was added.
- After the input correction, all six game tests and its Clippy checks passed,
  and engine verification passed again.
- CLI `check` accepted the game and CLI `run -- --smoke` successfully opened
  and exited the native game workflow.
- The maintainer manually ran the game and confirmed the corrected result.

These are local Windows results. A fresh-checkout CI run for this change has
not been observed in this session; existing CI remains the cross-platform gate.

## Accepted scope and explicit deferrals

- Native audio output is an opt-in subsystem service composed by the example.
  Automatic operating-system suspend/resume integration and device recovery
  remain deferred; game pause/resume and shutdown are exercised. Perceptual
  sound quality, output latency, and other-platform devices are unmeasured.
- Rich fonts, layout, keyboard UI focus, streaming audio, broader audio formats,
  broad-phase collision, collision layers, continuous collision, and rigid-body
  response remain extensions driven by demonstrated game requirements.
- Reflection editing, nested persisted data, entity references, durable user
  saves, and automatic scene-document loading in the application loop remain
  outside the scalar subset. ADR 0001 remains proposed; milestone closure does
  not stabilize that persistence decision.
- Native file watching, audio reload, custom derived loaders, asset unloading,
  external compilation dependencies, automatic restart, and packaging remain
  deferred as documented in the subsystem contracts.
- Representative runtime performance, compile-time baselines, binary size,
  memory usage, large-scene/reload scale, and cross-platform measurements remain
  explicitly deferred. No stronger determinism or performance guarantee is
  established by this game.

The next risk-reduction increment is Milestone 3 coordinate systems.
