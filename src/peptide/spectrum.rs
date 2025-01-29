use std::collections::HashMap;
use std::error::Error;
use crate::peptide::mass::read_masses;
use crate::peptide::peptide::get_subpeptides;

fn calculate_peptide_mass(peptide: &str, masses: &HashMap<char, usize>) -> Result<usize, Box<dyn Error>> {
    Ok(peptide.chars().map(|c| masses.get(&c).unwrap()).sum())
}

pub fn generate_spectrum(peptide: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let masses = read_masses()?;
    let subpeptides = get_subpeptides(peptide)?;
    println!("{:?}", subpeptides);

    // Calculate masses for each subpeptide
    let mut spectrum = subpeptides
        .iter()
        .map(|subpeptide| calculate_peptide_mass(subpeptide, &masses))
        .collect::<Result<Vec<_>, _>>()?;

    // Sort the spectrum
    spectrum.sort();
    Ok(spectrum)
}

mod tests {
    use std::error::Error;
    use crate::peptide::spectrum::generate_spectrum;

    #[test]
    fn test_debruijn_string1() -> Result<(), Box<dyn Error>> {
        let peptide = "LEQN";

        assert_eq!(generate_spectrum(peptide)?, vec![0, 113, 114, 128, 129, 227, 242, 242, 257, 355, 356, 370, 371, 484]);
        Ok(())
    }
}