use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use clap::Parser;

use crate::Cli;

struct ProjectWorkspace {
    root: PathBuf,
}

impl ProjectWorkspace {
    fn create() -> Result<Self> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "gridthorn-cli-command-test-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&root)
            .with_context(|| format!("failed to create test workspace {}", root.display()))?;
        Ok(Self { root })
    }

    fn project(&self) -> PathBuf {
        self.root.join("minimal-game")
    }
}

impl Drop for ProjectWorkspace {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.root)
            && self.root.exists()
        {
            eprintln!(
                "failed to remove command test workspace {}: {error}",
                self.root.display()
            );
        }
    }
}

fn local_engine_path() -> Result<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(|root| root.join("crates/gridthorn"))
        .context("CLI crate must be inside the workspace crates directory")
}

#[test]
fn generated_project_can_be_checked_and_run() -> Result<()> {
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

    Cli::try_parse_from(["gridthorn", "check", &project_argument])?.execute()?;
    Cli::try_parse_from(["gridthorn", "run", &project_argument])?.execute()?;
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
