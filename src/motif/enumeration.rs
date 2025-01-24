use crate::motif::neighbors::neighbors;
use crate::utils::DNA;
use std::error::Error;
use crate::utils::hamming::hamming_distance;

pub fn motif_enumeration(
    dna: &[String],
    kmer_length: usize,
    max_diff: usize,
) -> Result<Vec<String>, Box<dyn Error>> {
    // Early return for invalid inputs
    if dna.is_empty() || kmer_length == 0 || max_diff > kmer_length {
        return Ok(Vec::new());
    }

    // Get all possible k-mer neighbors from the first sequence
    let mut neighbor_set = std::collections::HashSet::new();
    let first_seq = &dna[0];
    for i in 0..=first_seq.len() - kmer_length {
        let kmer = &first_seq[i..i + kmer_length];
        neighbor_set.extend(neighbors(kmer, max_diff, &DNA)?);
    }

    // Find patterns that appear in all sequences
    let patterns: Vec<String> = neighbor_set
        .into_iter()
        .filter(|pattern| {
            dna[1..].iter().all(|sequence| {
                (0..=sequence.len() - kmer_length)
                    .any(|i| hamming_distance(pattern, &sequence[i..i + kmer_length]).unwrap() <= max_diff)
            })
        })
        .collect();

    Ok(patterns)
}

mod tests {
    use crate::motif::enumeration::motif_enumeration;
    use std::error::Error;

    #[test]
    fn test_motif_enumeration1() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("ATTTGGC"),
            format!("TGCCTTA"),
            format!("CGGTATC"),
            format!("GAAAATT"),
        ];
        let (k, d) = (3, 1);
        let mut motifs = motif_enumeration(&dna, k, d)?;
        motifs.sort();
        assert_eq!(motifs, vec!["ATA", "ATT", "GTT", "TTT"]);
        Ok(())
    }

    #[test]
    fn test_motif_enumeration2() -> Result<(), Box<dyn Error>> {
        let dna = vec![format!("ACGT"), format!("ACGT"), format!("ACGT")];
        let (k, d) = (3, 0);
        let mut motifs = motif_enumeration(&dna, k, d)?;
        motifs.sort();
        assert_eq!(motifs, vec!["ACG", "CGT"]);
        Ok(())
    }

    #[test]
    fn test_motif_enumeration3() -> Result<(), Box<dyn Error>> {
        let dna = vec![format!("AAAAA"), format!("AAAAA"), format!("AAAAA")];
        let (k, d) = (3, 1);
        let mut motifs = motif_enumeration(&dna, k, d)?;
        motifs.sort();
        let mut ans = vec![
            "AAG", "AGA", "ATA", "AAA", "ACA", "AAC", "TAA", "GAA", "CAA", "AAT",
        ];
        ans.sort();
        assert_eq!(motifs, ans);
        Ok(())
    }

    #[test]
    fn test_motif_enumeration4() -> Result<(), Box<dyn Error>> {
        let dna = vec![format!("AAAAA"), format!("AAAAA"), format!("AAAAA")];
        let (k, d) = (3, 3);
        let mut motifs = motif_enumeration(&dna, k, d)?;
        motifs.sort();
        let mut ans = vec![
            "GCA", "TTA", "GAT", "GCC", "CTT", "AAA", "TTG", "ATT", "TCC", "CAA", "TTT", "CCT",
            "CAC", "ACT", "GCT", "GGG", "GAC", "TAC", "CGC", "CAT", "AGG", "ACA", "TAT", "CTA",
            "CGT", "CTC", "CGA", "CAG", "GCG", "GTT", "GTC", "CCG", "GGC", "AGC", "TCT", "ATC",
            "GTG", "CGG", "TGA", "TGT", "GAG", "AGA", "CCC", "ATG", "TAG", "CTG", "AGT", "ATA",
            "CCA", "ACG", "GAA", "GTA", "GGT", "TTC", "GGA", "TGC", "AAT", "ACC", "AAG", "TGG",
            "TCG", "TCA", "TAA", "AAC",
        ];
        ans.sort();
        assert_eq!(motifs, ans);
        Ok(())
    }

    #[test]
    fn test_motif_enumeration5() -> Result<(), Box<dyn Error>> {
        let dna = vec![format!("AAAAA"), format!("AAAAA"), format!("AACAA")];
        let (k, d) = (3, 0);
        let mut motifs = motif_enumeration(&dna, k, d)?;
        motifs.sort();
        let ans: Vec<String> = vec![];
        assert_eq!(motifs, ans);
        Ok(())
    }

    #[test]
    fn test_motif_enumeration6() -> Result<(), Box<dyn Error>> {
        let dna = vec![format!("AACAA"), format!("AAAAA"), format!("AAAAA")];
        let (k, d) = (3, 0);
        let mut motifs = motif_enumeration(&dna, k, d)?;
        motifs.sort();
        let ans: Vec<String> = vec![];
        assert_eq!(motifs, ans);
        Ok(())
    }
}
