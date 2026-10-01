use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser;

use super::workspace::ProjectWorkspace;
use crate::{Cli, SDK_VERSION};

/// Minimal local SDK stand-in isolates Cargo profile routing from engine behavior.
fn create_build_fixture(root: &Path) {
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("sdk/src")).unwrap();
    fs::write(
        root.join("sdk/Cargo.toml"),
        format!("[package]\nname = 'gridthorn'\nversion = '{SDK_VERSION}'\nedition = '2024'\n"),
    )
    .unwrap();
    fs::write(root.join("sdk/src/lib.rs"), "").unwrap();
    fs::write(root.join("Cargo.toml"), format!("[package]\nname = 'cli-build-fixture'\nversion = '0.1.0'\nedition = '2024'\n[dependencies]\ngridthorn = {{ path = 'sdk', version = '{SDK_VERSION}' }}\n[workspace]\n")).unwrap();
    fs::write(root.join("gridthorn.toml"), format!("format_version = 1\n[project]\nname = 'cli-build-fixture'\n[engine]\nversion = '{SDK_VERSION}'\n")).unwrap();
    fs::write(
        root.join("src/main.rs"),
        "fn main() { println!(\"build-fixture-ok\"); }\n",
    )
    .unwrap();
}

fn target(root: &Path) -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR").map_or_else(
        || root.join("target"),
        |path| {
            let path = PathBuf::from(path);
            if path.is_absolute() {
                path
            } else {
                std::env::current_dir().unwrap().join(path)
            }
        },
    )
}

#[test]
fn builds_debug_and_release_profiles_and_reports_failures() {
    let workspace = ProjectWorkspace::create().unwrap();
    let root = workspace.root.join("project");
    create_build_fixture(&root);
    let argument = root.to_string_lossy().into_owned();
    for (profile, release) in [("debug", false), ("release", true)] {
        let mut arguments = vec!["gridthorn", "build", argument.as_str()];
        if release {
            arguments.push("--release");
        }
        Cli::try_parse_from(arguments).unwrap().execute().unwrap();
        let binary = target(&root)
            .join(profile)
            .join(format!("cli-build-fixture{}", std::env::consts::EXE_SUFFIX));
        let output = Command::new(binary).output().unwrap();
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout).unwrap().trim(),
            "build-fixture-ok"
        );
    }
    fs::write(root.join("src/main.rs"), "invalid Rust").unwrap();
    let error = Cli::try_parse_from(["gridthorn", "build", &argument, "--release"])
        .unwrap()
        .execute()
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("cargo build failed with exit code")
    );
    fs::write(root.join("Cargo.toml"), "invalid TOML").unwrap();
    let error = Cli::try_parse_from(["gridthorn", "build", &argument])
        .unwrap()
        .execute()
        .unwrap_err();
    assert!(error.to_string().contains("invalid Cargo manifest"));
}
