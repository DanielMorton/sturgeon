use crate::utils::dna_complement;
use std::collections::HashMap;
use std::error::Error;

pub fn shared_kmers(k: usize, s: &str, t: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut kmer_positions = HashMap::new();
    let mut kmer_locs = Vec::new();

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
            for &i in positions {
                kmer_locs.push((i, j));
            }
        }
        let complement = dna_complement(&kmer)?;
        if let Some(positions) = kmer_positions.get(&complement) {
            for &i in positions {
                kmer_locs.push((i, j));
            }
        }
    }

    kmer_locs.sort();
    kmer_locs.dedup();

    Ok(kmer_locs)
}

pub fn synteny_to_chromosome(synteny: &[(usize, usize)], k: usize, s: &str, t: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let chromosome1 = synteny.iter().enumerate().map(|(i, _)| i as i32 + 1).collect::<Vec<_>>();
    let mut chro2 = synteny.iter().enumerate()
        .map(|(i, &(s1, s2))| {
            if s[s1..s1+k] == t[s2 .. s2+k] {
                (s2, i as i32 + 1)
            } else {
                (s2, -(i as i32 + 1))
            }
        }).collect::<Vec<_>>();
    chro2.sort();
    let chromosome2 = chro2.into_iter().map(|(_, i)| i).collect::<Vec<_>>();
    Ok((chromosome1, chromosome2))
}

#[cfg(test)]
mod tests {
    use crate::motif::synteny::shared_kmers;
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
