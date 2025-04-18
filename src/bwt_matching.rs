use crate::burrows_wheeler::BWTArgs;
use crate::bwt::{
    bw_match_counts, bw_match_counts_fasta, fasta_burrows_wheeler_transform,
    fasta_burrows_wheeler_transform_sa_is,
};
use crate::utils::{dna_complement, print_hms, Fasta, DNA_BW_N};
use clap::Parser;
use rayon::prelude::*;
use std::cmp::max;
use std::error::Error;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct BWTMatchingArgs {
    #[arg(long, required = true, value_name = "genome_file")]
    genome_file: String,
    #[arg(long, required = true, value_name = "pattern_file")]
    pattern_file: String,
}

impl BWTMatchingArgs {
    pub fn get_genome_file(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.genome_file.to_owned())
    }

    pub fn get_pattern_file(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.pattern_file.to_owned())
    }
}

pub fn run_bwt_matching(args: BWTMatchingArgs) -> Result<(), Box<dyn Error>> {
    let genome_file = args.get_genome_file()?;
    let fasta = Fasta::read_file_component(&genome_file)?;

    let pattern_file = args.get_pattern_file()?;
    let patterns = Fasta::read_file(&pattern_file)?;
    let reverse_patterns = patterns
        .iter()
        .map(|f| dna_complement(&f.text))
        .collect::<Result<Vec<_>, _>>()?;
    let patterns = patterns.into_iter().map(|p| p.text).collect::<Vec<_>>();
    let patterns = patterns.iter().map(|p| p.as_str()).collect::<Vec<&str>>();
    let patterns = [patterns, reverse_patterns.iter().map(|s| s.as_str()).collect()].concat();
    println!("{:?}", patterns);

    let start = Instant::now();

    let (bwt, _) = fasta_burrows_wheeler_transform_sa_is(&fasta, &DNA_BW_N)?;
    let pattern_matches = bw_match_counts(&bwt, &patterns, &DNA_BW_N, 1)?;
    print_hms(&start);

    println!("{:?}", pattern_matches);
    Ok(())
}
