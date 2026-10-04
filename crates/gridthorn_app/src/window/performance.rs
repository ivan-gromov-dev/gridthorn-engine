use std::time::{Duration, Instant};

mod display;
mod frames;

const SAMPLE_LIMIT: usize = 240;

/// Bounded native callback diagnostics, excluding event-loop waiting.
pub(super) struct WindowPerformance {
    enabled: bool,
    frames: frames::FrameSamples,
    preparation: Vec<Duration>,
    redraw: Vec<Duration>,
    extraction: Vec<Duration>,
}

impl WindowPerformance {
    pub(super) fn new() -> Self {
        Self {
            enabled: std::env::var_os("GRIDTHORN_WINDOW_PERFORMANCE").is_some(),
            frames: frames::FrameSamples::default(),
            preparation: Vec::new(),
            redraw: Vec::new(),
            extraction: Vec::new(),
        }
    }

    pub(super) fn report_configuration(&self, window: &winit::window::Window) {
        if self.enabled {
            display::report(window);
        }
    }

    pub(super) fn start(&self) -> Option<Instant> {
        self.enabled.then(Instant::now)
    }

    pub(super) fn preparation(&mut self, start: Option<Instant>) {
        if let Some(start) = start {
            self.frames.preparation(start.elapsed());
        }
        Self::record(&mut self.preparation, start);
    }

    pub(super) fn redraw(&mut self, start: Option<Instant>) {
        if let Some(start) = start {
            self.frames.redraw(start.elapsed());
        }
        Self::record(&mut self.redraw, start);
    }

    pub(super) fn extraction_start(&self) -> Option<Instant> {
        (self.enabled && self.extraction.len() < SAMPLE_LIMIT).then(Instant::now)
    }

    pub(super) fn extraction(&mut self, start: Option<Instant>) {
        Self::record(&mut self.extraction, start);
    }

    fn record(samples: &mut Vec<Duration>, start: Option<Instant>) {
        if samples.len() < SAMPLE_LIMIT
            && let Some(start) = start
        {
            samples.push(start.elapsed());
        }
    }
}

impl Drop for WindowPerformance {
    fn drop(&mut self) {
        if !self.enabled {
            return;
        }
        eprintln!("window_cpu,phase,sample,elapsed_us");
        self.frames.report();
        for (phase, samples) in [
            ("preparation", &self.preparation),
            ("redraw", &self.redraw),
            ("extraction", &self.extraction),
        ] {
            for (index, elapsed) in samples.iter().enumerate() {
                eprintln!("window_cpu,{phase},{index},{}", elapsed.as_micros());
            }
        }
    }
}

#[cfg(test)]
#[path = "performance/test/mod.rs"]
mod test;
