use std::error::Error;
use crate::manhattan::direction::Direction;

pub fn build_backtrack(s: &str, t: &str) -> Result<Vec<Vec<Direction>>, Box<dyn Error>> {

    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    // Initialize score and backtrack matrices
    let mut score = vec![vec![0; t.len() + 1]; s.len() + 1];
    let mut backtrack = vec![vec![Direction::Down; t.len() + 1]; s.len() + 1];

    // Fill the matrices
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            // Get scores for each possible move
            let mut diag = score[i-1][j-1];
            let up = score[i-1][j];
            let left = score[i][j-1];

            // If characters match, add to diagonal score
            if s_bytes[i-1] == t_bytes[j-1] {
                diag += 1;
            }

            // Choose the best move
            score[i][j] = up;
            backtrack[i][j] = Direction::Down;

            if left > score[i][j] {
                score[i][j] = left;
                backtrack[i][j] = Direction::Right;
            }

            if diag > score[i][j] {
                score[i][j] = diag;
                backtrack[i][j] = Direction::Diagonal;
            }
        }
    }

    Ok(backtrack)
}