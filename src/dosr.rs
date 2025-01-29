use crate::motif::{
    get_consensus, median_list, motif_to_profile, randomized_motif_search, score_consensus,
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

    #[arg(long = "min", required = true, value_parser = value_parser!(usize))]
    min_kmer_length: usize,

    #[arg(long = "max", required = true, value_parser = value_parser!(usize))]
    max_kmer_length: usize,
}

impl DosRArgs {
    fn get_input(&self) -> Result<String, DosRParseError> {
        let result = self.input.clone();
        Ok(result)
    }
    fn get_num_iters(&self) -> Result<usize, DosRParseError> {
        Ok(self.num_iters)
    }
    fn get_min_kmer_length(&self) -> Result<usize, DosRParseError> {
        Ok(self.min_kmer_length)
    }
    fn get_max_kmer_length(&self) -> Result<usize, DosRParseError> {
        Ok(self.max_kmer_length)
    }
}
pub fn run_median(args: DosRArgs) -> Result<(), Box<dyn Error>> {
    let input_file = args.get_input()?;
    let (min_kmer_length, max_kmer_length) =
        (args.get_min_kmer_length()?, args.get_max_kmer_length()?);
    let dna = fs::read_to_string(input_file)?
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    for k in min_kmer_length..=max_kmer_length {
        let mut median = median_list(&dna, k)?;
        median.sort();
        println!("{} {}", k, median.len());
        median.iter().for_each(|s| println!("{}", s));
    }
    Ok(())
}

pub fn run_random(args: DosRArgs) -> Result<(), Box<dyn Error>> {
    let input_file = args.get_input()?;
    let num_iters = args.get_num_iters()?;
    let (min_kmer_length, max_kmer_length) =
        (args.get_min_kmer_length()?, args.get_max_kmer_length()?);
    let dna = fs::read_to_string(input_file)?
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    for k in min_kmer_length..=max_kmer_length {
        let motifs = randomized_motif_search(&dna, k, num_iters)?;
        let profile = motif_to_profile(&motifs, Some(1.0))?;
        let consensus = get_consensus(&motifs, &profile)?;
        let score = score_consensus(&motifs, &consensus)?;
        println!("{} {} {}", k, consensus, score);
    }
    Ok(())
}
