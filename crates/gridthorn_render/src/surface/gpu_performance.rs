use std::{
    sync::{Mutex, mpsc},
    time::Duration,
};
use wgpu::{Buffer, BufferUsages, Device, QuerySet, Queue};

const SAMPLE_LIMIT: usize = 240;

/// One asynchronous readback slot; pending GPU work never blocks rendering.
pub(super) struct GpuPerformance {
    query: QuerySet,
    resolve: Buffer,
    readback: Buffer,
    period: f32,
    frame: usize,
    active: Option<usize>,
    pending: Option<(usize, Mutex<mpsc::Receiver<bool>>)>,
    samples: Vec<(usize, Duration)>,
    skipped: usize,
    errors: usize,
}

impl GpuPerformance {
    pub fn new(device: &Device, queue: &Queue, first_frame: usize) -> Self {
        let query = device.create_query_set(&wgpu::QuerySetDescriptor {
            label: Some("gridthorn performance timestamps"),
            ty: wgpu::QueryType::Timestamp,
            count: 2,
        });
        let buffer = |label, usage| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: 16,
                usage,
                mapped_at_creation: false,
            })
        };
        Self {
            query,
            resolve: buffer(
                "gridthorn timestamp resolve",
                BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
            ),
            readback: buffer(
                "gridthorn timestamp readback",
                BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            ),
            period: queue.get_timestamp_period(),
            frame: first_frame,
            active: None,
            pending: None,
            samples: Vec::new(),
            skipped: 0,
            errors: 0,
        }
    }

    pub fn begin(&mut self, device: &Device) {
        if self.pending.is_some() {
            if device.poll(wgpu::PollType::Poll).is_err() {
                self.errors += 1;
            }
            self.collect();
        }
        self.active = None;
        if self.frame < SAMPLE_LIMIT {
            if self.pending.is_none() {
                self.active = Some(self.frame);
            } else {
                self.skipped += 1;
            }
        }
        self.frame = self.frame.saturating_add(1);
    }

    pub fn writes(&self) -> Option<wgpu::RenderPassTimestampWrites<'_>> {
        self.active.map(|_| wgpu::RenderPassTimestampWrites {
            query_set: &self.query,
            beginning_of_pass_write_index: Some(0),
            end_of_pass_write_index: Some(1),
        })
    }

    pub fn resolve(&self, encoder: &mut wgpu::CommandEncoder) {
        if self.active.is_some() {
            encoder.resolve_query_set(&self.query, 0..2, &self.resolve, 0);
            encoder.copy_buffer_to_buffer(&self.resolve, 0, &self.readback, 0, 16);
        }
    }

    pub fn submitted(&mut self) {
        if let Some(frame) = self.active.take() {
            let (sender, receiver) = mpsc::channel();
            self.readback
                .map_async(wgpu::MapMode::Read, .., move |result| {
                    let _ = sender.send(result.is_ok());
                });
            self.pending = Some((frame, Mutex::new(receiver)));
        }
    }

    fn collect(&mut self) {
        let result = self.pending.as_ref().and_then(|(_, receiver)| {
            match receiver.try_lock().ok()?.try_recv() {
                Ok(result) => Some(result),
                Err(mpsc::TryRecvError::Disconnected) => Some(false),
                Err(mpsc::TryRecvError::Empty) => None,
            }
        });
        let Some(result) = result else {
            return;
        };
        let Some((frame, _)) = self.pending.take() else {
            return;
        };
        if result {
            let duration = self
                .readback
                .get_mapped_range(..)
                .ok()
                .and_then(|view| timestamp_duration(&view, self.period));
            self.readback.unmap();
            if let Some(duration) = duration {
                self.samples.push((frame, duration));
            } else {
                self.errors += 1;
            }
        } else {
            self.errors += 1;
        }
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "GPU timestamp ticks convert to seconds using the backend nanosecond period"
)]
fn timestamp_duration(bytes: &[u8], period: f32) -> Option<Duration> {
    if !period.is_finite() || period <= 0.0 {
        return None;
    }
    let start = u64::from_le_bytes(bytes.get(..8)?.try_into().ok()?);
    let end = u64::from_le_bytes(bytes.get(8..16)?.try_into().ok()?);
    Duration::try_from_secs_f64(
        end.checked_sub(start)? as f64 * f64::from(period) / 1_000_000_000.0,
    )
    .ok()
}

impl Drop for GpuPerformance {
    fn drop(&mut self) {
        self.collect();
        eprintln!("render_gpu,frame,pass_us");
        for (frame, duration) in &self.samples {
            eprintln!("render_gpu,{frame},{}", duration.as_micros());
        }
        eprintln!(
            "render_gpu_summary,collected={},skipped={},errors={},pending={}",
            self.samples.len(),
            self.skipped,
            self.errors,
            self.pending.is_some()
        );
    }
}

#[cfg(test)]
#[path = "gpu_performance/test/mod.rs"]
mod test;
