use crate::utils::InvalidNucleotideError;
use std::collections::HashMap;
use std::error::Error;

/*fn count_nucleotides(s: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let counts = s.chars().fold(vec![0, 0, 0, 0], |mut counts, nucleotide| {
        match nucleotide {
            'A' => counts[0] += 1,
            'C' => counts[1] += 1,
            'G' => counts[2] += 1,
            'T' => counts[3] += 1,
            c => Err(InvalidNucleotideError::new(c)),
        }
        counts
    });
    Ok(counts)
}*/
