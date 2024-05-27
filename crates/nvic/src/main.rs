use anyhow::Result;
use clap::{Parser, Subcommand};

mod api;
mod dump;
mod protoc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Dump the protocol definitions read from Neovim
    Dump {
        #[arg(short, long)]
        raw: bool,
    },
    /// Generate the Rust protocol definitions
    Protoc {},
}

fn main() -> Result<()> {
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
