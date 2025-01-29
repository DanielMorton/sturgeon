use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Peptide {
    sequence: Vec<usize>,
}

impl Peptide {
    fn new() -> Self {
        Peptide {
            sequence: Vec::new(),
        }
    }

    fn from(sequence: &[usize]) -> Self {
        Peptide {
            sequence: sequence.to_vec(),
        }
    }

    fn len(&self) -> usize {
        self.sequence.len()
    }

    fn mass(&self) -> usize {
        self.sequence.iter().sum()
    }

    fn to_string(&self) -> String {
        self.sequence
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join("-")
    }
}

fn expand(
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

fn get_cyclospectrum(peptide: &Peptide) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut spectrum = vec![0]; // Start with mass 0

    // Double the sequence for cyclic subpeptides
    let doubled =
        Peptide::from(&[peptide.sequence.as_slice(), peptide.sequence.as_slice()].concat());

    // Generate all possible subpeptides
    for len in 1..peptide.len() {
        for start in 0..peptide.len() {
            let subpeptide = doubled.sequence[start..start + len].to_vec();
            spectrum.push(subpeptide.iter().sum());
        }
    }
    spectrum.push(peptide.mass());
    spectrum.sort();
    Ok(spectrum)
}

fn is_consistent(peptide: &Peptide, target_spectrum: &[usize]) -> Result<bool, Box<dyn Error>> {
    let peptide_spectrum = get_cyclospectrum(peptide)?;

    // Count frequencies in both spectra
    let mut target_freq = HashMap::new();
    let mut peptide_freq = HashMap::new();

    for &mass in target_spectrum {
        *target_freq.entry(mass).or_insert(0) += 1;
    }

    for &mass in &peptide_spectrum {
        *peptide_freq.entry(mass).or_insert(0) += 1;
    }

    // Check if peptide spectrum frequencies don't exceed target frequencies
    for (&mass, &count) in peptide_freq.iter() {
        if count > *target_freq.get(&mass).unwrap_or(&0) {
            return Ok(false);
        }
    }

    Ok(true)
}

fn parent_mass(spectrum: &[usize]) -> Result<usize, Box<dyn Error>> {
    Ok(*spectrum.iter().max().unwrap())
}

fn cyclopeptide_sequencing(
    spectrum: &[usize],
    amino_masses: &[usize],
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut candidate_peptides = HashSet::new();
    candidate_peptides.insert(Peptide::new());
    let mut final_peptides = Vec::new();
    let target_mass = parent_mass(spectrum)?;

    while !candidate_peptides.is_empty() {
        // Expand candidates
        let candidates = expand(&candidate_peptides, amino_masses)?;
        candidate_peptides.clear();

        for peptide in candidates {
            let mass = peptide.mass();
            if mass == target_mass {
                if get_cyclospectrum(&peptide)? == spectrum {
                    final_peptides.push(peptide.to_string());
                }
            } else if mass < target_mass && is_consistent(&peptide, spectrum)? {
                candidate_peptides.insert(peptide);
            }
        }
    }
    Ok(final_peptides)
}

mod tests {
    use std::error::Error;
    use crate::peptide::mass::make_mass_vector;
    use crate::peptide::peptide::cyclopeptide_sequencing;

    #[test]
    fn test_cyclopeptide_sequencing1() -> Result<(), Box<dyn Error>> {
        let spectrum = vec![0, 113, 128, 186, 241, 299, 314, 427];
        let mut ans = vec!["186-128-113", "186-113-128", "128-186-113", "128-113-186", "113-186-128", "113-128-186"];
        ans.sort();
        let amino_masses = make_mass_vector()?;
        let  mut cyclo = cyclopeptide_sequencing(&spectrum, &amino_masses)?;
        cyclo.sort();
        assert_eq!(
            cyclo,
            ans
        );
        Ok(())
    }
}