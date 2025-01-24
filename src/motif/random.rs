use crate::motif::greedy::profile_most_probable_kmer;
use crate::motif::profile::{motif_to_profile, score_motifs};
use rand::Rng;
use std::error::Error;

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

mod tests {

}
