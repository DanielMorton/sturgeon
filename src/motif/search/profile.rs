use crate::utils::hamming::hamming_distance;
use crate::utils::{InvalidNucleotidePositionError, DNA, DNA_INDEX};
use std::error::Error;

pub fn motif_to_profile(
    motifs: &[String],
    laplace: Option<f64>,
) -> Result<[Vec<f64>; 4], Box<dyn Error>> {
    let cols = motifs[0].len();

    let mut profile: [Vec<f64>; 4] = std::array::from_fn(|_| vec![laplace.unwrap_or(0.0); cols]);

    for motif in motifs {
        for (j, nucleotide) in motif.chars().enumerate() {
            let i = *DNA_INDEX
                .get(&nucleotide)
                .ok_or_else(|| InvalidNucleotidePositionError::new(nucleotide, j))?;
            profile[i][j] += 1.0;
        }
    }

    let motif_count = motifs.len() as f64;
    // Normalize
    for col in 0..cols {
        for row in 0..4 {
            profile[row][col] /= motif_count;
        }
    }

    Ok(profile)
}

pub fn score_motifs(motifs: &[String], profile: &[Vec<f64>; 4]) -> Result<usize, Box<dyn Error>> {
    let k = motifs[0].len();
    let mut consensus = String::new();

    for j in 0..k {
        let mut max_prob = 0.0;
        let mut consensus_nucleotide = '\0';

        for (i, n) in DNA.iter().enumerate() {
            let prob = profile[i][j];
            if prob > max_prob {
                max_prob = prob;
                consensus_nucleotide = *n;
            }
        }

        consensus.push(consensus_nucleotide as char);
    }

    Ok(motifs
        .iter()
        .map(|motif| hamming_distance(motif, &consensus))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum())
}
