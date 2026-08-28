use std::error::Error;

use gridthorn_render::RenderSurfaceError;
use thiserror::Error;

type BoxedError = Box<dyn Error + Send + Sync>;

/// Failure produced by window application initialization or execution.
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// The platform event loop could not be created or executed.
    #[error("window event loop failed")]
    EventLoop {
        #[source]
        source: BoxedError,
    },

    /// The platform window could not be created.
    #[error("window creation failed")]
    WindowCreation {
        #[source]
        source: BoxedError,
    },

    /// Renderer initialization or frame presentation failed.
    #[error(transparent)]
    Renderer(#[from] RenderSurfaceError),
}

impl ApplicationError {
    pub(super) fn event_loop(source: impl Error + Send + Sync + 'static) -> Self {
        Self::EventLoop {
            source: Box::new(source),
        }
    }

    pub(super) fn window_creation(source: impl Error + Send + Sync + 'static) -> Self {
        Self::WindowCreation {
            source: Box::new(source),
        }
    }
}

#[cfg(test)]
mod test;
