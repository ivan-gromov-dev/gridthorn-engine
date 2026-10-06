# Runtime API ownership and scope

This document separates engine mechanisms from game policy. Domain documents
define current provisional contracts; the [roadmap](ROADMAP.md) tracks future
platform/device controls and debugging tools. Planned items below are not
implemented APIs or stable compatibility promises.

## Ownership rule

The SDK owns reusable mechanisms requiring platform access, rendering, input
arbitration or consistent lifecycle/error contracts. Games own rules, content,
screens and product policies. Add an API only when it serves a concrete workload;
do not add generic subsystems solely to reproduce a reference game's features.
Optional native capabilities must not become dependencies of headless simulation.
UI actions affecting the world enqueue game commands at the established tick
boundary. UI animation and device events do not change authoritative time.

| Concern              | Planned engine API responsibility                                                                     | Game responsibility                                                                                    |
| -------------------- | ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Keyboard and pointer | Physical/logical keys, modifiers, repeat, ordered input, wheel, capture and focus-loss cancellation   | Shortcut choices, action meanings and interaction rules                                                |
| Text editing         | Unicode/IME events, composition, selection, caret, clipboard and focus integration                    | Field validation, names and accepted content                                                           |
| Fonts and languages  | Font assets/fallback, shaping, bidi, wrapping, measurement and DPI                                    | Font selection, authored text and language coverage                                                    |
| Localization         | Locale/message lookup, catalog validation, parameters, plural/select rules, fallback and formatting   | Translations, supported locales, language menu, translator workflow and catalog content                |
| UI                   | Composition/layout, clipping/scrolling, controls, themes, event routing, focus and generic navigation | Screens, visual design, tooltips and game-specific widgets                                             |
| Popups and dialogs   | Layer order, modal input scopes, focus restoration, configurable dismissal and event consumption      | When a window opens, whether dismissal is allowed, confirmation behavior and whether simulation pauses |
| Animation            | Existing sprite playback plus reusable presentation-property interpolation                            | Clips, timing, transitions and character state machines                                                |
| Devices and windows  | Enumeration, capabilities, selection/configuration, applied-state feedback and loss/change events     | Settings menu, preferred devices, persistence and confirmation/revert policies                         |
| Audio                | Device lifecycle, optional capture, buses and validated gain/mute                                     | Music/effect categories, routing choices and preferences                                               |
| Action bindings      | Reusable binding descriptions and evaluation over supported input sources                             | Default bindings, conflicts, rebinding UX and per-profile storage                                      |
| Filesystem           | Standard platform directory discovery and existing validated world-save file APIs                     | Directories/files, profile IDs, save slots, metadata, backups, retention, autosave and migrations      |
| Tycoon systems       | Existing ticks, commands, RNG, snapshots and typed-root persistence                                   | Economy, research graphs, technology caps, market events and progression                               |

Translation support means runtime localization primitives and actionable catalog
errors. A translation-management application, hosted translation service,
automatic translation or editorial approval workflow is outside these milestones.
The provisional runtime localization service uses validated Fluent catalog assets
and ICU decimal formatting, exercised by a concrete multilingual headless example;
see [LOCALIZATION.md](LOCALIZATION.md) for its explicit fallback, syntax and limits.

## Implementation order and acceptance

Milestone 4 proceeds from complete desktop input to text/IME, font rendering and
localization, then layout/controls, routing and modal windows. Keep physical
shortcuts distinct from layout-dependent logical keys and committed text. Validate
Cyrillic, an IME language, right-to-left shaping, combining characters, fallback
fonts and DPI changes; publish the tested coverage instead of claiming every
language is supported. Controller navigation hooks must be testable before native
controller support arrives in Milestone 5. Focus restoration, nested dialogs,
outside clicks and held-input cancellation need behavioral tests. Consumed input
must not also activate the game world or a lower UI layer.

Milestone 5 proceeds through window/display capabilities, graphics selection and
pacing, controllers/input devices, then audio devices and platform directories.
Graphics enumeration and initialization selection are implemented provisionally;
see [GRAPHICS_ADAPTERS.md](GRAPHICS_ADAPTERS.md) for compatibility and restart semantics.
Explicit surface policies and software frame caps are implemented provisionally;
see [PRESENTATION.md](PRESENTATION.md) for requested/applied feedback, independent
runtime wakes and platform acceptance limits.
Report requested and effective configurations separately. Adapter selection may
require restarting or recreating rendering resources; do not promise live GPU
switching without validation. Display refresh, presentation rate limits and fixed
tick frequency are separate controls. Unsupported exclusive fullscreen, per-device
keyboard/mouse selection or capture permissions need explicit results. Device
identifiers have documented lifetimes and are revalidated when loading preferences.

Native validation covers available Windows, Linux and macOS environments and
records untested combinations explicitly. Examples belong in the sibling
`gridthorn-examples` repository and demonstrate mechanisms with small game-owned
settings/profile code. They must not turn a settings screen, saves manager or
research-tree implementation into privileged engine functionality. Each increment
retains the repository verification, documentation and boundary-check requirements.

## Existing scope retained

Desktop keyboard/pointer input is now implemented provisionally; [INPUT.md](INPUT.md)
documents physical/logical keys, repeat, modifiers, ordered events, wheel and capture.

Unicode/IME text sessions and plain-text clipboard are now implemented provisionally;
[INPUT.md](INPUT.md) documents their lifecycle and validation scope.
Font assets, script fallback, shaping/bidi, measurement, wrapping and DPI rendering
are implemented provisionally; [TEXT.md](TEXT.md) records coverage and limitations.
Current UI provides [composition, sizing, layout, anchoring, clipping, scrolling,
themes and reusable controls](UI.md), with explicit value commands, ordered routing,
hit testing, focus/navigation hooks, text editing/selection and local pointer capture
with explicit world-input consumption. Rich styled text,
modal UI and device selection remain planned. Existing sprite animation supports
uniform-duration atlas frames, looping, one-shot playback and pause/restart.

World saves already accept caller-selected paths: any slot can be loaded. They
store a game-owned typed authoritative root through a game codec, not arbitrary
ECS state or profiles. File I/O is synchronous, parent directories must exist,
and file documents are limited to 16 MiB; compatibility and deferrals remain in
[WORLD_SAVES.md](WORLD_SAVES.md). These milestones do not implicitly add cloud
saves, asynchronous saves, arbitrary ECS capture or cross-release migrations.

Audio output processing releases completed voice handles even when the command
queue is empty. Active voice counts exclude completed voices and include paused
voices. Shared clip clones reuse converted frames within a single command batch,
with retention capped at 16 entries and 1 MiB of converted frames; per-voice
volume and looping settings remain independent. Cleanup requires continued
output processing. Native output release signals
shutdown asynchronously without a synchronous thread join; backend data may
remain alive after drop. [PERFORMANCE.md](PERFORMANCE.md) records the measured
release/latency envelope. Software loopback is not acoustic latency.
Automatic Windows power-event routing to runtime/audio hooks is not implemented;
sleep must not be assumed to reset runtime elapsed time through those hooks.
Device switching/error recovery, acoustic latency, hours-long sessions and
cross-platform validation remain Milestone 5/deployment follow-ups.
