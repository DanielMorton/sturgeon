use std::collections::HashMap;
use std::error::Error;

pub(crate) fn char_counts(
    bwt_bytes: &[u8],
    char_map: &HashMap<u8, usize>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut char_counts = vec![0; char_map.len()];
    for b in bwt_bytes {
        char_counts[*char_map.get(b).unwrap()] += 1;
    }
    Ok(char_counts)
}

pub(crate) fn get_first_col_starts(counts: &[usize]) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut total = 0;
    Ok(counts
        .iter()
        .map(|&c| {
            let fcs = total;
            total += c;
            fcs
        })
        .collect::<Vec<_>>())
}
