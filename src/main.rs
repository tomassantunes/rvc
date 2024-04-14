use anyhow;
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    // Inialize the repository
    Init,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            fs::create_dir(".rvc").unwrap();
            fs::create_dir(".rvc/objects").unwrap();
            fs::create_dir(".rvc/refs").unwrap();
            fs::create_dir(".rvc/refs/heads").unwrap();
            fs::File::create(".rvc/HEAD").unwrap();
            fs::File::create(".rvc/index").unwrap();
            println!("Initialized the repositoruy.");
        }
    }

    Ok(())
}
