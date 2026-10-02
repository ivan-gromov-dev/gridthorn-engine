use gridthorn_input::{PointerCaptureMode, PointerCaptureStatus};
use winit::window::CursorGrabMode;

#[derive(Default)]
pub(super) struct NativeCapture {
    effective: PointerCaptureMode,
}

impl NativeCapture {
    pub(super) fn effective(&self) -> PointerCaptureMode {
        self.effective
    }

    pub(super) fn apply(
        &mut self,
        requested: PointerCaptureMode,
        focused: bool,
        mut apply: impl FnMut(PointerCaptureMode) -> Result<(), String>,
    ) -> PointerCaptureStatus {
        let error = if requested != PointerCaptureMode::None && !focused {
            Some(gridthorn_input::PointerCaptureError::Unfocused)
        } else {
            match apply(requested) {
                Ok(()) => {
                    self.effective = requested;
                    None
                }
                Err(error) => Some(gridthorn_input::PointerCaptureError::Platform {
                    message: format!("{requested:?} failed: {error}"),
                }),
            }
        };
        PointerCaptureStatus {
            requested,
            effective: self.effective,
            error,
        }
    }
}

pub(super) fn grab_mode(mode: PointerCaptureMode) -> CursorGrabMode {
    match mode {
        PointerCaptureMode::None => CursorGrabMode::None,
        PointerCaptureMode::Confined => CursorGrabMode::Confined,
        PointerCaptureMode::Locked => CursorGrabMode::Locked,
    }
}

#[cfg(test)]
mod test;
