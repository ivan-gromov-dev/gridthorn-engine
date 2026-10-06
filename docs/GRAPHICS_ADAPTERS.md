# Graphics adapter selection

## Independent device and rendering API choices

`GraphicsSelection` separates optional `device: GraphicsDeviceKey` and
`api: GraphicsBackend` preferences. Each can be automatic independently.
Use `WindowedApplication::with_graphics_selection` or
`WindowApplication::with_graphics_selection` before running. A missing device/API
combination is rejected; an explicit API never silently changes to another API.
`GraphicsSelection::resolve` validates a pair against a surface-specific inventory
without requesting a device. `SurfaceRenderer::with_graphics_selection` applies it
when constructing the native renderer. The earlier backend-record selection API
remains available; the last selection builder call wins.

`graphics_devices(&inventory)` groups backend records by reported vendor, model
ID and model name. Known OpenGL `/PCIe/SSE2` name suffixes are removed. OpenGL's
missing model ID can be associated only when that vendor/name identifies exactly
one group. Otherwise its record remains separate. API capabilities stay on each
record; software adapters are reported as separate model groups when identified
separately by their backend metadata.

These are revalidated device-model preferences, not guaranteed physical-device
identities. Two identical physical cards can share a group; duplicate records for
one API cause an ambiguity error for an explicit device selection. Different
backend names or IDs can prevent grouping. Enumeration cannot guarantee physical
cross-API identity when the platform metadata does not establish it. Group order
and model identifiers are not stable persistence identities.

The settings example displays a card list and a separate API list. It shows only
compatible APIs for the chosen card and resets an unsupported staged API to
automatic when the card changes. Saving persists both independent preferences;
the previous combined preference file is read and migrated when next saved.
The GPU restart contract below is unchanged. Domain tests cover grouping,
automatic choices, missing combinations, indistinguishable devices, API filtering
and preference migration.

Milestone 5 implements provisional native adapter enumeration and initialization
selection through engine-owned types. No backend handles enter game code.

`gridthorn::enumerate_graphics_adapters()` queries enabled native backends without
opening a window or requesting devices. Each entry contains a `GraphicsAdapterKey`
(backend, vendor, device and reported name), a software flag and compatibility.
An empty list means no adapter was exposed. `UnsupportedLimits` rejects adapters
that cannot satisfy the renderer's default requested device limits.
`RequiresSurfaceValidation` means those limits pass but window presentation has
not been tested; it is not a promise that device creation will succeed.

Pass a key to `WindowedApplication::with_graphics_adapter` or
`WindowApplication::with_graphics_adapter` before `run`. Initialization re-enumerates
adapters and validates the requested adapter against the actual window surface.
Missing, ambiguous and incompatible preferences produce typed renderer errors
wrapped by `ApplicationError::Renderer`. An explicit selection never silently
falls back. Without a preference, the backend's default adapter policy remains.
Device creation can still fail after compatibility checks, with its source error
retained. Software adapters are reported and are not excluded by engine policy.

After successful initialization, `GraphicsAdapters` is inserted into the runtime
world before Startup. It contains all exposed adapters with surface-specific
`Compatible`, `UnsupportedSurface` or `UnsupportedLimits` results, plus the key of
the effective adapter. Custom lifecycles receive the same owned snapshot through
`graphics_adapters_initialized` before `started`. Renderer-free applications do
not enumerate adapters or publish this resource.

Keys are revalidated preferences, not persistent physical identities. Names and
backend availability can change with drivers; multiple identical adapters may
share a key and selection then reports ambiguity. Enumeration ordering is not
stable. Persistence and fallback choices belong to the game.

## Restart contract

The windowed application chooses its GPU only at initialization. To change GPU,
end the current run and start the application again with a new preference. There
is no runtime switch request or automatic device-loss migration. Platform event
loop restrictions can require restarting the process for the new run.

The low-level renderer can be dropped and recreated with
`SurfaceRenderer::with_adapter`; its new device, queue, pipeline and upload caches
are independent. The caller owns window lifetime and must resubmit presentation
data. This is not a windowed runtime resource-migration contract.

## Validation and limits

Domain tests cover selection, missing and ambiguous keys, incompatibility reasons,
facade access and runtime publication. The opt-in Windows native test
`enumerates_selects_recreates_and_rejects_missing_native_adapter` validates actual
surface inventory, explicit renderer recreation for compatible adapters,
presentation and missing-preference rejection. Native acceptance depends on the
adapters available on the test machine. On 2026-10-06 the Windows native test
passed on an NVIDIA GeForce RTX 3070 through Vulkan, Direct3D 12 and OpenGL,
and on the Microsoft Basic Render Driver through Direct3D 12. These are backend
entries, not four physical GPUs. Linux/macOS, physical multi-GPU switching,
hot unplug and device-loss recovery are not established by this increment.

The sibling `desktop-displays` settings example shows the effective GPU and
surface inventory and accepts `--list-adapters` and `--adapter INDEX` at launch.
Its menu can select a GPU and save that key for the next application launch,
or restore automatic selection. Preferences are stored by the example itself;
the SDK still validates the requested adapter at initialization. The native
UI-save/relaunch workflow passed from Vulkan to Direct3D 12 on the RTX 3070.
Its `--adapter-smoke` workflow passed with automatic selection and explicit
Direct3D 12 selection on the RTX 3070; headless layout, domain tests and Clippy
also passed. GPU preferences and restart policy remain game-owned.

The separate card/API UI workflow passed on 2026-10-06: one RTX 3070 group exposed
Vulkan, Direct3D 12 and OpenGL, and the software adapter exposed Direct3D 12.
Saving the card and Direct3D 12 separately kept Vulkan effective until relaunch,
then initialized Direct3D 12. API-only OpenGL selection and explicit software-card
selection passed; software-card/Vulkan selection was rejected. The previous
OpenGL preference file was also accepted through the example's migration path.
