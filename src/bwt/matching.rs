use crate::bwt::bwt::char_counts;
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
    println!("{}", pattern);
    let pattern_bytes = pattern.as_bytes();
    let p_len = pattern_bytes.len();

    let mut top = 0;
    let mut bottom = fm_index.len() - 1;

    println!("First Col Starts {:?}", first_col_starts);
    println!("Counts {:?}", counts);

    println!("{:?}", fm_index);
    // Match pattern from end to beginning
    for i in (0..p_len).rev() {
        let symbol = *char_map.get(&pattern_bytes[i]).unwrap();

        // Skip symbols not in the text
        if counts[symbol] == 0 {
            return Ok(0);
        }

        // Update range
        if i == p_len - 1 {
            top = first_col_starts[symbol];
        } else {
            top = first_col_starts[symbol] + fm_index[top - 1][symbol];
        }
        bottom = first_col_starts[symbol] + fm_index[bottom][symbol] - 1;
        println!("TB {} {}", top, bottom);;

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

fn calculate_fm_index(
    bwt_bytes: &[u8],
    char_map: &HashMap<u8, usize>,
    fw_step: usize
) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let length = bwt_bytes.len();
    let char_count = char_map.len(); // Number of distinct characters
    let mut fm_index = Vec::new();

    let mut fw_row = vec![0; char_count];
    for (i, &byte) in bwt_bytes.iter().enumerate() {
        if let Some(&idx) = char_map.get(&byte) {
            fw_row[idx] += 1;
        }

        if i % fw_step == 0 {
            fm_index.push(fw_row.clone());
        }
    }

    Ok(fm_index)
}

pub fn bw_matching_fasta(
    bwt: &str,
    patterns: &[Fasta],
    char_map: &HashMap<u8, usize>,
    fw_step: usize
) -> Result<Vec<usize>, Box<dyn Error>> {
    let pattern_strings = patterns.iter().map(|f| f.text.as_str()).collect::<Vec<_>>();
    bw_matching(bwt, &pattern_strings, char_map, fw_step)
}

// More optimized implementation using arrays instead of HashMaps for better performance
pub fn bw_matching(
    bwt: &str,
    patterns: &[&str],
    char_map: &HashMap<u8, usize>,
    fw_step: usize
) -> Result<Vec<usize>, Box<dyn Error>> {
    let bwt_bytes = bwt.as_bytes();

    // Count character occurrences
    let counts = char_counts(bwt_bytes, char_map)?;

    // Calculate starting positions in first column
    let first_col_starts = get_first_col_starts(&counts)?;

    // Build the occurrence array more efficiently
    let fm_index = calculate_fm_index(bwt_bytes, char_map, fw_step)?;
    println!("{:?}", fm_index);

    // Match each pattern
    let results = patterns
        .iter()
        .map(|&pattern| bwt_match_count(&first_col_starts, &fm_index, &counts, char_map, pattern))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::bwt::matching::bw_matching;
    use crate::utils::DNA_BW;
    use std::error::Error;
    use crate::bwt::bwt::burrows_wheeler_transform_sa_is;

    #[test]
    fn test_bw_matching1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            bw_matching(
                "TCCTCTATGAGATCCTATTCTATGAAACCTTCA$GACCAAAATTCTCCGGC",
                &vec!["CCT", "CAC", "GAG", "CAG", "ATC"],
                &DNA_BW,
            1)?,
            vec![2, 1, 1, 0, 1]
        );
        Ok(())
    }

    #[test]
    fn test_bw_matching2() -> Result<(), Box<dyn Error>> {
        let text = "AATCGGGTTCAATCGGGGT";
        let bwt = burrows_wheeler_transform_sa_is(text, &DNA_BW)?;
        println!("{}", bwt);
        assert_eq!(
            bw_matching(
                &bwt,
                &vec!["ATCG", "GGGT"],
                &DNA_BW, 1)?,
            vec![2, 2]
        );
        Ok(())
    }

    #[test]
    fn test_bw_matching3() -> Result<(), Box<dyn Error>> {
        let text = "ATATATATAT";
        let bwt = burrows_wheeler_transform_sa_is(text, &DNA_BW)?;
        println!("{}", bwt);
        assert_eq!(
            bw_matching(
                &bwt,
                &vec!["GT", "AGCT", "TAA", "AAT", "AATAT"],
                &DNA_BW, 1)?,
            vec![0; 5]
        );
        Ok(())
    }
}
