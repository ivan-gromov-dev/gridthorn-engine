use std::ffi::OsString;
use std::path::Path;

use anyhow::Result;

use crate::cargo_process;
use crate::project;

pub(crate) fn execute(project_path: &Path, game_args: &[OsString]) -> Result<()> {
    let project_root = project::validate(project_path)?;
    cargo_process::run(&project_root, game_args)
}
