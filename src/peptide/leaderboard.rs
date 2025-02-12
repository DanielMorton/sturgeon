use crate::peptide::peptide::{expand, parent_mass, Peptide};
use crate::peptide::score::{score_cyclopeptide, score_linpeptide};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;

pub fn leaderboard_cyclopeptide_sequencing(
    spectrum: &[usize],
    amino_masses: &[usize],
    n: usize,
) -> Result<Peptide, Box<dyn Error>> {
    let mut leaderboard = HashSet::new();
    leaderboard.insert(Peptide::new());
    let mut leader_peptide = Peptide::new();
    let mut leader_score = 0;
    let target_mass = parent_mass(spectrum)?;

    while !leaderboard.is_empty() {
        // Expand candidates
        leaderboard = expand(&leaderboard, amino_masses)?;
        let mut to_remove = Vec::new();

        // Check each peptide
        for peptide in leaderboard.iter() {
            let peptide_mass = peptide.mass();
            match peptide_mass.cmp(&target_mass) {
                Ordering::Greater => to_remove.push(peptide.clone()),
                Ordering::Equal => {
                    let score = score_cyclopeptide(peptide, spectrum)?;
                    if score > leader_score {
                        leader_peptide = peptide.clone();
                        leader_score = score;
                    }
                }
                Ordering::Less => (),
            }
        }

        // Remove peptides that exceed target mass
        for peptide in to_remove {
            leaderboard.remove(&peptide);
        }

        // Trim leaderboard to keep top N scoring peptides
        leaderboard = trim(leaderboard, spectrum, n)?;
    }

    Ok(leader_peptide)
}

pub fn leaderboard_cyclopeptide_list(
    spectrum: &[usize],
    amino_masses: &[usize],
    n: usize,
) -> Result<Vec<Peptide>, Box<dyn Error>> {
    let mut leaderboard = HashSet::new();
    leaderboard.insert(Peptide::new());
    let mut leader_peptides = vec![];
    let mut leader_score = 0;
    let target_mass = parent_mass(spectrum)?;

    while !leaderboard.is_empty() {
        // Expand candidates
        leaderboard = expand(&leaderboard, amino_masses)?;
        let mut to_remove = Vec::new();

        // Check each peptide
        for peptide in leaderboard.iter() {
            let peptide_mass = peptide.mass();
            match peptide_mass.cmp(&target_mass) {
                Ordering::Greater => to_remove.push(peptide.clone()),
                Ordering::Equal => {
                    let peptide_score = score_cyclopeptide(peptide, spectrum)?;
                    match peptide_score.cmp(&leader_score) {
                        Ordering::Greater => {
                            leader_peptides.clear();
                            leader_peptides.push(peptide.clone());
                            leader_score = peptide_score;
                        }
                        Ordering::Equal => leader_peptides.push(peptide.clone()),
                        Ordering::Less => (),
                    }
                    to_remove.push(peptide.clone());
                }
                Ordering::Less => (),
            }
        }

        // Remove peptides that exceed target mass
        for peptide in to_remove {
            leaderboard.remove(&peptide);
        }

        // Trim leaderboard to keep top N scoring peptides
        leaderboard = trim(leaderboard, spectrum, n)?;
    }

    Ok(leader_peptides)
}

pub fn trim(
    leaderboard: HashSet<Peptide>,
    spectrum: &[usize],
    n: usize,
) -> Result<HashSet<Peptide>, Box<dyn Error>> {
    if n >= leaderboard.len() {
        return Ok(leaderboard);
    }

    // Score all peptides
    let mut scored_peptides = leaderboard
        .into_iter()
        .map(|peptide| {
            let score = score_linpeptide(&peptide, spectrum)?;
            Ok((peptide, score))
        })
        .collect::<Result<Vec<(_, _)>, Box<dyn Error>>>()?;

    // Sort by score in descending order
    scored_peptides.sort_by(|a, b| b.1.cmp(&a.1));

    // Get the score at position n
    let min_score = scored_peptides[n - 1].1;

    // Keep all peptides with scores >= min_score
    Ok(scored_peptides
        .into_iter()
        .take_while(|(_, score)| *score >= min_score)
        .map(|(peptide, _)| peptide.clone())
        .collect())
}
#[cfg(test)]
mod tests {
    use crate::peptide::leaderboard::leaderboard_cyclopeptide_sequencing;
    use crate::peptide::make_mass_vector;
    use crate::peptide::peptide::Peptide;
    use std::error::Error;

    #[test]
    fn test_leaderboard_cyclopeptide_sequencing1() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![0, 71, 113, 129, 147, 200, 218, 260, 313, 331, 347, 389, 460];
        let amino_masses = make_mass_vector()?;
        let peptide = leaderboard_cyclopeptide_sequencing(&spectrum, &amino_masses, 10)?;
        assert_eq!(peptide, Peptide::from(&vec![113, 147, 71, 129]));
        Ok(())
    }
}
