use crate::peptide::leaderboard::trim;
use crate::peptide::peptide::{expand, parent_mass, Peptide};
use crate::peptide::score::score_cyclopeptide;
use crate::utils::vec_to_count;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;

fn get_convolution_alphabet(
    spectrum: &[usize],
    min_diff: Option<usize>,
    max_diff: Option<usize>,
    max_length: usize,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let convolution = spectral_convolution(spectrum, min_diff, max_diff)?;
    Ok(filter_convolution(&convolution, max_length)?
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>())
}

fn filter_convolution(
    convolution: &[usize],
    max_length: usize,
) -> Result<Vec<usize>, Box<dyn Error>> {
    // For each difference, calculate how many times it should appear
    let diff_counts = vec_to_count(convolution)?;

    let mut sorted_diffs = diff_counts.clone().into_iter().collect::<Vec<(_, _)>>();
    sorted_diffs.sort_by(|a, b| b.1.cmp(&a.1));

    if sorted_diffs.len() < max_length {
        return Ok(convolution.to_owned());
    }

    // Take top max_length differences
    let threshold_count = sorted_diffs[max_length - 1].1;

    // Filter convolution based on frequency threshold
    Ok(convolution
        .iter()
        .filter(|diff| diff_counts.get(diff).is_some_and(|&c| c >= threshold_count))
        .copied()
        .collect())
}

fn spectral_convolution(
    spectrum: &[usize],
    min_diff: Option<usize>,
    max_diff: Option<usize>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut convolution = Vec::new();
    let mut spectrum_sorted = spectrum.to_vec();
    spectrum_sorted.sort();

    // Generate all possible differences
    for i in 0..spectrum_sorted.len() {
        for j in i + 1..spectrum_sorted.len() {
            let diff = spectrum_sorted[j] - spectrum_sorted[i];
            if diff >= min_diff.unwrap_or(1) && diff <= max_diff.unwrap_or(usize::MAX) {
                // Skip zero differences
                convolution.push(diff);
            }
        }
    }

    Ok(convolution)
}

pub fn convolution_cyclopeptide_sequencing(
    spectrum: &[usize],
    m: usize,
    n: usize,
) -> Result<Peptide, Box<dyn Error>> {
    let alphabet = get_convolution_alphabet(spectrum, Some(57), Some(200), m)?;
    let mut leaderboard = HashSet::from([Peptide::new()]);
    let mut leader_peptide = Peptide::new();
    let mut leader_score = score_cyclopeptide(&leader_peptide, spectrum)?;
    let target_mass = parent_mass(spectrum)?;

    while !leaderboard.is_empty() {
        leaderboard = expand(&leaderboard, &alphabet)?;
        let mut to_remove = Vec::new();

        // Remove peptides that are too large
        leaderboard.retain(|peptide| peptide.mass() <= target_mass);

        for peptide in &leaderboard {
            let peptide_mass = peptide.mass();
            match peptide_mass.cmp(&target_mass) {
                Ordering::Greater => to_remove.push(peptide.clone()),
                Ordering::Equal => {
                    let peptide_score = score_cyclopeptide(peptide, spectrum)?;

                    if peptide_score > leader_score {
                        leader_peptide = peptide.clone();
                        leader_score = peptide_score;
                    }
                }
                Ordering::Less => (),
            }
        }

        for peptide in to_remove {
            leaderboard.remove(&peptide);
        }

        leaderboard = trim(leaderboard, spectrum, n)?;
    }

    Ok(leader_peptide)
}

pub fn convolution_cyclopeptide_list(
    spectrum: &[usize],
    m: usize,
    n: usize,
) -> Result<Vec<Peptide>, Box<dyn Error>> {
    let alphabet = get_convolution_alphabet(spectrum, Some(57), Some(200), m)?;
    let mut leaderboard = HashSet::from([Peptide::new()]);
    let mut leader_peptides = vec![];
    let mut leader_score = 0;
    let target_mass = parent_mass(spectrum)?;

    while !leaderboard.is_empty() {
        leaderboard = expand(&leaderboard, &alphabet)?;
        let mut to_remove = Vec::new();

        // Remove peptides that are too large
        leaderboard.retain(|peptide| peptide.mass() <= target_mass);

        for peptide in &leaderboard {
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

        for peptide in to_remove {
            leaderboard.remove(&peptide);
        }

        leaderboard = trim(leaderboard, spectrum, n)?;
    }

    Ok(leader_peptides)
}

mod tests {
    use crate::peptide::convolution::{convolution_cyclopeptide_sequencing, spectral_convolution};
    use std::error::Error;

    #[test]
    fn test_spectral_convolution1() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![0, 137, 186, 323];
        let mut convolution = spectral_convolution(&spectrum, None, None)?;
        convolution.sort();
        assert_eq!(convolution, vec![49, 137, 137, 186, 186, 323]);
        Ok(())
    }

    #[test]
    fn test_convolution_cyclopeptide_sequencing1() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![
            57, 57, 71, 99, 129, 137, 170, 186, 194, 208, 228, 265, 285, 299, 307, 323, 356, 364,
            394, 422, 493,
        ];
        let peptide = convolution_cyclopeptide_sequencing(&spectrum, 20, 60)?;
        assert_eq!(peptide.to_string(), "99-71-137-57-72-57");
        Ok(())
    }
}
