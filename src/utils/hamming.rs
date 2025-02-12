use thiserror::Error;

#[derive(Error, Debug)]
pub enum LengthMismatchError {
    #[error("String Lengths are not equal. {0} != {1}")]
    NotEqualError(usize, usize),

    #[error("Pattern length must be less than text length. {0} > {1}")]
    PatternTooLongError(usize, usize),
}

pub fn hamming_distance(text1: &str, text2: &str) -> Result<usize, LengthMismatchError> {
    if text1.len() != text2.len() {
        return Err(LengthMismatchError::NotEqualError(text1.len(), text2.len()));
    }

    Ok(text1
        .chars()
        .zip(text2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count())
}

pub fn min_hamming_distance(pattern: &str, text: &str) -> Result<usize, LengthMismatchError> {
    let pattern_len = pattern.len();
    let text_len = text.len();

    if pattern_len > text_len {
        return Err(LengthMismatchError::PatternTooLongError(
            pattern_len,
            text_len,
        ));
    }

    let mut distance = pattern_len;
    for i in 0..=text_len - pattern_len {
        let current_distance = hamming_distance(pattern, &text[i..i + pattern_len])?;
        distance = distance.min(current_distance);
    }

    Ok(distance)
}
#[cfg(test)]
mod tests {
    use crate::utils::hamming::{hamming_distance, min_hamming_distance, LengthMismatchError};

    #[test]
    fn test_hamming_distance1() -> Result<(), LengthMismatchError> {
        let text1 = "GGGCCGTTGGT";
        let text2 = "GGACCGTTGAC";
        assert_eq!(hamming_distance(text1, text2)?, 3);
        Ok(())
    }

    #[test]
    fn test_hamming_distance2() -> Result<(), LengthMismatchError> {
        let text1 = "AAAA";
        let text2 = "TTTT";
        assert_eq!(hamming_distance(text1, text2)?, 4);
        Ok(())
    }

    #[test]
    fn test_hamming_distance3() -> Result<(), LengthMismatchError> {
        let text1 = "ACGTACGT";
        let text2 = "TACGTACG";
        assert_eq!(hamming_distance(text1, text2)?, 8);
        Ok(())
    }

    #[test]
    fn test_hamming_distance4() -> Result<(), LengthMismatchError> {
        let text1 = "ACGTACGT";
        let text2 = "CCCCCCCC";
        assert_eq!(hamming_distance(text1, text2)?, 6);
        Ok(())
    }

    #[test]
    fn test_hamming_distance5() -> Result<(), LengthMismatchError> {
        let text1 = "ACGTACGT";
        let text2 = "TGCATGCA";
        assert_eq!(hamming_distance(text1, text2)?, 8);
        Ok(())
    }

    #[test]
    fn test_hamming_distance6() -> Result<(), LengthMismatchError> {
        let text1 = "GATAGCAGCTTCTGAACTGGTTACCTGCCGTGAGTAAATTAAAATTTTATTGACTTAGGTCACTAAATACT";
        let text2 = "AATAGCAGCTTCTCAACTGGTTACCTCGTATGAGTAAATTAGGTCATTATTGACTCAGGTCACTAACGTCT";
        assert_eq!(hamming_distance(text1, text2)?, 15);
        Ok(())
    }

    #[test]
    fn test_hamming_distance7() -> Result<(), LengthMismatchError> {
        let text1 = "AGAAACAGACCGCTATGTTCAACGATTTGTTTTATCTCGTCACCGGGATATTGCGGCCACTCATCGGTCAGTTGATTACGCAGGGCGTAAATCGCCAGAATCAGGCTG";
        let text2 = "AGAAACCCACCGCTAAAAACAACGATTTGCGTAGTCAGGTCACCGGGATATTGCGGCCACTAAGGCCTTGGATGATTACGCAGAACGTATTGACCCAGAATCAGGCTC";
        assert_eq!(hamming_distance(text1, text2)?, 28);
        Ok(())
    }

    #[test]
    fn test_min_hamming_distance1() -> Result<(), LengthMismatchError> {
        let pattern = "ACG";
        let text = "ACGT";
        assert_eq!(min_hamming_distance(pattern, text)?, 0);
        Ok(())
    }
}
