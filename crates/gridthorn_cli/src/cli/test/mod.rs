use clap::{Parser, error::ErrorKind};

use super::Cli;

#[test]
fn exposes_expected_commands() {
    for command in ["new", "run", "check", "build"] {
        let parsed = Cli::try_parse_from(["gridthorn", command, "sample"]);
        assert!(parsed.is_ok(), "failed to parse `{command}` command");
    }
}

#[test]
fn parses_release_build_and_validated_watch_options() {
    assert!(Cli::try_parse_from(["gridthorn", "build", "--release"]).is_ok());
    assert!(Cli::try_parse_from(["gridthorn", "check"]).is_ok());
    assert!(Cli::try_parse_from(["gridthorn", "check", "--watch", "--interval-ms", "20"]).is_ok());
    assert!(Cli::try_parse_from(["gridthorn", "check", "--watch", "--interval-ms", "0"]).is_err());
    assert!(Cli::try_parse_from(["gridthorn", "check", "--interval-ms", "20"]).is_err());
}

#[test]
fn reports_cli_version() {
    let error = Cli::try_parse_from(["gridthorn", "--version"])
        .expect_err("--version must exit after displaying version information");

    assert_eq!(error.kind(), ErrorKind::DisplayVersion);
    assert!(
        error
            .to_string()
            .contains(concat!("gridthorn ", env!("CARGO_PKG_VERSION")))
    );
}
