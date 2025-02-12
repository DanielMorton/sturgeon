use crate::utils::DNA_INDEX;
use rand::prelude::SliceRandom;
use std::error::Error;

pub fn random_kmer(text: &str, k: usize, profile: &[Vec<f64>]) -> Result<String, Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    // Calculate probabilities for each k-mer
    let probs: Vec<f64> = (0..=text.len() - k)
        .map(|i| {
            let kmer = &text[i..i + k];
            kmer.chars().enumerate().fold(1.0, |prob, (j, b)| {
                prob * profile[*DNA_INDEX.get(&b).unwrap()][j]
            })
        })
        .collect();

    let sel = probs
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .choose_weighted(&mut rng, |(_, &p)| p)?
        .0;

    Ok(text[sel..sel + k].to_owned())
}
