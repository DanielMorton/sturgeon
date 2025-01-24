use crate::ori::{run_ori, OriArgs};
use clap::{Parser, Subcommand};
use std::error::Error;

mod graph;
mod motif;
mod ori;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "ori")]
    Ori(OriArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Ori(args) => run_ori(args),
    }
}
