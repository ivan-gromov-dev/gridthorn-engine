use std::io;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{ApplicationError, WindowConfig, WindowLifecycle};

use super::WinitApplication;

struct ShutdownProbe(Arc<AtomicBool>);

impl WindowLifecycle for ShutdownProbe {
    fn shutdown(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

#[test]
fn shuts_down_lifecycle_when_event_loop_execution_fails() {
    let shutdown = Arc::new(AtomicBool::new(false));
    let state = WinitApplication::new(WindowConfig::default(), ShutdownProbe(shutdown.clone()));
    let event_error = ApplicationError::event_loop(io::Error::other("event loop failed"));

    let result = state.finish(Err(event_error));

    assert!(shutdown.load(Ordering::SeqCst));
    assert!(matches!(result, Err(ApplicationError::EventLoop { .. })));
}
