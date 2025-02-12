use crate::utils::hamming::hamming_distance;
use std::error::Error;

fn approximate_pattern_matching(
    text: &str,
    pattern: &str,
    max_diff: usize,
) -> Result<Vec<usize>, Box<dyn Error>> {
    // Early returns for edge cases
    if pattern.is_empty() || pattern.len() > text.len() {
        return Ok(Vec::new());
    }

    let pattern_len = pattern.len();
    let text_len = text.len();

    // Pre-allocate with estimated capacity
    let mut matches = Vec::with_capacity((text_len / pattern_len).max(1));

    for i in 0..=text_len - pattern_len {
        if let Ok(distance) = hamming_distance(&text[i..i + pattern_len], pattern) {
            if distance <= max_diff {
                matches.push(i);
            }
        }
    }

    Ok(matches)
}

fn approximate_pattern_count(
    text: &str,
    pattern: &str,
    max_diff: usize,
) -> Result<usize, Box<dyn Error>> {
    Ok(approximate_pattern_matching(text, pattern, max_diff)?.len())
}
#[cfg(test)]
mod tests {
    use crate::motif::r#match::{approximate_pattern_count, approximate_pattern_matching};
    use std::error::Error;
    use std::fs;

    #[test]
    fn test_approximate_pattern_matching1() -> Result<(), Box<dyn Error>> {
        let pattern = "ATTCTGGA";
        let text = "CGCCCGAATCCAGAACGCATTCCCATATTTCGGGACCACTGGCCTCCACGGTACGGACGTCAATCAAAT";
        assert_eq!(
            approximate_pattern_matching(text, pattern, 3)?,
            vec![6, 7, 26, 27]
        );
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_matching2() -> Result<(), Box<dyn Error>> {
        let pattern = "AAA";
        let text = "TTTTTTAAATTTTAAATTTTTT";
        assert_eq!(
            approximate_pattern_matching(text, pattern, 2)?,
            vec![4, 5, 6, 7, 8, 11, 12, 13, 14, 15]
        );
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_matching3() -> Result<(), Box<dyn Error>> {
        let pattern = "GAGCGCTGG";
        let text = "GAGCGCTGGGTTAACTCGCTACTTCCCGACGAGCGCTGTGGCGCAAATTGGCGATGAAACTGCAGAGAGAACTGGTCATCCAACTGAATTCTCCCCGCTATCGCATTTTGATGCGCGCCGCGTCGATT";
        assert_eq!(
            approximate_pattern_matching(text, pattern, 2)?,
            vec![0, 30, 66]
        );
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_matching4() -> Result<(), Box<dyn Error>> {
        let pattern = "AATCCTTTCA";
        let text = "CCAAATCCCCTCATGGCATGCATTCCCGCAGTATTTAATCCTTTCATTCTGCATATAAGTAGTGAAGGTATAGAAACCCGTTCAAGCCCGCAGCGGTAAAACCGAGAACCATGATGAATGCACGGCGATTGCGCCATAATCCAAACA";
        assert_eq!(
            approximate_pattern_matching(text, pattern, 3)?,
            vec![3, 36, 74, 137]
        );
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_matching5() -> Result<(), Box<dyn Error>> {
        let pattern = "CCGTCATCC";
        let text = "CCGTCATCCGTCATCCTCGCCACGTTGGCATGCATTCCGTCATCCCGTCAGGCATACTTCTGCATATAAGTACAAACATCCGTCATGTCAAAGGGAGCCCGCAGCGGTAAAACCGAGAACCATGATGAATGCACGGCGATTGC";
        assert_eq!(
            approximate_pattern_matching(text, pattern, 3)?,
            vec![0, 7, 36, 44, 48, 72, 79, 112]
        );
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_matching6() -> Result<(), Box<dyn Error>> {
        let pattern = "TTT";
        let text = "AAAAAA";
        assert_eq!(
            approximate_pattern_matching(text, pattern, 3)?,
            vec![0, 1, 2, 3]
        );
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_matching6a() -> Result<(), Box<dyn Error>> {
        let pattern = "TTT";
        let text = "AAAAAA";
        assert_eq!(approximate_pattern_matching(text, pattern, 2)?, vec![]);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_matching7() -> Result<(), Box<dyn Error>> {
        let pattern = "CCA";
        let text = "CCACCT";
        assert_eq!(approximate_pattern_matching(text, pattern, 0)?, vec![0]);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_matching8() -> Result<(), Box<dyn Error>> {
        let pattern = "AGGTACAT";
        let text = match fs::read_to_string("input_approximate_pattern_matching.txt") {
            Ok(g) => g,
            Err(e) => panic!("{}", e),
        };
        let output = match fs::read_to_string("output_approximate_pattern_matching.txt") {
            Ok(g) => g,
            Err(e) => panic!("{}", e),
        }
        .split(" ")
        .map(|s| str::parse::<usize>(s).unwrap())
        .collect::<Vec<_>>();
        assert_eq!(approximate_pattern_matching(&text, pattern, 5)?, output);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_count1() -> Result<(), Box<dyn Error>> {
        let pattern = "ATA";
        let text = "CGATCGAGTACCATAAG";
        assert_eq!(approximate_pattern_count(text, pattern, 1)?, 3);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_count2() -> Result<(), Box<dyn Error>> {
        let pattern = "AAA";
        let text = "TTTTTTAAATTTTAAATTTTTT";
        assert_eq!(approximate_pattern_count(text, pattern, 2)?, 10);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_count3() -> Result<(), Box<dyn Error>> {
        let pattern = "GAGG";
        let text = "TTTAGAGCCTTCAGAGG";
        assert_eq!(approximate_pattern_count(text, pattern, 2)?, 4);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_count4() -> Result<(), Box<dyn Error>> {
        let pattern = "AATC";
        let text = "CGATGCATTAAATCC";
        assert_eq!(approximate_pattern_count(text, pattern, 2)?, 7);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_count5() -> Result<(), Box<dyn Error>> {
        let pattern = "CCC";
        let text = "ACCCGCCCTCCCGGC";
        assert_eq!(approximate_pattern_count(text, pattern, 1)?, 11);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_count6() -> Result<(), Box<dyn Error>> {
        let pattern = "TTT";
        let text = "AAAAAA";
        assert_eq!(approximate_pattern_count(text, pattern, 3)?, 4);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_count7() -> Result<(), Box<dyn Error>> {
        let pattern = "CCA";
        let text = "CCACCT";
        assert_eq!(approximate_pattern_count(text, pattern, 0)?, 1);
        Ok(())
    }

    #[test]
    fn test_approximate_pattern_count8() -> Result<(), Box<dyn Error>> {
        let pattern = "TACAG";
        let text = "GAATCCGCCAAGTACCAAGATGTAAGTGAGGAGCGCTTAGGTCTGTACTGCGCATAAGCCTTAACGCGAAGTATGGATATGCTCCCCGGATACAGGTTTGGGATTTGGCGGTTACCTAAGCTAACGGTGAGACCGATATGACGAGGTTCCTATCTTAATCATATTCACATACTGAACGAGGCGCCCAGTTTCTTCTCACCAATATGTCAGGAAGCTACAGTGCAGCATTATCCACACCATTCCACTTATCCTTGAACGGAAGTCTTATGCGAAGATTATTCTGAGAAGCCCTTGTGCCCTGCATCACGATTTGCAGACTGACAGGGAATCTTAAGGCCACTCAAA";
        assert_eq!(approximate_pattern_count(text, pattern, 2)?, 27);
        Ok(())
    }
}
