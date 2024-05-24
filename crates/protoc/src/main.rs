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
    Dump {
        #[arg(short, long)]
        raw: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Dump { raw }) => {
            let output = Command::new("nvim").arg("--api-info").output().unwrap();
            let v: api::Api = rmps::from_slice(&output.stdout).unwrap();
            if *raw {
                println!("{:#?}", v);
            } else {
                println!("API v{}.{}", v.version.major, v.version.minor);
                println!("Functions:");
                for f in v.functions {
                    if f.deprecated_since.is_some() {
                        continue;
                    }
                    print!("\t{}(", f.name);
                    let args = f
                        .parameters
                        .iter()
                        .map(|p| format!("{}: {}", p.1, p.0))
                        .collect::<Vec<String>>()
                        .join(", ");
                    print!("{})", args);
                    print!(") -> {}", f.return_type);
                    println!();
                }
                println!("UI Events:");
                for e in v.ui_events {
                    let params = e
                        .parameters
                        .iter()
                        .map(|p| format!("{}: {}", p.1, p.0))
                        .collect::<Vec<String>>()
                        .join(", ");
                    println!("\t{}({})", e.name, params);
                }
                println!("UI Options:");
                for o in v.ui_options {
                    println!("\t{}", o);
                }
                println!("Types:");
                for t in v.types {
                    println!("\t{}", t.0);
                }
            }
        }
        None => {
            println!("No subcommand specified.");
            std::process::exit(1);
        }
    }
}
