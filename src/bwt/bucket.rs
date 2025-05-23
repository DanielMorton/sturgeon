use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

pub(crate) fn char_buckets<T: Eq + Hash>(
    text_bytes: &[T],
    char_map: &HashMap<T, usize>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut char_counts = vec![0; char_map.len()];
    for b in text_bytes {
        if char_map.contains_key(b) {
            char_counts[*char_map.get(b).unwrap()] += 1;
        }
    }
    Ok(char_counts)
}

pub(crate) fn find_bucket_tails(bucket_sizes: &[usize]) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut offset = 0;
    Ok(bucket_sizes
        .iter()
        .map(|&s| {
            offset += s;
            offset
        })
        .collect::<Vec<_>>())
}

pub(crate) fn find_bucket_heads(bucket_sizes: &[usize]) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut offset = 1;
    Ok(bucket_sizes
        .iter()
        .map(|&s| {
            let head = offset;
            offset += s;
            head
        })
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use crate::bwt::bucket::char_buckets;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_char_buckets1() -> Result<(), Box<dyn Error>> {
        let char_map = (0..7)
            .map(|n| (b'a' + n, n as usize))
            .collect::<HashMap<_, _>>();
        assert_eq!(
            char_buckets("cabbage".as_bytes(), &char_map)?,
            vec![2, 2, 1, 0, 1, 0, 1]
        );
        Ok(())
    }
}
