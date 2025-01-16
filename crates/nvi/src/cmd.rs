//! The standard Nvi command line interface.

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use tokio::sync::broadcast;

use tracing_log::AsTrace;
use tracing_subscriber::prelude::*;

use crate::{connect, demo::Demos, docs, error::Result, process, NviPlugin};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to a Neovim instance on a socket
    Connect {
        /// Address to connect to
        addr: String,

        #[command(flatten)]
        verbose: Verbosity<InfoLevel>,
    },
    /// Show plugin documentation
    Docs,
    /// List available demos
    Demos,
    /// Inspect the plugin
    Inspect,
    /// Launch an interactive Neovim session listening on a socket
    Nvim {
        /// Unix domain socket path for communicating with Neovim
        socket: String,
        /// Start Neovim with your config (don't use --clean)
        #[arg(long)]
        no_clean: bool,
    },
    /// Run a specific demo
    RunDemo {
        /// Name of the demo to run
        name: String,
    },
    /// Run an interactive Neovim session with the plugin connected
    Run {
        /// Unix domain socket path for communicating with Neovim
        #[arg(default_value = "/tmp/nvi.sock")]
        socket: String,

        /// Start Neovim with your config (don't use --clean)
        #[arg(long)]
        no_clean: bool,

        #[command(flatten)]
        verbose: Verbosity<InfoLevel>,
    },
}

async fn inner_run<T>(plugin: T, demos: Option<Demos>) -> Result<()>
where
    T: NviPlugin + Unpin + Sync + 'static,
{
    let cli = Cli::parse();
    match &cli.command {
        Commands::Connect { addr, verbose } => {
            let fmt_layer = tracing_subscriber::fmt::layer()
                .without_time()
                .with_target(true);
            tracing_subscriber::registry()
                .with(fmt_layer)
                .with(verbose.log_level_filter().as_trace())
                .init();
            let (tx, _rx) = broadcast::channel(16);
            connect::connect_unix(tx, addr.clone(), plugin).await
        }
        Commands::Demos => {
            if let Some(demos) = demos {
                let demos = demos.list();
                if demos.is_empty() {
                    println!("No demos available.");
                } else {
                    for name in demos {
                        println!("{}", name);
                    }
                }
            } else {
                println!("No demos available.");
            }
            Ok(())
        }
        Commands::Docs => {
            let name = plugin.name();
            let docs = plugin.docs()?;
            let methods = plugin.inspect();
            let hl = plugin.highlights()?;

            println!(
                "{}",
                docs::render_docs(docs::Formats::Terminal, &name, &docs, hl, methods)?
            );
            Ok(())
        }
        Commands::Inspect => {
            println!("{:#?}", plugin.inspect());
            Ok(())
        }
        Commands::Nvim { socket, no_clean } => {
            let socket = std::path::PathBuf::from(socket);
            let nvtask = process::start_nvim_cmdline(socket, !no_clean).await?;
            let neovim_handle = tokio::spawn(async move { nvtask.wait_with_output().await });
            let err = neovim_handle
                .await
                .map_err(|e| crate::error::Error::Internal {
                    msg: format!("Error waiting for Neovim process: {}", e),
                });

            // Reset terminal state
            let _ = std::process::Command::new("reset").status();

            err??;

            Ok(())
        }
        Commands::RunDemo { name } => {
            if let Some(demos) = demos {
                demos.run(name, plugin).await
            } else {
                eprintln!("No demos available.");
                Ok(())
            }
        }
        Commands::Run {
            socket: _,
            no_clean,
            verbose,
        } => {
            let fmt_layer = tracing_subscriber::fmt::layer()
                .without_time()
                .with_target(true);
            tracing_subscriber::registry()
                .with(fmt_layer)
                .with(verbose.log_level_filter().as_trace())
                .init();

            let tempdir = tempfile::tempdir()?;
            let socket_path = tempdir.path().join("nvim.socket");

            let (shutdown_tx, _) = broadcast::channel(1);
            let neovim_task = process::start_nvim_cmdline(&socket_path, !no_clean).await?;
            let neovim_handle = tokio::spawn(async move { neovim_task.wait_with_output().await });

            let plugin_shutdown = shutdown_tx.clone();
            let plugin_task =
                tokio::spawn(connect::connect_unix(plugin_shutdown, socket_path, plugin));

            let (plugin_result, neovim_result) = tokio::join!(plugin_task, neovim_handle);

            let plugin_result = plugin_result
                .map_err(|e| crate::error::Error::Internal {
                    msg: format!("plugin task failed: {}", e),
                })
                .and_then(|r| r);

            let _ = neovim_result.map_err(|e| crate::error::Error::Internal {
                msg: format!("neovim task failed: {}", e),
            });

            // Reset terminal state
            let _ = std::process::Command::new("reset").status();

            if let Err(e) = plugin_result {
                eprintln!("Plugin error: {}", e);
            }
            Ok(())
        }
    }
}

/// A variant of run() that takes a demos collection.
pub async fn run<T>(service: T, demos: Option<Demos>) -> Result<()>
where
    T: NviPlugin + Unpin + Sync + 'static,
{
    inner_run(service, demos).await
}
