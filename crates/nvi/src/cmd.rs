//! The standard Nvi command line interface.

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use tokio::sync::broadcast;

use tracing_log::AsTrace;
use tracing_subscriber::prelude::*;

use crate::{demo::Demos, error::Result, NviPlugin};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Connect {
        /// Address to connect to
        addr: String,

        #[command(flatten)]
        verbose: Verbosity<InfoLevel>,
    },
    /// List available demos
    Demos,
    /// Inspect the plugin
    Inspect,
    /// Run a specific demo
    RunDemo {
        /// Name of the demo to run
        name: String,
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
            crate::connect_unix(tx, addr.clone(), plugin).await
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
        Commands::Inspect => {
            println!("{:#?}", plugin.inspect());
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
    }
}

/// A variant of run() that takes a demos collection.
pub async fn run<T>(service: T, demos: Option<Demos>) -> Result<()>
where
    T: NviPlugin + Unpin + Sync + 'static,
{
    inner_run(service, demos).await
}
