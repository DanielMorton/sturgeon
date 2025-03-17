use crate::bwt::counts::{char_counts, get_first_col_starts};
use crate::bwt::fm::calculate_fm_index;
use std::collections::HashMap;
use std::error::Error;

pub(crate) fn bw_matching(
    first_col_starts: &[usize],
    fm_index: &[Vec<usize>],
    counts: &[usize],
    char_map: &HashMap<u8, usize>,
    pattern: &str,
) -> Result<Option<(usize, usize)>, Box<dyn Error>> {
    let pattern_bytes = pattern.as_bytes();
    let pattern_len = pattern_bytes.len();

    let mut top = 0;
    let mut bottom = fm_index.len() - 1;

    // Helper function to update the range
    fn update_range(
        symbol: usize,
        top: usize,
        bottom: usize,
        first_col_starts: &[usize],
        fm_index: &[Vec<usize>],
        is_last_iter: bool,
    ) -> (usize, usize) {
        let new_top = if is_last_iter {
            first_col_starts[symbol]
        } else {
            first_col_starts[symbol] + fm_index[top - 1][symbol]
        };
        let new_bottom = first_col_starts[symbol] + fm_index[bottom][symbol] - 1;
        (new_top, new_bottom)
    }

    // Match pattern from end to beginning
    for (i, &byte) in pattern_bytes.iter().enumerate().rev() {
        let symbol = *char_map
            .get(&byte)
            .ok_or(format!("Symbol {} not found in char_map", byte.to_string()))?;

        if counts[symbol] == 0 {
            return Ok(None);
        }

        let is_last_iter = i == pattern_len - 1;
        (top, bottom) = update_range(
            symbol,
            top,
            bottom,
            first_col_starts,
            fm_index,
            is_last_iter,
        );

        if top > bottom {
            return Ok(None);
        }
    }
    Ok(Some((top, bottom)))
}

pub fn bw_match_positions(
    bwt: &str,
    suffixes: &[usize],
    patterns: &[&str],
    char_map: &HashMap<u8, usize>,
    fw_step: usize,
) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
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
        .map(|&pattern| {
            bw_match_position(
                &first_col_starts,
                suffixes,
                &fm_index,
                &counts,
                char_map,
                pattern,
            )
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(results)
}

pub fn bw_match_position(
    first_col_starts: &[usize],
    suffixes: &[usize],
    fm_index: &[Vec<usize>],
    counts: &[usize],
    char_map: &HashMap<u8, usize>,
    pattern: &str,
) -> Result<Vec<usize>, Box<dyn Error>> {
    Ok(
        if let Some((top, bottom)) =
            bw_matching(first_col_starts, fm_index, counts, char_map, pattern)?
        {
            let mut starts = suffixes[top..=bottom].to_vec();
            starts.sort();
            starts
        } else {
            Vec::new()
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::bwt::bwt::burrows_wheeler_transform_sa_is;
    use crate::bwt::matching::bw_match_positions;
    use crate::utils::DNA_BW;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_bw_match_position1() -> Result<(), Box<dyn Error>> {
        let text = "AATCGGGTTCAATCGGGGT";
        let (bwt, sa) = burrows_wheeler_transform_sa_is(text, &DNA_BW)?;
        let matches = bw_match_positions(&bwt, &sa, &["ATCG", "GGGT"], &DNA_BW, 1)?;
        assert_eq!(matches, vec![vec![1, 11], vec![4, 15]]);
        Ok(())
    }

    #[test]
    fn test_bw_match_position2() -> Result<(), Box<dyn Error>> {
        let text = "ATATATATAT";
        let (bwt, sa) = burrows_wheeler_transform_sa_is(text, &DNA_BW)?;
        let matches = bw_match_positions(
            &bwt,
            &sa,
            &["GT", "AGCT", "TAA", "AAT", "AATAT"],
            &DNA_BW,
            1,
        )?;
        assert_eq!(matches, vec![vec![], vec![], vec![], vec![], vec![]]);
        Ok(())
    }

    #[test]
    fn test_bw_match_position3() -> Result<(), Box<dyn Error>> {
        let text = "bananas";
        let char_map = HashMap::from([(b'$', 0), (b'a', 1), (b'b', 2), (b'n', 3), (b's', 4)]);
        let (bwt, sa) = burrows_wheeler_transform_sa_is(text, &char_map)?;
        let matches = bw_match_positions(&bwt, &sa, &["ana", "as"], &char_map, 1)?;
        assert_eq!(matches, vec![vec![1, 3], vec![5]]);
        Ok(())
    }

    #[test]
    fn test_bw_match_position4() -> Result<(), Box<dyn Error>> {
        let text = "AAACAA";
        let (bwt, sa) = burrows_wheeler_transform_sa_is(text, &DNA_BW)?;
        let matches = bw_match_positions(&bwt, &sa, &["AA"], &DNA_BW, 1)?;
        assert_eq!(matches, vec![vec![0, 1, 4]]);
        Ok(())
    }
}
