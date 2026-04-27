//! Error types for xtask commands.

use std::{io, process::ExitStatus, result};

use thiserror::Error;

/// Result type used by xtask commands.
pub type Result<T> = result::Result<T, Error>;

/// Errors that can occur while running xtask commands.
#[derive(Debug, Error)]
pub enum Error {
    /// A child process could not be started or waited on.
    #[error("failed to run command")]
    Io {
        /// The underlying IO error.
        #[from]
        source: io::Error,
    },

    /// A child process exited unsuccessfully.
    #[error("command failed: {command} ({status})")]
    CommandFailed {
        /// The shell-like command description.
        command: String,
        /// The process exit status.
        status: ExitStatus,
    },

    /// The xtask manifest directory was not inside the workspace.
    #[error("xtask crate is not inside the workspace")]
    WorkspaceRoot,
}
