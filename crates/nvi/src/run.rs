use tokio::sync::broadcast;

use crate::error::Result;
use crate::NviService;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tracing::Level;
use tracing_log::AsTrace;
use tracing_subscriber::{
    filter::{FilterExt, Targets},
    prelude::*,
};

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

pub async fn inner_run<T>(service: T) -> Result<()>
where
    T: NviService + Unpin + 'static,
{
    let cli = Cli::parse();
    match &cli.command {
        Commands::Connect { addr, verbose } => {
            println!("{}, {:?}", addr, verbose.log_level_filter().as_trace());

            let filter = Targets::new()
                // Filter out overly verbose logs from msgpack_rpc
                .with_target("msgpack_rpc", Level::TRACE)
                .not();
            let fmt_layer = tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_filter(filter);
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

pub async fn run<T>(service: T)
where
    T: NviService + Unpin + 'static,
{
    match inner_run(service).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}
