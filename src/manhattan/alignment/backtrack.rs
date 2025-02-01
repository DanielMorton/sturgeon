use crate::manhattan::alignment::alignment::AlignmentResult;
use crate::manhattan::direction::Direction;
use num::Num;
use std::error::Error;
use std::fmt::Debug;
use std::ops::{Mul, Neg};

pub fn backtrack_alignment<T>(
    backtrack: &[Vec<Direction>],
    s1: &str,
    s2: &str,
    score: T,
) -> Result<AlignmentResult<T>, Box<dyn Error>>
where
    T: Num + Debug + Copy + Ord + Mul + Neg<Output = T>,
{
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let mut align1 = String::new();
    let mut align2 = String::new();
    let mut i = s1.len();
    let mut j = s2.len();

    while i > 0 || j > 0 {
        match backtrack[i][j] {
            Direction::Coordinate(x, y) => {
                i = x;
                j = y;
            }
            Direction::Diagonal => {
                align1 = format!("{}{}", s1_bytes[i - 1] as char, align1);
                align2 = format!("{}{}", s2_bytes[j - 1] as char, align2);
                i -= 1;
                j -= 1;
            }
            Direction::Left => {
                align1 = format!("-{}", align1);
                align2 = format!("{}{}", s2_bytes[j - 1] as char, align2);
                j -= 1;
            }
            Direction::None => break,
            Direction::Up => {
                align1 = format!("{}{}", s1_bytes[i - 1] as char, align1);
                align2 = format!("-{}", align2);
                i -= 1;
            }
            Direction::Start => {
                i = 0;
                j = 0;
            }
        }
    }

    Ok(AlignmentResult::new(score, &align1, &align2))
}
