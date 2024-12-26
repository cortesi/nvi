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

const DEFAULT_SOCKET_PATH: &str = "nvic.socket";

#[derive(Subcommand)]
enum Commands {
    /// Run an addon attached to neovim, all in one shot
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

    /// Start neovim with nvic socket
    Nvim {
        #[arg(long)]
        /// Run nvim in headless mode
        headless: bool,

        #[arg(long)]
        /// Lua file path to execute in nvim on startup
        lua: Option<String>,

        #[arg(long)]
        /// Socket path to use
        socket: Option<String>,
    },

    /// Start a plugin
    Plugin {
        #[arg(long)]
        /// Enable tracing in the addon
        trace: bool,

        #[arg(long)]
        /// Socket path to use
        socket: Option<String>,

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
        Some(Commands::Nvim {
            headless,
            lua,
            socket,
        }) => {
            let socket_path = socket.as_deref().unwrap_or(DEFAULT_SOCKET_PATH);
            run::start_nvim(socket_path.into(), *headless, lua.clone()).await?
        }
        Some(Commands::Plugin {
            trace,
            socket,
            path,
        }) => {
            let socket_path = socket.as_deref().unwrap_or(DEFAULT_SOCKET_PATH);
            run::start_plugin(path.to_string(), socket_path.into(), *trace, false).await?
        }
        None => {
            unreachable!()
        }
    };
    Ok(())
}
