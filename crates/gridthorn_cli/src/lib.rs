//! Implementation of the `gridthorn` command-line interface.

mod cargo_process;
mod cli;
mod commands;
mod diagnostics;
mod project;

pub use cli::Cli;
pub use diagnostics::init_diagnostics;

pub(crate) const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
