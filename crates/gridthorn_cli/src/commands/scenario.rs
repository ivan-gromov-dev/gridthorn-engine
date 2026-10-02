use std::path::Path;

use anyhow::Result;

use crate::project;

pub(crate) fn execute(path: &Path) -> Result<()> {
    let root = project::validate(path)?;
    let simulation = project::simulation::load(&root)?;
    for name in simulation.scenarios {
        println!("{name}");
    }
    Ok(())
}
