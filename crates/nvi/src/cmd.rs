//! The standard Nvi command line interface.

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tokio::sync::broadcast;
use tracing_log::AsTrace;
use tracing_subscriber::prelude::*;

use crate::{error::Result, NviService};

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
}

async fn inner_run<T>(service: T) -> Result<()>
where
    T: NviService + Unpin + Sync + 'static,
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
            crate::connect_unix(tx, addr.clone(), service).await?;
        }
    }
    Ok(())
}

/// Expose the standard Nvi command line interface. Call this from your your `main` function.
pub async fn run<T>(service: T)
where
    T: NviService + Unpin + Sync + 'static,
{
    match inner_run(service).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}
