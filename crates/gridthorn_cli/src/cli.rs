use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;
use crate::project::TemplateKind;

/// Gridthorn project creation and development commands.
#[derive(Debug, Parser)]
#[command(name = "gridthorn", version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    /// Create a Gridthorn project from an embedded template.
    New {
        /// Project directory and Cargo package name.
        project: PathBuf,

        /// Embedded project template to use.
        #[arg(long, value_enum, default_value_t = TemplateKind::Minimal)]
        template: TemplateKind,

        /// Use a local Gridthorn checkout instead of the registry release.
        #[arg(long, value_name = "PATH")]
        engine_path: Option<PathBuf>,
    },

    /// Build and run a Gridthorn project through Cargo.
    Run {
        /// Project directory.
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Arguments forwarded to the game after `--`.
        #[arg(last = true)]
        game_args: Vec<OsString>,
    },

    /// Validate the project manifest and run `cargo check`.
    Check {
        /// Project directory.
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Recheck changed Rust sources and project/Cargo configuration until Ctrl+C.
        #[arg(long)]
        watch: bool,

        /// Content polling interval in milliseconds (watch mode only).
        #[arg(long, requires = "watch", default_value = "500", value_parser = clap::value_parser!(u64).range(1..))]
        interval_ms: u64,
    },

    /// Compile a validated Gridthorn project through Cargo.
    Build {
        /// Project directory.
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Produce an optimized release build instead of the default debug build.
        #[arg(long)]
        release: bool,
    },
}

impl Cli {
    /// Execute the selected command.
    ///
    /// # Errors
    ///
    /// Returns an error when project generation, validation, or the delegated
    /// Cargo command fails.
    pub fn execute(self) -> Result<()> {
        match self.command {
            CliCommand::New {
                project,
                template,
                engine_path,
            } => commands::create::execute(&project, template, engine_path.as_deref()),
            CliCommand::Run { path, game_args } => commands::run::execute(&path, &game_args),
            CliCommand::Check {
                path,
                watch,
                interval_ms,
            } => {
                if watch {
                    commands::watch::execute(&path, std::time::Duration::from_millis(interval_ms))
                } else {
                    commands::check::execute(&path)
                }
            }
            CliCommand::Build { path, release } => commands::build::execute(&path, release),
        }
    }
}

#[cfg(test)]
mod test;
