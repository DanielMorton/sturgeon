use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub(crate) fn read_codon() -> Result<HashMap<String, String>, Box<dyn Error>> {
    Ok(fs::read_to_string("RNA_codon_table.txt")?
        .split('\n')
        .map(|codon| codon.split(' ').collect::<Vec<_>>())
        .filter(|codon| codon.len() == 2)
        .map(|codon| (codon[0].to_owned(), codon[1].to_owned()))
        .collect::<HashMap<_, _>>())
}

pub(crate) fn reverse_codon(
    codon: &HashMap<String, String>,
) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    Ok(codon
        .iter()
        .fold(HashMap::new(), |mut acc, (rna, protein)| {
            acc.entry(protein.clone()).or_default().push(rna.clone());
            acc
        }))
}
