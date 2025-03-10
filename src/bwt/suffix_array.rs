use crate::bwt::bucket::char_buckets;
use crate::bwt::lms::{
    accurate_lms_sort, build_type_map, guess_lms_sort, induce_sort_l, induce_sort_s,
};
use crate::bwt::summary::summarize_suffix_array;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

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

pub fn suffix_array_induced_sorting<T: Copy + Display + Eq + Hash + Into<usize> + Ord>(
    text_bytes: &[T],
    char_map: &HashMap<T, usize>,
) -> Result<Vec<usize>, Box<dyn Error>> {
    let type_map = build_type_map(text_bytes)?;

    let bucket_sizes = char_buckets(text_bytes, char_map)?;
    let mut guessed_suffix_array = guess_lms_sort(text_bytes, char_map, &bucket_sizes, &type_map)?;
    induce_sort_l(
        &mut guessed_suffix_array,
        text_bytes,
        char_map,
        &bucket_sizes,
        &type_map,
    )?;
    induce_sort_s(
        &mut guessed_suffix_array,
        text_bytes,
        char_map,
        &bucket_sizes,
        &type_map,
    )?;
    let guessed_suffix_array = guessed_suffix_array
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let (summary_string, summary_alphabet_size, summary_suffix_offsets) =
        summarize_suffix_array(text_bytes, &guessed_suffix_array, &type_map)?;

    let summary_suffix_array = make_summary_suffix_array(&summary_string, summary_alphabet_size)?;

    let mut suffix_array = accurate_lms_sort(
        text_bytes,
        char_map,
        &bucket_sizes,
        &summary_suffix_array,
        &summary_suffix_offsets,
    )?;
    induce_sort_l(
        &mut suffix_array,
        text_bytes,
        char_map,
        &bucket_sizes,
        &type_map,
    )?;
    induce_sort_s(
        &mut suffix_array,
        text_bytes,
        char_map,
        &bucket_sizes,
        &type_map,
    )?;

    Ok(suffix_array.into_iter().flatten().collect::<Vec<_>>())
}

pub(crate) fn make_summary_suffix_array(
    summary_string: &[usize],
    summary_alphabet_size: usize,
) -> Result<Vec<usize>, Box<dyn Error>> {
    if summary_alphabet_size == summary_string.len() {
        // Every character appears exactly once, so we can use bucket sort
        let mut summary_suffix_array = vec![usize::MAX; summary_string.len() + 1];

        // Always include the empty suffix at the beginning
        summary_suffix_array[0] = summary_string.len();

        // Direct indexing approach - more efficient than looping
        for (x, &y) in summary_string.iter().enumerate() {
            summary_suffix_array[y + 1] = x;
        }

        Ok(summary_suffix_array)
    } else {
        // More complex case - use recursion
        let mut char_set = summary_string
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        char_set.sort();
        let char_map = char_set
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<_, _>>();
        suffix_array_induced_sorting(summary_string, &char_map)
    }
}

#[cfg(test)]
mod tests {
    use crate::bwt::bucket::char_buckets;
    use crate::bwt::lms::{build_type_map, guess_lms_sort, induce_sort_l, induce_sort_s};
    use crate::bwt::suffix_array::{
        make_summary_suffix_array, suffix_array, suffix_array_induced_sorting,
    };
    use crate::bwt::summary::summarize_suffix_array;
    use std::collections::HashMap;
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

    #[test]
    fn test_make_summary_suffix_array1() -> Result<(), Box<dyn Error>> {
        let cabbage = "cabbage".as_bytes();
        let char_map = (0..7)
            .map(|n| (b'a' + n, n as usize))
            .collect::<HashMap<_, _>>();
        let cabbage_bucket = char_buckets(cabbage, &char_map)?;
        let cabbage_types = build_type_map(cabbage)?;
        let mut guessed_suffix_array =
            guess_lms_sort(cabbage, &char_map, &cabbage_bucket, &cabbage_types)?;
        induce_sort_l(
            &mut guessed_suffix_array,
            cabbage,
            &char_map,
            &cabbage_bucket,
            &cabbage_types,
        )?;
        induce_sort_s(
            &mut guessed_suffix_array,
            cabbage,
            &char_map,
            &cabbage_bucket,
            &cabbage_types,
        )?;
        let guessed_suffix_array = guessed_suffix_array
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let (summary_string, summary_alphabet_size, summary_suffix_offsets) =
            summarize_suffix_array(cabbage, &guessed_suffix_array, &cabbage_types)?;
        let cabbage_summary_suffix_array =
            make_summary_suffix_array(&summary_string, summary_alphabet_size)?;
        assert_eq!(cabbage_summary_suffix_array, vec![3, 2, 0, 1]);
        Ok(())
    }

    #[test]
    fn test_suffix_array_induced_sorting1() -> Result<(), Box<dyn Error>> {
        let text = "cabbage";
        let char_map = (0..7)
            .map(|n| (b'a' + n, n as usize))
            .collect::<HashMap<_, _>>();
        assert_eq!(
            suffix_array_induced_sorting(text.as_bytes(), &char_map)?,
            vec![7, 1, 4, 3, 2, 0, 6, 5]
        );
        Ok(())
    }

    #[test]
    fn test_suffix_array_induced_sorting2() -> Result<(), Box<dyn Error>> {
        let text = "baabaabac";
        let char_map = (0..3)
            .map(|n| (b'a' + n, n as usize))
            .collect::<HashMap<_, _>>();
        assert_eq!(
            suffix_array_induced_sorting(text.as_bytes(), &char_map)?,
            vec![9, 1, 4, 2, 5, 7, 0, 3, 6, 8]
        );
        Ok(())
    }
}
