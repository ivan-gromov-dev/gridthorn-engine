use gridthorn_input::InputEvent;
use gridthorn_render::RenderFrame;

use super::{ApplicationError, WindowControl};

/// Provisional hooks for exercising application lifecycle behavior.
pub trait WindowLifecycle {
    /// Publish inventory only in response to an explicit refresh or selection request.
    /// Queries run on the event-loop thread; the snapshot contains only owned data.
    fn displays_changed(&mut self, _displays: crate::display::Displays) {}

    /// Report pending, applied or failed placement for the latest selection request.
    fn monitor_selection_changed(&mut self, _selection: crate::display::MonitorSelection) {}

    /// Publish physical pixels per logical pixel after creation and every DPI change.
    fn scale_factor_changed(&mut self, _scale_factor: f64) {}

    /// Publish the actual physical size after creation and every resize, including zero.
    fn resized(&mut self, _width: u32, _height: u32) {}

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

    /// Collect one engine-owned keyboard or mouse event for the next frame.
    ///
    /// # Errors
    ///
    /// Returns a contextual application failure that stops the event loop.
    fn input(&mut self, _event: InputEvent) -> Result<(), ApplicationError> {
        Ok(())
    }

    /// Return the immutable presentation snapshot for the next redraw.
    fn render_frame(&mut self) -> RenderFrame {
        RenderFrame::default()
    }

    /// Pause platform-dependent work without advancing simulation time.
    fn suspended(&mut self) {}

    /// Resume platform-dependent work without including suspended elapsed time.
    fn resumed(&mut self) {}

    /// Release application services after the event loop stops.
    fn shutdown(&mut self) {}
}

impl WindowLifecycle for () {}
