use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};
use num::Num;
use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::alignment::backtrack::backtrack_alignment;
use crate::manhattan::direction::Direction;

fn fitting_backtrack<T>(s: &str,
                        t: &str,
                        blosum: &HashMap<(char, char), T>,
                        indel_penalty: T) -> Result<(Vec<Vec<Direction>>, T), Box<dyn Error>>
    where
        T: Num + Debug + Copy + Ord + Mul + Neg<Output = T>,
{
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    // Still need backtrack matrix for path reconstruction
    let mut backtrack = vec![vec![Direction::None; t.len() + 1]; s.len() + 1];

    // Single vector for scores
    let mut current_row = vec![T::zero(); t.len() + 1];

    // Initialize first row
    for j in 1..=t.len() {
        current_row[j] = current_row[j - 1] - indel_penalty;
        backtrack[0][j] = Direction::Left;
    }

    let mut max_score = T::zero();
    let mut max_i = 0;

    // Variables to store the previous diagonal score
    let mut prev_diagonal;
    let mut prev_last;
    let mut temp;

    for i in 1..=s.len() {
        println!("{:?}", current_row);
        backtrack[i][0] = Direction::Start;
        prev_diagonal = current_row[0];
        prev_last = current_row[t.len()];
        current_row[0] = T::zero(); // Local alignment can start anywhere

        for j in 1..=t.len() {
            // Store the current score before updating
            temp = current_row[j];

            // Calculate scores using the single vector
            let diagonal_score = prev_diagonal + *blosum.get(&(s_chars[i - 1], t_chars[j - 1])).unwrap();
            let up_score = current_row[j] - indel_penalty;
            let left_score = current_row[j - 1] - indel_penalty;

            // Find maximum score and direction
            if diagonal_score >= up_score && diagonal_score >= left_score {
                current_row[j] = diagonal_score;
                backtrack[i][j] = Direction::Diagonal;
            } else if left_score >= up_score {
                current_row[j] = left_score;
                backtrack[i][j] = Direction::Left;
            } else {
                current_row[j] = up_score;
                backtrack[i][j] = Direction::Up;
            }

            // Update previous diagonal for next iteration
            prev_diagonal = temp;
        }
        if current_row[t.len()] > prev_last {
            max_score = current_row[t.len()];
            max_i = i;
        }
    }

    // Store the endpoint coordinates in the backtrack matrix
    if max_score > current_row[t.len()] {
        current_row[t.len()] = max_score;
        backtrack[s.len()][t.len()] = Direction::Coordinate(max_i, t.len());
    }

    Ok((backtrack,  current_row[t.len()]))
}

pub fn fitting_alignment<T>(
    s: &str,
    t: &str,
    blosum: &HashMap<(char, char), T>,
    indel_penalty: T,
) -> Result<AlignmentResult<T>, Box<dyn Error>>
    where
        T: Num + Debug + Copy + Ord + Mul + Neg<Output = T>, {

    // Initialize the score and backtrack matrices
    let (backtrack, score) = fitting_backtrack(s, t, blosum, indel_penalty)?;

    println!("{:?}", backtrack);
    // Backtrack to find the alignment
    backtrack_alignment(&backtrack, s, t, score)
}

mod tests {
    use crate::manhattan::alignment::alignment::AlignmentResult;
    use crate::manhattan::alignment::local::local_alignment;
    use std::error::Error;
    use crate::manhattan::alignment::fitting::fitting_alignment;
    use crate::utils::blosum_matrix;

    #[test]
    fn test_fitting_alignment1() -> Result<(), Box<dyn Error>> {
        let blosum = blosum_matrix()?;
        assert_eq!(
            fitting_alignment("DISCREPANTLY", "PATENT", &blosum, 1)?,
            AlignmentResult::new(20, "PA--NT", "PATENT")
        );
        Ok(())
    }

    #[test]
    fn test_fitting_alignment2() -> Result<(), Box<dyn Error>> {
        let blosum = blosum_matrix()?;
        assert_eq!(
            fitting_alignment("ARKANSAS", "SASS", &blosum, 1)?,
            AlignmentResult::new(11, "SA-S", "SASS")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment3() -> Result<(), Box<dyn Error>> {
        let blosum = blosum_matrix()?;
        assert_eq!(
            fitting_alignment("DISCREPANTLY", "DISCRETE", &blosum, 1)?,
            AlignmentResult::new(34, "DISCREPANT-", "DISCRE---TE")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment4() -> Result<(), Box<dyn Error>> {
        let blosum = blosum_matrix()?;
        assert_eq!(
            fitting_alignment("CANT", "CA", &blosum, 1)?,
            AlignmentResult::new(34, "CA", "CA")
        );
        Ok(())
    }
}