# Provisional component and resource reflection

The public SDK exposes `Reflect`, `FieldMetadata`, `ValueKind`, `ReflectValue`,
and `ReflectionRegistry`. Types opt in explicitly with a stable application-owned
identifier, ordered field metadata, and an owned value snapshot. Register each
storage role separately during application construction. Ordinary world types
remain usable without reflection. Backend ECS types never cross this API.

`types()` enumerates components before resources, then sorts by stable type name.
Field and value order follows the declared metadata. Inspection requires a
registered type name and an entity for components; missing storage returns `None`.
Unknown types, duplicate registration, empty/whitespace names, repeated fields,
wrong value counts/kinds, and non-finite floats produce typed `ReflectionError`
values. Rejected registrations leave the registry unchanged. Identifiers are
case-sensitive and must be unique within each role; one Rust type can occupy both
roles. Callbacks must obey the documented read-only contract.

Snapshots support booleans, signed/unsigned 64-bit integers, finite 64-bit floats,
and UTF-8 text. Expose only intended data; omit caches, handles, and live tasks.
Reflection does not infer which fields are authoritative. Inspection itself does
not mutate the world and snapshots do not retain world borrows. Registry metadata
and adapters are Send + Sync; world access follows the schedule/world contract.

Run the compiled headless usage example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_reflection_basics
```

This is a provisional scalar inspection foundation for future scene persistence.
Derive macros, nested collections, enums, dynamic field editing, automatic type
registration and custom editor representations remain planned. The separate
[provisional scene service](SCENES.md) now provides explicit authoritative opt-in,
schema 1 scalar persistence, prepared loading, and migration hooks; read-only
reflection registration by itself does not enable serialization.
Large-world inspection latency, snapshot allocations, binary size, and platform
measurements are explicitly deferred until a concrete inspector/scene workload.
Registration scans registered types; inspection currently scans metadata for the
requested stable identifier and copies only that instance's exposed fields.
