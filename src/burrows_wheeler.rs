use std::collections::{HashMap, HashSet};
use crate::bwt::{fasta_burrows_wheeler_transform, suffix_array, suffix_array_induced_sorting};
use crate::utils::{print_hms, Fasta, DNA_INDEX, DNA_BYTES};
use clap::Parser;
use std::error::Error;
use std::fs;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct BWTArgs {
    #[arg(long, required = true, value_name = "INPUT")]
    input: String,
}

impl BWTArgs {
    pub fn get_input(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.input.to_owned())
    }
}

pub fn run_bwt(args: BWTArgs) -> Result<(), Box<dyn Error>> {
    let input_file = args.get_input()?;
    let fasta = Fasta::read_file(&input_file)?;

    let start = Instant::now();
    let text = format!("{}$", &fasta.text);
    let suffix_array = suffix_array(&text)?;
    print_hms(&start);
    let start = Instant::now();
    let sa_is = suffix_array_induced_sorting(&fasta.text.as_bytes(), &DNA_BYTES)?;
    print_hms(&start);
    //println!("Suffix Array Len {}", suffix_array.len());
    println!("SA-IS Len {}", sa_is.len());
    println!("{}", suffix_array == sa_is);
    //let start = Instant::now();
    //let bwt = fasta_burrows_wheeler_transform(&fasta)?;
    //print_hms(&start);

    //let output_file = format!("{}_bwt.txt", input_file.split('.').next().unwrap());
    //fs::write(output_file, bwt).expect("Unable to write file");
    Ok(())
}
