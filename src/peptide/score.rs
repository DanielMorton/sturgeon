use crate::peptide::peptide::Peptide;
use crate::utils::vec_to_count;
use std::cmp::min;
use std::error::Error;

pub fn score_cyclopeptide(peptide: &Peptide, spectrum: &[usize]) -> Result<usize, Box<dyn Error>> {
    // Get theoretical spectrum for the peptide
    let theoretical_spectrum = peptide.get_cyclospectrum()?;

    // Create frequency maps for both spectra
    let theoretical_freq = vec_to_count(&theoretical_spectrum)?;
    let experimental_freq = vec_to_count(spectrum)?;

    // Calculate score based on shared masses
    let mut score = 0;
    for (&mass, &theoretical_count) in &theoretical_freq {
        if let Some(&experimental_count) = experimental_freq.get(&mass) {
            score += min(theoretical_count, experimental_count);
        }
    }

    Ok(score)
}

pub fn score_linpeptide(peptide: &Peptide, spectrum: &[usize]) -> Result<usize, Box<dyn Error>> {
    // Get theoretical spectrum for the peptide
    let theoretical_spectrum = peptide.get_linspectrum()?;

    // Create frequency maps for both spectra
    let theoretical_freq = vec_to_count(&theoretical_spectrum)?;
    let experimental_freq = vec_to_count(spectrum)?;

    // Calculate score based on shared masses
    let mut score = 0;
    for (&mass, &theoretical_count) in &theoretical_freq {
        if let Some(&experimental_count) = experimental_freq.get(&mass) {
            score += min(theoretical_count, experimental_count);
        }
    }

    Ok(score)
}
#[cfg(test)]
mod tests {
    use crate::peptide::mass::read_masses;
    use crate::peptide::peptide::Peptide;
    use crate::peptide::score::score_cyclopeptide;
    use std::error::Error;

    #[test]
    fn test_score_cyclopeptide1() -> Result<(), Box<dyn Error>> {
        let text = "NQEL";
        let amino_masses = read_masses()?;
        let peptide = Peptide::from_string(text, &amino_masses);
        let spectrum = vec![0, 99, 113, 114, 128, 227, 257, 299, 355, 356, 370, 371, 484];
        assert_eq!(score_cyclopeptide(&peptide, &spectrum)?, 11);
        Ok(())
    }
}
