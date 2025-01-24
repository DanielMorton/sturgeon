use crate::motif::neighbors::neighbors;
use crate::utils::DNA;
use std::error::Error;
use crate::utils::hamming::min_hamming_distance;

pub fn median_string(dna: &[String], k: usize) -> Result<String, Box<dyn Error>> {
    // Initialize with k consecutive 'A's
    let initial_pattern = "A".repeat(k);
    let mut kmers = neighbors(&initial_pattern, k, &DNA)?
        .into_iter()
        .collect::<Vec<_>>();
    kmers.sort();

    let mut min_distance = k * dna.len();
    let mut median = kmers[0].clone();

    for kmer in &kmers {
        let distances: Result<Vec<usize>, _> = dna
            .iter()
            .map(|sequence| min_hamming_distance(kmer, sequence))
            .collect();
        let total_distance = distances?.iter().sum();

        if total_distance < min_distance {
            median = kmer.clone();
            min_distance = total_distance;
        }
    }
    Ok(median)
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
