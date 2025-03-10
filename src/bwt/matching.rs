use crate::bwt::bwt::{char_counts, COUNTS};
use std::error::Error;

fn bwt_match_count(
    first_col_starts: &[usize; COUNTS],
    occ: &[[usize; COUNTS]],
    counts: &[usize; COUNTS],
    pattern: &str,
) -> Result<usize, Box<dyn Error>> {
    let pattern_bytes = pattern.as_bytes();
    let p_len = pattern_bytes.len();

    let mut top = 0;
    let mut bottom = occ.len() - 2;

    // Match pattern from end to beginning
    for i in (0..p_len).rev() {
        let symbol = pattern_bytes[i] as usize;

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

// More optimized implementation using arrays instead of HashMaps for better performance
fn bw_matching(bwt: &str, patterns: &[&str]) -> Result<Vec<usize>, Box<dyn Error>> {
    let bwt_bytes = bwt.as_bytes();
    let n = bwt_bytes.len();

    // Count character occurrences
    let counts = char_counts(bwt_bytes)?;

    // Calculate starting positions in first column
    let mut first_col_starts = [0; COUNTS];
    let mut total = 0;
    for i in 0..COUNTS {
        if counts[i] > 0 {
            first_col_starts[i] = total;
            total += counts[i];
        }
    }
    println!(
        "{} {} {} {}",
        first_col_starts[b'A' as usize],
        first_col_starts[b'C' as usize],
        first_col_starts[b'G' as usize],
        first_col_starts[b'T' as usize]
    );

    // Build the occurrence array more efficiently
    let mut occ = vec![[0; COUNTS]; n + 1];
    for i in 0..n {
        if i > 0 {
            for j in 0..COUNTS {
                occ[i + 1][j] = occ[i][j];
            }
        }
        occ[i + 1][bwt_bytes[i] as usize] += 1;
    }

    // Match each pattern
    let results = patterns
        .iter()
        .map(|&pattern| bwt_match_count(&first_col_starts, &occ, &counts, pattern))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::bwt::matching::bw_matching;
    use std::error::Error;

    #[test]
    fn test_bw_matching1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            bw_matching(
                "TCCTCTATGAGATCCTATTCTATGAAACCTTCA$GACCAAAATTCTCCGGC",
                &vec!["CCT", "CAC", "GAG", "CAG", "ATC"]
            )?,
            vec![2, 1, 1, 0, 1]
        );
        Ok(())
    }
}
