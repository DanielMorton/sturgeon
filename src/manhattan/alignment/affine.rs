use crate::manhattan::alignment::affine_classes::{AffineBacktrack, AffineScore};
use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::direction::Direction;
use num::{Bounded, Num};
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};
use std::os::unix::raw::time_t;

fn add_score<T>(value: T, score: T) -> T
where
    T: Num + Copy + Bounded,
{
    if value != T::min_value() {
        value + score
    } else {
        value
    }
}

fn affine_backtrack<T>(
    s: &str,
    t: &str,
    match_reward: T,
    mismatch_penalty: T,
    gap_opening: T,
    gap_extension: T,
) -> Result<(AffineBacktrack, T, Direction), Box<dyn Error>>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T> + Bounded,
{
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    let (s_len, t_len) = (s.len(), t.len());

    let mut scores = AffineScore::new(s_len + 1, t_len + 1, gap_opening, gap_extension);
    let mut backtrack = AffineBacktrack::new(s_len + 1, t_len + 1);

    // Fill matrices
    for i in 1..=s_len {
        for j in 1..=t_len {
            let up_from_diagonal = add_score(scores.diagonal[i - 1][j], -gap_opening);
            let up_from_up = add_score(scores.up[i - 1][j], -gap_extension);

            scores.up[i][j] = up_from_diagonal.max(up_from_up);
            backtrack.up[i][j] = if scores.up[i][j] == up_from_up {
                Direction::Up
            } else {
                Direction::Diagonal
            };

            let left_from_diagonal = add_score(scores.diagonal[i][j - 1], -gap_opening);
            let left_from_left = add_score(scores.left[i][j - 1], -gap_extension);

            scores.left[i][j] = left_from_diagonal.max(left_from_left);
            backtrack.left[i][j] = if scores.left[i][j] == left_from_left {
                Direction::Left
            } else {
                Direction::Diagonal
            };
            // Calculate score for match/mismatch
            let match_score = if s_bytes[i - 1] == t_bytes[j - 1] {
                match_reward
            } else {
                -mismatch_penalty
            };

            // Fill M matrix
            let diagonal_from_diagonal = add_score(scores.diagonal[i - 1][j - 1], match_score);

            scores.diagonal[i][j] = diagonal_from_diagonal
                .max(scores.up[i][j])
                .max(scores.left[i][j]);

            // Set traceback for M
            backtrack.diagonal[i][j] = if scores.diagonal[i][j] == scores.left[i][j] {
                Direction::Left
            } else if scores.diagonal[i][j] == scores.up[i][j] {
                Direction::Up
            } else {
                Direction::Diagonal
            };
        }
    }

    let score = scores.diagonal[s_len][t_len]
        .max(scores.up[s_len][t_len])
        .max(scores.left[s_len][t_len]);
    let score_matrix = if score == scores.left[s_len][t_len] {
        Direction::Left
    } else if score == scores.up[s_len][t_len] {
        Direction::Up
    } else {
        Direction::Diagonal
    };
    Ok((backtrack, score, score_matrix))
}
fn affine_gap_alignment<T>(
    s: &str,
    t: &str,
    match_reward: T,
    mismatch_penalty: T,
    gap_opening: T,
    gap_extension: T,
) -> Result<AlignmentResult<T>, Box<dyn Error>>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T> + Bounded,
{
    let (backtrack, score, score_matrix) = affine_backtrack(
        s,
        t,
        match_reward,
        mismatch_penalty,
        gap_opening,
        gap_extension,
    )?;
    backtrack_affine(&backtrack, s, t, score, &score_matrix)
}

