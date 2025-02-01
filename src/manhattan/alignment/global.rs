use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::alignment::backtrack::backtrack_alignment;
use crate::manhattan::direction::Direction;
use crate::utils::transpose;
use num::Num;
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};

fn global_backtrack<T>(
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

    // We still need the full backtrack matrix for the path reconstruction
    let mut backtrack = vec![vec![Direction::None; t.len() + 1]; s.len() + 1];

    // Single vector for scores - length of the longer string plus one
    let mut current_row = vec![T::zero(); t.len() + 1];

    // Initialize first row
    for j in 1..=t.len() {
        current_row[j] = current_row[j - 1] - indel_penalty;
        backtrack[0][j] = Direction::Left;
    }

    // Variables to store the previous diagonal score
    let mut prev_diagonal;
    let mut temp;

    // Process row by row
    for i in 1..=s.len() {
        prev_diagonal = current_row[0];
        current_row[0] = current_row[0] - indel_penalty;
        backtrack[i][0] = Direction::Up;

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

    Ok((backtrack, current_row[t.len()]))
}

pub fn global_alignment<T>(
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
    let (backtrack, score) = global_backtrack(s, t, match_reward, mismatch_penalty, indel_penalty)?;

    // Backtrack to find the alignment
    backtrack_alignment(&backtrack, s, t, score)
}

mod tests {
    use crate::manhattan::alignment::alignment::AlignmentResult;
    use crate::manhattan::alignment::global::global_alignment;
    use std::error::Error;

    #[test]
    fn test_global_alignment1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            global_alignment("GAGA", "GAT", 1, 1, 2)?,
            AlignmentResult::new(-1, "GAGA", "GA-T")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            global_alignment("ACG", "ACT", 1, 3, 1)?,
            AlignmentResult::new(0, "ACG-", "AC-T")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment3() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            global_alignment("AT", "AG", 1, 1, 1)?,
            AlignmentResult::new(0, "AT", "AG")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment4() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            global_alignment("TCA", "CA", 2, 5, 1)?,
            AlignmentResult::new(3, "TCA", "-CA")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment5() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            global_alignment("TTTTCCTT", "CC", 1, 10, 1)?,
            AlignmentResult::new(-4, "TTTTCCTT", "----CC--")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment6() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            global_alignment("ACAGATTAG", "T", 2, 3, 2)?,
            AlignmentResult::new(-14, "ACAGATTAG", "------T--")
        );
        Ok(())
    }

    #[test]
    fn test_global_alignment7() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            global_alignment("G", "ACATACGATG", 3, 1, 2)?,
            AlignmentResult::new(-15, "---------G", "ACATACGATG")
        );
        Ok(())
    }
}
