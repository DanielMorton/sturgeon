use crate::peptide::{
    convolution_cyclopeptide_list, cyclopeptide_sequencing,
    leaderboard_cyclopeptide_list, make_mass_vector,
};
use crate::utils::print_hms;
use clap::Parser;
use std::error::Error;
use std::fs;
use std::time::Instant;

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

pub fn run_leader_cyclo(args: CycloArgs) -> Result<(), Box<dyn Error>> {
    let input = args.input;
    let buffer = fs::read_to_string(input)?;
    let mut text = buffer.split('\n');
    let n = text.next().ok_or("Empty file.")?.parse::<usize>()?;
    let spectrum = text
        .next()
        .ok_or("Missing Spectrum")?
        .split(' ')
        .map(|m| m.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let amino_masses = (57..=200).collect::<Vec<_>>(); //make_mass_vector()?;
    let start = Instant::now();
    let cyclo = leaderboard_cyclopeptide_list(&spectrum, &amino_masses, n)?;
    for c in cyclo {
        println!("{}", c.to_string());
    }
    print_hms(&start);
    Ok(())
}

pub fn run_convo_cyclo(args: CycloArgs) -> Result<(), Box<dyn Error>> {
    let input = args.input;
    let buffer = fs::read_to_string(input)?;
    let mut text = buffer.split('\n');
    let m = text.next().ok_or("Empty file.")?.parse::<usize>()?;
    let n = text
        .next()
        .ok_or("Missing Leaderboard Size")?
        .parse::<usize>()?;
    let spectrum = text
        .next()
        .ok_or("Missing Spectrum")?
        .split(' ')
        .map(|m| m.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let start = Instant::now();
    let cyclo = convolution_cyclopeptide_list(&spectrum, m, n)?;
    for c in cyclo[..86].iter() {
        print!("{} ", c.to_string());
    }
    println!();
    print_hms(&start);
    Ok(())
}
