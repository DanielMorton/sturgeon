use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::alignment::backtrack::backtrack_alignment;
use crate::manhattan::direction::Direction;
use num::Num;
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};

fn overlap_backtrack<T>(
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

    // Initialize first row
    for j in 1..=t.len() {
        current_row[j] = current_row[j - 1] - indel_penalty;
        backtrack[0][j] = Direction::Left;
    }

    let mut max_score = T::zero();
    let mut max_j = 0;

    // Variables to store the previous diagonal score
    let mut prev_diagonal;
    let mut temp;

    for i in 1..=s.len() {
        backtrack[i][0] = Direction::Start;
        prev_diagonal = current_row[0];
        current_row[0] = T::zero(); // Local alignment can start anywhere

        for j in 1..=t.len() {
            // Store the current score before updating
            temp = current_row[j];

            // Calculate scores using the single vector
            let match_score = if s_chars[i - 1] == t_chars[j - 1] {
                match_reward
            } else {
                -mismatch_penalty
            };

            // Calculate scores using the single vector
            let diagonal_score = prev_diagonal + match_score;
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
    }

    for j in 1..=t.len() {
        if current_row[j] > max_score {
            max_score = current_row[j];
            max_j = j;
        }
    }

    // Store the endpoint coordinates in the backtrack matrix
    if max_score > current_row[t.len()] {
        current_row[t.len()] = max_score;
        backtrack[s.len()][t.len()] = Direction::Coordinate(s.len(), max_j);
    }

    Ok((backtrack, current_row[t.len()]))
}

pub fn overlap_alignment<T>(
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
    let (backtrack, score) =
        overlap_backtrack(s, t, match_reward, mismatch_penalty, indel_penalty)?;

    // Backtrack to find the alignment
    backtrack_alignment(&backtrack, s, t, score)
}

mod tests {
    use crate::manhattan::alignment::alignment::AlignmentResult;
    use crate::manhattan::alignment::overlap::overlap_alignment;
    use std::error::Error;

    #[test]
    fn test_overlap_alignment1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            overlap_alignment("GAGA", "GAT", 1, 1, 2)?,
            AlignmentResult::new(2, "GA", "GA")
        );
        Ok(())
    }

    #[test]
    fn test_overlap_alignment2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            overlap_alignment("CCAT", "AT", 1, 1, 1)?,
            AlignmentResult::new(2, "AT", "AT")
        );
        Ok(())
    }

    #[test]
    fn test_overlap_alignment3() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            overlap_alignment("GAT", "CAT", 1, 5, 1)?,
            AlignmentResult::new(1, "-AT", "CAT")
        );
        Ok(())
    }

    #[test]
    fn test_overlap_alignment4() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            overlap_alignment("ATCACT", "AT", 1, 5, 1)?,
            AlignmentResult::new(1, "ACT", "A-T")
        );
        Ok(())
    }

    #[test]
    fn test_overlap_alignment5() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            overlap_alignment("ATCACT", "ATG", 1, 1, 5)?,
            AlignmentResult::new(0, "", "")
        );
        Ok(())
    }

    #[test]
    fn test_overlap_alignment6() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            overlap_alignment("CAGAGATGGCCG", "ACG", 3, 2, 1)?,
            AlignmentResult::new(5, "-CG", "ACG")
        );
        Ok(())
    }

    #[test]
    fn test_overlap_alignment7() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            overlap_alignment("CTT", "AGCATAAAGCATT", 2, 3, 1)?,
            AlignmentResult::new(0, "", "")
        );
        Ok(())
    }
}
