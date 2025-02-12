use std::error::Error;
use std::fs;
use std::time::Instant;
use clap::{Parser, value_parser};
use crate::genome::two_break_distance;
use crate::motif::{shared_kmers, synteny_to_chromosome};
use crate::utils::{Fasta, print_hms};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SyntenyArgs {
    #[arg(long, required = true, value_name = "file1")]
    file1: String,

    #[arg(long, required = true, value_name = "file2")]
    file2: String,

    #[arg(short = 'k', required = true, value_parser = value_parser!(usize))]
    kmer_length: usize
}
pub fn run_synteny(args: SyntenyArgs) -> Result<(), Box<dyn Error>> {
    let dna1 = fs::read_to_string(args.file1)?;
    let dna2 = Fasta::read_file(args.file2)?;
    println!("E. coli Length {}", dna1.len());
    println!("S. enterica length {}", dna2.len());
    let start = Instant::now();
    let syn_matches = shared_kmers(args.kmer_length, &dna1, &dna2.text)?;
    println!("{}", syn_matches.len());
    print_hms(&start);

    let (chromosome1, chromosome2) = synteny_to_chromosome(&syn_matches, args.kmer_length, &dna1, &dna2.text)?;
    println!("Distance {}", two_break_distance(&vec![chromosome1.clone()],&vec![chromosome2.clone()])?);
    Ok(())
}