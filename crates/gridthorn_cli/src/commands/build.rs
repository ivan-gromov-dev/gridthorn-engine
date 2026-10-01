use std::path::Path;

use anyhow::Result;

use crate::{cargo_process, project};

pub(crate) fn execute(project_path: &Path, release: bool) -> Result<()> {
    let project_root = project::validate(project_path)?;
    cargo_process::build(&project_root, release)?;
    println!(
        "{} build completed for {}",
        if release { "Release" } else { "Debug" },
        project_root.display()
    );
    Ok(())
}
