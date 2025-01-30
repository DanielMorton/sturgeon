use std::collections::HashSet;
use std::error::Error;
use crate::peptide::peptide::Peptide;
use crate::peptide::score::score_cyclopeptide;

fn trim(leaderboard: HashSet<Peptide>, spectrum: &[usize], n: usize) -> Result<HashSet<Peptide>, Box<dyn Error>> {
    if n >= leaderboard.len() {
        return Ok(leaderboard);
    }

    // Score all peptides
    let mut scored_peptides = leaderboard
        .into_iter()
        .map(|peptide| {
            let score = score_cyclopeptide(&peptide, spectrum)?;
            Ok((peptide, score))
        })
        .collect::<Result<Vec<(_,_)>, Box<dyn Error>>>()?;

    // Sort by score in descending order
    scored_peptides.sort_by(|a, b| b.1.cmp(&a.1));

    // Get the score at position n
    let min_score = if n < scored_peptides.len() {
        scored_peptides[n - 1].1
    } else {
        scored_peptides.last().unwrap().1
    };

    // Keep all peptides with scores >= min_score
    Ok(scored_peptides
        .into_iter()
        .take_while(|(_, score)| *score >= min_score)
        .map(|(peptide, _)| peptide.clone())
        .collect())
}