fn backtrack_affine<T>(
    backtrack: &AffineBacktrack,
    s: &str,
    t: &str,
    score: T,
    score_matrix: &Direction,
) -> Result<AlignmentResult<T>, Box<dyn Error>>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T>,
{
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let mut align1 = String::new();
    let mut align2 = String::new();
    let mut i = s.len();
    let mut j = t.len();
    let mut current_matrix = score_matrix.clone();
    if current_matrix == Direction::Diagonal {
        current_matrix = backtrack.diagonal[i][j]
    } else if current_matrix == Direction::Up {
        current_matrix = backtrack.up[i][j]
    } else {
        current_matrix = backtrack.left[i][j]
    }

    while i > 0 || j > 0 {
        match current_matrix {
            Direction::Coordinate(x, y) => {
                i = x;
                j = y;
            }
            Direction::Diagonal => {
                let next_matrix = backtrack.diagonal[i][j];
                if next_matrix == Direction::Diagonal {
                    align1 = format!("{}{}", s_bytes[i - 1] as char, align1);
                    align2 = format!("{}{}", t_bytes[j - 1] as char, align2);
                    i -= 1;
                    j -= 1;
                }
                current_matrix = next_matrix;
            }
            Direction::Left => {
                align1 = format!("-{}", align1);
                align2 = format!("{}{}", t_bytes[j - 1] as char, align2);
                current_matrix = backtrack.left[i][j];
                j -= 1;
            }
            Direction::None => break,
            Direction::Start => {
                i = 0;
                j = 0;
            }
            Direction::Up => {
                align1 = format!("{}{}", s_bytes[i - 1] as char, align1);
                align2 = format!("-{}", align2);
                current_matrix = backtrack.up[i][j];
                i -= 1;
            }
        }
    }

    // Reverse and convert to strings

    Ok(AlignmentResult::new(score, &align1, &align2))
}

mod tests {
    use crate::manhattan::alignment::affine::affine_gap_alignment;
    use crate::manhattan::alignment::alignment::AlignmentResult;
    use std::error::Error;

    #[test]
    fn test_affine_gap_alignment1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("GA", "GTTA", 1, 3, 2, 1)?,
            AlignmentResult::new(-1, "G--A", "GTTA")
        );
        Ok(())
    }

    #[test]
    fn test_affine_gap_alignment2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("TTT", "TT", 1, 5, 3, 1)?,
            AlignmentResult::new(-1, "TTT", "TT-")
        );
        Ok(())
    }

    #[test]
    fn test_affine_gap_alignment3() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("GAT", "AT", 1, 5, 5, 1)?,
            AlignmentResult::new(-3, "GAT", "-AT")
        );
        Ok(())
    }

    #[test]
    fn test_affine_gap_alignment4() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("CCAT", "GAT", 1, 5, 2, 1)?,
            AlignmentResult::new(-3, "CC-AT", "--GAT")
        );
        Ok(())
    }

    #[test]
    fn test_affine_gap_alignment5() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("CAGGT", "TAC", 1, 2, 3, 2)?,
            AlignmentResult::new(-8, "CAGGT", "TAC--")
        );
        Ok(())
    }

    #[test]
    fn test_affine_gap_alignment6() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("GTTCCAGGTA", "CAGTAGTCGT", 2, 3, 3, 2)?,
            AlignmentResult::new(-8, "--GTTCCAG--GTA", "CAGT---AGTCGT-")
        );
        Ok(())
    }

    #[test]
    fn test_affine_gap_alignment7() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("AGCTAGCCTAG", "GT", 1, 3, 1, 1)?,
            AlignmentResult::new(-7, "AGCTAGCCTAG", "-G-T-------")
        );
        Ok(())
    }

    #[test]
    fn test_affine_gap_alignment8() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("AA", "CAGTGTCAGTA", 2, 1, 2, 1)?,
            AlignmentResult::new(-7, "-A--------A", "CAGTGTCAGTA")
        );
        Ok(())
    }

    #[test]
    fn test_affine_gap_alignment9() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            affine_gap_alignment("ACGTA", "ACT", 5, 2, 15, 5)?,
            AlignmentResult::new(-12, "ACGTA", "ACT--")
        );
        Ok(())
    }
}
