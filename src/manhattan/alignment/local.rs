use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::alignment::backtrack::backtrack_alignment;
use crate::manhattan::direction::Direction;
use num::Num;
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};

fn local_backtrack<T>(
    s: &str,
    t: &str,
    match_reward: T,
    mismatch_penalty: T,
    indel_penalty: T,
) -> Result<(Vec<Vec<Direction>>, T), Box<dyn Error>>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T>,
{
    let s_chars = s.as_bytes();
    let t_chars = t.as_bytes();

    // Still need backtrack matrix for path reconstruction
    let mut backtrack = vec![vec![Direction::None; t.len() + 1]; s.len() + 1];

    // Single vector for scores
    let mut current_row = vec![T::zero(); t.len() + 1];

    // Initialize first row with Direction::Start
    for j in 1..=t.len() {
        backtrack[0][j] = Direction::Start;
    }

    let mut max_score = T::zero();
    let (mut max_i, mut max_j) = (0, 0);

    // Variables to store the previous diagonal score
    let mut prev_diagonal;
    let mut temp;

    // Process row by row
    for i in 1..=s.len() {
        backtrack[i][0] = Direction::Start;
        prev_diagonal = current_row[0];
        current_row[0] = T::zero(); // Local alignment can start anywhere

        for j in 1..=t.len() {
            // Store the current score before updating
            temp = current_row[j];

            let match_score = if s_chars[i - 1] == t_chars[j - 1] {
                match_reward
            } else {
                -mismatch_penalty
            };

            // Calculate scores using the single vector
            let diagonal_score = prev_diagonal + match_score;
            let up_score = current_row[j] - indel_penalty;
            let left_score = current_row[j - 1] - indel_penalty;

            // Start with zero (local alignment can start anywhere)
            current_row[j] = T::zero();
            backtrack[i][j] = Direction::Start;

            // Update if diagonal is better
            if diagonal_score > current_row[j] {
                current_row[j] = diagonal_score;
                backtrack[i][j] = Direction::Diagonal;
            }

            // Update if left is better
            if left_score > current_row[j] {
                current_row[j] = left_score;
                backtrack[i][j] = Direction::Left;
            }

            // Update if up is better
            if up_score > current_row[j] {
                current_row[j] = up_score;
                backtrack[i][j] = Direction::Up;
            }

            // Track maximum score position
            if current_row[j] > max_score {
                max_score = current_row[j];
                max_i = i;
                max_j = j;
            }

            // Update previous diagonal for next iteration
            prev_diagonal = temp;
        }
    }

    // Store the endpoint coordinates in the backtrack matrix
    if max_score > current_row[t.len()] {
        current_row[t.len()] = max_score;
        backtrack[s.len()][t.len()] = Direction::Coordinate(max_i, max_j);
    }

    Ok((backtrack, current_row[t.len()]))
}

pub fn local_alignment<T>(
    s: &str,
    t: &str,
    match_reward: T,
    mismatch_penalty: T,
    indel_penalty: T,
) -> Result<AlignmentResult<T>, Box<dyn Error>>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T>,
{
    // Initialize the score and backtrack matrices
    let (backtrack, score) = local_backtrack(s, t, match_reward, mismatch_penalty, indel_penalty)?;

    // Backtrack to find the alignment
    backtrack_alignment(&backtrack, s, t, score)
}

mod tests {
    use crate::manhattan::alignment::alignment::AlignmentResult;
    use crate::manhattan::alignment::local::local_alignment;
    use std::error::Error;

    #[test]
    fn test_global_alignment1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            local_alignment("GAGA", "GAT", 1, 1, 2)?,
            AlignmentResult::new(2, "GA", "GA")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            local_alignment("AGC", "ATC", 3, 3, 1)?,
            AlignmentResult::new(4, "AG-C", "A-TC")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment3() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            local_alignment("AT", "AG", 1, 1, 1)?,
            AlignmentResult::new(1, "A", "A")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment4() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            local_alignment("TAACG", "ACGTG", 1, 1, 1)?,
            AlignmentResult::new(3, "ACG", "ACG")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment5() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            local_alignment("CAGAGATGGCCG", "ACG", 3, 2, 1)?,
            AlignmentResult::new(6, "CG", "CG")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment6() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            local_alignment("CTT", "AGCATAAAGCATT", 2, 3, 1)?,
            AlignmentResult::new(5, "C-TT", "CATT")
        );
        Ok(())
    }
}
