use crate::translation::translate_rna_code;
use clap::Parser;
use std::error::Error;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct TranslateArgs {
    #[arg(long, required = true, value_name = "INPUT")]
    input: String,
}
pub fn run_translation(args: TranslateArgs) -> Result<(), Box<dyn Error>> {
    let input = args.input;
    let rna = fs::read_to_string(input)?;
    let protein = translate_rna_code(&rna)?;
    println!("{}", protein);
    Ok(())
}
