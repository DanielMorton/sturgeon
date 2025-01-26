use crate::motif::neighbors::all_kmers;
use crate::utils::hamming::min_hamming_distance;
use std::error::Error;

pub fn median_string(dna: &[String], kmer_length: usize) -> Result<String, Box<dyn Error>> {
    // Initialize with k consecutive 'A's
    let mut kmers = all_kmers(kmer_length)?;
    kmers.sort();

    let mut min_distance = kmer_length * dna.len();
    let mut median = kmers[0].clone();

    for kmer in &kmers {
        let total_distance = score_kmer(&dna, kmer)?;

        if total_distance < min_distance {
            median = kmer.clone();
            min_distance = total_distance;
        }
    }
    Ok(median)
}

pub fn score_kmer(dna: &[String], kmer: &str) -> Result<usize, Box<dyn Error>> {
    let distances = dna
        .iter()
        .map(|sequence| min_hamming_distance(kmer, sequence))
        .collect::<Result<Vec<usize>, _>>()?;
    Ok(distances.iter().sum())
}

mod tests {
    use crate::motif::median::median_string;
    use std::error::Error;

    #[test]
    fn test_median1() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("AAATTGACGCAT"),
            format!("GACGACCACGTT"),
            format!("CGTCAGCGCCTG"),
            format!("GCTGAGCACCGG"),
            format!("AGTACGGGACAG"),
        ];
        let median = median_string(&dna, 3)?;
        assert_eq!(median, "ACG");
        Ok(())
    }

    #[test]
    fn test_median2() -> Result<(), Box<dyn Error>> {
        let dna = vec![format!("ACGT"), format!("ACGT"), format!("ACGT")];
        let median = median_string(&dna, 3)?;
        assert_eq!(median, "ACG");
        Ok(())
    }

    #[test]
    fn test_median3() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("ATA"),
            format!("ACA"),
            format!("AGA"),
            format!("AAT"),
            format!("AAC"),
        ];
        let median = median_string(&dna, 3)?;
        assert_eq!(median, "AAA");
        Ok(())
    }

    #[test]
    fn test_median4() -> Result<(), Box<dyn Error>> {
        let dna = vec![format!("AAG"), format!("AAT")];
        let median = median_string(&dna, 3)?;
        assert_eq!(median, "AAG");
        Ok(())
    }
}
