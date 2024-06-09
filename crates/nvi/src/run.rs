use tokio::sync::broadcast;

use crate::error::Result;
use crate::NviService;
use clap::{Parser, Subcommand};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

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
        /// Enable tracing
        #[arg(short, long)]
        trace: bool,
    },
}

pub async fn inner_run<T>(service: T) -> Result<()>
where
    T: NviService + Unpin + 'static,
{
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Connect { addr, trace } => {
            if *trace {
                tracing_subscriber::fmt()
                    .with_target(false)
                    .with_env_filter(
                        EnvFilter::builder()
                            .with_default_directive(LevelFilter::TRACE.into())
                            .parse("nvi")
                            .unwrap(),
                    )
                    .init();
            }

            let (tx, _rx) = broadcast::channel(16);
            crate::connect_unix(tx, addr.clone(), service).await?;

            println!("run: {addr}");
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
