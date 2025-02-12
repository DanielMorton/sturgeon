use crate::manhattan::direction::Direction;
use num::{Bounded, Num};

#[derive(Debug, Clone, PartialEq)]
pub struct AffineScore<T> {
    pub diagonal: Vec<Vec<T>>,
    pub up: Vec<Vec<T>>,
    pub left: Vec<Vec<T>>,
}

impl<T: Num + Copy + Bounded> AffineScore<T> {
    pub fn new(rows: usize, cols: usize, gap_opening: T, gap_extension: T) -> Self {
        let mut diagonal = vec![vec![T::zero(); cols]; rows];
        let mut up = vec![vec![T::zero(); cols]; rows];
        let mut left = vec![vec![T::zero(); cols]; rows];

        for i in 1..rows {
            up[i][0] = up[i - 1][0] - if i == 1 { gap_opening } else { gap_extension };
            left[i][0] = T::min_value();
            diagonal[i][0] = diagonal[i - 1][0] - if i == 1 { gap_opening } else { gap_extension }
        }

        // Initialize first row
        for j in 1..cols {
            up[0][j] = T::min_value();
            left[0][j] = left[0][j - 1] - if j == 1 { gap_opening } else { gap_extension };
            diagonal[0][j] = diagonal[0][j - 1] - if j == 1 { gap_opening } else { gap_extension };
        }
        AffineScore { diagonal, up, left }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AffineBacktrack {
    pub diagonal: Vec<Vec<Direction>>,
    pub up: Vec<Vec<Direction>>,
    pub left: Vec<Vec<Direction>>,
}

impl AffineBacktrack {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut diagonal = vec![vec![Direction::None; cols]; rows];
        let mut up = vec![vec![Direction::None; cols]; rows];
        let mut left = vec![vec![Direction::None; cols]; rows];
        for i in 1..rows {
            up[i][0] = Direction::Up;
            diagonal[i][0] = Direction::Up;
        }

        // Initialize first row
        for j in 1..cols {
            left[0][j] = Direction::Left;
            diagonal[0][j] = Direction::Left;
        }
        AffineBacktrack { diagonal, up, left }
    }
}
