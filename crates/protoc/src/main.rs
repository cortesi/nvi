//! The protoc crate contains the protocol compiler for nvi.
use anyhow::Result;
use clap::{Parser, Subcommand};

/// API types
mod api;
/// Documentation for API functions
mod docs;
/// Dump API definitions
mod dump;
/// Overrides for API generation
mod overrides;
/// Protocol compiler
mod protoc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(arg_required_else_help(true))]
/// CLI arguments
struct Cli {
    #[command(subcommand)]
    /// The command to execute
    command: Option<Commands>,
}

#[derive(Subcommand)]
/// Subcommands
enum Commands {
    /// Dump the protocol definitions read from Neovim
    Dump {
        #[arg(short, long)]
        /// Dump raw debug output
        raw: bool,
    },
    /// Generate the Rust protocol definitions
    Protoc {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Dump { raw }) => dump::dump(*raw)?,
        Some(Commands::Protoc {}) => protoc::protoc()?,
        None => {
            unreachable!()
        }
    };
    Ok(())
}
