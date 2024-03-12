use clap::{Parser, Subcommand};
use rmp_serde as rmps;
use std::process::Command;

mod api;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Dump the protocol definition read from Neovim
    Dump,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Dump) => {
            let output = Command::new("nvim").arg("--api-info").output().unwrap();
            let v: api::Api = rmps::from_slice(&output.stdout).unwrap();
            println!("{:#?}", v);
        }
        None => {
            println!("No subcommand specified.");
            std::process::exit(1);
        }
    }
}
