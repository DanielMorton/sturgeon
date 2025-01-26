use crate::dosr::{run_median, run_random, DosRArgs};
use crate::ori::{run_ori, OriArgs};
use clap::{Parser, Subcommand};
use std::error::Error;

mod dosr;
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
    #[command(name = "dosr-median")]
    DosRMedian(DosRArgs),
    #[command(name = "dosr-random")]
    DosRRandom(DosRArgs),
    #[command(name = "ori")]
    Ori(OriArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Ori(args) => run_ori(args),
        Commands::DosRMedian(args) => run_median(args),
        Commands::DosRRandom(args) => run_random(args),
    }
}
