use std::time::Duration;

/// Pairs all completed preparation callbacks with the next completed redraw.
#[derive(Default)]
pub(super) struct FrameSamples {
    pending: Duration,
    preparations: usize,
    samples: Vec<(usize, Duration)>,
}

impl FrameSamples {
    pub(super) fn preparation(&mut self, elapsed: Duration) {
        if self.samples.len() < super::SAMPLE_LIMIT {
            self.pending += elapsed;
            self.preparations += 1;
        }
    }

    pub(super) fn redraw(&mut self, elapsed: Duration) {
        if self.preparations != 0 && self.samples.len() < super::SAMPLE_LIMIT {
            self.samples
                .push((self.preparations, self.pending + elapsed));
        }
        self.pending = Duration::ZERO;
        self.preparations = 0;
    }

    pub(super) fn report(&self) {
        eprintln!("window_frame_cpu,sample,preparations,elapsed_us");
        for (index, (count, elapsed)) in self.samples.iter().enumerate() {
            eprintln!("window_frame_cpu,{index},{count},{}", elapsed.as_micros());
        }
    }
}

#[cfg(test)]
#[path = "test/frames.rs"]
mod test;
