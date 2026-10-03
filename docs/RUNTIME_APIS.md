# Planned runtime API scope — Milestones 4 and 5

This is a planning document approved on 2026-10-02, not an implemented API
contract or a stable compatibility promise. Milestones 1–3 remain complete for
their documented subsets. The [roadmap](ROADMAP.md) now schedules runtime input,
text/UI and desktop integration before professional debugging and the editor.
Concrete public types, backend choices and irreversible decisions require
focused validation and the existing ADR process during implementation.

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
Current UI provides asset-font and bitmap text, panels and buttons. Rich styled text, text-field editing,
modal UI and device selection remain planned. Existing sprite animation supports
uniform-duration atlas frames, looping, one-shot playback and pause/restart.

World saves already accept caller-selected paths: any slot can be loaded. They
store a game-owned typed authoritative root through a game codec, not arbitrary
ECS state or profiles. File I/O is synchronous, parent directories must exist,
and file documents are limited to 16 MiB; compatibility and deferrals remain in
[WORLD_SAVES.md](WORLD_SAVES.md). These milestones do not implicitly add cloud
saves, asynchronous saves, arbitrary ECS capture or cross-release migrations.
