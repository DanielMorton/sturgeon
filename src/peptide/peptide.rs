use crate::utils::vec_to_count;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Peptide {
    pub(crate) sequence: Vec<usize>,
}

impl Peptide {
    pub fn new() -> Self {
        Peptide {
            sequence: Vec::new(),
        }
    }

    pub fn from(sequence: &[usize]) -> Self {
        Peptide {
            sequence: sequence.to_vec(),
        }
    }

    pub fn from_string(text: &str, masses: &HashMap<char, usize>) -> Self {
        let sequence = text
            .chars()
            .map(|c| *masses.get(&c).unwrap())
            .collect::<Vec<_>>();
        Peptide { sequence }
    }

    pub fn len(&self) -> usize {
        self.sequence.len()
    }

    pub fn mass(&self) -> usize {
        self.sequence.iter().sum()
    }

    pub fn to_string(&self) -> String {
        self.sequence
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join("-")
    }

    pub fn get_cyclospectrum(&self) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut spectrum = vec![0]; // Start with mass 0

        // Double the sequence for cyclic subpeptides
        let doubled = Peptide::from(&[self.sequence.as_slice(), self.sequence.as_slice()].concat());

        // Generate all possible subpeptides
        for len in 1..self.len() {
            for start in 0..self.len() {
                let subpeptide = doubled.sequence[start..start + len].to_vec();
                spectrum.push(subpeptide.iter().sum());
            }
        }
        spectrum.push(self.mass());
        spectrum.sort();
        Ok(spectrum)
    }

    pub fn get_linspectrum(&self) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut spectrum = vec![0]; // Start with mass 0

        // Generate all possible subpeptides
        for len in 1..self.len() {
            for start in 0..=self.len() - len {
                let subpeptide = self.sequence[start..start + len].to_vec();
                spectrum.push(subpeptide.iter().sum());
            }
        }
        spectrum.sort();
        Ok(spectrum)
    }
}

pub fn expand(
    peptides: &HashSet<Peptide>,
    amino_masses: &[usize],
) -> Result<HashSet<Peptide>, Box<dyn Error>> {
    let mut expanded = HashSet::new();

    for peptide in peptides {
        for &mass in amino_masses {
            let mut new_peptide = peptide.clone();
            new_peptide.sequence.push(mass);
            expanded.insert(new_peptide);
        }
    }
    Ok(expanded)
}

fn is_consistent(
    peptide: &Peptide,
    target_freq: &HashMap<usize, usize>,
) -> Result<bool, Box<dyn Error>> {
    let peptide_spectrum = peptide.get_linspectrum()?;

    // Count frequencies in both spectra
    let peptide_freq = vec_to_count(&peptide_spectrum)?;

    // Check if peptide spectrum frequencies don't exceed target frequencies
    for (&mass, &count) in peptide_freq.iter() {
        if count > *target_freq.get(&mass).unwrap_or(&0) {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn parent_mass(spectrum: &[usize]) -> Result<usize, Box<dyn Error>> {
    Ok(*spectrum.iter().max().unwrap())
}

pub fn cyclopeptide_sequencing(
    spectrum: &[usize],
    amino_masses: &[usize],
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut candidate_peptides = HashSet::new();
    candidate_peptides.insert(Peptide::new());
    let mut final_peptides = Vec::new();
    let target_mass = parent_mass(spectrum)?;
    let target_freq = vec_to_count(spectrum)?;

    while !candidate_peptides.is_empty() {
        // Expand candidates
        let candidates = expand(&candidate_peptides, amino_masses)?;
        candidate_peptides.clear();

        for peptide in candidates {
            let mass = peptide.mass();
            if mass == target_mass {
                if &peptide.get_cyclospectrum()? == spectrum {
                    final_peptides.push(peptide.to_string());
                }
            } else if mass < target_mass && is_consistent(&peptide, &target_freq)? {
                candidate_peptides.insert(peptide);
            }
        }
    }
    Ok(final_peptides)
}

mod tests {
    use crate::peptide::mass::make_mass_vector;
    use crate::peptide::peptide::cyclopeptide_sequencing;
    use std::error::Error;

    #[test]
    fn test_cyclopeptide_sequencing1() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![0, 113, 128, 186, 241, 299, 314, 427];
        let mut ans = vec![
            "186-128-113",
            "186-113-128",
            "128-186-113",
            "128-113-186",
            "113-186-128",
            "113-128-186",
        ];
        ans.sort();
        let amino_masses = make_mass_vector()?;
        let mut cyclo = cyclopeptide_sequencing(&spectrum, &amino_masses)?;
        cyclo.sort();
        assert_eq!(cyclo, ans);
        Ok(())
    }
}
