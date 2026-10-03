use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const LIMIT: usize = 240;

/// Shared bounded diagnostics for cloned routers, printed when the final owner drops.
#[derive(Debug)]
pub(super) struct LayoutPerformance {
    samples: Mutex<Vec<[Duration; 4]>>,
    skipped: AtomicUsize,
}

impl LayoutPerformance {
    pub(super) fn new() -> Option<Arc<Self>> {
        std::env::var_os("GRIDTHORN_UI_PERFORMANCE").map(|_| {
            Arc::new(Self {
                samples: Mutex::new(Vec::new()),
                skipped: AtomicUsize::new(0),
            })
        })
    }

    pub(super) fn record(&self, sample: &LayoutSample) {
        if let Ok(mut samples) = self.samples.try_lock() {
            if samples.len() < LIMIT {
                samples.push(sample.values);
            }
        } else {
            self.skipped.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl Drop for LayoutPerformance {
    fn drop(&mut self) {
        let Ok(samples) = self.samples.get_mut() else {
            return;
        };
        eprintln!("ui_layout,sample,arrange_us,text_geometry_us,paint_us,focused_decoration_us");
        for (index, values) in samples.iter().enumerate() {
            let [arrange, geometry, paint, focused] = values.map(|value| value.as_micros());
            eprintln!("ui_layout,{index},{arrange},{geometry},{paint},{focused}");
        }
        eprintln!(
            "ui_layout_summary,collected={},skipped={}",
            samples.len(),
            self.skipped.load(Ordering::Relaxed)
        );
    }
}

pub(super) struct LayoutSample {
    enabled: bool,
    values: [Duration; 4],
}

impl LayoutSample {
    pub(super) fn new(enabled: bool) -> Self {
        Self {
            enabled,
            values: [Duration::ZERO; 4],
        }
    }

    pub(super) fn start(&self) -> Option<Instant> {
        self.enabled.then(Instant::now)
    }

    pub(super) fn record(&mut self, phase: usize, start: Option<Instant>) {
        if let Some(start) = start {
            self.values[phase] += start.elapsed();
        }
    }
}

#[cfg(test)]
#[path = "test/layout_performance.rs"]
mod test;
