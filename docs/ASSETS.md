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

Call `reload_changed` at a chosen frame boundary, before presentation extraction.
The runnable sibling `asset-reload` example uses `PollEvents`. The service reads
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

Current limits: synchronous polling reads all registered contents on every
call; preparation temporarily holds old and new bytes/textures. Graph traversal
uses straightforward ordered scans. Large-project latency, memory, and platform
watcher performance are explicitly unvalidated. Native watching, background
loading, debounce, unloading, custom derived-asset loaders, and audio reload
remain deferred. The roadmap item remains open until the broader workflow is
validated against representative game content.

Run the external facade example and its headless file-edit smoke from the engine:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_asset_reload
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_asset_reload -- --smoke
```
