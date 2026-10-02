use std::fs;

use clap::Parser;

use super::{build::create_build_fixture, workspace::ProjectWorkspace};
use crate::Cli;

#[test]
fn routes_headless_binary_arguments_profiles_and_failures() {
    let workspace = ProjectWorkspace::create().unwrap();
    let root = workspace.root.join("project");
    create_build_fixture(&root);
    let path = root.to_str().unwrap();
    let manifest = fs::read_to_string(root.join("gridthorn.toml")).unwrap();
    let configured = format!(
        "{manifest}\n[simulation]\nbinary = 'headless'\nscenarios = ['zebra', 'economy']\n"
    );
    fs::write(root.join("gridthorn.toml"), &configured).unwrap();
    fs::create_dir_all(root.join("src/bin")).unwrap();
    fs::write(root.join("src/bin/headless.rs"), r#"fn main() {
        let args: Vec<_> = std::env::args().skip(1).collect();
        assert_eq!(args, ["--scenario", "economy", "--ticks", "0", "--seed", "18446744073709551615"]);
        assert!(std::path::Path::new("gridthorn.toml").is_file());
        std::fs::write("headless-result", "initialized").unwrap();
    }"#).unwrap();
    Cli::try_parse_from(["gridthorn", "scenario", "list", path])
        .unwrap()
        .execute()
        .unwrap();
    assert_eq!(
        crate::project::simulation::load(&root).unwrap().scenarios,
        ["economy", "zebra"]
    );
    for release in [false, true] {
        let mut args = vec![
            "gridthorn",
            "simulate",
            path,
            "--scenario",
            "economy",
            "--ticks",
            "0",
            "--seed",
            "18446744073709551615",
        ];
        if release {
            args.push("--release");
        }
        Cli::try_parse_from(args).unwrap().execute().unwrap();
        assert_eq!(
            fs::read_to_string(root.join("headless-result")).unwrap(),
            "initialized"
        );
        fs::remove_file(root.join("headless-result")).unwrap();
    }
    let run = |name| {
        Cli::try_parse_from([
            "gridthorn",
            "simulate",
            path,
            "--scenario",
            name,
            "--ticks",
            "0",
            "--seed",
            "1",
        ])
        .unwrap()
        .execute()
    };
    assert!(
        run("unknown")
            .unwrap_err()
            .to_string()
            .contains("unknown scenario")
    );
    assert!(!root.join("headless-result").exists());
    fs::write(
        root.join("src/bin/headless.rs"),
        "fn main() { std::process::exit(7); }",
    )
    .unwrap();
    assert!(
        run("economy")
            .unwrap_err()
            .to_string()
            .contains("headless cargo run failed with exit code")
    );
    fs::write(
        root.join("gridthorn.toml"),
        configured.replace("'headless'", "'missing'"),
    )
    .unwrap();
    assert!(
        run("economy")
            .unwrap_err()
            .to_string()
            .contains("headless cargo run failed")
    );
}

#[test]
fn rejects_invalid_or_missing_simulation_configuration() {
    let workspace = ProjectWorkspace::create().unwrap();
    let root = workspace.root.join("project");
    create_build_fixture(&root);
    let manifest = fs::read_to_string(root.join("gridthorn.toml")).unwrap();
    let list = || {
        Cli::try_parse_from(["gridthorn", "scenario", "list", root.to_str().unwrap()])
            .unwrap()
            .execute()
    };
    for invalid in [
        "binary = '../bad'\nscenarios = ['economy']",
        "binary = 'headless'\nscenarios = []",
        "binary = 'headless'\nscenarios = ['economy', 'economy']",
        "binary = 'headless'\nscenarios = [' padded']",
        "binary = 'headless'\nscenarios = ['economy']\nunknown = true",
    ] {
        fs::write(
            root.join("gridthorn.toml"),
            format!("{manifest}\n[simulation]\n{invalid}\n"),
        )
        .unwrap();
        assert!(list().is_err());
    }
    fs::write(root.join("gridthorn.toml"), manifest).unwrap();
    assert!(
        list()
            .unwrap_err()
            .to_string()
            .contains("no [simulation] configuration")
    );
}
