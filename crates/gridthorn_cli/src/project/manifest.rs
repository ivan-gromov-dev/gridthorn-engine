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

#[derive(Debug, Deserialize)]
struct CargoManifest {
    package: CargoPackage,
    dependencies: CargoDependencies,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CargoDependencies {
    gridthorn: CargoDependency,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CargoDependency {
    Version(String),
    Detailed { version: Option<String> },
}

impl CargoDependency {
    fn version_requirement(&self) -> Option<&str> {
        match self {
            Self::Version(requirement) => Some(requirement),
            Self::Detailed { version } => version.as_deref(),
        }
    }
}

pub(crate) fn validate(project_path: &Path) -> Result<PathBuf> {
    let root = project_path.canonicalize().with_context(|| {
        format!(
            "failed to resolve project directory {}",
            project_path.display()
        )
    })?;
    let cargo_manifest_path = root.join("Cargo.toml");
    ensure!(
        cargo_manifest_path.is_file(),
        "Cargo.toml not found in {}",
        root.display()
    );
    let cargo_source = fs::read_to_string(&cargo_manifest_path)
        .with_context(|| format!("failed to read {}", cargo_manifest_path.display()))?;
    let cargo_manifest: CargoManifest = toml::from_str(&cargo_source)
        .with_context(|| format!("invalid Cargo manifest {}", cargo_manifest_path.display()))?;

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
    validate_cargo_compatibility(&manifest, &cargo_manifest)?;
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

fn validate_cargo_compatibility(project: &ProjectManifest, cargo: &CargoManifest) -> Result<()> {
    ensure!(
        cargo.package.name == project.project.name,
        "project name `{}` in gridthorn.toml does not match Cargo package name `{}`",
        project.project.name,
        cargo.package.name
    );
    let dependency_requirement = cargo
        .dependencies
        .gridthorn
        .version_requirement()
        .context("Cargo dependency `gridthorn` must declare a version requirement")?;
    let dependency_requirement = VersionReq::parse(dependency_requirement).with_context(|| {
        format!(
            "invalid Cargo dependency `gridthorn` version requirement `{dependency_requirement}`"
        )
    })?;
    let cli_version =
        Version::parse(SDK_VERSION).context("CLI package version is not valid SemVer")?;
    ensure!(
        dependency_requirement.matches(&cli_version),
        "Cargo dependency requires Gridthorn {dependency_requirement}, but this CLI is {cli_version}"
    );
    Ok(())
}

#[cfg(test)]
mod test;
