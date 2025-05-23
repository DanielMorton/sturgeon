use crate::bwt::counts::char_counts;
use crate::bwt::suffix_array::suffix_array_bytes;
use crate::bwt::suffix_array_induced_sorting;
use crate::utils::Fasta;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

const DOLLAR_SIGN: u8 = b'$';

pub fn burrows_wheeler_transform(text: &str) -> Result<String, Box<dyn Error>> {
    let text_bytes = text.as_bytes();
    let suffixes = suffix_array_bytes(text_bytes)?;
    let n = text.len();

    // Construct BWT
    let bwt = suffixes
        .iter()
        .map(|&s| text_bytes[(s + n - 1) % n])
        .collect::<Vec<_>>();

    Ok(String::from_utf8(bwt)?)
}

pub fn fasta_burrows_wheeler_transform(fasta: &Fasta) -> Result<String, Box<dyn Error>> {
    if fasta.text.ends_with('$') {
        burrows_wheeler_transform(&fasta.text)
    } else {
        let text = format!("{}$", fasta.text);
        burrows_wheeler_transform(&text)
    }
}

pub fn burrows_wheeler_transform_sa_is(
    text: &str,
    char_map: &HashMap<u8, usize>,
) -> Result<(String, Vec<usize>), Box<dyn Error>> {
    let text_bytes = text.as_bytes();
    let suffixes = suffix_array_induced_sorting(text_bytes, char_map)?;
    let n = text.len();

    let get_bwt_character = |suffix_index: usize| {
        if suffix_index == 0 {
            DOLLAR_SIGN
        } else {
            text_bytes[(suffix_index + n - 1) % n]
        }
    };

    let bwt = suffixes
        .iter()
        .map(|&s| get_bwt_character(s))
        .collect::<Vec<_>>();
    Ok((String::from_utf8(bwt)?, suffixes))
}

pub fn fasta_burrows_wheeler_transform_sa_is(
    fasta: &Fasta,
    char_map: &HashMap<u8, usize>,
) -> Result<(String, Vec<usize>), Box<dyn Error>> {
    burrows_wheeler_transform_sa_is(&fasta.text, char_map)
}

fn calculate_start_positions(counts: &[usize]) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut cumulative_sum = 0;
    Ok(counts
        .iter()
        .map(|&count| {
            let prev_sum = cumulative_sum;
            cumulative_sum += count;
            prev_sum
        })
        .collect())
}

// Helper function for calculating LF-mapping occurrences
fn calculate_char_occurrences(
    bwt_bytes: &[u8],
    char_map: &HashMap<u8, usize>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut accumulated_occurrences = vec![0; char_map.len()];
    Ok(bwt_bytes
        .iter()
        .map(|c| {
            let pos = *char_map.get(c).unwrap();
            let occ = accumulated_occurrences[pos];
            accumulated_occurrences[pos] += 1;
            occ
        })
        .collect::<Vec<_>>())
}

pub fn inverse_burrows_wheeler_transform(
    bwt: &str,
    char_map: &HashMap<u8, usize>,
) -> Result<String, Box<dyn Error>> {
    let n = bwt.len();
    let bwt_bytes = bwt.as_bytes();
    if n <= 1 {
        return Ok(bwt.to_owned());
    }

    // Count character occurrences
    let counts = char_counts(bwt_bytes, char_map)?;

    // Calculate starting positions for each character in first column
    let start_pos = calculate_start_positions(&counts)?;

    // Create smaller count array for LF-mapping

    let char_occ = calculate_char_occurrences(bwt_bytes, char_map)?;

    // Find the dollar sign position
    let mut idx = bwt_bytes
        .iter()
        .position(|&b| b == DOLLAR_SIGN)
        .ok_or("No $ char. Not a valid BWT string.")?;

    // Reconstruct the original string
    let mut result = Vec::with_capacity(n - 1);

    for _ in 0..n - 1 {
        // Move to the next character in the original string
        let c = *char_map.get(&bwt_bytes[idx]).unwrap();
        idx = start_pos[c] + char_occ[idx];

        // Don't add the '$' to the result
        result.push(bwt_bytes[idx]);
    }

    result.reverse();
    result.push(DOLLAR_SIGN);
    Ok(String::from_utf8(result)?)
}

#[cfg(test)]
mod tests {
    use crate::bwt::bwt::{burrows_wheeler_transform, inverse_burrows_wheeler_transform};
    use crate::utils::DNA_BW;
    use std::error::Error;

    #[test]
    fn test_burrows_wheeler_transform1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            burrows_wheeler_transform("GCGTGCCTGGTCA$")?,
            "ACTGGCT$TGCGGC"
        );
        Ok(())
    }

    #[test]
    fn test_burrows_wheeler_transform2() -> Result<(), Box<dyn Error>> {
        assert_eq!(burrows_wheeler_transform("AATCAATC$")?, "CC$AATTAA");
        Ok(())
    }

    #[test]
    fn test_burrows_wheeler_transform3() -> Result<(), Box<dyn Error>> {
        assert_eq!(burrows_wheeler_transform("AAAAAAAAAA$")?, "AAAAAAAAAA$");
        Ok(())
    }

    #[test]
    fn test_burrows_wheeler_transform4() -> Result<(), Box<dyn Error>> {
        assert_eq!(burrows_wheeler_transform("GAGCAT$")?, "TGCG$AA");
        Ok(())
    }

    #[test]
    fn test_inverse_burrows_wheeler_transform1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            inverse_burrows_wheeler_transform("TTCCTAACG$A", &DNA_BW)?,
            "TACATCACGT$"
        );
        Ok(())
    }

    #[test]
    fn test_inverse_burrows_wheeler_transform2() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            inverse_burrows_wheeler_transform("T$ACG", &DNA_BW)?,
            "ACGT$"
        );
        Ok(())
    }

    #[test]
    fn test_inverse_burrows_wheeler_transform3() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            inverse_burrows_wheeler_transform("AAAAAAAAAA$", &DNA_BW)?,
            "AAAAAAAAAA$"
        );
        Ok(())
    }

    #[test]
    fn test_inverse_burrows_wheeler_transform4() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            inverse_burrows_wheeler_transform("TGCG$AA", &DNA_BW)?,
            "GAGCAT$"
        );
        Ok(())
    }
}
