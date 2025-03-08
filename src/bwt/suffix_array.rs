use std::error::Error;

pub fn suffix_array(text: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let text_bytes = text.as_bytes();
    suffix_array_bytes(text_bytes)
}

// More efficient implementation using suffix ranks and doubling algorithm
pub(crate) fn suffix_array_bytes(text_bytes: &[u8]) -> Result<Vec<usize>, Box<dyn Error>> {
    let n = text_bytes.len();

    let mut suffix = (0..n).collect::<Vec<usize>>();
    let mut rank = text_bytes.iter().map(|&t| t as i32).collect::<Vec<_>>();
    let mut tmp = vec![0; n];

    // Sort suffixes by initial character
    suffix.sort_by_key(|&i| rank[i]);

    // Doubling algorithm: iterate with increasing substring lengths
    let mut k = 1;
    while k < n {
        // Compute temporary ranks for the next iteration
        let mut r = 0;
        let mut prev_rank = rank[suffix[0]];
        tmp[suffix[0]] = 0;

        for i in 1..n {
            if rank[suffix[i]] == prev_rank
                && suffix[i] + k < n
                && suffix[i - 1] + k < n
                && rank[suffix[i] + k / 2] == rank[suffix[i - 1] + k / 2]
            {
                tmp[suffix[i]] = r;
            } else {
                r += 1;
                tmp[suffix[i]] = r;
                prev_rank = rank[suffix[i]];
            }
        }

        // Update ranks
        rank = tmp.clone();

        // If all suffixes have unique ranks, we're done
        if rank[suffix[n - 1]] == n as i32 - 1 {
            break;
        }

        // Sort by pair of ranks
        suffix.sort_by(|&a, &b| {
            if rank[a] != rank[b] {
                return rank[a].cmp(&rank[b]);
            }

            let rank_a_k = if a + k < n { rank[a + k] } else { -1 };
            let rank_b_k = if b + k < n { rank[b + k] } else { -1 };

            rank_a_k.cmp(&rank_b_k)
        });

        k *= 2;
    }

    Ok(suffix)
}

#[cfg(test)]
mod tests {
    use crate::bwt::suffix_array::suffix_array;
    use std::error::Error;

    #[test]
    fn test_suffix_array1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            suffix_array("AACGATAGCGGTAGA$")?,
            vec![15, 14, 0, 1, 12, 6, 4, 2, 8, 13, 3, 7, 9, 10, 11, 5]
        );
        Ok(())
    }

    #[test]
    fn test_suffix_array2() -> Result<(), Box<dyn Error>> {
        assert_eq!(suffix_array("AATCAATC$")?, vec![8, 4, 0, 5, 1, 7, 3, 6, 2]);
        Ok(())
    }

    #[test]
    fn test_suffix_array3() -> Result<(), Box<dyn Error>> {
        assert_eq!(suffix_array("ATCG$")?, vec![4, 0, 2, 3, 1]);
        Ok(())
    }

    #[test]
    fn test_suffix_array4() -> Result<(), Box<dyn Error>> {
        assert_eq!(suffix_array("AAACA$")?, vec![5, 4, 0, 1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_suffix_array5() -> Result<(), Box<dyn Error>> {
        assert_eq!(suffix_array("ABCFED$")?, vec![6, 0, 1, 2, 5, 4, 3]);
        Ok(())
    }
}
