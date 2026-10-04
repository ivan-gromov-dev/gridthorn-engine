use std::time::{Duration, Instant};

const LIMIT: usize = 8192;

struct Sample {
    elapsed: Duration,
    units: usize,
}

/// Bounded successful text-service operation timings, collected only when requested.
#[derive(Default)]
pub(super) struct TextPerformance {
    samples: [Vec<Sample>; 6],
    totals: [usize; 6],
}

impl TextPerformance {
    pub(super) fn new() -> Option<Self> {
        std::env::var_os("GRIDTHORN_TEXT_PERFORMANCE").map(|_| Self::default())
    }

    pub(super) fn start(&self, operation: usize) -> Option<Instant> {
        (self.samples[operation].len() < LIMIT).then(Instant::now)
    }

    pub(super) fn record(&mut self, operation: usize, start: Option<Instant>, units: usize) {
        self.record_elapsed(operation, start.map(|start| start.elapsed()), units);
    }

    pub(super) fn record_elapsed(
        &mut self,
        operation: usize,
        elapsed: Option<Duration>,
        units: usize,
    ) {
        self.totals[operation] = self.totals[operation].saturating_add(1);
        if self.samples[operation].len() < LIMIT
            && let Some(elapsed) = elapsed
        {
            self.samples[operation].push(Sample { elapsed, units });
        }
    }
}

impl Drop for TextPerformance {
    fn drop(&mut self) {
        eprintln!("text_cpu,operation,sample,elapsed_us,units");
        for (operation, name) in [
            "layout",
            "rasterize",
            "shape",
            "extract",
            "raster_loop",
            "snapshot",
        ]
        .iter()
        .enumerate()
        {
            for (index, sample) in self.samples[operation].iter().enumerate() {
                eprintln!(
                    "text_cpu,{name},{index},{},{}",
                    sample.elapsed.as_micros(),
                    sample.units
                );
            }
            eprintln!(
                "text_cpu_summary,operation={name},collected={},successful={}",
                self.samples[operation].len(),
                self.totals[operation]
            );
        }
    }
}

#[cfg(test)]
#[path = "test/performance.rs"]
mod test;
