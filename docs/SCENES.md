# Provisional versioned scene serialization

`gridthorn_scene` exposes a scalar persistence service through the `gridthorn`
facade. It depends only on the engine-owned world/reflection boundary and existing
format libraries. This is an explicit load/reset service; application lifecycle
execution and active-scene/game-state transitions remain with the caller.

## Register and capture

Implement `Reflect` for field names/kinds and implement `SceneData` to validate
and reconstruct a complete type from scalar values in metadata declaration order.
Register authoritative components/resources with `SceneRegistry`. Reflection
registration alone does not opt a type into persistence. Constructors must be
pure and preserve exposed values exactly; transient fields are reconstructed.
Runtime handles, caches, tasks, and events must remain outside persisted fields.
Stable asset paths can be exposed as text and validated by the domain constructor.

`capture(&mut world, &scene)` collects every entity owned by that scene,
including empty entities, plus every currently present registered global resource.
Only registered component fields are captured. Other scenes and persistent
entities are excluded. Unregistered components on captured entities are omitted
and must be reconstructed by the application after load when needed.

Entity order is normalized by ascending runtime storage index, component/resource
type order by stable identifier, and field order by name. Repeated capture of
unchanged storage yields identical TOML. Array position identifies an entity only
within that document; backend entity IDs and cross-entity references are omitted.
Reallocation after loading can change capture order; byte identity across repeated
load/save cycles and stable durable entity identities are not promised.

## Document and compatibility

Schema 1 uses strict TOML with these required root fields:

```toml
format = "gridthorn.scene"
schema_version = 1
engine = "=0.2.0"
scene = "level.one"
entities = []
resources = []
```

Entities contain `components` arrays. Each component/resource record contains a
`type_name` and a named `fields` table. Each scalar contains explicit `kind` and
`value` fields. Supported kinds are `bool`, `integer`, `unsigned`, `float`, and
`text`. Unsigned values use canonical decimal text to preserve the full u64 range
despite TOML's signed integer limit; signed integers retain the full i64 range.
Floats must be finite. This representation introduces no simulation arithmetic
or stronger cross-platform authoritative floating-point guarantee.

Capture defaults to the exact engine release. `engine` is an explicitly validated
SemVer requirement; broader requirements are a project compatibility decision.
All document types and this wire format remain provisional. Renaming a registered
type or field requires an explicit migration; there is no automatic Rust-name
inference or silent field loss. Unknown root/record/scalar structure is rejected
by parsing. Unknown required types, repeated type records, unknown/missing fields,
wrong scalar kinds, and domain-invalid values are rejected during preparation.
Future schemas fail rather than being interpreted as the current schema.

## Prepare, then commit

`SceneDocument::from_toml` parses structure; parsing alone does not validate
compatibility or domain values. `SceneRegistry::prepare(&document)` validates the
envelope, checks every record, and constructs all components and resources without
borrowing the live world. It returns `PreparedScene` only when all values succeed.
Dropping a prepared scene discards it. Late failures therefore preserve the
previous entities and resources in full.

At an explicit load/reset boundary, `prepared.commit(&mut world)` removes entities
owned by the document's scene, recreates entities in document order, and overlays
the listed global resources. Persistent entities, other scenes, and resources
absent from the document remain unchanged. Resources are not scene-owned and are
not automatically removed on scene exit. No scheduler runs during commit and no
fallible domain constructor executes then. The transaction covers recoverable
data failures; allocation failure and panics/side effects in user destructors are
outside the contract. Constructors and reflection callbacks must obey their pure
read/construct contracts.

The returned `SceneLoad` contains the scene and fresh EntityIds in document order.
Old scene EntityIds are stale. This service neither updates `SceneController` nor
runs `SceneTransition` construction systems; callers must coordinate those
operations and avoid reconstructing duplicate scene content.

## Migrations and usage

`SceneMigrations` registers explicit one-version-at-a-time transformations.
`upgrade(document)` validates the compatibility envelope, invokes steps in
ascending schema order, and validates their resulting envelopes/version advances.
Missing steps, duplicate registration, callback failure, and unsupported future
versions are errors. Callbacks must be independent of ambient time, filesystem
order, random streams, and mutable global state. Upgraded data still passes the
normal registry preparation and domain constructors before any world mutation.

Schema 1 is the first engine scene schema. No historical automatic migration is
claimed. Schema 0 in tests/example is synthetic legacy data using the same outer
document shape, with a project-owned field rename. Legacy formats with a different
outer structure require a project adapter before this migration service.

Run the public headless round-trip/recovery/migration example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_scene_serialization
```

## Scope and deferred validation

No filesystem writes occur in this service. Atomic file replacement, crash-safe
user saves, nested collections, enums, entity/asset reference resolution, and
automatic app-loop load integration are deferred. This increment implements the
scalar scene document and transactional data application boundary, not a world
save system. Application code supplies I/O and rebuilds presentation resources.
Windows release measurements cover 1000/10000 entities with registered integer
and multilingual string components, caller file I/O, serializer/parser costs,
early/late rollback and phase/workflow heap peaks; see
[PERFORMANCE_REVIEW.md](PERFORMANCE_REVIEW.md). At 10000 entities TOML dominates
latency and allocation peaks; prepare/commit remains an explicit load boundary.
This is not a live-frame loading budget. Nested game workloads, binary size,
physical cold storage and cross-platform measurements need further validation.
Registry lookup and capture currently scan registered metadata for each entity;
do not assume inspector-scale or large-world performance.

The format/boundary rationale remains a [proposed ADR](adr/0001-scene-persistence.md)
pending game and cross-platform validation; no stable compatibility guarantee is
declared by this increment.
