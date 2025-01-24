use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct InvalidNucleotidePositionError {
    character: char,
    position: usize,
}

impl Error for InvalidNucleotidePositionError {}

impl InvalidNucleotidePositionError {
    pub fn new(character: char, position: usize) -> Self {
        InvalidNucleotidePositionError {
            character,
            position,
        }
    }
}

impl std::fmt::Display for InvalidNucleotidePositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Invalid nucleotide '{}' at position {}",
            self.character, self.position
        )
    }
}

fn reverse_complement(
    pattern: &str,
    complement_map: &HashMap<char, char>,
) -> Result<String, InvalidNucleotidePositionError> {
    let mut result = String::with_capacity(pattern.len());

    for (p, c) in pattern.chars().rev().enumerate() {
        if let Some(&complement) = complement_map.get(&c) {
            result.push(complement);
        } else {
            return Err(InvalidNucleotidePositionError::new(
                c,
                pattern.len() - p - 1,
            ));
        }
    }

    Ok(result)
}

pub fn dna_complement(pattern: &str) -> Result<String, InvalidNucleotidePositionError> {
    reverse_complement(pattern, &DNA_COMPLEMENT_MAP)
}

pub fn rna_complement(pattern: &str) -> Result<String, InvalidNucleotidePositionError> {
    reverse_complement(pattern, &RNA_COMPLEMENT_MAP)
}

mod tests {
    use crate::utils::reverse::{dna_complement, InvalidNucleotidePositionError};
    use std::error::Error;

    #[test]
    fn test_dna_complement1() -> Result<(), Box<dyn Error>> {
        let dna = "AAAACCCGGT";
        let dna_reverse = dna_complement(dna)?;
        assert_eq!(dna_reverse, format!("{}", "ACCGGGTTTT"));
        Ok(())
    }

    #[test]
    fn test_dna_complement2() -> Result<(), Box<dyn Error>> {
        let dna = "ACACAC";
        let dna_reverse = dna_complement(dna)?;
        assert_eq!(dna_reverse, format!("{}", "GTGTGT"));
        Ok(())
    }

    #[test]
    fn test_dna_complement_error() -> Result<(), Box<dyn Error>> {
        let dna = "ADACAC";
        let dna_reverse = dna_complement(dna).unwrap_err();
        assert_eq!(dna_reverse, InvalidNucleotidePositionError::new('D', 1));
        Ok(())
    }
}
