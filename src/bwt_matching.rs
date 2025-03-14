use crate::burrows_wheeler::BWTArgs;
use crate::bwt::{
    bw_matching, bw_matching_fasta, fasta_burrows_wheeler_transform,
    fasta_burrows_wheeler_transform_sa_is,
};
use crate::utils::{dna_complement, print_hms, Fasta, DNA_BW, DNA_BYTES};
use clap::Parser;
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
    let fasta = Fasta::read_file(&genome_file)?
        .get(0) // Assume single genome sequence
        .ok_or("Genome file is empty")?
        .clone();

    let pattern_file = args.get_pattern_file()?;
    let patterns = Fasta::read_file(&pattern_file)?;

    let start = Instant::now();
    let bwt = fasta_burrows_wheeler_transform_sa_is(&fasta, &DNA_BYTES)?;
    print_hms(&start);

    let start = Instant::now();
    let forward_matches = bw_matching_fasta(&bwt, &patterns, &DNA_BW, 1)?;
    print_hms(&start);

    let reverse_patterns = patterns
        .iter()
        .map(|f| dna_complement(&f.text))
        .collect::<Result<Vec<_>, _>>()?;
    let reverse_patterns = reverse_patterns
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>();

    let start = Instant::now();
    let reverse_matches = bw_matching(&bwt, &reverse_patterns, &DNA_BW, 1)?;
    print_hms(&start);

    let matches = forward_matches
        .iter()
        .zip(reverse_matches.iter())
        .map(|(&f, &r)| max(f, r))
        .collect::<Vec<_>>();
    let mismatches = patterns
        .iter()
        .zip(matches.iter())
        .filter(|(_, &m)| m == 0)
        .map(|(f, _)| f)
        .collect::<Vec<_>>();

    println!("{:?}", mismatches);
    Ok(())
}
