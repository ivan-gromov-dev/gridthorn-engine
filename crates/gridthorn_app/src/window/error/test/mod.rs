use gridthorn_render::RenderSurfaceError;

use super::ApplicationError;

#[test]
fn preserves_renderer_failure_through_application_error_boundary() {
    let error = ApplicationError::from(RenderSurfaceError::SurfaceLost);

    assert_eq!(error.to_string(), "GPU presentation surface was lost");
    assert!(matches!(
        error,
        ApplicationError::Renderer(RenderSurfaceError::SurfaceLost)
    ));
}
