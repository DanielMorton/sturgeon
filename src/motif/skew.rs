use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct InvalidNucleotideError {
    character: char,
}

impl Error for InvalidNucleotideError {}

impl InvalidNucleotideError {
    fn new(character: char) -> Self {
        InvalidNucleotideError { character }
    }
}

impl std::fmt::Display for InvalidNucleotideError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid nucleotide '{}'", self.character)
    }
}
fn skew_score(c: char) -> Result<i32, InvalidNucleotideError> {
    match c {
        'A' | 'T' => Ok(0),
        'C' => Ok(-1),
        'G' => Ok(1),
        _ => Err(InvalidNucleotideError::new(c)),
    }
}

pub(crate) fn minimum_skew(genome: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    if genome.is_empty() {
        return Ok(Vec::new());
    }

    // Pre-allocate results vector with estimated capacity
    let mut positions = Vec::new();
    let mut min_score = 0;
    let mut current_score = 0;

    // Track positions and score in a single pass
    for (i, c) in genome.chars().enumerate() {
        current_score += skew_score(c)?;

        if current_score < min_score {
            // New minimum found, clear previous positions
            min_score = current_score;
            positions.clear();
            positions.push(i + 1);
        } else if current_score == min_score {
            positions.push(i + 1);
        }
    }

    Ok(positions)
}

mod tests {
    use crate::motif::skew::{minimum_skew, skew_score, InvalidNucleotideError};
    use std::error::Error;
    use std::fs;

    #[test]
    fn test_skew() -> Result<(), Box<dyn Error>> {
        assert_eq!(skew_score('C')?, -1);
        assert_eq!(skew_score('G')?, 1);
        assert_eq!(skew_score('A')?, 0);
        assert_eq!(skew_score('T')?, 0);
        Ok(())
    }

    #[test]
    fn test_minimum_skew1() -> Result<(), Box<dyn Error>> {
        let genome = "TAAAGACTGCCGAGAGGCCAACACGAGTGCTAGAACGAGGGGCGTAAACGCGGGTCCGAT";
        assert_eq!(minimum_skew(genome)?, vec![11, 24]);
        Ok(())
    }

    #[test]
    fn test_minimum_skew2() -> Result<(), Box<dyn Error>> {
        let genome = "ACCG";
        assert_eq!(minimum_skew(genome)?, vec![3]);
        Ok(())
    }

    #[test]
    fn test_minimum_skew3() -> Result<(), Box<dyn Error>> {
        let genome = "ACCC";
        assert_eq!(minimum_skew(genome)?, vec![4]);
        Ok(())
    }

    #[test]
    fn test_minimum_skew4() -> Result<(), Box<dyn Error>> {
        let genome = "CCGGGT";
        assert_eq!(minimum_skew(genome)?, vec![2]);
        Ok(())
    }

    #[test]
    fn test_minimum_skew5() -> Result<(), Box<dyn Error>> {
        let genome = "CCGGCCGG";
        assert_eq!(minimum_skew(genome)?, vec![2, 6]);
        Ok(())
    }

    #[test]
    fn test_minimum_skew6() -> Result<(), Box<dyn Error>> {
        let genome = match fs::read_to_string("input_skew.txt") {
            Ok(g) => g,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            minimum_skew(&genome)?,
            vec![89969, 89970, 89971, 90345, 90346]
        );
        Ok(())
    }

    #[test]
    fn test_minimum_skewA() -> Result<(), Box<dyn Error>> {
        let genome = "AAAAA";
        assert_eq!(minimum_skew(genome)?, vec![1, 2, 3, 4, 5]);
        Ok(())
    }
}
