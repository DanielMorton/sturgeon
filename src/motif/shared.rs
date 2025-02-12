use crate::utils::dna_complement;
use std::collections::HashMap;
use std::error::Error;

fn shared_kmers(k: usize, s: &str, t: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut kmer_positions = HashMap::new();
    let mut result = Vec::new();

    // Store positions of k-mers from first string
    for i in 0..=s.len() - k {
        let kmer = &s[i..i + k];
        kmer_positions
            .entry(kmer.to_owned())
            .or_insert_with(Vec::new)
            .push(i);
    }

    // Find matching k-mers in second string
    for j in 0..=t.len() - k {
        let kmer = &t[j..j + k];
        if let Some(positions) = kmer_positions.get(kmer) {
            for &pos_i in positions {
                result.push((pos_i, j));
            }
        }
        let complement = dna_complement(&kmer)?;
        if let Some(positions) = kmer_positions.get(&complement) {
            for &pos_i in positions {
                result.push((pos_i, j));
            }
        }
    }

    result.sort();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::motif::shared::shared_kmers;
    use std::error::Error;

    #[test]
    fn test_shared_kmers1() -> Result<(), Box<dyn Error>> {
        let (s, t) = ("AAACTCATC", "TTTCAAATC");
        let ans = vec![(0, 0), (0, 4), (4, 2), (6, 6)];
        assert_eq!(shared_kmers(3, s, t)?, ans);
        Ok(())
    }

    #[test]
    fn test_shared_kmers2() -> Result<(), Box<dyn Error>> {
        let (s, t) = ("AATCGATG", "AAGGGGGGTG");
        let ans = vec![(0, 0), (6, 8)];
        assert_eq!(shared_kmers(2, s, t)?, ans);
        Ok(())
    }

    #[test]
    fn test_shared_kmers3() -> Result<(), Box<dyn Error>> {
        let (s, t) = ("AAAAATTC", "GTACGAGGCG");
        let ans = vec![];
        assert_eq!(shared_kmers(3, s, t)?, ans);
        Ok(())
    }

    #[test]
    fn test_shared_kmers4() -> Result<(), Box<dyn Error>> {
        let (s, t) = ("AAAAA", "GAAATCG");
        let ans = vec![(0, 1), (1, 1), (2, 1)];
        assert_eq!(shared_kmers(3, s, t)?, ans);
        Ok(())
    }

    #[test]
    fn test_shared_kmers5() -> Result<(), Box<dyn Error>> {
        let (s, t) = ("GGGTG", "GGGTG");
        let ans = vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 2), (3, 3)];
        assert_eq!(shared_kmers(2, s, t)?, ans);
        Ok(())
    }

    #[test]
    fn test_shared_kmers6() -> Result<(), Box<dyn Error>> {
        let (s, t) = ("ATCGAAGG", "CCTTCGAT");
        let mut ans = vec![
            (5, 0),
            (4, 1),
            (3, 2),
            (1, 3),
            (2, 3),
            (1, 4),
            (2, 4),
            (0, 5),
        ];
        ans.sort();
        assert_eq!(shared_kmers(3, s, t)?, ans);
        Ok(())
    }
}
