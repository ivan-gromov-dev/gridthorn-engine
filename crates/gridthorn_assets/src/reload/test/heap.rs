use std::time::Instant;

#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;

fn profiler() -> dhat::Profiler {
    dhat::Profiler::builder()
        .testing()
        .trim_backtraces(Some(4))
        .build()
}

/// Attribute new allocations within a phase; pre-existing inputs are excluded.
pub(in crate::reload) fn phase<T>(label: &str, operation: impl FnOnce() -> T) -> T {
    let mode = std::env::var("GRIDTHORN_IO_HEAP").unwrap_or_default();
    let profile = (mode == "phase").then(profiler);
    let start = Instant::now();
    let result = operation();
    let elapsed = start.elapsed().as_nanos();
    let stats = profile.as_ref().map(|_| dhat::HeapStats::get());
    drop(profile);
    if let Some(stats) = stats {
        println!(
            "io_phase,{label},{elapsed},heap,{},{},{},{}",
            stats.total_blocks, stats.total_bytes, stats.max_bytes, stats.curr_bytes
        );
    } else if mode != "workflow" {
        println!("io_phase,{label},{elapsed},cpu,0,0,0,0");
    }
    result
}

/// Include setup, inputs, retained output and cleanup in the workflow heap peak.
pub(in crate::reload) fn workflow(label: &str, operation: impl FnOnce()) {
    memory_hold();
    let profile = (std::env::var("GRIDTHORN_IO_HEAP").as_deref() == Ok("workflow")).then(profiler);
    operation();
    let stats = profile.as_ref().map(|_| dhat::HeapStats::get());
    drop(profile);
    if let Some(stats) = stats {
        println!(
            "io_workflow,{label},{},{},{},{}",
            stats.total_blocks, stats.total_bytes, stats.max_bytes, stats.curr_bytes
        );
    }
    memory_hold();
}

pub(in crate::reload) fn samples() -> usize {
    if std::env::var_os("GRIDTHORN_IO_HEAP").is_some() {
        1
    } else {
        std::env::var("GRIDTHORN_IO_SAMPLES").map_or(21, |value| value.parse().unwrap())
    }
}

pub(in crate::reload) fn sizes(default: &[usize]) -> Vec<usize> {
    std::env::var("GRIDTHORN_IO_SIZE")
        .map_or_else(|_| default.to_vec(), |value| vec![value.parse().unwrap()])
}

fn memory_hold() {
    if let Ok(value) = std::env::var("GRIDTHORN_IO_HOLD_MS") {
        std::thread::sleep(std::time::Duration::from_millis(value.parse().unwrap()));
    }
}
