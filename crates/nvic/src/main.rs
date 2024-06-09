use anyhow::Result;
use clap::{Parser, Subcommand};

mod api;
mod dump;
mod overrides;
mod protoc;
mod run;

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
    /// Run an addon attached to neovim
    Run {
        #[arg(long)]
        /// Run nvim in headless mode
        headless: bool,

        #[arg(long)]
        /// Enable tracing in the addon
        trace: bool,

        #[arg(long)]
        /// Lua file path to execute in nvim on startup
        lua: Option<String>,

        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Dump { raw }) => dump::dump(*raw)?,
        Some(Commands::Protoc {}) => protoc::protoc()?,
        Some(Commands::Run {
            headless,
            path,
            trace,
            lua,
        }) => run::run(path, *headless, *trace, lua.clone()).await?,
        None => {
            unreachable!()
        }
    };
    Ok(())
}
