use anyhow::{self, Context};
use clap::{Parser, Subcommand};

pub mod commands;
pub mod utils;

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
    Commit {
        // commit message
        message: String,
    },
}

// repo
// - .rvc
// -- commits
// --- v1
// ---- repo em v1
// --- v2
// -- commit_messages.txt

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
        Command::Commit {message} => {
            commands::commit(message).context("Failed to commit.")?;
            println!("Commit executed successfully.");
        }
    }

    Ok(())
}
