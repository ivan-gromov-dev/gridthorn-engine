use super::WindowControl;

/// Provisional hooks for exercising application lifecycle behavior.
pub trait WindowLifecycle {
    /// Run once after the window and renderer have initialized.
    fn started(&mut self, _control: &mut WindowControl) {}

    /// Run before the event loop waits for more platform events.
    fn idle(&mut self, _control: &mut WindowControl) {}
}

impl WindowLifecycle for () {}
