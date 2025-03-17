use std::collections::HashMap;
use std::error::Error;

pub(crate) fn calculate_fm_index(
    bwt_bytes: &[u8],
    char_map: &HashMap<u8, usize>,
    fw_step: usize,
) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let char_count = char_map.len(); // Number of distinct characters
    let mut fm_index = Vec::with_capacity(bwt_bytes.len() / fw_step + 1);

    let mut fw_row = vec![0; char_count];
    for (i, &byte) in bwt_bytes.iter().enumerate() {
        if let Some(&idx) = char_map.get(&byte) {
            fw_row[idx] += 1;
        }

        if i % fw_step == 0 {
            fm_index.push(fw_row.clone());
        }
    }

    Ok(fm_index)
}

#[cfg(test)]
mod tests {
    use crate::bwt::fm::calculate_fm_index;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_fm_index1() -> Result<(), Box<dyn Error>> {
        let bwt_bytes = "abba$aa".as_bytes();
        let char_map = HashMap::from([(b'$', 0), (b'a', 1), (b'b', 2)]);
        let fm_index = calculate_fm_index(&bwt_bytes, &char_map, 1)?;
        assert_eq!(
            fm_index,
            vec![
                vec![0, 1, 0],
                vec![0, 1, 1],
                vec![0, 1, 2],
                vec![0, 2, 2],
                vec![1, 2, 2],
                vec![1, 3, 2],
                vec![1, 4, 2]
            ]
        );
        Ok(())
    }
    #[test]
    fn test_fm_index2() -> Result<(), Box<dyn Error>> {
        let bwt_bytes = "abba$aa".as_bytes();
        let char_map = HashMap::from([(b'$', 0), (b'a', 1), (b'b', 2)]);
        let fm_index = calculate_fm_index(&bwt_bytes, &char_map, 5)?;
        assert_eq!(fm_index, vec![vec![0, 1, 0], vec![1, 3, 2]]);
        Ok(())
    }
}
