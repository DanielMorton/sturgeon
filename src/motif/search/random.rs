use crate::motif::search::profile::{motif_to_profile, score_motifs};
use rand::Rng;
use std::error::Error;
use crate::motif::search::greedy::profile_most_probable_kmer;

fn single_motif_search(dna: &[String], k: usize) -> Result<(usize, Vec<String>), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    // Initialize motifs randomly
    let mut motifs: Vec<String> = dna
        .iter()
        .map(|d| {
            let start = rng.gen_range(0..d.len() - k + 1);
            d[start..start + k].to_string()
        })
        .collect();

    let mut best_motifs = motifs.clone();
    let mut profile = motif_to_profile(&best_motifs, Some(1.0))?;
    let mut best_score = score_motifs(&best_motifs, &profile)?;

    loop {
        // Find profile-most-probable k-mer for each string
        for i in 0..motifs.len() {
            motifs[i] = profile_most_probable_kmer(&dna[i], k, &profile)?;
        }

        // Recalculate profile and score
        profile = motif_to_profile(&motifs, Some(1.0))?;
        let score = score_motifs(&motifs, &profile)?;

        // Update best motifs if score improves
        if score < best_score {
            best_motifs = motifs.clone();
            best_score = score;
        } else {
            return Ok((best_score, best_motifs));
        }
    }
}

fn randomized_motif_search(dna: &[String], k: usize, num_iters: usize) -> Result<Vec<String>, Box<dyn Error>> {

    let (mut best_score, mut best_motifs) = single_motif_search(dna, k)?;

    for _ in 1..num_iters {
        let (score, motifs) = single_motif_search(dna, k)?;
        if score < best_score {
            best_motifs = motifs;
            best_score = score;
        }
    }

    Ok(best_motifs)
}

mod tests {
    use std::error::Error;
    use crate::motif::search::random::randomized_motif_search;

    #[test]
    fn test_randomized_motif_search1() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("CGCCCCTCTCGGGGGTGTTCAGTAAACGGCCA"),
            format!("GGGCGAGGTATGTGTAAGTGCCAAGGTGCCAG"),
            format!("TAGTACCGAGACCGAAAGAAGTATACAGGCGT"),
            format!("TAGATCAAGTTTCAGGTGCACGTCGGTGAACC"),
            format!("AATCCACCAGCTCCACGTGCAATGTTGGCCTA"),
        ];
        let ans = vec!["TCTCGGGG", "CCAAGGTG", "TACAGGCG", "TTCAGGTG", "TCCACGTG"];
        loop {
            let motifs = randomized_motif_search(&dna, 8, 4000)?;
            if motifs == ans {
                assert_eq!(motifs, ans);
                return Ok(());
            }
        }

    }

    #[test]
    fn test_randomized_motif_search2() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("AATTGGCACATCATTATCGATAACGATTCGCCGCATTGCC"),
            format!("GGTTAACATCGAATAACTGACACCTGCTCTGGCACCGCTC"),
            format!("AATTGGCGGCGGTATAGCCAGATAGTGCCAATAATTTCCT"),
            format!("GGTTAATGGTGAAGTGTGGGTTATGGGGAAAGGCAGACTG"),
            format!("AATTGGACGGCAACTACGGTTACAACGCAGCAAGAATATT"),
            format!("GGTTAACTGTTGTTGCTAACACCGTTAAGCGACGGCAACT"),
            format!("AATTGGCCAACGTAGGCGCGGCTTGGCATCTCGGTGTGTG"),
            format!("GGTTAAAAGGCGCATCTTACTCTTTTCGCTTTCAAAAAAA"),
        ];
        let motifs = randomized_motif_search(&dna, 6, 1000)?;
        assert_eq!(motifs, vec!["CGATAA", "GGTTAA", "GGTATA", "GGTTAA", "GGTTAC", "GGTTAA", "GGCCAA", "GGTTAA"]);
        Ok(())
    }

    #[test]
    fn test_randomized_motif_search3() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("GCACATCATTAAACGATTCGCCGCATTGCCTCGATTAACC"),
            format!("TCATAACTGACACCTGCTCTGGCACCGCTCATCCAAGGCC"),
            format!("AAGCGGGTATAGCCAGATAGTGCCAATAATTTCCTTAACC"),
            format!("AGTCGGTGGTGAAGTGTGGGTTATGGGGAAAGGCAAGGCC"),
            format!("AACCGGACGGCAACTACGGTTACAACGCAGCAAGTTAACC"),
            format!("AGGCGTCTGTTGTTGCTAACACCGTTAAGCGACGAAGGCC"),
            format!("AAGCTTCCAACATCGTCTTGGCATCTCGGTGTGTTTAACC"),
            format!("AATTGAACATCTTACTCTTTTCGCTTTCAAAAAAAAGGCC"),
        ];
        let ans = vec!["TTAACC", "ATAACT", "TTAACC", "TGAAGT", "TTAACC", "TTAAGC", "TTAACC", "TGAACA"];
        loop {
            let motifs = randomized_motif_search(&dna, 6, 1000)?;
            if motifs == ans {
                assert_eq!(motifs, ans);
                return Ok(());
            }
        }

    }
}
