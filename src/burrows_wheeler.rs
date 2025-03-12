use std::collections::HashSet;
use crate::bwt::fasta_burrows_wheeler_transform_sa_is;
use crate::utils::{print_hms, Fasta, DNA_BYTES, DNA_BYTES_N};
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

    println!("{}", fasta.iter().map(|f| f.text.len()).max().unwrap());
    let start = Instant::now();
    let bwt = fasta_burrows_wheeler_transform_sa_is(&fasta[0], &DNA_BYTES_N)?;
    print_hms(&start);

    let output_file = format!("{}_bwt.txt", input_file.split('.').next().unwrap());
    fs::write(output_file, bwt).expect("Unable to write file");
    Ok(())
}
