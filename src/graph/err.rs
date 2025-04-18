use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub(crate) struct EmptyGraphError;

impl Error for EmptyGraphError {}

impl Display for EmptyGraphError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Empty Graph.")
    }
}

#[derive(Debug)]
pub(crate) struct EmptyPathError;

impl Error for EmptyPathError {}

impl Display for EmptyPathError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Empty Path List.")
    }
}

#[derive(Debug)]
pub(crate) struct InvalidPathError {
    position: usize,
    prev: String,
    curr: String,
}

impl Error for InvalidPathError {}

impl InvalidPathError {
    pub(crate) fn new(position: usize, prev: &str, curr: &str) -> Self {
        let (p, c) = (prev.to_owned(), curr.to_owned());
        InvalidPathError {
            position,
            prev: p,
            curr: c,
        }
    }
}

impl Display for InvalidPathError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Invalid Path at position {}: prefix of '{}' does not match suffix of '{}'.",
            self.position, self.curr, self.prev
        )
    }
}
