use super::{
    CargoDependencies, CargoDependency, CargoManifest, CargoPackage, EngineSection,
    ProjectManifest, ProjectSection, validate_cargo_compatibility, validate_engine_requirement,
};
use crate::SDK_VERSION;

#[test]
fn rejects_incompatible_engine_requirement() {
    let error = validate_engine_requirement(">=99.0.0")
        .expect_err("a future engine requirement must be rejected");
    assert!(error.to_string().contains("this CLI is"));
}

#[test]
fn accepts_matching_project_and_cargo_manifests() {
    validate_cargo_compatibility(&project("sample"), &cargo("sample", Some(SDK_VERSION)))
        .expect("matching manifests must be compatible");
}

#[test]
fn rejects_a_cargo_package_name_that_differs_from_the_project() {
    let error = validate_cargo_compatibility(
        &project("project-name"),
        &cargo("cargo-name", Some(SDK_VERSION)),
    )
    .expect_err("different names must be rejected");

    assert!(
        error
            .to_string()
            .contains("does not match Cargo package name")
    );
}

#[test]
fn rejects_an_incompatible_cargo_dependency_version() {
    let error =
        validate_cargo_compatibility(&project("sample"), &cargo("sample", Some(">=99.0.0")))
            .expect_err("future dependency must be rejected");

    assert!(error.to_string().contains("Cargo dependency requires"));
}

#[test]
fn rejects_a_path_dependency_without_a_version_contract() {
    let error = validate_cargo_compatibility(&project("sample"), &cargo("sample", None))
        .expect_err("unversioned path dependency must be rejected");

    assert!(
        error
            .to_string()
            .contains("must declare a version requirement")
    );
}

fn project(name: &str) -> ProjectManifest {
    ProjectManifest {
        format_version: 1,
        project: ProjectSection {
            name: name.to_owned(),
        },
        engine: EngineSection {
            version: format!("^{SDK_VERSION}"),
        },
    }
}

fn cargo(name: &str, version: Option<&str>) -> CargoManifest {
    CargoManifest {
        package: CargoPackage {
            name: name.to_owned(),
        },
        dependencies: CargoDependencies {
            gridthorn: CargoDependency::Detailed {
                version: version.map(str::to_owned),
            },
        },
    }
}
