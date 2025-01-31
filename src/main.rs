#![allow(dead_code)]

use crate::cyclo::{run_convo_cyclo, run_cyclo, run_leader_cyclo, CycloArgs};
use crate::dosr::{run_median, run_random, DosRArgs};
use crate::ori::{run_ori, OriArgs};
use crate::translate::{run_translation, TranslateArgs};
use clap::{Parser, Subcommand};
use std::error::Error;

mod cyclo;
mod dosr;
mod graph;
mod motif;
mod ori;
mod peptide;
mod translate;
mod translation;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "convolution")]
    ConvolutionCyclopeptideSequencing(CycloArgs),
    #[command(name = "cyclo-sequence")]
    CyclopeptideSequencing(CycloArgs),
    #[command(name = "dosr-median")]
    DosRMedian(DosRArgs),
    #[command(name = "dosr-random")]
    DosRRandom(DosRArgs),
    #[command(name = "leaderboard")]
    LeaderBoardCyclopeptideSequencing(CycloArgs),
    #[command(name = "ori")]
    Ori(OriArgs),
    #[command(name = "translate")]
    Translate(TranslateArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::ConvolutionCyclopeptideSequencing(args) => run_convo_cyclo(args),
        Commands::CyclopeptideSequencing(args) => run_cyclo(args),
        Commands::DosRMedian(args) => run_median(args),
        Commands::DosRRandom(args) => run_random(args),
        Commands::LeaderBoardCyclopeptideSequencing(args) => run_leader_cyclo(args),
        Commands::Ori(args) => run_ori(args),
        Commands::Translate(args) => run_translation(args),
    }
}
