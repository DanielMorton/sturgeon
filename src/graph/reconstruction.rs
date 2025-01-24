use crate::graph::err::{EmptyPathError, InvalidPathError};
use std::error::Error;

fn genome_path(path: &[&str]) -> Result<String, Box<dyn Error>> {
    if path.is_empty() {
        return Err(Box::new(EmptyPathError));
    }

    let mut genome = path[0].to_owned();

    for i in 1..path.len() {
        // Check if the prefix of path[i] matches the suffix of path[i-1]
        if !path[i].starts_with(&path[i - 1][1..]) {
            return Err(Box::new(InvalidPathError::new(i, &path[i - 1], &path[i])));
            // Return None if the match fails
        }

        // Add the last character of path[i] to the result
        if let Some(last_char) = path[i].chars().last() {
            genome.push(last_char);
        }
    }
    Ok(genome)
}
