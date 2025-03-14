#![allow(dead_code)]

use crate::burrows_wheeler::{run_bwt, BWTArgs};
use crate::bwt_matching::{run_bwt_matching, BWTMatchingArgs};
use crate::cyclo::{run_convo_cyclo, run_cyclo, run_leader_cyclo, CycloArgs};
use crate::dosr::{run_median, run_random, DosRArgs};
use crate::ori::{run_ori, OriArgs};
use crate::pylogeny::{run_phylogeny, PhylogenyArgs};
use crate::synteny::{run_synteny, SyntenyArgs};
use crate::translate::{run_translation, TranslateArgs};
use crate::upgma::{run_neighbor_join, run_upgma};
use clap::{Parser, Subcommand};
use std::error::Error;

mod burrows_wheeler;
mod bwt;
mod bwt_matching;
mod cyclo;
mod dosr;
mod genome;
mod graph;
mod manhattan;
mod motif;
mod ori;
mod peptide;
mod pylogeny;
mod synteny;
mod translate;
mod translation;
mod upgma;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "bwt")]
    BWT(BWTArgs),
    #[command(name = "bwt-matching")]
    BWTMatching(BWTMatchingArgs),
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
    #[command(name = "neighbor-join")]
    NeighborJoin(PhylogenyArgs),
    #[command(name = "ori")]
    Ori(OriArgs),
    #[command(name = "phylogeny")]
    Phylogeny(PhylogenyArgs),
    #[command(name = "synteny")]
    Synteny(SyntenyArgs),
    #[command(name = "translate")]
    Translate(TranslateArgs),
    #[command(name = "upgma")]
    UPGMA(PhylogenyArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::BWT(args) => run_bwt(args),
        Commands::BWTMatching(args) => run_bwt_matching(args),
        Commands::ConvolutionCyclopeptideSequencing(args) => run_convo_cyclo(args),
        Commands::CyclopeptideSequencing(args) => run_cyclo(args),
        Commands::DosRMedian(args) => run_median(args),
        Commands::DosRRandom(args) => run_random(args),
        Commands::LeaderBoardCyclopeptideSequencing(args) => run_leader_cyclo(args),
        Commands::NeighborJoin(args) => run_neighbor_join(args),
        Commands::Ori(args) => run_ori(args),
        Commands::Phylogeny(args) => run_phylogeny(args),
        Commands::Synteny(args) => run_synteny(args),
        Commands::Translate(args) => run_translation(args),
        Commands::UPGMA(args) => run_upgma(args),
    }
}
