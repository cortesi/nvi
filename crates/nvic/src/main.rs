use anyhow::Result;
use clap::{Parser, Subcommand};

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
