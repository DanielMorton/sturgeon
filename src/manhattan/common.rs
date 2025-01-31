use std::error::Error;
use crate::manhattan::backtrack::build_backtrack;
use crate::manhattan::direction::Direction;

fn output_lcs(backtrack: &[Vec<Direction>], s: &str, i: usize, j: usize) -> Result<String, Box<dyn Error>> {
    if i == 0 || j == 0 {
        return Ok(String::new());
    }

    match backtrack[i][j] {
        Direction::Down => output_lcs(backtrack, s, i - 1, j),
        Direction::Right => output_lcs(backtrack, s, i, j - 1),
        Direction::Diagonal => {
            let mut result = output_lcs(backtrack, s, i - 1, j - 1)?;
            result.push(s.chars().nth(i - 1).unwrap());
            Ok(result)
        }
    }
}

fn lcs(s: &str, t: &str) -> Result<String, Box<dyn Error>> {
    let backtrack = build_backtrack(s, t)?;
    output_lcs(&backtrack, s, s.len(), t.len())
}

mod tests {
    use std::error::Error;
    use crate::manhattan::common::lcs;

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