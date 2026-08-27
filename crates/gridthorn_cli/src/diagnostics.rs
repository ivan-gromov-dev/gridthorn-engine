use tracing_subscriber::EnvFilter;

/// Configure human-readable diagnostics controlled by `RUST_LOG`.
pub fn init_diagnostics() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ignored = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .try_init();
}
