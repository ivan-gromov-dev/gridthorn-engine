use std::{path::Path, time::Duration};

use anyhow::{Context, Result};

use crate::watching::{CheckOutcome, WatchSession};

pub(crate) fn execute(project_path: &Path, interval: Duration) -> Result<()> {
    let root = project_path.canonicalize().with_context(|| {
        format!(
            "failed to resolve watch directory {}",
            project_path.display()
        )
    })?;
    let (mut session, outcome) = WatchSession::start(&root)?;
    report(outcome);
    println!(
        "Watching Rust sources and project configuration in {}; stop with Ctrl+C",
        root.display()
    );
    let mut last_scan_error = None;
    loop {
        std::thread::sleep(interval);
        match session.poll() {
            Ok(outcome) => {
                last_scan_error = None;
                if let Some(outcome) = outcome {
                    report(outcome);
                }
            }
            Err(error) => {
                let message = error.to_string();
                if last_scan_error.as_ref() != Some(&message) {
                    eprintln!("{message}; retaining previous watch snapshot");
                }
                last_scan_error = Some(message);
            }
        }
    }
}

fn report(outcome: CheckOutcome) {
    match outcome {
        CheckOutcome::Passed => println!("Watch check passed"),
        CheckOutcome::Failed(error) => {
            eprintln!("Watch check failed: {error}; waiting for source/configuration changes");
        }
    }
}
