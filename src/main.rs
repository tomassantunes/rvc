use anyhow::{self, Context};
use clap::{Parser, Subcommand};

pub mod commands;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    // Inialize the repository
    Init,
    Add {
        // The path to the file to add
        path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            commands::init().context("Failed to initialize the repository.")?;
            println!("Initialized the repositoruy.");
        }
        Command::Add { path } => {
            commands::add(path).context("Failed to add the file.")?;
            println!("Added the file.");
        }
    }

    Ok(())
}
