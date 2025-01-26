use crate::motif::{
    get_consensus, median_string, motif_to_profile, randomized_motif_search, score_consensus,
    score_kmer,
};
use clap::{value_parser, Parser};
use std::error::Error;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DosRParseError {
    #[error("Input File {0} does not exist")]
    InvalidInput(String),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct DosRArgs {
    #[arg(long, required = true, value_name = "INPUT")]
    input: String,

    #[arg(short = 'n', required = false, value_parser = value_parser!(usize), default_value="10000")]
    num_iters: usize,
}

impl DosRArgs {
    pub fn get_input(&self) -> Result<String, DosRParseError> {
        let result = self.input.clone();
        Ok(result)
    }

    pub fn get_num_iters(&self) -> Result<usize, DosRParseError> {
        Ok(self.num_iters)
    }
}
pub fn run_median(args: DosRArgs) -> Result<(), Box<dyn Error>> {
    let input_file = args.get_input()?;
    let dna = fs::read_to_string(input_file)?
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    for k in 8..=12 {
        let median = median_string(&dna, k)?;
        let score = score_kmer(&dna, &median)?;
        println!("{} {} {}", k, median, score);
    }
    Ok(())
}

pub fn run_random(args: DosRArgs) -> Result<(), Box<dyn Error>> {
    let input_file = args.get_input()?;
    let num_iters = args.get_num_iters()?;
    let dna = fs::read_to_string(input_file)?
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    for k in 8..=12 {
        let motifs = randomized_motif_search(&dna, k, num_iters)?;
        let profile = motif_to_profile(&motifs, Some(1.0))?;
        let consensus = get_consensus(&motifs, &profile)?;
        let score = score_consensus(&motifs, &consensus)?;
        println!("{} {} {}", k, consensus, score);
    }
    Ok(())
}
