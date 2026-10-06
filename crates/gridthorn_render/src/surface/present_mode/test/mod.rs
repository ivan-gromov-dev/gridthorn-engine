use super::*;

#[test]
fn capabilities_exclude_automatic_fallback_and_reject_unavailable_explicit_policy() {
    let capabilities = supported(&[
        wgpu::PresentMode::AutoNoVsync,
        wgpu::PresentMode::Fifo,
        wgpu::PresentMode::AutoVsync,
    ]);
    assert_eq!(capabilities, vec![PresentMode::Fifo]);
    assert!(validate(&capabilities, PresentMode::Fifo).is_ok());
    assert!(matches!(
        validate(&capabilities, PresentMode::Immediate),
        Err(super::super::RenderSurfaceError::UnsupportedPresentMode {
            mode: PresentMode::Immediate
        })
    ));
    for mode in [
        PresentMode::Fifo,
        PresentMode::FifoRelaxed,
        PresentMode::Immediate,
        PresentMode::Mailbox,
    ] {
        assert_eq!(PresentMode::from_native(mode.native()), Some(mode));
    }
    assert_eq!(
        PresentMode::from_native(wgpu::PresentMode::AutoNoVsync),
        None
    );
}
