use crate::bwt::{fasta_burrows_wheeler_transform, fasta_burrows_wheeler_transform_sa_is};
use crate::utils::{print_hms, Fasta, DNA_BW_N, DNA_BYTES};
use clap::Parser;
use std::collections::{HashMap, HashSet};
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
    let chromosomes = fasta.iter().filter(|&f|f.title.contains("reference primary assembly") &&
        !f.title.contains("unlocalized genomic contig,") &&
         f.title.contains("Homo sapiens chromosome "))
        .map(|f| f.upper()).collect::<Vec<_>>();
    for (i, ch) in chromosomes.iter().enumerate() {
        println!("{} {}", i, ch.title);
    }


   /* let start = Instant::now();
    let bwt = fasta_burrows_wheeler_transform(&chromosomes[0])?;
    print_hms(&start);*/

    let start = Instant::now();
    let (bwt_sa, _) = fasta_burrows_wheeler_transform_sa_is(&chromosomes[20], &DNA_BW_N)?;
    print_hms(&start);

    println!("{}", bwt_sa.len());
    //let output_file = format!("{}_bwt.txt", input_file.split('.').next().unwrap());
    //fs::write(output_file, bwt).expect("Unable to write file");
    Ok(())
}
