use crate::bwt::{fasta_burrows_wheeler_transform, fasta_burrows_wheeler_transform_sa_is, suffix_array, suffix_array_induced_sorting};
use crate::utils::{print_hms, Fasta, DNA_BYTES};
use clap::Parser;
use std::error::Error;
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
    let bwt = fasta_burrows_wheeler_transform(&fasta)?;
    print_hms(&start);

    let start = Instant::now();
    let bwt_sa_is = fasta_burrows_wheeler_transform_sa_is(&fasta, &DNA_BYTES)?;
    print_hms(&start);
    println!("{} {}", bwt.len(), bwt_sa_is.len());
    println!("{}", &bwt[..10]);
    println!("{}", &bwt_sa_is[..10]);
    println!("{}", bwt == bwt_sa_is);

    let output_file = format!("{}_bwt.txt", input_file.split('.').next().unwrap());
    //fs::write(output_file, bwt).expect("Unable to write file");
    Ok(())
}
