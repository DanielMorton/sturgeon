#![allow(dead_code)]
use std::error::Error;

pub fn get_subpeptides(peptide: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut subpeptides = vec![String::new()]; // Empty string for mass 0
    let doubled = peptide.repeat(2);

    // Get all possible subpeptides of different lengths
    for length in 1..=peptide.len() - 1 {
        for start in 0..peptide.len() {
            let subpeptide = doubled[start..start + length].to_owned();
            subpeptides.push(subpeptide);
        }
    }
    subpeptides.push(peptide.to_owned());
    Ok(subpeptides)
}

fn count_peptides(target_mass: usize, aa_masses: &[usize]) -> Result<usize, Box<dyn Error>> {
    let min_mass = *aa_masses.iter().min().unwrap();

    // If target mass is smaller than smallest amino acid mass
    if target_mass < min_mass {
        return panic!("Target mass must be positive");
    }

    // dp[i] represents number of peptides with mass i
    let mut dp = vec![0; (target_mass + 1)];
    dp[0] = 1; // Base case: empty peptide (mass 0) counts as 1 way

    // For each possible mass
    for current_mass in 1..=target_mass {
        // Try adding each amino acid
        for &amino_mass in aa_masses {
            // If we can add this amino acid without exceeding target mass
            if amino_mass <= current_mass {
                let prev_mass = (current_mass - amino_mass);
                dp[current_mass] += dp[prev_mass];
            }
        }
    }

    Ok(dp[target_mass])
}

mod tests {
    use crate::peptide::aa_peptide::count_peptides;
    use crate::peptide::mass::make_mass_vector;
    use std::error::Error;

    #[test]
    fn test_debruijn_string1() -> Result<(), Box<dyn Error>> {
        let mass = 1024;
        let masses = make_mass_vector()?;

        assert_eq!(count_peptides(mass, &masses)?, 14712706211);
        Ok(())
    }
}
