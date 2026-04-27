//! Workspace maintenance commands.

mod error;

use std::{env, ffi::OsStr, path::PathBuf, process::Command};

use clap::{Parser, Subcommand};
use error::{Error, Result};

/// Command-line arguments for the xtask binary.
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// The workspace task to run.
    #[command(subcommand)]
    command: Task,
}

/// Maintenance tasks supported by this workspace.
#[derive(Debug, Subcommand)]
enum Task {
    /// Format the workspace and run clippy with automatic fixes.
    Tidy,
    /// Run the workspace test suite.
    Test,
}

impl Task {
    /// Run the selected maintenance task.
    fn run(self) -> Result<()> {
        match self {
            Self::Tidy => tidy(),
            Self::Test => test(),
        }
    }
}

/// Run the standard formatting and linting commands.
fn tidy() -> Result<()> {
    run_cargo([
        "+nightly",
        "fmt",
        "--all",
        "--",
        "--config-path",
        "./rustfmt-nightly.toml",
    ])?;
    run_cargo([
        "clippy",
        "-q",
        "--fix",
        "--all",
        "--all-targets",
        "--all-features",
        "--allow-dirty",
        "--tests",
        "--examples",
    ])
}

/// Run the standard test command.
fn test() -> Result<()> {
    run_cargo(["nextest", "run", "--all"])
}

/// Run a cargo command in the workspace root.
fn run_cargo<const N: usize>(args: [&str; N]) -> Result<()> {
    let workspace_root = workspace_root()?;
    let status = Command::new("cargo")
        .args(args)
        .current_dir(workspace_root)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::CommandFailed {
            command: format_command("cargo", args),
            status,
        })
    }
}

/// Return the workspace root from the xtask crate location.
fn workspace_root() -> Result<PathBuf> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(PathBuf::from)
        .ok_or(Error::WorkspaceRoot)
}

/// Render a command and its arguments for diagnostics.
fn format_command<const N: usize>(program: &str, args: [&str; N]) -> String {
    let mut command = program.to_string();
    for arg in args {
        command.push(' ');
        command.push_str(OsStr::new(arg).to_string_lossy().as_ref());
    }
    command
}

/// Run the parsed command.
fn main() -> Result<()> {
    Args::parse().command.run()
}
