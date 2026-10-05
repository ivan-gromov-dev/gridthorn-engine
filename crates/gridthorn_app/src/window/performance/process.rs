use std::time::{Duration, Instant};

const LIMIT: usize = 4096;

/// Whole-process CPU deltas between completed redraws, including every thread.
///
/// OS accounting excludes descheduled time. Wall intervals include waits and
/// event-loop work; neither interval is a sum of nested callback timers.
pub(super) struct ProcessSamples {
    origin: Instant,
    previous: Option<(Duration, Duration)>,
    frames: u64,
    errors: usize,
    samples: Vec<(u64, Duration, Duration, Duration)>,
}

impl Default for ProcessSamples {
    fn default() -> Self {
        Self {
            origin: Instant::now(),
            previous: None,
            frames: 0,
            errors: 0,
            samples: Vec::new(),
        }
    }
}

impl ProcessSamples {
    pub(super) fn redraw(&mut self) {
        if self.samples.len() >= LIMIT {
            return;
        }
        let cpu = cpu_time::ProcessTime::try_now().map(|time| time.as_duration());
        self.record(cpu.ok(), self.origin.elapsed());
    }

    fn record(&mut self, cpu: Option<Duration>, elapsed: Duration) {
        if self.samples.len() >= LIMIT {
            return;
        }
        self.frames += 1;
        let Some(cpu) = cpu else {
            self.errors += 1;
            self.previous = None;
            return;
        };
        if let Some((previous_cpu, previous_wall)) = self.previous
            && let (Some(cpu_delta), Some(wall_delta)) = (
                cpu.checked_sub(previous_cpu),
                elapsed.checked_sub(previous_wall),
            )
        {
            self.samples
                .push((self.frames, cpu_delta, wall_delta, elapsed));
        }
        self.previous = Some((cpu, elapsed));
    }

    pub(super) fn report(&self) {
        eprintln!(
            "process_cpu_configuration,scope=all_process_threads,clock=os_accounting,limit={LIMIT},errors={}",
            self.errors
        );
        eprintln!("process_frame_cpu,frame,cpu_ns,wall_ns,elapsed_ns");
        for (frame, cpu, wall, elapsed) in &self.samples {
            eprintln!(
                "process_frame_cpu,{frame},{},{},{}",
                cpu.as_nanos(),
                wall.as_nanos(),
                elapsed.as_nanos()
            );
        }
    }
}

#[cfg(test)]
#[path = "test/process.rs"]
mod test;
