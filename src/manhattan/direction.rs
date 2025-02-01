#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Direction {
    Coordinate(usize, usize),
    Diagonal,
    Left,
    None,
    Start,
    Up,
}
