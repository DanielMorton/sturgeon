use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

pub fn blosum_matrix() -> Result<HashMap<(char, char), i32>, Box<dyn Error>> {
    let content = read_to_string("BLOSUM62.txt")?;
    let mut matrix = HashMap::new();
    let lines = content.lines().collect::<Vec<_>>();

    // Extract the header line to get the order of amino acids
    let header = lines[0];
    let amino_acids = header
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

    // Iterate over the remaining lines to parse the matrix
    for line in lines[1..].iter() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let row_aa = parts[0].chars().next().unwrap();

        for (col_aa, score_str) in amino_acids.iter().zip(parts[1..].iter()) {
            let score = score_str.parse::<i32>().unwrap();
            matrix.insert((row_aa, *col_aa), score);
        }
    }

    Ok(matrix)
}
