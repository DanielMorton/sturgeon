use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::alignment::backtrack::backtrack_alignment;
use crate::manhattan::direction::Direction;
use num::Num;
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};

fn local_backtrck<T>(
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
    let mut max_score = T::zero();
    let (mut max_i, mut max_j) = (0, 0);

    // Initialize first row and column
    for i in 1..=s1.len() {
        if i > 0 {
            backtrack[i][0] = Direction::Start;
        }
    }
    for j in 1..=s2.len() {
        if j > 0 {
            backtrack[0][j] = Direction::Start;
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

            backtrack[i][j] = Direction::Start;

            // Find the maximum score and its direction
            if diagonal_score > score[i][j] {
                score[i][j] = diagonal_score;
                backtrack[i][j] = Direction::Diagonal;
            }

            if left_score > score[i][j] {
                score[i][j] = left_score;
                backtrack[i][j] = Direction::Left;
            }

            if up_score > score[i][j] {
                score[i][j] = up_score;
                backtrack[i][j] = Direction::Up;
            }

            if score[i][j] > max_score {
                max_score = score[i][j];
                (max_i, max_j) = (i, j);
            }
        }
    }

    if max_score > score[s1.len()][s2.len()] {
        score[s1.len()][s2.len()] = max_score;
        backtrack[s1.len()][s2.len()] = Direction::Coordinate(max_i, max_j)
    }

    Ok((backtrack, score[s1.len()][s2.len()]))
}

pub fn local_alignment<T>(
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
    let (backtrack, score) = local_backtrck(s1, s2, match_reward, mismatch_penalty, indel_penalty)?;

    // Backtrack to find the alignment
    backtrack_alignment(&backtrack, s1, s2, score)
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
