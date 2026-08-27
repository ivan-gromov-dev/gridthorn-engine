use std::path::Path;

use anyhow::Result;

use crate::cargo_process;
use crate::project;

pub(crate) fn execute(project_path: &Path) -> Result<()> {
    let project_root = project::validate(project_path)?;
    cargo_process::check(&project_root)?;
    println!("Project at {} is compatible", project_root.display());
    Ok(())
}
