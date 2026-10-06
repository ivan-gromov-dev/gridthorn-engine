use std::error::Error;

use thiserror::Error;

type BoxedError = Box<dyn Error + Send + Sync>;

/// Failure produced while creating, configuring, or rendering a surface.
#[derive(Debug, Error)]
pub enum RenderSurfaceError {
    /// This explicit policy is not supported by the initialized surface/adapter pair.
    #[error("present mode {mode:?} is unsupported by this surface/adapter")]
    UnsupportedPresentMode { mode: super::PresentMode },
    /// The selected device/API combination is not exposed by this inventory.
    #[error("graphics device/API selection is unavailable: {selection:?}")]
    GraphicsSelectionUnavailable { selection: crate::GraphicsSelection },
    /// Multiple indistinguishable devices expose the requested API.
    #[error("graphics device/API selection is ambiguous: {selection:?}")]
    GraphicsSelectionAmbiguous { selection: crate::GraphicsSelection },
    /// The requested adapter key is no longer exposed by the native backends.
    #[error("requested graphics adapter is unavailable: {key:?}")]
    AdapterUnavailable { key: crate::GraphicsAdapterKey },
    /// Multiple adapters share the requested key; none is selected silently.
    #[error("requested graphics adapter is ambiguous: {key:?}")]
    AdapterAmbiguous { key: crate::GraphicsAdapterKey },
    /// The requested adapter fails renderer or window compatibility validation.
    #[error("requested graphics adapter is incompatible: {key:?}: {reason:?}")]
    AdapterIncompatible {
        key: crate::GraphicsAdapterKey,
        reason: crate::GraphicsAdapterCompatibility,
    },
    /// The GPU surface could not be created for the platform window.
    #[error("GPU surface creation failed")]
    SurfaceCreation {
        #[source]
        source: BoxedError,
    },
    /// No compatible graphics adapter was available.
    #[error("compatible graphics adapter request failed")]
    AdapterRequest {
        #[source]
        source: BoxedError,
    },
    /// The logical graphics device could not be created.
    #[error("graphics device request failed")]
    DeviceRequest {
        #[source]
        source: BoxedError,
    },
    /// The selected adapter cannot present to this surface.
    #[error("selected graphics adapter has no compatible surface configuration")]
    UnsupportedConfiguration,
    /// The presentation surface was lost and must be recreated.
    #[error("GPU presentation surface was lost")]
    SurfaceLost,
    /// Frame acquisition encountered a captured validation failure.
    #[error("GPU surface frame acquisition failed validation")]
    SurfaceValidation,
}

impl RenderSurfaceError {
    pub(super) fn surface_creation(source: impl Error + Send + Sync + 'static) -> Self {
        Self::SurfaceCreation {
            source: Box::new(source),
        }
    }

    pub(super) fn adapter_request(source: impl Error + Send + Sync + 'static) -> Self {
        Self::AdapterRequest {
            source: Box::new(source),
        }
    }

    pub(super) fn device_request(source: impl Error + Send + Sync + 'static) -> Self {
        Self::DeviceRequest {
            source: Box::new(source),
        }
    }
}
