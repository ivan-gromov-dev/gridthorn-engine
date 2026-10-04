# Provisional localization runtime API

Implemented on 2026-10-03 through `gridthorn::localization`. The independent
`gridthorn_localization` crate deliberately supports headless presentation use.
It owns immutable catalog assets and locale/message formatting; it has no
Gridthorn dependency, window, GPU, ECS, simulation, ambient OS locale or global
state. The facade only re-exports its engine-owned types. APIs remain provisional.

## Locale selection and lookup

`LocaleId::new` validates/canonicalizes Unicode language identifiers such as
`en-US`, `ru`, `ar-EG`, `ja` and `zh-Hant-TW`. Language/script/region/variants are
supported; Unicode extensions and private-use tags are excluded. IDs are exact
catalog keys: selecting `en-GB` does not implicitly select `en`. Games own their
supported languages, language menu, persistence and OS preference negotiation.

`Localization::new(catalogs, selected, fallback)` requires an installed catalog
for every locale and rejects duplicate catalogs or repeated chain entries.
`select_locale` replaces the entire ordered chain only after validation succeeds.
The primary locale followed by explicit fallback locales is available through
`locale_chain`. There is no implicit default language or parent-language lookup.

`MessageId::new` accepts a case-sensitive ASCII letter followed by ASCII letters,
digits, underscores or hyphens. Message IDs are independent of text and asset
paths. `format` returns `LocalizedMessage { text, locale }`. It looks up the ID
in chain order, using the first catalog that contains it. References within that
message resolve only in the same catalog. Plural rules and number formatting
use the resolved catalog's locale; `format_number` uses the selected locale.

Absent IDs return `MissingMessage` with the attempted chain. Missing variables,
numeric type mismatches and resolver limits return `Format` with message and
locale context. A translation that fails formatting does not silently fall back.
No partial backend output is returned. Extra supplied parameters are ignored.

## Validated catalog assets

`CatalogAsset::load(locale, path)` reads bounded UTF-8 Fluent `.ftl` data.
`from_source(locale, String)` applies the same validation in memory. The game
explicitly supplies the locale; filenames do not determine it. Assets are
immutable, `Send + Sync`, and clones share the parsed resource. `message_ids`
iterates sorted message IDs. Translation coverage may differ between catalogs.

Validation rejects syntax errors (including missing select defaults), duplicate
IDs/variants/options, missing referenced messages, cycles, unsupported constructs
and out-of-contract numeric literals. Diagnostics include locale, entry and reason;
parser errors retain byte-offset diagnostics. File errors include the path.
Catalogs are bounded to 1 MiB and 4096 messages, expression nesting to 16 and
message-reference depth to 32. A quote/comment-aware lexical guard bounds parser
nesting to 32 before entering the backend. Reference validation memoizes graph depth.

The supported Fluent subset is messages with values, comments, Unicode text,
literal text/numbers, text/numeric variables, same-catalog message references,
nested select expressions and `NUMBER` with one numeric literal or variable.
Terms, attributes, arbitrary functions and other NUMBER options are rejected.
Use separate message IDs for labels/tooltips. This strict subset can expand
through explicit API changes; unsupported data is never silently accepted.

```ftl
welcome = Добро пожаловать, { $name }!
workers = { $count ->
    [0] Нет работников
    [one] { $count } работник
    [few] { $count } работника
    [many] { $count } работников
   *[other] { $count } работника
}
balance = Баланс: { NUMBER($amount, minimumFractionDigits: 2) }
role = { $role ->
    [worker] Работник
   *[other] Посетитель
}
```

`install_catalog` adds a locale and rejects an already installed one.
`replace_catalog` requires an existing locale and prepares its bundle before
replacement. Failed asset decoding/preparation leaves the current service intact;
old asset clones and returned strings remain usable. Games load/validate candidate
assets away from latency-sensitive polling and publish explicitly at presentation
boundaries. Locale selection and publication never change authoritative state.
No automatic registration, watcher, background loader or reload integration is
introduced. Existing `AssetStore` raw sources may feed `from_source` after game-owned
UTF-8 decoding/validation; raw-source publication alone does not validate catalogs.

