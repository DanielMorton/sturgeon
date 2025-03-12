use crate::bwt::bwt::char_counts;
use std::collections::HashMap;
use std::error::Error;

fn bwt_match_count(
    first_col_starts: &[usize],
    occ: &[Vec<usize>],
    counts: &[usize],
    char_map: &HashMap<u8, usize>,
    pattern: &str,
) -> Result<usize, Box<dyn Error>> {
    let pattern_bytes = pattern.as_bytes();
    let p_len = pattern_bytes.len();

    let mut top = 0;
    let mut bottom = occ.len() - 2;

    // Match pattern from end to beginning
    for i in (0..p_len).rev() {
        let symbol = *char_map.get(&pattern_bytes[i]).unwrap();

        // Skip symbols not in the text
        if counts[symbol] == 0 {
            return Ok(0);
        }

        // Update range
        top = first_col_starts[symbol] + occ[top][symbol];
        bottom = first_col_starts[symbol] + occ[bottom + 1][symbol] - 1;

        if top > bottom {
            return Ok(0);
        }
    }

    Ok(bottom - top + 1)
}

fn get_first_col_starts(counts: &[usize]) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut total = 0;
    Ok(counts
        .iter()
        .map(|&c| {
            let fcs = total;
            total += c;
            fcs
        })
        .collect::<Vec<_>>())
}

fn calculate_occurrences(
    bwt_bytes: &[u8],
    char_map: &HashMap<u8, usize>,
) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let length = bwt_bytes.len();
    let char_count = char_map.len(); // Number of distinct characters
    let mut occurrences = vec![vec![0; char_count]; length + 1];

    for (i, &byte) in bwt_bytes.iter().enumerate() {
        if i > 0 {
            let occ = occurrences[i].clone();
            occurrences[i + 1].copy_from_slice(&occ);
        }
        // Update the count for the current byte
        if let Some(&idx) = char_map.get(&byte) {
            occurrences[i + 1][idx] += 1;
        }
    }

    Ok(occurrences)
}

// More optimized implementation using arrays instead of HashMaps for better performance
fn bw_matching(
    bwt: &str,
    patterns: &[&str],
    char_map: &HashMap<u8, usize>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let bwt_bytes = bwt.as_bytes();

    // Count character occurrences
    let counts = char_counts(bwt_bytes, char_map)?;

    // Calculate starting positions in first column
    let first_col_starts = get_first_col_starts(&counts)?;

    // Build the occurrence array more efficiently
    let occ = calculate_occurrences(bwt_bytes, char_map)?;

    // Match each pattern
    let results = patterns
        .iter()
        .map(|&pattern| bwt_match_count(&first_col_starts, &occ, &counts, char_map, pattern))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::bwt::matching::bw_matching;
    use crate::utils::DNA_BW;
    use std::error::Error;

    #[test]
    fn test_bw_matching1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            bw_matching(
                "TCCTCTATGAGATCCTATTCTATGAAACCTTCA$GACCAAAATTCTCCGGC",
                &vec!["CCT", "CAC", "GAG", "CAG", "ATC"],
                &DNA_BW
            )?,
            vec![2, 1, 1, 0, 1]
        );
        Ok(())
    }
}
