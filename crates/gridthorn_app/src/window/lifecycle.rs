use super::{ApplicationError, WindowControl};

/// Provisional hooks for exercising application lifecycle behavior.
pub trait WindowLifecycle {
    /// Run once after the window and renderer have initialized.
    ///
    /// # Errors
    ///
    /// Returns a contextual application failure that stops the event loop.
    fn started(&mut self, _control: &mut WindowControl) -> Result<(), ApplicationError> {
        Ok(())
    }

    /// Run before the event loop waits for more platform events.
    ///
    /// # Errors
    ///
    /// Returns a contextual application failure that stops the event loop.
    fn idle(&mut self, _control: &mut WindowControl) -> Result<(), ApplicationError> {
        Ok(())
    }

    /// Pause platform-dependent work without advancing simulation time.
    fn suspended(&mut self) {}

    /// Resume platform-dependent work without including suspended elapsed time.
    fn resumed(&mut self) {}

    /// Release application services after the event loop stops.
    fn shutdown(&mut self) {}
}

impl WindowLifecycle for () {}
