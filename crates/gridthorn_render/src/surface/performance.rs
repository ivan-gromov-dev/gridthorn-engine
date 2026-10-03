use std::time::{Duration, Instant};

const SAMPLE_LIMIT: usize = 240;

#[derive(Default)]
pub(super) struct PipelineSample {
    pub geometry: Duration,
    pub resources: Duration,
    pub vertices: usize,
    pub vertex_bytes: usize,
}

pub(super) struct FrameSample {
    pub acquire: Duration,
    pub pipeline: PipelineSample,
    pub encode: Duration,
    pub submit: Duration,
    pub present: Duration,
}

/// Bounded opt-in CPU diagnostics; printing happens after rendering stops.
pub(super) struct SurfacePerformance {
    enabled: bool,
    samples: Vec<FrameSample>,
}

impl SurfacePerformance {
    pub fn new() -> Self {
        Self {
            enabled: std::env::var_os("GRIDTHORN_RENDER_PERFORMANCE").is_some(),
            samples: Vec::new(),
        }
    }

    pub fn clock(&self) -> Option<Instant> {
        (self.enabled && self.samples.len() < SAMPLE_LIMIT).then(Instant::now)
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn record(&mut self, sample: FrameSample) {
        if self.samples.len() < SAMPLE_LIMIT {
            self.samples.push(sample);
        }
    }
}

impl Drop for SurfacePerformance {
    fn drop(&mut self) {
        if !self.enabled {
            return;
        }
        eprintln!(
            "render_cpu,frame,acquire_us,geometry_us,resources_us,encode_us,submit_us,present_us,vertices,vertex_bytes"
        );
        for (index, sample) in self.samples.iter().enumerate() {
            eprintln!(
                "render_cpu,{index},{},{},{},{},{},{},{},{}",
                sample.acquire.as_micros(),
                sample.pipeline.geometry.as_micros(),
                sample.pipeline.resources.as_micros(),
                sample.encode.as_micros(),
                sample.submit.as_micros(),
                sample.present.as_micros(),
                sample.pipeline.vertices,
                sample.pipeline.vertex_bytes
            );
        }
    }
}
