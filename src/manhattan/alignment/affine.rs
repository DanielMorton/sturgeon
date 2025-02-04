use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::direction::Direction;
use num::{Bounded, Num};
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};

fn add_score<T>(value: T, score: T) -> T
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T> + Bounded,
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
) -> Result<
    (
        Vec<Vec<Direction>>,
        Vec<Vec<Direction>>,
        Vec<Vec<Direction>>,
        T,
        Direction,
    ),
    Box<dyn Error>,
>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T> + Bounded,
{
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let mut diagonal = vec![vec![T::zero(); t.len() + 1]; s.len() + 1];
    let mut up = vec![vec![T::zero(); t.len() + 1]; s.len() + 1];
    let mut left = vec![vec![T::zero(); t.len() + 1]; s.len() + 1];
    let mut diagonal_backtrack = vec![vec![Direction::None; t.len() + 1]; s.len() + 1];
    let mut up_backtrack = vec![vec![Direction::None; t.len() + 1]; s.len() + 1];
    let mut left_backtrack = vec![vec![Direction::None; t.len() + 1]; s.len() + 1];

    // Initialize first column
    for i in 1..=s.len() {
        up[i][0] = up[i - 1][0] - if i == 1 { gap_opening } else { gap_extension };
        up_backtrack[i][0] = Direction::Up;
        left[i][0] = T::min_value();
        diagonal[i][0] = up[i][0];
        diagonal_backtrack[i][0] = Direction::Up;
    }

    // Initialize first row
    for j in 1..=t.len() {
        up[0][j] = T::min_value();
        left[0][j] = left[0][j - 1] - if j == 1 { gap_opening } else { gap_extension };
        left_backtrack[0][j] = Direction::Left;
        diagonal[0][j] = left[0][j];
        diagonal_backtrack[0][j] = Direction::Left;
    }

    // Fill matrices
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            let up_from_diagonal = add_score(diagonal[i - 1][j], -gap_opening);
            let up_from_up = add_score(up[i - 1][j], -gap_extension);

            up[i][j] = up_from_diagonal.max(up_from_up);
            up_backtrack[i][j] = if up[i][j] == up_from_up {
                Direction::Up
            } else {
                Direction::Diagonal
            };

            let left_from_diagonal = add_score(diagonal[i][j - 1], -gap_opening);
            let left_from_left = add_score(left[i][j - 1], -gap_extension);

            left[i][j] = left_from_diagonal.max(left_from_left);
            left_backtrack[i][j] = if left[i][j] == left_from_left {
                Direction::Left
            } else {
                Direction::Diagonal
            };
            // Calculate score for match/mismatch
            let match_score = if s_chars[i - 1] == t_chars[j - 1] {
                match_reward
            } else {
                -mismatch_penalty
            };

            // Fill M matrix
            let diagonal_from_diagonal = add_score(diagonal[i - 1][j - 1], match_score);

            diagonal[i][j] = diagonal_from_diagonal.max(up[i][j]).max(left[i][j]);

            println!(
                "{} {} d {:?} u {:?} l {:?}",
                i, j, diagonal_from_diagonal, up[i][j], left[i][j]
            );
            // Set traceback for M
            diagonal_backtrack[i][j] = if diagonal[i][j] == left[i][j] {
                Direction::Left
            } else if diagonal[i][j] == up[i][j] {
                Direction::Up
            } else {
                Direction::Diagonal
            };
        }
    }
    println!("{:?}", diagonal);
    println!("{:?}", up);
    println!("{:?}", left);

    let score = diagonal[s.len()][t.len()]
        .max(up[s.len()][t.len()])
        .max(left[s.len()][t.len()]);
    let score_matrix = if score == left[s.len()][t.len()] {
        Direction::Left
    } else if score == up[s.len()][t.len()] {
        Direction::Up
    } else {
        Direction::Diagonal
    };
    Ok((
        diagonal_backtrack,
        up_backtrack,
        left_backtrack,
        score,
        score_matrix,
    ))
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
    let (diagonal_backtrack, up_backtrack, left_backtrack, score, score_matrix) = affine_backtrack(
        s,
        t,
        match_reward,
        mismatch_penalty,
        gap_opening,
        gap_extension,
    )?;
    println!("{:?}", diagonal_backtrack);
    println!("{:?}", up_backtrack);
    println!("{:?}", left_backtrack);
    backtrack_affine(
        &diagonal_backtrack,
        &up_backtrack,
        &left_backtrack,
        s,
        t,
        score,
        &score_matrix,
    )
}

fn backtrack_affine<T>(
    diagonal_backtrack: &[Vec<Direction>],
    up_backtrack: &[Vec<Direction>],
    left_backtrack: &[Vec<Direction>],
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
        current_matrix = diagonal_backtrack[i][j]
    } else if current_matrix == Direction::Up {
        current_matrix = up_backtrack[i][j]
    } else {
        current_matrix = left_backtrack[i][j]
    }

    while i > 0 || j > 0 {
        println!("{:?} {} {}", current_matrix, i, j);
        match current_matrix {
            Direction::Coordinate(x, y) => {
                i = x;
                j = y;
            }
            Direction::Diagonal => {
                let next_matrix = diagonal_backtrack[i][j];
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
                current_matrix = left_backtrack[i][j];
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
                current_matrix = up_backtrack[i][j];
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
