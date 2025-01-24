use crate::motif::frequency::{freq_map, frequent_word_set};
use std::error::Error;

fn find_clumps(
    genome: &str,
    window: usize,
    kmer_length: usize,
    min_freq: usize,
) -> Result<Vec<String>, Box<dyn Error>> {
    if genome.len() < window || window < kmer_length {
        return Ok(Vec::new());
    }

    // Initialize frequency map and patterns set for first window
    let mut word_freq = freq_map(&genome[..window], kmer_length);
    let mut patterns = frequent_word_set(&genome[..window], &word_freq, kmer_length, min_freq);
    // Slide window over genome
    for i in 0..(genome.len() - window) {
        // Remove leftmost k-mer frequency
        let left_kmer = &genome[i..i + kmer_length];
        if let Some(&count) = word_freq.get(left_kmer) {
            if count == 1 {
                word_freq.remove(left_kmer);
            } else {
                word_freq.insert(left_kmer.to_owned(), count - 1);
            }
        }

        // Add rightmost k-mer frequency
        let right_kmer = genome[(i + window - kmer_length + 1)..(i + window + 1)].to_owned();
        let freq = word_freq.entry(right_kmer.clone()).or_insert(0);
        *freq += 1;

        // Add to patterns if frequency threshold met
        if *freq >= min_freq {
            patterns.insert(right_kmer);
        }
    }

    Ok(patterns.into_iter().collect())
}

mod tests {
    use crate::motif::clumps::find_clumps;
    use std::error::Error;
    use std::fs;

    #[test]
    fn test_find_clumps0() -> Result<(), Box<dyn Error>> {
        let genome = "CGGACTCGACAGATGTGAAGAACGACAATGTGAAGACTCGACACGACAGAGTGAAGAGAAGAGGAAACATTGTAA";
        let mut ans = find_clumps(genome, 50, 5, 4)?;
        ans.sort();
        assert_eq!(ans, vec!["CGACA", "GAAGA"]);
        Ok(())
    }

    #[test]
    fn test_find_clumps1() -> Result<(), Box<dyn Error>> {
        let genome = "CGGACTCGACAGATGTGAAGAAATGTGAAGACTGAGTGAAGAGAAGAGGAAACACGACACGACATTGCGACATAATGTACGAATGTAATGTGCCTATGGC";
        let mut ans = find_clumps(genome, 74, 5, 4)?;
        ans.sort();
        assert_eq!(ans, vec!["AATGT", "CGACA", "GAAGA"]);
        Ok(())
    }

    #[test]
    fn test_find_clumps2() -> Result<(), Box<dyn Error>> {
        let genome = "AAAACGTCGAAAAA";
        let ans = find_clumps(genome, 4, 2, 2)?;
        assert_eq!(ans, vec!["AA"]);
        Ok(())
    }

    #[test]
    fn test_find_clumps3() -> Result<(), Box<dyn Error>> {
        let genome = "ACGTACGT";
        let mut ans = find_clumps(genome, 5, 1, 2)?;
        ans.sort();
        assert_eq!(ans, vec!["A", "C", "G", "T"]);
        Ok(())
    }

    #[test]
    fn test_find_clumps4() -> Result<(), Box<dyn Error>> {
        let genome = "CCACGCGGTGTACGCTGCAAAAAGCCTTGCTGAATCAAATAAGGTTCCAGCACATCCTCAATGGTTTCACGTTCTTCGCCAATGGCTGCCGCCAGGTTATCCAGACCTACAGGTCCACCAAAGAACTTATCGATTACCGCCAGCAACAATTTGCGGTCCATATAATCGAAACCTTCAGCATCGACATTCAACATATCCAGCG";
        let mut ans = find_clumps(genome, 25, 3, 3)?;
        ans.sort();
        assert_eq!(ans, vec!["AAA", "CAG", "CAT", "CCA", "GCC", "TTC"]);
        Ok(())
    }

    #[test]
    fn test_find_clumps5() -> Result<(), Box<dyn Error>> {
        let genome = match fs::read_to_string("input_clumps.txt") {
            Ok(g) => g,
            Err(e) => panic!("{}", e),
        };
        let mut ans = find_clumps(&genome, 566, 11, 19)?;
        ans.sort();
        assert_eq!(ans, vec!["AAACCAGGTGG"]);
        Ok(())
    }
}
