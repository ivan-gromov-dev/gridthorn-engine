# ADR 0005: Explicit presentation localization with validated Fluent catalogs

- Status: Proposed
- Date: 2026-10-03
- Owners: project maintainers
- Supersedes: none
- Superseded by: none

## Context

Milestone 4 requires locale selection, message identity, validated assets, fallback,
parameters, plural/select and locale-aware formatting. These services must work
without native text rendering and must not mutate authoritative simulation.
Raw asset storage has no schema validation; font shaping does not translate text.

## Decision

Use an independent presentation domain crate with no local dependencies,
re-exported through `gridthorn::localization`. It owns immutable, bounded UTF-8
Fluent catalog assets and explicit runtime publication. Fluent evaluates the
validated message subset and CLDR plural/select rules; ICU4X formats bounded
binary64 decimal values. Backend types stay private. The service introduces no
OS discovery, global state, automatic reload, ECS or native platform requirement.

Resolve exact locale/message IDs through a game-owned ordered fallback chain.
Use the resolved catalog locale for both plural rules and decimal formatting.
Missing messages may fall back; resolver/type failures return contextual errors.
Validate local references and prepare replacements before publication.
Preserve Fluent bidi isolation. [LOCALIZATION.md](../LOCALIZATION.md) specifies
the supported syntax, NUMBER options, precision, diagnostics and limits.

## Alternatives and consequences

Hand-authored plural tables and a new message DSL would duplicate language rules
and translation tooling. A full ICU date/currency/message stack would add broader
formatting policy without a concrete workload. Housing runtime selection in the
asset store would mix publication/lookup policy with generic storage; housing it
in rendering would prevent independent headless use. The dedicated crate owns a
clear catalog/locale lifecycle and keeps foundation/simulation dependencies clean.

The strict Fluent subset excludes terms/attributes/custom functions. Decimal
precision is explicitly bounded to protect backend plural operands. Catalog
compatibility and translation workflows remain provisional; games supply locales,
content, language menus and persistence. General date/time/currency formatting and
large-catalog performance remain future work. No architecture exception is needed.

## Validation

Domain/facade tests and the sibling localization example validate four languages,
cardinal/ordinal/select rules, ICU decimal data, explicit fallback and catalog
replacement. The decision remains Proposed pending maintainer review and broader
game/platform validation; implementation does not stabilize the format or API.
