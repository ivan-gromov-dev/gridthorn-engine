use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

use super::super::{check, create, run};
use crate::project::TemplateKind;

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

    create::execute(
        &project_path,
        TemplateKind::Minimal,
        Some(&local_engine_path()?),
    )?;
    assert!(project_path.join("Cargo.toml").is_file());
    assert!(project_path.join("gridthorn.toml").is_file());
    assert!(project_path.join("src/main.rs").is_file());

    check::execute(&project_path)?;
    run::execute(&project_path, &[])?;
    Ok(())
}
