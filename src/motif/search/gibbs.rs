use crate::motif::search::profile::{motif_to_profile, score_motifs};
use crate::motif::search::random_kmer::random_kmer;
use rand::Rng;
use std::error::Error;

fn single_gibbs_sample(
    dna: &[String],
    kmer_length: usize,
    iterations: usize,
) -> Result<(usize, Vec<String>), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    // Initialize motifs randomly
    let mut motifs: Vec<String> = dna
        .iter()
        .map(|d| {
            let start = rng.gen_range(0..d.len() - kmer_length + 1);
            d[start..start + kmer_length].to_string()
        })
        .collect();

    let mut best_motifs = motifs.clone();
    let mut profile = motif_to_profile(&best_motifs, Some(1.0))?;
    let mut best_score = score_motifs(&best_motifs, &profile)?;

    for _ in 0..iterations {
        // Reset motifs to best motifs
        motifs = best_motifs.clone();

        // Choose a random motif to replace
        let m = rng.gen_range(0..dna.len());

        // Create sub-motifs without the selected motif
        let sub_motifs: Vec<String> = motifs
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != m)
            .map(|(_, motif)| motif.clone())
            .collect();

        // Create profile from sub-motifs
        profile = motif_to_profile(&sub_motifs, Some(1.0))?;

        // Replace the selected motif with a random k-mer
        motifs[m] = random_kmer(&dna[m], kmer_length, &profile)?;

        // Recalculate profile and score
        let current_profile = motif_to_profile(&motifs, Some(1.0))?;
        let score = score_motifs(&motifs, &current_profile)?;

        // Update best motifs if score improves
        if score < best_score {
            best_motifs = motifs;
            best_score = score;
        }
    }

    Ok((best_score, best_motifs))
}
fn gibbs_sampler(
    dna: &[String],
    kmer_length: usize,
    gibbs_iterations: usize,
    gibbs_runs: usize,
) -> Result<Vec<String>, Box<dyn Error>> {
    let (mut best_score, mut best_motifs) = single_gibbs_sample(dna, kmer_length, gibbs_iterations)?;

    for _ in 1..gibbs_runs {
        let (score, motifs) = single_gibbs_sample(dna, kmer_length, gibbs_iterations)?;
        if score < best_score {
            best_motifs = motifs;
            best_score = score;
        }
    }
    Ok(best_motifs)
}

mod tests {
    use std::error::Error;
    use crate::motif::search::gibbs::gibbs_sampler;

    #[test]
    fn test_gibbs_sampler1() -> Result<(), Box<dyn Error>> {
        let dna = vec![
            format!("CGCCCCTCTCGGGGGTGTTCAGTAAACGGCCA"),
            format!("GGGCGAGGTATGTGTAAGTGCCAAGGTGCCAG"),
            format!("TAGTACCGAGACCGAAAGAAGTATACAGGCGT"),
            format!("TAGATCAAGTTTCAGGTGCACGTCGGTGAACC"),
            format!("AATCCACCAGCTCCACGTGCAATGTTGGCCTA")
        ];
        let ans = vec!["AACGGCCA", "AAGTGCCA", "TAGTACCG", "AAGTTTCA", "ACGTGCAA"];
        loop {
            let motifs = gibbs_sampler(&dna, 8, 100, 1000)?;
            if motifs == ans {
                assert_eq!(
                    motifs,
                    ans
                );
                return Ok(());
            }
        }
    }
}