use std::{collections::BTreeSet, fs, path::Path};

use anyhow::{Context, Result, ensure};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SimulationManifest {
    pub(crate) binary: String,
    pub(crate) scenarios: Vec<String>,
}

impl SimulationManifest {
    pub(crate) fn validate(&self) -> Result<()> {
        ensure!(
            !self.binary.is_empty()
                && self
                    .binary
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            "simulation.binary must be a Cargo binary name containing only ASCII letters, digits, '-' or '_'"
        );
        ensure!(
            !self.scenarios.is_empty(),
            "simulation.scenarios must declare at least one scenario"
        );
        let mut names = BTreeSet::new();
        for name in &self.scenarios {
            ensure!(
                !name.is_empty() && name.trim() == name && !name.chars().any(char::is_control),
                "invalid simulation scenario name `{name}`: use a nonempty, unpadded name without control characters"
            );
            ensure!(names.insert(name), "duplicate simulation scenario `{name}`");
        }
        Ok(())
    }
}

pub(crate) fn load(root: &Path) -> Result<SimulationManifest> {
    #[derive(Deserialize)]
    struct Manifest {
        simulation: Option<SimulationManifest>,
    }
    let path = root.join("gridthorn.toml");
    let source =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let manifest: Manifest = toml::from_str(&source)
        .with_context(|| format!("invalid simulation configuration in {}", path.display()))?;
    let mut simulation = manifest.simulation.with_context(|| format!("{} has no [simulation] configuration; declare a dedicated headless binary and scenario names", path.display()))?;
    simulation.validate()?;
    simulation.scenarios.sort();
    Ok(simulation)
}
