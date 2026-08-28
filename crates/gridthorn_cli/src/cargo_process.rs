use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, ExitStatus};

use anyhow::{Context, Result, bail};
use tracing::info;

pub(crate) fn check(project_root: &Path) -> Result<()> {
    execute("check", project_root, &[])
}

pub(crate) fn run(project_root: &Path, game_args: &[OsString]) -> Result<()> {
    let mut command = cargo_command("run", project_root);
    if !game_args.is_empty() {
        command.arg("--").args(game_args);
    }
    execute_command("cargo run", project_root, &mut command)
}

fn execute(action: &str, project_root: &Path, arguments: &[OsString]) -> Result<()> {
    let mut command = cargo_command(action, project_root);
    command.args(arguments);
    execute_command(&format!("cargo {action}"), project_root, &mut command)
}

fn cargo_command(action: &str, project_root: &Path) -> Command {
    let mut command = Command::new("cargo");
    command
        .arg(action)
        .arg("--manifest-path")
        .arg(project_root.join("Cargo.toml"));
    command
}

fn execute_command(action: &str, project_root: &Path, command: &mut Command) -> Result<()> {
    info!(
        component = "cli",
        operation = action,
        project = %project_root.display(),
        "starting Cargo subprocess"
    );
    let status = command
        .status()
        .with_context(|| format!("failed to start Cargo in {}", project_root.display()))?;
    info!(
        component = "cli",
        operation = action,
        success = status.success(),
        exit_code = status.code(),
        "Cargo subprocess completed"
    );
    require_success(action, status)
}

fn require_success(action: &str, status: ExitStatus) -> Result<()> {
    if status.success() {
        Ok(())
    } else if let Some(code) = status.code() {
        bail!("{action} failed with exit code {code}")
    } else {
        bail!("{action} was terminated by a signal")
    }
}
