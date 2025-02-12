use crate::manhattan::backtrack::build_backtrack;
use crate::manhattan::direction::Direction;
use std::error::Error;

fn output_lcs(
    backtrack: &[Vec<Direction>],
    s: &str,
    i: usize,
    j: usize,
) -> Result<String, Box<dyn Error>> {
    if i == 0 || j == 0 {
        return Ok(String::new());
    }

    match backtrack[i][j] {
        Direction::Coordinate(x, y) => output_lcs(backtrack, s, x, y),
        Direction::Diagonal => {
            let mut result = output_lcs(backtrack, s, i - 1, j - 1)?;
            result.push(s.chars().nth(i - 1).unwrap());
            Ok(result)
        }
        Direction::Left => output_lcs(backtrack, s, i, j - 1),
        Direction::None => panic!("No direction at {} {}", i, j),
        Direction::Start => output_lcs(backtrack, s, 0, 0),
        Direction::Up => output_lcs(backtrack, s, i - 1, j),
    }
}

fn lcs(s: &str, t: &str) -> Result<String, Box<dyn Error>> {
    let backtrack = build_backtrack(s, t)?;
    output_lcs(&backtrack, s, s.len(), t.len())
}
#[cfg(test)]
mod tests {
    use crate::manhattan::common::lcs;
    use std::error::Error;

    #[test]
    fn test_lcs1() -> Result<(), Box<dyn Error>> {
        assert_eq!(lcs("GACT", "ATG")?, "AT");
        Ok(())
    }

    #[test]
    fn test_lcs2() -> Result<(), Box<dyn Error>> {
        assert_eq!(lcs("ACTGAG", "GACTGG")?, "ACTGG");
        Ok(())
    }

    #[test]
    fn test_lcs3() -> Result<(), Box<dyn Error>> {
        assert_eq!(lcs("AC", "AC")?, "AC");
        Ok(())
    }

    #[test]
    fn test_lcs4() -> Result<(), Box<dyn Error>> {
        assert_eq!(lcs("GGGGT", "CCCCT")?, "T");
        Ok(())
    }

    #[test]
    fn test_lcs5() -> Result<(), Box<dyn Error>> {
        assert_eq!(lcs("TGGGG", "TCCCC")?, "T");
        Ok(())
    }

    #[test]
    fn test_lcs6() -> Result<(), Box<dyn Error>> {
        assert_eq!(lcs("AA", "CGTGGAT")?, "A");
        Ok(())
    }

    #[test]
    fn test_lcs7() -> Result<(), Box<dyn Error>> {
        assert_eq!(lcs("GGTGACGT", "CT")?, "CT");
        Ok(())
    }
}
