/// Native graphics API used by an adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphicsBackend {
    /// Vulkan.
    Vulkan,
    /// Direct3D 12.
    Direct3D12,
    /// Metal.
    Metal,
    /// OpenGL.
    OpenGl,
    /// Browser graphics or another backend.
    Other,
}

/// Adapter model preference independent of rendering API; revalidate at every launch.
/// Equal model identifiers do not establish equal physical devices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphicsDeviceKey {
    /// Reported vendor identifier.
    pub vendor: u32,
    /// Reported model identifier; OpenGL may lack this value.
    pub device: u32,
    /// Reported model name with the known OpenGL transport suffix removed.
    pub name: String,
}

/// Separate device and rendering API preferences; `None` means automatic selection.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GraphicsSelection {
    /// Optional device-model preference.
    pub device: Option<GraphicsDeviceKey>,
    /// Optional rendering API preference.
    pub api: Option<GraphicsBackend>,
}

/// Device-model group and the backend records available for it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphicsDevice {
    /// Revalidated model preference, independent of rendering API.
    pub key: GraphicsDeviceKey,
    /// API-specific capabilities. Multiple records for one API are ambiguous.
    pub apis: Vec<GraphicsAdapter>,
}

/// Revalidated adapter preference, not a durable physical-device identifier.
/// Identical devices can share a key; explicit selection then reports ambiguity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphicsAdapterKey {
    /// Native graphics API.
    pub backend: GraphicsBackend,
    /// Backend-reported vendor identifier.
    pub vendor: u32,
    /// Backend-reported device identifier.
    pub device: u32,
    /// Backend-reported adapter name.
    pub name: String,
}

/// Static renderer requirements and, when queried with a window, presentation support.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphicsAdapterCompatibility {
    /// Renderer requirements pass; presentation still requires a native window.
    RequiresSurfaceValidation,
    /// Renderer requirements and this window's presentation requirements pass.
    Compatible,
    /// The adapter does not satisfy the renderer's requested device limits.
    UnsupportedLimits,
    /// The adapter cannot present to this window.
    UnsupportedSurface,
}

/// Owned graphics adapter information; no backend handles cross this boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphicsAdapter {
    /// Preference key for explicit initialization.
    pub key: GraphicsAdapterKey,
    /// Whether the backend reports a CPU/software adapter.
    pub software: bool,
    /// Compatibility with renderer requirements and the queried surface.
    pub compatibility: GraphicsAdapterCompatibility,
}

/// Initialization snapshot for a native renderer, available before startup schedules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphicsAdapters {
    /// All adapters exposed by enabled native backends, including rejected ones.
    pub adapters: Vec<GraphicsAdapter>,
    /// Adapter whose device was successfully initialized.
    pub selected: GraphicsAdapterKey,
}
