use std::fs;
use std::path::Path;

use anyhow::{Context, Result, ensure};
use tracing::{info, warn};

use crate::project::{self, GeneratedProject, TemplateKind};

pub(crate) fn execute(
    project_path: &Path,
    template: TemplateKind,
    engine_path: Option<&Path>,
) -> Result<()> {
    let project_name = project_path
        .file_name()
        .and_then(|name| name.to_str())
        .context("project path must end in a valid UTF-8 directory name")?;
    project::validate_name(project_name)?;
    ensure!(
        !project_path.exists(),
        "project destination already exists: {}",
        project_path.display()
    );

    let generated = project::render(template, project_name, engine_path)?;
    let result = write_project(project_path, &generated);
    if result.is_err()
        && project_path.exists()
        && let Err(error) = fs::remove_dir_all(project_path)
    {
        warn!(
            component = "cli",
            path = %project_path.display(),
            %error,
            "could not remove incomplete project directory"
        );
    }
    result?;

    info!(
        component = "cli",
        project = %project_path.display(),
        "created Gridthorn project"
    );
    println!("Created `{project_name}` at {}", project_path.display());
    println!("Next: cd {} && gridthorn run", project_path.display());
    Ok(())
}

fn write_project(project_path: &Path, generated: &GeneratedProject) -> Result<()> {
    fs::create_dir_all(project_path.join("src")).with_context(|| {
        format!(
            "failed to create project directory {}",
            project_path.display()
        )
    })?;

    write_file(&project_path.join("Cargo.toml"), &generated.cargo_manifest)?;
    write_file(
        &project_path.join("gridthorn.toml"),
        &generated.project_manifest,
    )?;
    write_file(&project_path.join("src/main.rs"), &generated.main_source)?;
    write_file(&project_path.join(".gitignore"), &generated.gitignore)?;
    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    fs::write(path, contents)
        .with_context(|| format!("failed to write generated file {}", path.display()))
}
