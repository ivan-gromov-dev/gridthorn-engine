# 0004: Font assets and isolated multilingual presentation

- Status: Proposed
- Date: 2026-10-03

## Context

The diagnostic bitmap font cannot shape Cyrillic or non-Latin text. Milestone 4
requires real font assets, script fallback, bidi, measurement, wrapping and DPI.
These must remain presentation services with engine-owned public types and no
GPU or font-database state in authoritative world persistence.

## Decision

Use fontdb for asset validation and cosmic-text for shaping/layout/fallback and
Swash rasterization behind private renderer types. Load an explicit game-owned
set of fonts with an explicit locale; do not scan system fonts. Advanced shaping
is always enabled. Logical layouts are owned by one text service; immutable raster
snapshots can outlive it. Native window DPI is a separate application resource.
The initial renderer submits raster spans through the ordered colored UI pipeline.

## Alternatives

- Extending the bitmap table cannot provide contextual shaping or proper marks.
- Direct shaping/bidi/line breaking libraries would require implementing and
  validating their integration and cluster fallback ourselves.
- A GPU text backend would add atlas/resource lifecycle and renderer-version
  coupling before the core text contract is validated.
- System font discovery would make packaged coverage depend on each host.

## Consequences and validation

The existing bitmap API is preserved. New APIs remain provisional. Backend types
and IDs do not cross the SDK boundary. Fixture-based domain tests and the public
multilingual example validate the supported scope in [TEXT.md](../TEXT.md).
Raster spans increase geometry cost; throughput, atlas optimization, binary size,
cache memory and cross-platform native validation remain deferred before stability.
This record remains proposed pending broader game/platform validation.
