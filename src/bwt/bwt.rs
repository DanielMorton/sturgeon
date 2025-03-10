use crate::bwt::suffix_array::suffix_array_bytes;
use crate::utils::Fasta;
use std::error::Error;

pub const COUNTS: usize = 256;

pub(crate) fn char_counts(bwt_bytes: &[u8]) -> Result<[usize; COUNTS], Box<dyn Error>> {
    let mut char_counts = [0; COUNTS];
    for &b in bwt_bytes {
        char_counts[b as usize] += 1;
    }
    Ok(char_counts)
}

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

pub fn inverse_burrows_wheeler_transform(bwt: &str) -> Result<String, Box<dyn Error>> {
    let n = bwt.len();
    let bwt_bytes = bwt.as_bytes();
    if n <= 1 {
        return Ok(bwt.to_owned());
    }

    // Count character occurrences
    let counts = char_counts(bwt_bytes)?;

    // Calculate starting positions for each character in first column
    let mut cum_sum = 0;
    let start_pos = counts
        .iter()
        .map(|c| {
            let prev_sum = cum_sum;
            cum_sum += c;
            prev_sum
        })
        .collect::<Vec<_>>();

    // Create smaller count array for LF-mapping

    let mut char_counts_so_far = [0; COUNTS];
    let char_occ = bwt_bytes
        .iter()
        .map(|&c| {
            let occ = char_counts_so_far[c as usize];
            char_counts_so_far[c as usize] += 1;
            occ
        })
        .collect::<Vec<_>>();

    // Find the dollar sign position
    let mut idx = bwt_bytes
        .iter()
        .position(|&b| b == b'$')
        .ok_or("No $ char. Not a valid BWT string.")?;

    // Reconstruct the original string
    let mut result = Vec::with_capacity(n - 1);

    for _ in 0..n - 1 {
        // Move to the next character in the original string
        let c = bwt_bytes[idx] as usize;
        idx = start_pos[c] + char_occ[idx] as usize;

        // Don't add the '$' to the result
        result.push(bwt_bytes[idx]);
    }

    result.reverse();
    result.push(b'$');
    Ok(String::from_utf8(result)?)
}

#[cfg(test)]
mod tests {
    use crate::bwt::bwt::{burrows_wheeler_transform, inverse_burrows_wheeler_transform};
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
            inverse_burrows_wheeler_transform("TTCCTAACG$A")?,
            "TACATCACGT$"
        );
        Ok(())
    }

    #[test]
    fn test_inverse_burrows_wheeler_transform2() -> Result<(), Box<dyn Error>> {
        assert_eq!(inverse_burrows_wheeler_transform("T$ACG")?, "ACGT$");
        Ok(())
    }

    #[test]
    fn test_inverse_burrows_wheeler_transform3() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            inverse_burrows_wheeler_transform("AAAAAAAAAA$")?,
            "AAAAAAAAAA$"
        );
        Ok(())
    }

    #[test]
    fn test_inverse_burrows_wheeler_transform4() -> Result<(), Box<dyn Error>> {
        assert_eq!(inverse_burrows_wheeler_transform("TGCG$AA")?, "GAGCAT$");
        Ok(())
    }
}
