use std::path::Path;

use anyhow::{Context, Result, ensure};
use clap::ValueEnum;

use crate::SDK_VERSION;

const CARGO_TEMPLATE: &str = include_str!("../../templates/minimal/Cargo.toml.template");
const PROJECT_TEMPLATE: &str = include_str!("../../templates/minimal/gridthorn.toml.template");
const MAIN_TEMPLATE: &str = include_str!("../../templates/minimal/src/main.rs.template");
const GAME_TEMPLATE: &str = include_str!("../../templates/minimal/src/game/mod.rs.template");
const MODEL_TEMPLATE: &str = include_str!("../../templates/minimal/src/game/model.rs.template");
const GITIGNORE_TEMPLATE: &str = include_str!("../../templates/minimal/gitignore.template");

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub(crate) enum TemplateKind {
    #[default]
    Minimal,
}

pub(crate) struct GeneratedProject {
    pub(crate) cargo_manifest: String,
    pub(crate) project_manifest: String,
    pub(crate) main_source: String,
    pub(crate) game_source: String,
    pub(crate) model_source: String,
    pub(crate) gitignore: String,
}

pub(crate) fn render(
    template: TemplateKind,
    project_name: &str,
    engine_path: Option<&Path>,
) -> Result<GeneratedProject> {
    ensure!(
        matches!(template, TemplateKind::Minimal),
        "unsupported project template"
    );

    let dependency = dependency_spec(engine_path)?;
    Ok(GeneratedProject {
        cargo_manifest: CARGO_TEMPLATE
            .replace("{{project_name}}", project_name)
            .replace("{{gridthorn_dependency}}", &dependency),
        project_manifest: PROJECT_TEMPLATE
            .replace("{{project_name}}", project_name)
            .replace("{{engine_version}}", SDK_VERSION),
        main_source: MAIN_TEMPLATE.replace("{{project_name}}", project_name),
        game_source: GAME_TEMPLATE.to_owned(),
        model_source: MODEL_TEMPLATE.to_owned(),
        gitignore: GITIGNORE_TEMPLATE.to_owned(),
    })
}

fn dependency_spec(engine_path: Option<&Path>) -> Result<String> {
    let Some(engine_path) = engine_path else {
        return Ok(format!("\"{SDK_VERSION}\""));
    };

    let absolute_path = engine_path.canonicalize().with_context(|| {
        format!(
            "failed to resolve local engine path {}",
            engine_path.display()
        )
    })?;
    ensure!(
        absolute_path.join("Cargo.toml").is_file(),
        "local engine path has no Cargo.toml: {}",
        absolute_path.display()
    );

    let path = toml::Value::String(absolute_path.to_string_lossy().into_owned()).to_string();
    Ok(format!("{{ version = \"{SDK_VERSION}\", path = {path} }}"))
}

#[cfg(test)]
mod test;
