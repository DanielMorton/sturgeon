use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct InvalidNucleotideError {
    character: char,
}

impl Error for InvalidNucleotideError {}

impl InvalidNucleotideError {
    pub(crate) fn new(character: char) -> Self {
        InvalidNucleotideError { character }
    }
}

impl Display for InvalidNucleotideError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Invalid nucleotide '{}'", self.character)
    }
}

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

impl Display for InvalidNucleotidePositionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Invalid nucleotide '{}' at position {}",
            self.character, self.position
        )
    }
}
