use clap::{Parser, error::ErrorKind};

use super::Cli;

#[test]
fn exposes_expected_commands() {
    for command in ["new", "run", "check"] {
        let parsed = Cli::try_parse_from(["gridthorn", command, "sample"]);
        assert!(parsed.is_ok(), "failed to parse `{command}` command");
    }
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
