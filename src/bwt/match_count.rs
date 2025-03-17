use crate::bwt::counts::{char_counts, get_first_col_starts};
use crate::bwt::fm::calculate_fm_index;
use crate::bwt::matching::bw_matching;
use crate::utils::Fasta;
use std::collections::HashMap;
use std::error::Error;

fn bwt_match_count(
    first_col_starts: &[usize],
    fm_index: &[Vec<usize>],
    counts: &[usize],
    char_map: &HashMap<u8, usize>,
    pattern: &str,
) -> Result<usize, Box<dyn Error>> {
    Ok(
        if let Some((top, bottom)) =
            bw_matching(first_col_starts, fm_index, counts, char_map, pattern)?
        {
            bottom - top + 1
        } else {
            0
        },
    )
}

pub fn bw_match_counts_fasta(
    bwt: &str,
    patterns: &[Fasta],
    char_map: &HashMap<u8, usize>,
    fw_step: usize,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let pattern_strings = patterns.iter().map(|f| f.text.as_str()).collect::<Vec<_>>();
    bw_match_counts(bwt, &pattern_strings, char_map, fw_step)
}

// More optimized implementation using arrays instead of HashMaps for better performance
pub fn bw_match_counts(
    bwt: &str,
    patterns: &[&str],
    char_map: &HashMap<u8, usize>,
    fw_step: usize,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let bwt_bytes = bwt.as_bytes();

    // Count character occurrences
    let counts = char_counts(bwt_bytes, char_map)?;

    // Calculate starting positions in first column
    let first_col_starts = get_first_col_starts(&counts)?;

    // Build the occurrence array more efficiently
    let fm_index = calculate_fm_index(bwt_bytes, char_map, fw_step)?;

    // Match each pattern
    let results = patterns
        .iter()
        .map(|&pattern| bwt_match_count(&first_col_starts, &fm_index, &counts, char_map, pattern))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::bwt::bwt::burrows_wheeler_transform_sa_is;
    use crate::bwt::match_count::bw_match_counts;
    use crate::utils::DNA_BW;
    use std::error::Error;

    #[test]
    fn test_bw_matching1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            bw_match_counts(
                "TCCTCTATGAGATCCTATTCTATGAAACCTTCA$GACCAAAATTCTCCGGC",
                &vec!["CCT", "CAC", "GAG", "CAG", "ATC"],
                &DNA_BW,
                1
            )?,
            vec![2, 1, 1, 0, 1]
        );
        Ok(())
    }

    #[test]
    fn test_bw_matching2() -> Result<(), Box<dyn Error>> {
        let text = "AATCGGGTTCAATCGGGGT";
        let (bwt, _) = burrows_wheeler_transform_sa_is(text, &DNA_BW)?;
        assert_eq!(
            bw_match_counts(&bwt, &vec!["ATCG", "GGGT"], &DNA_BW, 1)?,
            vec![2, 2]
        );
        Ok(())
    }

    #[test]
    fn test_bw_matching3() -> Result<(), Box<dyn Error>> {
        let text = "ATATATATAT";
        let (bwt, _) = burrows_wheeler_transform_sa_is(text, &DNA_BW)?;
        assert_eq!(
            bw_match_counts(&bwt, &vec!["GT", "AGCT", "TAA", "AAT", "AATAT"], &DNA_BW, 1)?,
            vec![0; 5]
        );
        Ok(())
    }
}
