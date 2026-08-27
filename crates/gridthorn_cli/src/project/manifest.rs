use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use semver::{Version, VersionReq};
use serde::Deserialize;

use super::name;
use crate::SDK_VERSION;

const PROJECT_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectManifest {
    format_version: u32,
    project: ProjectSection,
    engine: EngineSection,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectSection {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EngineSection {
    version: String,
}

pub(crate) fn validate(project_path: &Path) -> Result<PathBuf> {
    let root = project_path.canonicalize().with_context(|| {
        format!(
            "failed to resolve project directory {}",
            project_path.display()
        )
    })?;
    ensure!(
        root.join("Cargo.toml").is_file(),
        "Cargo.toml not found in {}",
        root.display()
    );

    let manifest_path = root.join("gridthorn.toml");
    let source = fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;
    let manifest: ProjectManifest = toml::from_str(&source)
        .with_context(|| format!("invalid project manifest {}", manifest_path.display()))?;

    ensure!(
        manifest.format_version == PROJECT_FORMAT_VERSION,
        "unsupported project format version {}; this CLI supports version {}",
        manifest.format_version,
        PROJECT_FORMAT_VERSION
    );
    name::validate(&manifest.project.name).context("invalid project name in gridthorn.toml")?;
    validate_engine_requirement(&manifest.engine.version)?;
    Ok(root)
}

fn validate_engine_requirement(requirement: &str) -> Result<()> {
    let requirement = VersionReq::parse(requirement)
        .with_context(|| format!("invalid engine version requirement `{requirement}`"))?;
    let cli_version =
        Version::parse(SDK_VERSION).context("CLI package version is not valid SemVer")?;
    ensure!(
        requirement.matches(&cli_version),
        "project requires Gridthorn {requirement}, but this CLI is {cli_version}"
    );
    Ok(())
}

#[cfg(test)]
mod test;
