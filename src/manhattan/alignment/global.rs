use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::alignment::backtrack::backtrack_alignment;
use crate::manhattan::direction::Direction;
use num::Num;
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};

fn global_backtrck<T>(
    s1: &str,
    s2: &str,
    match_reward: T,
    mismatch_penalty: T,
    indel_penalty: T,
) -> Result<(Vec<Vec<Direction>>, T), Box<dyn Error>>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T>,
{
    let mut score = vec![vec![T::zero(); s2.len() + 1]; s1.len() + 1];
    let mut backtrack = vec![vec![Direction::None; s2.len() + 1]; s1.len() + 1];

    // Initialize first row and column
    for i in 1..=s1.len() {
        score[i][0] = score[i - 1][0] - indel_penalty;
        if i > 0 {
            backtrack[i][0] = Direction::Up;
        }
    }
    for j in 1..=s2.len() {
        score[0][j] = score[0][j - 1] - indel_penalty;
        if j > 0 {
            backtrack[0][j] = Direction::Left;
        }
    }

    // Fill the matrices
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            let match_score = if s1_bytes[i - 1] == s2_bytes[j - 1] {
                match_reward
            } else {
                -mismatch_penalty
            };

            let diagonal_score = score[i - 1][j - 1] + match_score;
            let up_score = score[i - 1][j] - indel_penalty;
            let left_score = score[i][j - 1] - indel_penalty;

            // Find the maximum score and its direction
            score[i][j] = diagonal_score;
            backtrack[i][j] = Direction::Diagonal;

            if left_score > score[i][j] {
                score[i][j] = left_score;
                backtrack[i][j] = Direction::Left;
            }

            if up_score > score[i][j] {
                score[i][j] = up_score;
                backtrack[i][j] = Direction::Up;
            }
        }
    }
    Ok((backtrack, score[s1.len()][s2.len()]))
}

pub fn global_alignment<T>(
    s1: &str,
    s2: &str,
    match_reward: T,
    mismatch_penalty: T,
    indel_penalty: T,
) -> Result<AlignmentResult<T>, Box<dyn Error>>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T>,
{
    // Initialize the score and backtrack matrices
    let (backtrack, score) =
        global_backtrck(s1, s2, match_reward, mismatch_penalty, indel_penalty)?;

    // Backtrack to find the alignment
    backtrack_alignment(&backtrack, s1, s2, score)
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
