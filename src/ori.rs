use crate::motif::{frequent_words_with_mismatches_reverse_complement, minimum_skew};
use crate::utils::{Fasta, DNA};
use clap::{value_parser, Parser};
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OriParseError {
    #[error("Input File {0} does not exist")]
    InvalidInput(String),

    #[error("Invalid Kmer size input: {0}")]
    InvalidKmerSize(usize),

    #[error("Invalid maximum hamming distance: {0}")]
    InvalidDistance(usize),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct OriArgs {
    #[arg(long, required = true, value_name = "INPUT")]
    input: String,

    #[arg(short = 'k', required = true, value_parser = value_parser!(usize))]
    kmer_length: usize,

    #[arg(short = 'd', required = true, value_parser = value_parser!(usize))]
    distance: usize,

    #[arg(short = 'w', required = false, value_parser = value_parser!(usize), default_value="500")]
    window: usize,
}

impl OriArgs {
    pub fn get_input(&self) -> Result<String, OriParseError> {
        Ok(self.input.to_owned())
    }

    pub fn get_kmer_length(&self) -> Result<usize, OriParseError> {
        Ok(self.kmer_length)
    }

    pub fn get_distance(&self) -> Result<usize, OriParseError> {
        Ok(self.distance)
    }

    pub fn get_window(&self) -> Result<usize, OriParseError> {
        Ok(self.window)
    }
}

pub fn run_ori(args: OriArgs) -> Result<(), Box<dyn Error>> {
    let input_file = args.get_input()?;
    let fasta = Fasta::read_file_component(input_file)?;

    let min_skew = minimum_skew(&fasta.text)?;

    let window = args.get_window()?;
    let kmer_length = args.get_kmer_length()?;
    let max_distance = args.get_distance()?;

    let ori_start = min_skew[0];
    let ori_end = ori_start + window;
    let kmers = frequent_words_with_mismatches_reverse_complement(
        &fasta.text[ori_start..ori_end],
        kmer_length,
        max_distance,
        &DNA,
    )?;
    for kmer in kmers {
        println!("{}", kmer);
    }
    Ok(())
}
