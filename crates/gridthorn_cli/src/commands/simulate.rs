use std::{ffi::OsString, path::Path};

use anyhow::{Result, ensure};

use crate::{cargo_process, project};

pub(crate) fn execute(
    path: &Path,
    scenario: &str,
    ticks: u64,
    seed: u64,
    release: bool,
) -> Result<()> {
    let root = project::validate(path)?;
    let simulation = project::simulation::load(&root)?;
    ensure!(
        simulation.scenarios.iter().any(|name| name == scenario),
        "unknown scenario `{scenario}` in {}; use `gridthorn scenario list` to inspect declared scenarios",
        root.display()
    );
    let arguments = [
        "--scenario".into(),
        OsString::from(scenario),
        "--ticks".into(),
        ticks.to_string().into(),
        "--seed".into(),
        seed.to_string().into(),
    ];
    cargo_process::simulate(&root, &simulation.binary, release, &arguments)
}
