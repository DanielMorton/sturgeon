use crate::peptide::{cyclopeptide_sequencing, make_mass_vector};
use clap::Parser;
use std::error::Error;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CycloArgs {
    #[arg(long, required = true, value_name = "INPUT")]
    input: String,
}

pub fn run_cyclo(args: CycloArgs) -> Result<(), Box<dyn Error>> {
    let input = args.input;
    let spectrum = fs::read_to_string(input)?
        .split(' ')
        .map(|m| m.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let amino_masses = make_mass_vector()?;
    let cyclo = cyclopeptide_sequencing(&spectrum, &amino_masses)?;
    for c in &cyclo {
        println!("{}", c);
    }
    println!("{}", cyclo.len());
    Ok(())
}
