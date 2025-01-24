use crate::motif::search::profile::{motif_to_profile, score_motifs};
use crate::utils::{InvalidNucleotidePositionError, DNA_INDEX};
use std::error::Error;

pub fn profile_most_probable_kmer(
    text: &str,
    k: usize,
    profile: &[Vec<f64>; 4],
) -> Result<String, Box<dyn Error>> {
    let mut kmer = text[0..k].to_string();
    let mut max_prob = 0.0;
    let text_chars: Vec<char> = text.chars().collect();

    for i in 0..=text.len() - k {
        let text_slice = &text_chars[i..i + k];
        let prob = text_slice
            .iter()
            .enumerate()
            .map(|(j, &c)| {
                let idx = *DNA_INDEX
                    .get(&c)
                    .ok_or_else(|| InvalidNucleotidePositionError::new(c, j))?;
                Ok(profile[idx][j])
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?
            .iter()
            .product();

        if prob > max_prob {
            kmer = text_slice.iter().collect();
            max_prob = prob;
        }
    }

    Ok(kmer)
}

fn greedy_motif_search_laplace(
    dna: &[String],
    k: usize,
    laplace: Option<f64>,
) -> Result<Vec<String>, Box<dyn Error>> {
    // Initialize best motifs with first k-mers
    let mut best_motifs: Vec<String> = dna.iter().map(|d| d[0..k].to_string()).collect();

    let mut best_profile = motif_to_profile(&best_motifs, laplace)?;
    let mut best_score = score_motifs(&best_motifs, &best_profile)?;

    // Iterate through possible first motifs
    for i in 0..=dna[0].len() - k {
        let mut motifs = vec![dna[0][i..i + k].to_string()];
        let mut profile = motif_to_profile(&motifs, laplace)?;

        // Find most probable motifs for other DNA strands
        for j in 1..dna.len() {
            let new_motif = profile_most_probable_kmer(&dna[j], k, &profile)?;
            motifs.push(new_motif);
            profile = motif_to_profile(&motifs, laplace)?;
        }

        let score = score_motifs(&motifs, &profile)?;
        if score < best_score {
            best_motifs = motifs;
            best_profile = profile;
            best_score = score;
        }
    }
    Ok(best_motifs)
}

fn greedy_motif_search(dna: &[String], k: usize) -> Result<Vec<String>, Box<dyn Error>> {
    greedy_motif_search_laplace(dna, k, None)
}

mod tests {
    use std::error::Error;
    use crate::motif::search::greedy::{greedy_motif_search, greedy_motif_search_laplace, profile_most_probable_kmer};

    #[test]
    fn test_profile_most_probable_kmer1() -> Result<(), Box<dyn Error>> {
        let text = "ACCTGTTTATTGCCTAAGTTCCGAACAAACCCAATATAGCCCGAGGGCCT";
        let k = 5;
        let profile: [Vec<f64>; 4] = [
            vec![0.2, 0.2, 0.3, 0.2, 0.3],
            vec![0.4, 0.3, 0.1, 0.5, 0.1],
            vec![0.3, 0.3, 0.5, 0.2, 0.4],
            vec![0.1, 0.2, 0.1, 0.1, 0.2],
        ];
        let kmer = profile_most_probable_kmer(text, k, &profile)?;
        assert_eq!(kmer, "CCGAG");
        Ok(())
    }

    #[test]
    fn test_profile_most_probable_kmer2() -> Result<(), Box<dyn Error>> {
        let text = "AGCAGCTTTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATCTGAACTGGTTACCTGCCGTGAGTAAAT";
        let k = 8;
        let profile: [Vec<f64>; 4] = [
            vec![0.7, 0.2, 0.1, 0.5, 0.4, 0.3, 0.2, 0.1],
            vec![0.2, 0.2, 0.5, 0.4, 0.2, 0.3, 0.1, 0.6],
            vec![0.1, 0.3, 0.2, 0.1, 0.2, 0.1, 0.4, 0.2],
            vec![0.0, 0.3, 0.2, 0.0, 0.2, 0.3, 0.3, 0.1],
        ];
        let kmer = profile_most_probable_kmer(text, k, &profile)?;
        assert_eq!(kmer, "AGCAGCTT");
        Ok(())
    }

    #[test]
    fn test_profile_most_probable_kmer3() -> Result<(), Box<dyn Error>> {
        let text =
            "TTACCATGGGACCGCTGACTGATTTCTGGCGTCAGCGTGATGCTGGTGTGGATGACATTCCGGTGCGCTTTGTAAGCAGAGTTTA";
        let k = 12;
        let profile: [Vec<f64>; 4] = [
            vec![0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.1, 0.2, 0.3, 0.4, 0.5],
            vec![0.3, 0.2, 0.1, 0.1, 0.2, 0.1, 0.1, 0.4, 0.3, 0.2, 0.2, 0.1],
            vec![0.2, 0.1, 0.4, 0.3, 0.1, 0.1, 0.1, 0.3, 0.1, 0.1, 0.2, 0.1],
            vec![0.3, 0.4, 0.1, 0.1, 0.1, 0.1, 0.0, 0.2, 0.4, 0.4, 0.2, 0.3],
        ];
        let kmer = profile_most_probable_kmer(text, k, &profile)?;
        assert_eq!(kmer, "AAGCAGAGTTTA");
        Ok(())
    }

    #[test]
    fn test_greedy_motif_search1() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("GGCGTTCAGGCA"),
            format!("AAGAATCAGTCA"),
            format!("CAAGGAGTTCGC"),
            format!("CACGTCAATCAC"),
            format!("CAATAATATTCG"),
        ];
        let motifs = greedy_motif_search(&dna, 3)?;
        assert_eq!(motifs, vec!["CAG", "CAG", "CAA", "CAA", "CAA"]);
        Ok(())
    }

    #[test]
    fn test_greedy_motif_search2() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("GCCCAA"),
            format!("GGCCTG"),
            format!("AACCTA"),
            format!("TTCCTT"),
        ];
        let motifs = greedy_motif_search(&dna, 3)?;
        assert_eq!(motifs, vec!["GCC", "GCC", "AAC", "TTC"]);
        Ok(())
    }

    #[test]
    fn test_greedy_motif_search3() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("GAGGCGCACATCATTATCGATAACGATTCGCCGCATTGCC"),
            format!("TCATCGAATCCGATAACTGACACCTGCTCTGGCACCGCTC"),
            format!("TCGGCGGTATAGCCAGAAAGCGTAGTGCCAATAATTTCCT"),
            format!("GAGTCGTGGTGAAGTGTGGGTTATGGGGAAAGGCAGACTG"),
            format!("GACGGCAACTACGGTTACAACGCAGCAACCGAAGAATATT"),
            format!("TCTGTTGTTGCTAACACCGTTAAAGGCGGCGACGGCAACT"),
            format!("AAGCGGCCAACGTAGGCGCGGCTTGGCATCTCGGTGTGTG"),
            format!("AATTGAAAGGCGCATCTTACTCTTTTCGCTTTCAAAAAAA"),
        ];
        let motifs = greedy_motif_search(&dna, 5)?;
        assert_eq!(
            motifs,
            vec!["GAGGC", "TCATC", "TCGGC", "GAGTC", "GCAGC", "GCGGC", "GCGGC", "GCATC"]
        );
        Ok(())
    }

    #[test]
    fn test_greedy_motif_search4() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("GCAGGTTAATACCGCGGATCAGCTGAGAAACCGGAATGTGCGT"),
            format!("CCTGCATGCCCGGTTTGAGGAACATCAGCGAAGAACTGTGCGT"),
            format!("GCGCCAGTAACCCGTGCCAGTCAGGTTAATGGCAGTAACATTT"),
            format!("AACCCGTGCCAGTCAGGTTAATGGCAGTAACATTTATGCCTTC"),
            format!("ATGCCTTCCGCGCCAATTGTTCGTATCGTCGCCACTTCGAGTG"),
        ];
        let motifs = greedy_motif_search(&dna, 6)?;
        assert_eq!(
            motifs,
            vec!["GTGCGT", "GTGCGT", "GCGCCA", "GTGCCA", "GCGCCA"]
        );
        Ok(())
    }

    #[test]
    fn test_greedy_motif_search5() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("GACCTACGGTTACAACGCAGCAACCGAAGAATATTGGCAA"),
            format!("TCATTATCGATAACGATTCGCCGGAGGCCATTGCCGCACA"),
            format!("GGAGTCTGGTGAAGTGTGGGTTATGGGGCAGACTGGGAAA"),
            format!("GAATCCGATAACTGACACCTGCTCTGGCACCGCTCTCATC"),
            format!("AAGCGCGTAGGCGCGGCTTGGCATCTCGGTGTGTGGCCAA"),
            format!("AATTGAAAGGCGCATCTTACTCTTTTCGCTTAAAATCAAA"),
            format!("GGTATAGCCAGAAAGCGTAGTTAATTTCGGCTCCTGCCAA"),
            format!("TCTGTTGTTGCTAACACCGTTAAAGGCGGCGACGGCAACT"),
        ];
        let motifs = greedy_motif_search(&dna, 5)?;
        assert_eq!(
            motifs,
            vec!["GCAGC", "TCATT", "GGAGT", "TCATC", "GCATC", "GCATC", "GGTAT", "GCAAC"]
        );
        Ok(())
    }

    #[test]
    fn test_greedy_motif_search_laplace5() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("GGCGTTCAGGCA"),
            format!("AAGAATCAGTCA"),
            format!("CAAGGAGTTCGC"),
            format!("CACGTCAATCAC"),
            format!("CAATAATATTCG"),
        ];
        let motifs = greedy_motif_search_laplace(&dna, 3, Some(1.0))?;
        assert_eq!(motifs, vec!["TTC", "ATC", "TTC", "ATC", "TTC"]);
        Ok(())
    }
}