## Parameters, plural/select and formatting

`MessageParameters::set_text` validates the variable name and limits each text
value to 64 KiB. Parameters are plain data; text containing Fluent syntax is not
evaluated. `set_number` accepts finite binary64 values with magnitude at most
10^12 and at most six fractional decimal digits in their shortest representation.
Invalid updates preserve the previous parameter. This is a bounded presentation
contract, not exact decimal accounting or an authoritative serialization codec.

Numeric selectors use Fluent's CLDR cardinal rules (`zero`, `one`, `two`, `few`,
`many`, `other`); explicit numeric variants take priority. String selectors use
exact string matches and the declared default branch. `NUMBER($n, type: "ordinal")`
selects ordinal categories. `minimumFractionDigits` (integer 0..6) affects visible
plural operands as well as padding. Variables passed to NUMBER must have a
consistent numeric type throughout the resolved message/reference graph, including
unselected branches; provided text is rejected before backend evaluation.

ICU4X decimal formatting supplies locale-sensitive separators, grouping, signs
and default numbering systems. Supported NUMBER options are `type` (`"cardinal"`
or `"ordinal"`), `minimumFractionDigits` and `useGrouping` (`"true"`/`"false"`).
`format_number(value, minimum_fraction_digits, grouping)` exposes the same decimal
presentation independently of a message. Neither API truncates or rounds decimals.
For example, `12345.5` with minimum width 2 becomes `12,345.50` in `en-US`,
`12 345,50` in `ru` and `١٢٬٣٤٥٫٥٠` in `ar-EG`. CLDR's default for generic `ar`
uses Latin digits; games should select their intended region explicitly.

Fluent inserts Unicode bidi isolation around interpolations. Preserve the returned
text when sending it to `TextSystem`. Font/shaping locale is independently owned
by the game and is not silently changed by localization. This service and its
assets/parameters are `Send + Sync`; mutations require exclusive mutable access.
Lookup performs no filesystem I/O. Formatters are cached per bundle after first
use; standalone `format_number` constructs a formatter per call.

Currency, percentages, units, compact numbers, significant-digit/rounding options,
date/time/calendar/timezone formatting, custom numbering-system preferences,
translation authoring tools and OS language discovery remain outside this
provisional decimal/message contract. ICU's compiled number data and Fluent's
plural data are dependency-versioned; untested locale behavior is not a universal
language-coverage guarantee. Unknown backend locales may inherit fallback data.

## Public usage and evidence

The sibling example uses the facade, real `.ftl` assets and all four languages:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_localization --locked
cargo test --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_localization --locked
```

It selects languages, formats parameters/plural/select/numbers, reports resolved
fallback locales, rejects malformed replacement data and explicitly publishes
a valid replacement. Domain tests cover English, Russian, Arabic and Japanese
cardinals, English ordinals, decimals, exact variants, string selection, references,
fallback order, missing IDs/parameters, numeric type errors, bidi isolation, bounds,
canonical IDs, duplicate/cyclic/missing references, atomic selection/replacement,
and Send/Sync. A facade test covers locale switching and fallback without platform
initialization. Windows execution is validated; Linux/macOS and native rendering
of localized messages have not been exercised by this increment.

The provisional format/ownership decision is [ADR 0005](adr/0005-localization-catalogs.md).

## Localization performance disposition

The Milestone 4.5 [domain review](PERFORMANCE_REVIEW.md#runtime-world-input-localization-domain-review-2026-10-04)
records repeated Windows release measurements for four locales, warm formatting
and simple/complex catalogs through 4096 messages. Complex source validation and
cyclic-candidate rejection can exceed a 16.67 ms frame; keep candidate validation
away from latency-sensitive polling as required above. Preparing/replacing a
validated bundle is cheaper but still consumes presentation time. The review
preserves the implementation and its bounded explicit-publication policy. These
measurements do not guarantee a frame budget for arbitrary message graphs,
simultaneous multi-locale replacement or caller-retained catalogs/output. Wider
reference fanout, heap peaks, worker handoff and cross-platform timings remain
explicit follow-ups; no automatic localization worker is introduced.
