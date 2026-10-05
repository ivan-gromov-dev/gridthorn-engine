use std::time::{Duration, Instant};

const SAMPLE_LIMIT: usize = 240;
const PHASES: [&str; 6] = [
    "poll_events",
    "input_transitions",
    "fixed_updates",
    "update",
    "post_update",
    "render",
];

/// Bounded schedule durations, excluding host event-loop waiting.
pub(super) struct RuntimePerformance {
    enabled: bool,
    samples: [Vec<Duration>; 6],
}

impl RuntimePerformance {
    pub(super) fn new() -> Self {
        Self::with_enabled(std::env::var_os("GRIDTHORN_RUNTIME_PERFORMANCE").is_some())
    }

    fn with_enabled(enabled: bool) -> Self {
        Self {
            enabled,
            samples: std::array::from_fn(|_| Vec::new()),
        }
    }

    pub(super) fn start(&self, phase: usize) -> Option<Instant> {
        (self.enabled && self.samples[phase].len() < SAMPLE_LIMIT).then(Instant::now)
    }

    pub(super) fn record(&mut self, phase: usize, start: Option<Instant>) {
        if self.samples[phase].len() < SAMPLE_LIMIT
            && let Some(start) = start
        {
            self.samples[phase].push(start.elapsed());
        }
    }
}

impl Drop for RuntimePerformance {
    fn drop(&mut self) {
        if !self.enabled {
            return;
        }
        eprintln!("runtime_cpu,phase,sample,elapsed_us");
        for (phase, samples) in PHASES.iter().zip(&self.samples) {
            for (index, elapsed) in samples.iter().enumerate() {
                eprintln!("runtime_cpu,{phase},{index},{}", elapsed.as_micros());
            }
        }
    }
}

#[cfg(test)]
#[path = "test/performance.rs"]
mod test;
