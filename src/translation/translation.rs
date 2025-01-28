use crate::translation::codon::read_codon;
use std::collections::HashMap;
use std::error::Error;

pub fn translate_rna(
    pattern: &str,
    genetic_code: &HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let mut peptide = String::new();
    let mut i = 0;

    while i + 3 <= pattern.len() {
        let codon = &pattern[i..i + 3];
        if let Some(&ref amino_acid) = genetic_code.get(codon) {
            peptide.push_str(&amino_acid);
        } else {
            break;
        }
        i += 3;
    }
    Ok(peptide)
}

pub fn translate_rna_code(pattern: &str) -> Result<String, Box<dyn Error>> {
    let genetic_code = read_codon()?;
    translate_rna(pattern, &genetic_code)
}

mod tests {
    use crate::translation::translation::translate_rna;
    use std::error::Error;

    #[test]
    fn test_debruijn_string1() -> Result<(), Box<dyn Error>> {
        let rna = format!("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA");

        assert_eq!(translate_rna(&rna)?, "MAMAPRTEINSTRING");
        Ok(())
    }
}
