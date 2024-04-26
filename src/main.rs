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
    Init,
    Add {
        path: String,
    },
    Commit {
        message: String,
    },
    CatFile {
        path: String,
    },
    Push,
    Config {
        option: String,
        value: String
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            commands::init().context("Failed to initialize the repository.")?;
        }
        Command::Add { path } => {
            commands::add(path).context("Failed to add the file.")?;
        }
        Command::Commit {message} => {
            commands::commit(message).context("Failed to commit.")?;
        }
        Command::CatFile { path } => {
            commands::cat_file(path).context("Failed to cat file.")?;
        }
        Command::Push => {
            commands::push().context("Failed to push to remote.")?;
        }
        Command::Config { option, value } => {
            commands::config(option, value).context("Failed to configure.")?;
        }
    }

    Ok(())
}
