# Provisional asset dependencies and reload

`gridthorn::AssetStore` owns registered raw sources and decoded PNG/PNM textures.
`AssetId` is a case-sensitive, forward-slash path relative to the store root. It
remains the same across content edits and process restarts; renaming the path
changes identity. IDs reject absolute paths, empty/dot segments, backslashes,
and reserved path characters. They do not canonicalize symlinks or case aliases
and are not a security boundary. Use consistent path casing and an absolute root.

```rust
use gridthorn::{AssetId, AssetStore};

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let mut assets = AssetStore::new("assets");
    let texture = AssetId::new("sprite.ppm")?;
    let scene = AssetId::new("scene.txt")?;
    assets.load_texture(texture.clone())?;
    assets.load_source(scene.clone())?;
    assets.set_dependencies(&scene, std::slice::from_ref(&texture))?;
    let invalidated = assets.reload_changed()?;
    let current_texture = assets.texture(&texture).cloned();
    Ok(())
}
```

Register dependencies before assigning edges. Duplicate registration is an
error; duplicate edges are collapsed. Missing dependencies, self-dependencies,
and indirect cycles are rejected without changing the previous graph.
Replacing a dependency list does not itself reload data. Dependencies are
explicit invalidation edges, not a parser or custom-loader system.

The synchronous `reload_changed` path reads
each registered file once, compares bytes, collects transitive dependents, and
prepares replacement textures. It commits all replacements only after every
read and decode succeeds. One broken file blocks the whole batch, including
unrelated changes; the next call retries against the last successful baseline.
This protects in-memory state but does not make concurrent external file writes
a filesystem transaction. A valid intermediate save may be observed.

The returned IDs are in dependency-first order with lexicographic tie-breaking.
Raw source dependents are included so game code can rebuild derived presentation
data. Raw bytes are not schema-validated. Authoritative game data needs its own
validated fixed-tick load/reset boundary. No changes happen automatically in
`ApplicationRuntime`; game code explicitly owns the service and scheduling.

Texture clones and source byte snapshots remain immutable. Newly extracted
frames must resolve their texture ID again to see the latest data; existing
frames retain their old allocation. Unaffected textures retain allocation
identity for batching. The store is `Send + Sync` and reload requires exclusive
mutable access. It starts no worker threads and introduces no new dependencies.

## Background preparation and frame publication

Transfer a fully registered store to `gridthorn::AssetReloader::new(store)` to
move subsequent content scans and decoding off the frame thread. Initial loads
remain synchronous. This uses one standard-library thread and no new dependency.
The registration set and dependency graph are fixed for the worker's lifetime.
`assets()` provides read-only access to the last published store.

- `request_reload()` starts one scan and returns `true`; while a request is
  running or a result awaits publication it returns `false` without queueing.
- `poll()` performs no file reads or decoding and returns immediately when no
  result is ready. At a chosen presentation boundary, it publishes a complete
  successful snapshot and returns `Some(changed_ids)`. An empty vector denotes
  a completed scan with no changes; `None` denotes no ready result.
- Failed scans return `AssetReloadError::Prepare` and leave the published
  snapshot and retry baseline intact. A new request retries the scan.
- `shutdown()` closes the request channel, joins the worker, and discards any
  unpublished result. It is idempotent; the published store remains readable.
  Dropping the service also joins. Shutdown may wait for active disk I/O, which
  cannot be cancelled; run it outside latency-sensitive frame processing.

Thread creation failures and unexpected worker termination have typed errors.
After termination, recreate the service to resume loading. `AssetReloader` is
`Send + Sync`; request, publication, and shutdown require exclusive mutable
access. Old texture and source snapshots remain valid across publication.

The caller chooses the request cadence. Busy requests are not remembered: after
consuming a result, issue another request to observe later changes. There is
at most one pending request/result, and snapshot publication never performs
disk I/O. It may release old allocations, so it is not a hard real-time bound.

The sibling `asset-reload` example requests scans every 250 ms and publishes
ready results in `PollEvents` before extracting render snapshots. Its shutdown
system joins the worker. Tests gate a worker to verify frame calls continue
without waiting, reject duplicate outstanding requests, verify publication only
on `poll`, and cover retry, panic diagnostics, idle/busy shutdown, and drop.

The roadmap item is complete for dependency-aware reload of raw sources and
PNG/PNM textures. Content polling still reads all registered files per scan;
preparation holds old and new data plus graph snapshots. Graph traversal uses
ordered scans. Large-project latency, memory, and cross-platform performance
measurements are explicitly deferred. Native file watching, debounce, dynamic
registration, unloading, custom derived-asset loaders, and audio reload remain
future extensions. No stable API or authoritative data-reload guarantee is made.

Run the external facade example and its headless file-edit smoke from the engine:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_asset_reload
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_asset_reload -- --smoke
```
