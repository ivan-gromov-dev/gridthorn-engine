use winit::window::{Fullscreen, Window};

/// Applies fullscreen using upstream winit on the happy path.
///
/// TODO(Milestone 5): integrate compatible upstream rejection recovery. Windows
/// mode-switch rejection may panic; cached fullscreen state is not proof of OS
/// acceptance. See docs/WINDOWS.md and the fullscreen recovery roadmap item.
pub(super) fn set_fullscreen(window: &Window, fullscreen: Option<Fullscreen>) {
    #[cfg(target_os = "windows")]
    if let (Some(Fullscreen::Exclusive(previous)), Some(Fullscreen::Exclusive(next))) =
        (window.fullscreen(), &fullscreen)
        && previous.monitor() != next.monitor()
    {
        window.set_fullscreen(None);
    }
    window.set_fullscreen(fullscreen);
}
