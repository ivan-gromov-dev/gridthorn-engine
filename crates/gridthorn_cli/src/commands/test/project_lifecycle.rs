use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, ensure};
use clap::Parser;

use crate::Cli;

use super::workspace::ProjectWorkspace;

fn local_engine_path() -> Result<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(|root| root.join("crates/gridthorn"))
        .context("CLI crate must be inside the workspace crates directory")
}

#[test]
fn generated_project_can_be_checked_and_run_offline() -> Result<()> {
    let workspace = ProjectWorkspace::create()?;
    let project_path = workspace.project();
    let project_argument = project_path.to_string_lossy().into_owned();
    let engine_argument = local_engine_path()?.to_string_lossy().into_owned();

    Cli::try_parse_from([
        "gridthorn",
        "new",
        &project_argument,
        "--engine-path",
        &engine_argument,
    ])?
    .execute()?;
    assert!(project_path.join("Cargo.toml").is_file());
    assert!(project_path.join("gridthorn.toml").is_file());
    assert!(project_path.join("src/main.rs").is_file());
    assert!(project_path.join("src/game/mod.rs").is_file());
    assert!(project_path.join("src/game/model.rs").is_file());

    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.lock"),
        project_path.join("Cargo.lock"),
    )
    .context("failed to seed generated project with verified workspace dependencies")?;

    run_offline_cli("check", &project_path, &[])?;
    run_offline_cli("run", &project_path, &["--", "--smoke"])?;
    Ok(())
}

fn run_offline_cli(action: &str, project_path: &Path, arguments: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["run", "--locked", "--offline", "--bin", "gridthorn", "--"])
        .arg(action)
        .arg(project_path)
        .args(arguments)
        .env("CARGO_NET_OFFLINE", "true")
        .status()
        .context("failed to launch offline CLI lifecycle check")?;
    ensure!(
        status.success(),
        "offline gridthorn {action} failed: {status}"
    );
    Ok(())
}

#[test]
fn check_reports_missing_project_files_with_context() -> Result<()> {
    let workspace = ProjectWorkspace::create()?;
    let invalid_project = workspace.root.join("invalid-project");
    fs::create_dir(&invalid_project)?;
    let project_argument = invalid_project.to_string_lossy().into_owned();
    let resolved_project = invalid_project.canonicalize()?;

    let error = Cli::try_parse_from(["gridthorn", "check", &project_argument])?
        .execute()
        .expect_err("an empty directory must not validate as a project");

    assert!(error.to_string().contains("Cargo.toml not found"));
    assert!(
        error
            .to_string()
            .contains(resolved_project.to_string_lossy().as_ref())
    );
    Ok(())
}

#[test]
fn check_rejects_cargo_manifest_drift_before_starting_cargo() -> Result<()> {
    let workspace = ProjectWorkspace::create()?;
    let project_path = workspace.project();
    let project_argument = project_path.to_string_lossy().into_owned();
    let engine_argument = local_engine_path()?.to_string_lossy().into_owned();
    Cli::try_parse_from([
        "gridthorn",
        "new",
        &project_argument,
        "--engine-path",
        &engine_argument,
    ])?
    .execute()?;
    let cargo_manifest_path = project_path.join("Cargo.toml");
    let cargo_manifest = fs::read_to_string(&cargo_manifest_path)?;
    fs::write(
        &cargo_manifest_path,
        cargo_manifest.replace("name = \"minimal-game\"", "name = \"drifted-name\""),
    )?;

    let error = Cli::try_parse_from(["gridthorn", "check", &project_argument])?
        .execute()
        .expect_err("manifest drift must fail before Cargo starts");

    assert!(
        error
            .to_string()
            .contains("does not match Cargo package name")
    );
    Ok(())
}
