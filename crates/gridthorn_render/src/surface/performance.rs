use std::time::{Duration, Instant};

const SAMPLE_LIMIT: usize = 240;

#[derive(Default)]
pub(super) struct PipelineSample {
    pub geometry: Duration,
    pub resources: Duration,
    pub vertices: usize,
    pub vertex_bytes: usize,
    pub uploaded_vertex_bytes: usize,
    pub colored_cache_hit: bool,
    pub textured_batches: usize,
    pub uploaded_texture_bytes: usize,
    pub retained_textures: usize,
    pub retained_texture_bytes: usize,
    pub retained_vertex_capacity_bytes: usize,
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
    last_present: Option<Instant>,
    intervals: Vec<Duration>,
}

impl SurfacePerformance {
    pub fn new() -> Self {
        Self {
            enabled: std::env::var_os("GRIDTHORN_RENDER_PERFORMANCE").is_some(),
            samples: Vec::new(),
            last_present: None,
            intervals: Vec::new(),
        }
    }

    pub fn clock(&self) -> Option<Instant> {
        (self.enabled && self.samples.len() < SAMPLE_LIMIT).then(Instant::now)
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn next_frame(&self) -> usize {
        self.samples.len()
    }

    pub fn record(&mut self, sample: FrameSample) {
        if self.samples.len() < SAMPLE_LIMIT {
            let now = Instant::now();
            self.intervals.push(
                self.last_present
                    .map_or(Duration::ZERO, |last| now.duration_since(last)),
            );
            self.last_present = Some(now);
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
            "render_cpu,frame,acquire_us,geometry_us,resources_us,encode_us,submit_us,present_us,vertices,vertex_bytes,uploaded_vertex_bytes,colored_cache_hit,host_present_interval_us,retained_vertex_capacity_bytes,textured_batches,uploaded_texture_bytes,retained_textures,retained_texture_bytes"
        );
        for (index, sample) in self.samples.iter().enumerate() {
            eprintln!(
                "render_cpu,{index},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                sample.acquire.as_micros(),
                sample.pipeline.geometry.as_micros(),
                sample.pipeline.resources.as_micros(),
                sample.encode.as_micros(),
                sample.submit.as_micros(),
                sample.present.as_micros(),
                sample.pipeline.vertices,
                sample.pipeline.vertex_bytes,
                sample.pipeline.uploaded_vertex_bytes,
                sample.pipeline.colored_cache_hit,
                self.intervals[index].as_micros(),
                sample.pipeline.retained_vertex_capacity_bytes,
                sample.pipeline.textured_batches,
                sample.pipeline.uploaded_texture_bytes,
                sample.pipeline.retained_textures,
                sample.pipeline.retained_texture_bytes,
            );
        }
    }
}
