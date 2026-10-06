use super::{
    FramePacer, PresentationConfig, PresentationError, PresentationOperation, PresentationState,
};
use gridthorn_render::SurfaceRenderer;

/// Event-loop-owned requests and pacing; GPU storage remains renderer-owned.
#[derive(Default)]
pub(in crate::window) struct NativePresentation {
    pub(in crate::window) pacer: FramePacer,
    pending: Option<(u64, PresentationConfig)>,
    published: Option<PresentationState>,
}
impl NativePresentation {
    pub(in crate::window) fn configure(
        &mut self,
        renderer: Option<&mut SurfaceRenderer>,
        id: u64,
        config: PresentationConfig,
    ) -> PresentationOperation {
        let result = renderer.map_or(Err(PresentationError::Unavailable), |renderer| {
            if renderer.set_present_mode(config.present_mode).is_ok() {
                Ok(())
            } else {
                Err(PresentationError::UnsupportedMode(config.present_mode))
            }
        });
        if let Err(error) = result {
            return PresentationOperation::Failed { id, error };
        }
        self.pacer.limit = config.frame_rate_limit;
        self.pending = Some((id, config));
        PresentationOperation::Pending { id }
    }
    pub(in crate::window) fn observe(
        &mut self,
        renderer: Option<&SurfaceRenderer>,
    ) -> (Option<PresentationState>, Option<PresentationOperation>) {
        let applied_mode = renderer.and_then(SurfaceRenderer::applied_present_mode);
        let modes = renderer.map_or(&[][..], SurfaceRenderer::present_modes);
        let operation = self
            .pending
            .filter(|(_, config)| applied_mode == Some(config.present_mode))
            .map(|(id, config)| {
                self.pending = None;
                PresentationOperation::Applied { id, config }
            });
        let changed = self
            .published
            .as_ref()
            .is_none_or(|state| {
                state.applied_mode != applied_mode
                    || state.frame_rate_limit != self.pacer.limit
                    || state.supported_modes != modes
            })
            .then(|| PresentationState {
                applied_mode,
                supported_modes: modes.to_vec(),
                frame_rate_limit: self.pacer.limit,
            });
        if let Some(state) = &changed {
            self.published = Some(state.clone());
        }
        (changed, operation)
    }
}
