use crate::translation::codon::read_codon;
use crate::translation::translation::translate_rna;
use crate::utils::{dna_complement, dna_to_rna};
use std::error::Error;

fn find_peptide_encodings(text: &str, peptide: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let genetic_code = read_codon()?;
    let peptide_len = peptide.len() * 3;
    let mut result = Vec::new();

    for i in 0..=text.len() - peptide_len {
        let dna_substring = &text[i..i + peptide_len];
        let rc_substring = dna_complement(dna_substring)?;

        // Check forward strand
        let rna = dna_to_rna(dna_substring)?;
        let translated_peptide = translate_rna(&rna, &genetic_code)?;
        if translated_peptide == peptide {
            result.push(dna_substring.to_owned());
        }

        // Check reverse complement
        let rc_rna = dna_to_rna(&rc_substring)?;
        let rc_translated_peptide = translate_rna(&rc_rna, &genetic_code)?;
        if rc_translated_peptide == peptide {
            result.push(dna_substring.to_owned());
        }
    }

    Ok(result)
}

mod tests {
    use crate::translation::peptide::find_peptide_encodings;
    use std::error::Error;

    #[test]
    fn test_debruijn_string1() -> Result<(), Box<dyn Error>> {
        let dna = "ATGGCCATGGCCCCCAGAACTGAGATCAATAGTACCCGTATTAACGGGTGA";
        let peptide = "MA";

        assert_eq!(
            find_peptide_encodings(dna, peptide)?,
            vec!["ATGGCC", "GGCCAT", "ATGGCC"]
        );
        Ok(())
    }
}
