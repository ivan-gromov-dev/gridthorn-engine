use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    gridthorn_cli::init_diagnostics();
    gridthorn_cli::Cli::parse().execute()
}
